use reqwest::{
    header::{HeaderMap, HeaderValue},
    Response, StatusCode,
};
use serde_json::Value;

use crate::{
    error::{Error, SmartcarError},
    response::meta::{self, Meta},
};

/// Able to build a URL query param
pub(crate) trait QueryString {
    /// Build a URL query param from the struct's fields
    fn query_string(&self) -> String;
}

pub(crate) fn get_bearer_token_header(access_token: &str) -> HeaderValue {
    let authorization = format!("Bearer {}", access_token);
    HeaderValue::from_str(authorization.as_str()).unwrap()
}

pub(crate) enum HttpVerb {
    GET,
    POST,
    DELETE,
}

/// Send an HTTP request to smartcar
pub(crate) async fn smartcar_request(
    url: &str,
    verb: HttpVerb,
    headers: HeaderMap,
    body: Option<Value>,
) -> Result<(Response, Meta), Error> {
    let req = reqwest::Client::new();
    let mut req_builder = match verb {
        HttpVerb::GET => req.get(url),
        HttpVerb::POST => req.get(url),
        HttpVerb::DELETE => req.get(url),
    };

    req_builder = req_builder.headers(headers);

    req_builder = match verb {
        HttpVerb::POST => match body {
            Some(b) => req_builder.json::<Value>(&b),
            None => req_builder,
        },
        _ => req_builder,
    };

    let res = req_builder.send().await?;

    if res.status() != StatusCode::OK {
        let sc_err = res.json::<SmartcarError>().await?;
        return Err(Error::SmartcarError(sc_err));
    }

    let meta = meta::generate_meta_from_headers(res.headers());

    Ok((res, meta))
}
