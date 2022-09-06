use crate::helpers::{get_auth_url, get_connect_url};
use crate::permission::Permissions;
use crate::request::QueryString;
use crate::response::Access;

use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;
use std::{collections::HashMap, env};

pub mod auth_url_options;
use self::auth_url_options::GetAuthUrlOptions;

pub struct AuthClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    test_mode: bool,
}

impl AuthClient {
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
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        test_mode: bool,
    ) -> AuthClient {
        AuthClient {
            client_id,
            client_secret,
            redirect_uri,
            test_mode,
        }
    }

    /// Get the URL that the user will go to for connecting their car
    pub fn get_auth_url(&self, permissions: Permissions, options: GetAuthUrlOptions) -> String {
        let scope_query = permissions.query_string();
        let option_query = options.query_string();
        let mut auth_query = self.query_string();

        if !option_query.contains("approval_prompt") {
            auth_query.push_str("&approval_prompt=auto");
        };

        format!(
            "{url}{path}?{auth}{scope}{option}",
            url = get_connect_url(),
            path = "/oauth/authorize",
            auth = auth_query,
            scope = scope_query,
            option = option_query,
        )
    }

    pub async fn exchange_code(&self, code: &str) -> Result<Access, Box<dyn Error>> {
        let form = HashMap::from([
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", self.redirect_uri.as_str()),
        ]);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", self.get_b64_auth_header_value());
        headers.insert(
            "content-type",
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );

        let body = reqwest::Client::new()
            .post(get_auth_url())
            .headers(headers)
            .form(&form)
            .send()
            .await?;

        let res = body.json::<Access>().await?;

        Ok(res)
    }

    fn get_b64_auth_header_value(&self) -> HeaderValue {
        let credentials = format!(
            "{client_id}:{client_secret}",
            client_id = self.client_id,
            client_secret = self.client_secret
        );
        let encoded = base64::encode(credentials.as_bytes());
        let header_value = format!("Basic {}", encoded.as_str());

        HeaderValue::from_str(header_value.as_str()).unwrap()
    }
}

impl QueryString for AuthClient {
    fn query_string(&self) -> String {
        let mut query_string = format!(
            "&client_id={id}\
            &client_secret={secret}\
            &redirect_uri={uri}\
            &response_type=code",
            id = self.client_id.as_str(),
            secret = self.client_secret.as_str(),
            uri = self.redirect_uri.as_str()
        );

        if self.test_mode {
            query_string.push_str("&mode=test");
        }

        query_string
    }
}
