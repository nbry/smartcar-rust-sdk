use std::env;

pub(crate) fn get_api_url() -> String {
    match env::var("SMARTCAR_API_ORIGIN") {
        Ok(api_url) => api_url,
        Err(_) => String::from("https://api.smartcar.com"),
    }
}

pub(crate) fn get_auth_url() -> String {
    match env::var("SMARTCAR_AUTH_ORIGIN") {
        Ok(api_url) => api_url,
        Err(_) => String::from("https://auth.smartcar.com/oauth/token"),
    }
}

pub(crate) fn get_connect_url() -> String {
    match env::var("SMARTCAR_CONNECT_URL") {
        Ok(api_url) => api_url,
        Err(_) => String::from("https://connect.smartcar.com"),
    }
}
