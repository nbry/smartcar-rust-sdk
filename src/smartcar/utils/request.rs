use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Error, Response,
};

const SMARTCAR_API_BASE_URL: &str = "https://smartcar.com";
const SMARTCAR_CONNECT_BASE_URL: &str = "https://connect.smartcar.com";

pub struct SmartcarRequestHeaders {
    header_map: HeaderMap,
}

impl SmartcarRequestHeaders {
    pub fn add_header(mut self, header: &str, value: &str) -> Self {
        self.header_map.insert(
            HeaderName::from_lowercase(header.as_bytes()).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
        self
    }
}

pub(super) async fn smartcar_get_request(
    path: &str,
    headers: SmartcarRequestHeaders,
) -> Result<Response, Error> {
    let url = format!("{}{}", SMARTCAR_API_BASE_URL, path);

    reqwest::Client::new()
        .get(url)
        .headers(headers.header_map)
        .send()
        .await
}
