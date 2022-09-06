use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest error")]
    RequestError(#[from] reqwest::Error),

    #[error("serde_json error")]
    DeserializeError(#[from] serde_json::Error),

    #[error("error response from api")]
    SmartcarError(SmartcarError),
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
#[error("{error_type}:{description}")]
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
