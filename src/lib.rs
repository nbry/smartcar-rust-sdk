#![allow(dead_code)]

pub mod auth_client;
pub mod error;
pub(crate) mod helpers;
pub mod permission;
pub mod request;
pub mod response;
pub mod vehicle;
pub mod webhooks;

use request::get_bearer_token_header;
use response::{Access, Vehicles};

pub async fn get_vehicles(
    access: &Access,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vehicles, error::Error> {
    let url = format!(
        "{api_url}/v2.0/vehicles",
        api_url = helpers::get_api_url().as_str()
    );

    let mut req = reqwest::Client::new().get(url).header(
        "Authorization",
        get_bearer_token_header(access.access_token.as_str()),
    );

    if let Some(l) = limit {
        req = req.query(&("limit", l));
    }
    if let Some(o) = offset {
        req = req.query(&("offset", o));
    }

    let vehicles = req.send().await?.json::<Vehicles>().await?;

    Ok(vehicles)
}
