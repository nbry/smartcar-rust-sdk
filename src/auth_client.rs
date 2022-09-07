use crate::helpers::{format_flag_query, get_connect_url, get_oauth_url};
use crate::request::{MultiQuery, SmartcarRequestBuilder};
use crate::response::meta::Meta;
use crate::response::Access;
use crate::ScopeBuilder;
use crate::{error, request};

use std::{collections::HashMap, env};

pub struct AuthUrlOptionsBuilder {
    force_prompt: Option<bool>,
    state: Option<String>,
    make_bypass: Option<String>,
    single_select: Option<bool>,
    single_select_by_vin: Option<String>,
    flags: Option<HashMap<String, String>>,
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

    pub fn set_force_prompt(mut self, enabled: bool) -> Self {
        self.force_prompt = Some(enabled);
        self
    }

    pub fn set_state(mut self, state: String) -> Self {
        self.state = Some(state);
        self
    }

    pub fn set_make_bypass(mut self, make: String) -> Self {
        self.make_bypass = Some(make);
        self
    }

    pub fn set_single_select(mut self, enabled: bool) -> Self {
        self.single_select = Some(enabled);
        self
    }

    pub fn set_single_select_by_vin(mut self, vin: String) -> Self {
        self.single_select_by_vin = Some(vin);
        self
    }

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
pub struct AuthClient {
    /// The application’s unique identifier, obtained
    client_id: String,

    /// The application secret identfier. If forgotten, it must be regenerated in the dashboard.
    client_secret: String,

    /// The URI a user will be redirected to after authorization.
    /// This value must match one of the redirect URIs set in the
    /// credentials tab of the dashboard.
    redirect_uri: String,

    /// Launch the Smartcar auth flow in test mode
    test_mode: bool,
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

    /// Generate the Smartcar Connect URL
    ///
    /// More info on [Smartcar Connect](https://smartcar.com/api#smartcar-connect)
    pub fn get_auth_url(&self, scope: &ScopeBuilder, options: AuthUrlOptionsBuilder) -> String {
        let mut url = get_connect_url();

        url.push_str("/oauth/authorize?scope=");
        url.push_str(scope.query_value().as_str());
        url.push_str("&response_type=code&");
        url.push_str(self.multi_query().as_str());

        let options_query = options.multi_query();
        if options_query.len() > 0 {
            if !options_query.contains("approval_prompt") {
                url.push_str("&approval_prompt=auto");
            };

            url.push_str("&");
            url.push_str(options_query.as_str());
        }

        url
    }

    /// Exhange your oauth code for an access token
    ///
    /// More info on [auth code exchange](https://smartcar.com/api#auth-code-exchange)
    pub async fn exchange_code(&self, code: &str) -> Result<(Access, Meta), error::Error> {
        let form = HashMap::from([
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", self.redirect_uri.as_str()),
        ]);

        let (res, meta) = SmartcarRequestBuilder::new(get_oauth_url(), request::HttpVerb::POST)
            .add_header(
                "Authorization",
                request::get_basic_b64_auth_header(&self.client_id, &self.client_secret).as_str(),
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
    let auth_url = ac.get_auth_url(&scope, options);

    let expecting =String::from("https://connect.smartcar.com/oauth/authorize?scope=read_engine_oil read_battery read_charge control_charge read_thermometer read_fuel read_location control_security read_odometer read_tires read_vehicle_info read_vin&response_type=code&client_id=test-client-id&client_secret=test-client-secret&redirect_uri=test.com&mode=test");

    assert_eq!(auth_url, expecting);
}

#[test]
#[should_panic]
fn create_auth_client_without_env_variables() {
    AuthClient::from_env(true);
}
