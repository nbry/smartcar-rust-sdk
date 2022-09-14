//! Everything needed for Smartcar Connect and getting tokens

use crate::helpers::{format_flag_query, get_connect_url, get_oauth_url};
use crate::request::{get_basic_b64_auth_header, HttpVerb, MultiQuery, SmartcarRequestBuilder};
use crate::response::{Access, Meta};
use crate::ScopeBuilder;
use crate::{error, request};

use std::{collections::HashMap, env};

/// Pass in options to build a Smartcar Connect URL.
///
/// [Info about Smartcar Connect](https://smartcar.com/docs/api/#smartcar-connect)
pub struct AuthUrlOptionsBuilder {
    pub force_prompt: Option<bool>,
    pub state: Option<String>,
    pub make_bypass: Option<String>,
    pub single_select: Option<bool>,
    pub single_select_by_vin: Option<String>,
    pub flags: Option<HashMap<String, String>>,
}

impl AuthUrlOptionsBuilder {
    pub fn new() -> AuthUrlOptionsBuilder {
        AuthUrlOptionsBuilder {
            force_prompt: None,
            state: None,
            make_bypass: None,
            single_select_by_vin: None,
            single_select: None,
            flags: None,
        }
    }

    /// Set the behavior of the approval dialog displayed to the user
    ///
    /// `true` -> only display approval dialog if user has not previously approved
    /// `false` -> ensure approval dialog is always shown
    ///
    /// [Info about Smartcar Connect](https://smartcar.com/docs/api/#smartcar-connect)
    pub fn set_force_prompt(mut self, enabled: bool) -> Self {
        self.force_prompt = Some(enabled);
        self
    }

    /// Set a value as a query parameter in the redirect_uri back to your application.
    /// This value is often used to identify a user and/or prevent cross-site request forgery.
    ///
    /// [Info about Smartcar Connect ](https://smartcar.com/docs/api/#smartcar-connect)
    pub fn set_state(mut self, state: String) -> Self {
        self.state = Some(state);
        self
    }

    /// Bypass the car brand selection screen.
    ///
    /// Valid names can be found [here](https://smartcar.com/docs/api/#makes)
    ///
    /// [Info about Smartcar Connect ](https://smartcar.com/docs/api/#smartcar-connect)
    /// [Info about Brand Select](https://smartcar.com/docs/api/#brand-select)
    pub fn set_make_bypass(mut self, make: String) -> Self {
        self.make_bypass = Some(make);
        self
    }

    /// Only allow user to select a single vehicle.
    ///
    /// Valid names can be found [here](https://smartcar.com/docs/api/#makes)
    ///
    /// [Info about Smartcar Connect ](https://smartcar.com/docs/api/#smartcar-connect)
    /// [Info about Single Select](https://smartcar.com/docs/api/#single-select)
    pub fn set_single_select(mut self, enabled: bool) -> Self {
        self.single_select = Some(enabled);
        self
    }

    /// Only allow user to select a single vehicle with a specific vin.
    ///
    /// Valid names can be found [here](https://smartcar.com/docs/api/#makes)
    ///
    /// [Info about Smartcar Connect ](https://smartcar.com/docs/api/#smartcar-connect)
    /// [Info about Single Select](https://smartcar.com/docs/api/#single-select)
    pub fn set_single_select_by_vin(mut self, vin: String) -> Self {
        self.single_select_by_vin = Some(vin);
        self
    }

    /// Set flags that your application has early access to
    ///
    /// [Info about Smartcar Connect ](https://smartcar.com/docs/api/#smartcar-connect)
    /// [Info about Flags](https://smartcar.com/docs/api/#flags)
    pub fn set_flags(mut self, flags: &HashMap<String, String>) -> Self {
        self.flags = Some(flags.to_owned());
        self
    }
}

impl MultiQuery for AuthUrlOptionsBuilder {
    fn vectorize(&self) -> Vec<(String, String)> {
        let mut query_string = Vec::new();

        if let Some(enabled) = self.force_prompt {
            if enabled == true {
                query_string.push(("approval_prompt".to_string(), "force".to_string()));
            }
        };

        if let Some(state) = &self.state {
            query_string.push(("state".to_string(), state.to_owned()));
        }

        if let Some(make) = &self.make_bypass {
            query_string.push(("make".to_string(), make.to_owned()));
        }

        if let Some(flags) = &self.flags {
            let flag_query = format_flag_query(flags);
            query_string.push(("flag".to_string(), flag_query.to_owned()));
        }

        match &self.single_select_by_vin {
            Some(vin) => {
                query_string.push(("single_select_vin".to_string(), vin.to_owned()));
                query_string.push(("single_select".to_string(), "true".to_string()));
            }
            None => {
                if let Some(enabled) = &self.single_select {
                    query_string.push(("single_select".to_string(), enabled.to_string()));
                }
            }
        }
        query_string
    }
}

#[test]
fn get_auth_url_options_query_build() {
    let options = AuthUrlOptionsBuilder::new()
        .set_make_bypass("mercedes".to_string())
        .set_state("no-michael-no-no-michael".to_string())
        .set_single_select_by_vin("THATISSONOTRIGHT".to_string())
        .set_force_prompt(true);

    let query = options.vectorize();

    let expecting = vec![
        ("approval_prompt".to_string(), "force".to_string()),
        ("state".to_string(), "no-michael-no-no-michael".to_string()),
        ("make".to_string(), "mercedes".to_string()),
        (
            "single_select_vin".to_string(),
            "THATISSONOTRIGHT".to_string(),
        ),
        ("single_select".to_string(), "true".to_string()),
    ];

    // O(n^2)... "shrugs"
    assert!(query.iter().all(|q| expecting.contains(q)));
}

/// Smartcar OAuth client for your application
///
/// Vist the [Smartcar Developer Portal](https://developer.smartcar.com)
/// to get these fields.
///
/// Login/Signup for a Smartcar account here [here](https://smartcar.com/subscribe)
#[derive(Debug)]
pub struct AuthClient {
    /// The application’s unique identifier, obtained
    pub client_id: String,

    /// The application secret identfier. If forgotten, it must be regenerated in the dashboard.
    pub client_secret: String,

    /// The URI a user will be redirected to after authorization.
    /// This value must match one of the redirect URIs set in the
    /// credentials tab of the dashboard.
    pub redirect_uri: String,

    /// Launch the Smartcar auth flow in test mode
    pub test_mode: bool,
}

impl AuthClient {
    /// Create an AuthClient instance from environment variables.
    /// This is the preferred way to create an AuthClient Instance.
    pub fn from_env(test_mode: bool) -> AuthClient {
        let client_id = env::var("SMARTCAR_CLIENT_ID")
            .expect("SMARTCAR_CLIENT_ID environment variable not set");

        let client_secret = env::var("SMARTCAR_CLIENT_SECRET")
            .expect("SMARTCAR_CLIENT_SECRET envionment variable not set");

        let redirect_uri = env::var("SMARTCAR_REDIRECT_URI")
            .expect("SMARTCAR_REDIRECT_URI envionment variable not set");

        AuthClient {
            client_id,
            client_secret,
            redirect_uri,
            test_mode,
        }
    }

    pub fn new(
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
        test_mode: bool,
    ) -> AuthClient {
        AuthClient {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            test_mode,
        }
    }

    /// Generate the Smartcar Connect URL, which will allow your userse to securely
    /// grant your application permissions to interact with their vehicle.
    ///
    /// [Info about Smartcar Connect](https://smartcar.com/docs/api/#smartcar-connect)
    pub fn get_auth_url(
        &self,
        scope: &ScopeBuilder,
        options: Option<&AuthUrlOptionsBuilder>,
    ) -> String {
        let mut url = get_connect_url();

        url.push_str("/oauth/authorize?scope=");
        url.push_str(&scope.query_value);
        url.push_str("&response_type=code&");
        url.push_str(&self.multi_query());

        if let Some(opt) = options {
            let options_query = opt.multi_query();
            if options_query.len() > 0 {
                url.push_str("&");
                url.push_str(&options_query);
            }
        }

        if !url.contains("approval_prompt") {
            url.push_str("&approval_prompt=auto");
        };

        url.replace(" ", "%20")
    }

    /// Exhange your oauth code for an access token
    ///
    /// [Info about auth code exchange](https://smartcar.com/api#auth-code-exchange)
    pub async fn exchange_code(&self, code: &str) -> Result<(Access, Meta), error::Error> {
        let form = HashMap::from([
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &self.redirect_uri),
        ]);

        let (res, meta) = SmartcarRequestBuilder::new(&get_oauth_url(), HttpVerb::POST)
            .add_header(
                "Authorization",
                &request::get_basic_b64_auth_header(&self.client_id, &self.client_secret),
            )
            .add_header("content_type", "application/x-www-form-urlencoded")
            .add_form(form)
            .send()
            .await?;

        let data = res.json::<Access>().await?;

        Ok((data, meta))
    }

    /// Use your refresh token to get a new set of tokens
    ///
    /// [Info about refresh token exchange](https://smartcar.com/api#refresh-token-exchange)
    pub async fn exchange_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(Access, Meta), error::Error> {
        let form = HashMap::from([
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ]);

        let (res, meta) = SmartcarRequestBuilder::new(&get_oauth_url(), HttpVerb::POST)
            .add_header(
                "Authorization",
                &get_basic_b64_auth_header(&self.client_id, &self.client_secret),
            )
            .add_header("content_type", "application/x-www-form-urlencoded")
            .add_form(form)
            .send()
            .await?;

        let data = res.json::<Access>().await?;

        Ok((data, meta))
    }
}

impl MultiQuery for AuthClient {
    fn vectorize(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();

        query.push(("client_id".to_string(), self.client_id.to_owned()));
        query.push(("client_secret".to_string(), self.client_secret.to_owned()));
        query.push(("redirect_uri".to_string(), self.redirect_uri.to_owned()));

        if self.test_mode {
            query.push(("mode".to_string(), "test".to_string()));
        }

        query
    }
}

#[test]
fn get_auth_url() {
    let ac = AuthClient::new("test-client-id", "test-client-secret", "test.com", true);
    let scope = ScopeBuilder::with_all_permissions();
    let options = AuthUrlOptionsBuilder::new();
    let auth_url = ac.get_auth_url(&scope, Some(&options));

    let expecting = String::from("https://connect.smartcar.com/oauth/authorize?scope=read_engine_oil%20read_battery%20read_charge%20control_charge%20read_thermometer%20read_fuel%20read_location%20control_security%20read_odometer%20read_tires%20read_vehicle_info%20read_vin&response_type=code&client_id=test-client-id&client_secret=test-client-secret&redirect_uri=test.com&mode=test&approval_prompt=auto");
    assert_eq!(auth_url, expecting);
}

#[test]
#[should_panic]
fn create_auth_client_without_env_variables() {
    AuthClient::from_env(true);
}
