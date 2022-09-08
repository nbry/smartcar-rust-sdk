use serde::Serialize;
use serde_json::Value;

use crate::error::Error;

#[derive(Serialize, Debug)]
pub(crate) struct BatchRequestPath {
    pub(crate) path: String,
}

#[derive(Serialize, Debug)]
pub(crate) struct BatchRequestBody {
    pub(crate) requests: Vec<BatchRequestPath>,
}

impl BatchRequestBody {
    pub fn add(&mut self, path: String) {
        self.requests.push(BatchRequestPath { path });
    }
}

pub(crate) fn build_batch_request_body(paths: Vec<String>) -> Result<Value, Error> {
    let mut batch_request_body = BatchRequestBody {
        requests: Vec::new(),
    };
    paths
        .iter()
        .for_each(|path| batch_request_body.add(path.to_string()));

    Ok(serde_json::to_value(&batch_request_body)?)
}
