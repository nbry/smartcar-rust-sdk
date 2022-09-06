#![allow(dead_code)]

pub mod auth_client;
pub mod error;
pub(crate) mod helpers;
pub mod permission;
pub mod request;
pub mod response;
pub mod vehicle;

use request::get_bearer_token_header;
use response::Vehicles;

pub async fn get_vehicles(
    access_token: &str,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vehicles, Box<dyn std::error::Error>> {
    let url = format!(
        "{api_url}/v2.0/vehicles",
        api_url = helpers::get_api_url().as_str()
    );

    let mut req = reqwest::Client::new()
        .get(url)
        .header("Authorization", get_bearer_token_header(access_token));

    if let Some(l) = limit {
        req = req.query(&("limit", l));
    }
    if let Some(o) = offset {
        req = req.query(&("offset", o));
    }

    let vehicles = req.send().await?.json::<Vehicles>().await?;

    Ok(vehicles)
}
