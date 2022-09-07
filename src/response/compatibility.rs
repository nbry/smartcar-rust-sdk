use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Capability {
    permission: String,
    endpoint: String,
    capable: bool,
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Compatibility {
    compatible: bool,
    reason: Option<String>,
    capabilities: Vec<Capability>,
}
