use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sdk error::the http request failed")]
    RequestError(#[from] reqwest::Error),

    #[error("sdk error::serde_json failed to serialize a struct")]
    DeserializeError(#[from] serde_json::Error),

    #[error("sdk error::serde_json failed to serialize a struct")]
    InvalidLength(#[from] hmac::digest::InvalidLength),

    #[error("smartcar error::error response from smartcar api")]
    SmartcarError(SmartcarError),
}

#[derive(Debug, Deserialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
#[error("{error_type}::{description}")]
pub struct SmartcarError {
    #[serde(rename = "type")]
    error_type: String,
    code: Option<String>,
    description: String,
    #[serde(rename = "docURL")]
    doc_url: String,
    status_code: i32,
    resolution: HashMap<String, Option<String>>,
    request_id: String,
}
