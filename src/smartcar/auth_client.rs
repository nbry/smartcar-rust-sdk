pub mod connect_url_options;

use std::env;

pub struct AuthClient {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub test_mode: bool,
}

impl AuthClient {
    pub fn from_env(test_mode: bool) -> AuthClient {
        SmartcarCredentialsBuilder::new(test_mode).build()
    }

    pub fn setup(test_mode: bool) -> SmartcarCredentialsBuilder {
        SmartcarCredentialsBuilder::new(test_mode)
    }

    // pub fn get_auth_url(scope: [Permission]) -> String {}

    // pub fn exchange_code(code: String) -> Access {}

    // pub fn exchange_refresh_token(refresh_token: String) {}
}

pub struct SmartcarCredentialsBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    redirect_uri: Option<String>,
    test_mode: bool,
}

impl SmartcarCredentialsBuilder {
    fn new(test_mode: bool) -> SmartcarCredentialsBuilder {
        let client_id = match env::var("SMARTCAR_CLIENT_ID") {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        let client_secret = match env::var("SMARTCAR_CLIENT_SECRET") {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        let redirect_uri = match env::var("SMARTCAR_REDIRECT_URI") {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        SmartcarCredentialsBuilder {
            client_id,
            client_secret,
            redirect_uri,
            test_mode,
        }
    }

    pub fn set_client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_string());
        self
    }

    pub fn set_client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = Some(client_secret.to_string());
        self
    }

    pub fn set_redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.redirect_uri = Some(redirect_uri.to_string());
        self
    }

    pub fn build(self) -> AuthClient {
        AuthClient {
            client_id: self
                .client_id
                .expect("'client_id' must either be built or set with env variables"),
            client_secret: self
                .client_secret
                .expect("'client_secret' must either be built or set with env variables"),
            redirect_uri: self
                .redirect_uri
                .expect("'redirect_uri' must either be built or set with env variables"),
            test_mode: self.test_mode,
        }
    }
}

#[test]
/// This test demonstrates the basic usage of the AuthClient
/// through SmartcarCredentialsBuilder and setting each param
fn auth_client_creation_with_builder() {
    let test_client_id = "test-client-id";
    let test_client_secret = "test-client-secret";
    let test_redirect_uri = "redirect-uri.com";

    let mut auth_client = AuthClient::setup(false)
        .set_client_id(test_client_id)
        .set_client_secret(test_client_secret)
        .set_redirect_uri(test_redirect_uri)
        .build();

    auth_client.client_id = String::from("aweifjaweijf");

    assert_eq!(auth_client.client_id, test_client_id);
    assert_eq!(auth_client.client_secret, test_client_secret);
    assert_eq!(auth_client.redirect_uri, test_redirect_uri);
}

#[test]
#[should_panic]
/// Missing any of the required fields will cause a panic
fn auth_client_creation_without_all_params_should_panic() {
    let test_client_id = "test-client-id";
    let test_client_secret = "test-client-secret";

    AuthClient::setup(false)
        .set_client_id(test_client_id)
        .set_client_secret(test_client_secret)
        .build();
}
