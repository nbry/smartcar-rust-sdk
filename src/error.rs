use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sdk error::the http request failed")]
    SdkReqwestFailure(#[from] reqwest::Error),

    #[error("sdk error::serde_json failed to (de)serialization failure")]
    SdkSerdeFailure(#[from] serde_json::Error),

    #[error("sdk error::hmac digest error")]
    SdkHmacInvalidLength(#[from] hmac::digest::InvalidLength),

    #[error("smartcar error::function call with missing params")]
    MissingParameters(String),

    #[error("smartcar error::error response from smartcar api")]
    SmartcarError(SmartcarError),
}

#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
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
