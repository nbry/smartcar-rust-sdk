use chrono::{DateTime, TimeZone, Utc};
use reqwest::header::HeaderMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Meta {
    #[serde(rename = "sc-data-age")]
    pub data_age: Option<DateTime<Utc>>,

    #[serde(rename = "sc-request-id")]
    pub request_id: Option<String>,

    #[serde(rename = "sc-unit-system")]
    pub unit_system: Option<String>,
}

pub(crate) fn generate_meta_from_headers(headers: &HeaderMap) -> Meta {
    let mut meta = Meta {
        data_age: None,
        unit_system: None,
        request_id: None,
    };

    if let Some(h) = headers.get("SC-Data-Age") {
        // e.g. format, "2022-09-05T19:57:31.037Z"
        let format = "%Y-%m-%dT%H:%M:%S%.3fZ";
        let date_str = h.to_str().expect("a string");
        let data_age = Utc.datetime_from_str(date_str, format);

        if let Ok(v) = data_age {
            meta.data_age = Some(v);
        }
    };
    if let Some(h) = headers.get("SC-Unit-System") {
        meta.unit_system = Some(h.to_str().expect("a string").to_string());
    };
    if let Some(h) = headers.get("SC-Request-Id") {
        meta.request_id = Some(h.to_str().expect("a string").to_string());
    };

    meta
}
