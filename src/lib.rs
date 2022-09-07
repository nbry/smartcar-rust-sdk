#![allow(dead_code)]

pub mod auth_client;
pub mod error;
pub(crate) mod helpers;
pub mod permission;
pub mod request;
pub mod response;
pub mod vehicle;
pub mod webhooks;

use std::{collections::HashMap, env};

use permission::Permissions;
use request::{get_bearer_token_header, HttpVerb, SmartcarRequestBuilder};
use response::{
    compatibility::Compatibility,
    meta::{self, Meta},
    Access, Vehicles,
};

/// Return a list of the user's vehicle ids
///
/// More info on [get all vehicles request](https://smartcar.com/api#get-all-vehicles)
pub async fn get_vehicles(
    access: &Access,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<(Vehicles, Meta), error::Error> {
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

    let res = req.send().await?;
    let meta = meta::generate_meta_from_headers(res.headers());
    let data = res.json::<Vehicles>().await?;

    Ok((data, meta))
}

pub struct CompatibilityOptions {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub flags: Option<HashMap<String, String>>,
}

pub async fn get_compatibility(
    vin: &str,
    scope: &Permissions,
    country: &str,
    options: Option<CompatibilityOptions>,
) -> Result<(Compatibility, Meta), error::Error> {
    let mut client_id = env::var("SMARTCAR_CLIENT_ID");
    let mut client_secret = env::var("SMARTCAR_CLIENT_SECRET");
    let url = format!("{}/v2.0/compatibility", helpers::get_api_url());

    println!("vin: {}", vin);

    let s = scope.query_value();
    println!("scope: {}", s);

    println!("country {}", country);

    let mut req = SmartcarRequestBuilder::new(url, HttpVerb::GET)
        .add_query("vin", vin)
        .add_query("scope", s.as_str())
        .add_query("country", country);

    println!("{:#?}", req);

    if let Some(opts) = options {
        if let Some(flags) = opts.flags {
            req = req.add_query("flags", helpers::format_flag_query(&flags).as_str());
        };

        if let Some(id) = opts.client_id {
            client_id = Ok(id);
        };

        if let Some(secret) = opts.client_secret {
            client_secret = Ok(secret);
        };
    };

    let id = match client_id {
        Err(_) =>return Err(error::Error::MissingParameters("compatibility::client id must be passed as an env variable (SMARTCAR_CLIENT_ID) OR via CompatibilityOptionsBuilder".to_string())),
        Ok(v) => v,
    };

    let secret = match client_secret {
        Err(_) => return Err(error::Error::MissingParameters("compatibility::client secret must be passed as an env variable (SMARTCAR_CLIENT_SECRET) OR via CompatibilityOptionsBuilder".to_string())),
        Ok(v) => v,
    };

    println!("client id: {}", id);
    println!("client secret: {}", secret);

    let header = request::get_basic_b64_auth_header(id.as_str(), secret.as_str());
    println!("header: {}", header);

    println!("{:#?}", req);

    let (res, meta) = req
        .add_header("Authorization", header.as_str())
        .send()
        .await?;

    let data = res.json::<Compatibility>().await?;

    Ok((data, meta))
}
