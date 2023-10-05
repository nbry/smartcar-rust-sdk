//! Classifies all potential errors of this SDK,
//! including the Smartcar API V2 Error response.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// All potential errors of the library
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
    SmartcarError(Box<SmartcarError>),

    #[error("choose ONE of vehicle_id OR user_id as a filter")]
    DeleteConnectionsFilterValidationError,
}

/// A detailed error response from Smartcar API
///
/// [More info about Smartcar Errors](https://smartcar.com/docs/api/#errors)
#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
#[error("{error_type}::{description}")]
pub struct SmartcarError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: Option<String>,
    pub description: String,
    #[serde(rename = "docURL")]
    pub doc_url: String,
    pub status_code: i32,
    pub resolution: HashMap<String, Option<String>>,
    pub request_id: String,
}
