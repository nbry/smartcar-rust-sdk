use chrono::{TimeZone, Utc};
use reqwest::header::HeaderMap;

use super::Meta;

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
