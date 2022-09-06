use reqwest::header::HeaderValue;

/// # Able to build a URL query param
pub(crate) trait QueryString {
    /// Build a URL query param from the struct's fields
    fn query_string(&self) -> String;
}

pub(crate) fn get_bearer_token_header(access_token: &str) -> HeaderValue {
    let authorization = format!("Bearer {}", access_token);
    HeaderValue::from_str(authorization.as_str()).unwrap()
}
