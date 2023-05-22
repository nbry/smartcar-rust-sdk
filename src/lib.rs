//! # `smartcar` - the Rust SDK for Smartcar API
//!
//! `smartcar` is a client library for sending requests to [Smartcar API](https://smartcar.com/docs).
//! Smartcar API lets you read vehicle data and send commands to vehicles using HTTP requests.
//!
//! To make requests to a vehicle from a web or mobile application, the end user must connect their vehicle
//! using [Smartcar Connect](https://smartcar.com/docs/api#smartcar-connect). This flow follows the OAuth
//! spec and will return a `code` which can be used to obtain an access token from Smartcar.
//!
//! The Smartcar Rust SDK provides methods to:
//!
//! 1. Generate the link to redirect to Connect.
//! 2. Make a request to Smartcar with the `code` obtained from Connect to obtain an
//!    access and refresh token
//! 3. Make requests to the Smartcar API to read vehicle data and send commands to
//!    vehicles using the access token obtained in step 2.
//!
//! Before integrating with Smartcar's SDK, you'll need to register an application in the
//! [Smartcar Developer portal](https://developer.smartcar.com). If you do not have access
//! to the dashboard, please [request access](https://smartcar.com/subscribe).
pub(crate) mod helpers;

use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    env,
};

use helpers::{format_flag_query, get_api_url};
use request::{get_bearer_token_header, HttpVerb, SmartcarRequestBuilder};
use response::{Access, Compatibility, Meta, User, Vehicles};

pub mod auth_client;
pub mod error;
pub mod request;
pub mod response;
pub mod vehicle;
pub mod webhooks;

/// Return the id of the vehicle owner who granted access to your application.
///
/// (https://smartcar.com/docs/api/#get-user)
pub async fn get_user(acc: &Access) -> Result<(User, Meta), error::Error> {
    let url = format!("{api_url}/v2.0/user", api_url = get_api_url());
    let (res, meta) = SmartcarRequestBuilder::new(&url, HttpVerb::GET)
        .add_header("Authorization", &get_bearer_token_header(&acc.access_token))
        .send()
        .await?;
    let data = res.json::<User>().await?;

    Ok((data, meta))
}

/// Return a list of the user's vehicle ids
///
/// More info on [get all vehicles request](https://smartcar.com/api#get-all-vehicles)
pub async fn get_vehicles(
    acc: &Access,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<(Vehicles, Meta), error::Error> {
    let url = format!("{api_url}/v2.0/vehicles", api_url = get_api_url());
    let mut req = SmartcarRequestBuilder::new(&url, HttpVerb::GET)
        .add_header("Authorization", &get_bearer_token_header(&acc.access_token));

    if let Some(l) = limit {
        req = req.add_query("limit", &l.to_string())
    }
    if let Some(o) = offset {
        req = req.add_query("offset", &o.to_string());
    }

    let (res, meta) = req.send().await?;
    let data = res.json::<Vehicles>().await?;

    Ok((data, meta))
}

/// Options for Compatibility API
pub struct CompatibilityOptions {
    /// Client ID from your dashboard
    /// Default: Will use SMARTCAR_CLIENT_ID env variable
    pub client_id: Option<String>,

    /// Client Secret, generated from your sadhboard
    /// Default: Will use SMARTAR_CLIENT_SECRET env variable
    pub client_secret: Option<String>,

    /// Optional flags that your application has early access to
    pub flags: Option<HashMap<String, String>>,
}

/// Given a VIN, country, and a list of permissions, determine:
/// 1. If the car is compatible with smartcar
/// 2. If the car is capable of the endpoints associated with each permisison
///
/// [Compatibility API](https://smartcar.com/api#compatibility-api)
pub async fn get_compatibility(
    vin: &str,
    scope: &ScopeBuilder,
    country: &str,
    options: Option<CompatibilityOptions>,
) -> Result<(Compatibility, Meta), error::Error> {
    let mut client_id = env::var("SMARTCAR_CLIENT_ID");
    let mut client_secret = env::var("SMARTCAR_CLIENT_SECRET");
    let url = format!("{}/v2.0/compatibility", get_api_url());

    let mut req = SmartcarRequestBuilder::new(&url, HttpVerb::GET)
        .add_query("vin", vin)
        .add_query("scope", &scope.query_value)
        .add_query("country", country);

    if let Some(opts) = options {
        if let Some(flags) = opts.flags {
            req = req.add_query("flags", &format_flag_query(&flags));
        };
        if let Some(id) = opts.client_id {
            client_id = Ok(id);
        };
        if let Some(secret) = opts.client_secret {
            client_secret = Ok(secret);
        };
    };

    let id = match client_id {
        Err(_) => {
            let msg = "compatibility::client id must be passed as an env variable (SMARTCAR_CLIENT_ID) OR via CompatibilityOptionsBuilder";
            return Err(error::Error::MissingParameters(msg.to_string()));
        }
        Ok(v) => v,
    };
    let secret = match client_secret {
        Err(_) => {
            let msg = "compatibility::client secret must be passed as an env variable (SMARTCAR_CLIENT_SECRET) OR via CompatibilityOptionsBuilder";
            return Err(error::Error::MissingParameters(msg.to_string()));
        }
        Ok(v) => v,
    };

    let (res, meta) = req
        .add_header(
            "Authorization",
            &request::get_basic_b64_auth_header(&id, &secret),
        )
        .send()
        .await?;

    let data = res.json::<Compatibility>().await?;

    Ok((data, meta))
}

/// A permission that your application is requesting access to during SmartcarConnect
///
/// [More info about Permissions](https://smartcar.com/docs/api/#permissions)
#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
pub enum Permission {
    ReadEngineOil,   // Read vehicle engine oil health
    ReadBattery,     // Read EV battery's capacity and state of charge
    ReadCharge,      // Know whether vehicle is charging
    ControlCharge,   // Start or stop your vehicle's charging
    ReadThermometer, // Read temperatures from inside and outside the vehicle
    ReadFuel,        // Read fuel tank level
    ReadLocation,    // Access location
    ControlSecurity, // Lock or unlock your vehicle
    ReadOdometer,    // Retrieve total distance traveled
    ReadTires,       // Read vehicle tire pressure
    ReadVehicleInfo, // Know make, model, and year
    ReadVin,         // Read VIN
}

impl Permission {
    fn as_str(&self) -> &str {
        match self {
            Permission::ReadEngineOil => "read_engine_oil",
            Permission::ReadBattery => "read_battery",
            Permission::ReadCharge => "read_charge",
            Permission::ControlCharge => "control_charge",
            Permission::ReadThermometer => "read_thermometer",
            Permission::ReadFuel => "read_fuel",
            Permission::ReadLocation => "read_location",
            Permission::ControlSecurity => "control_security",
            Permission::ReadOdometer => "read_odometer",
            Permission::ReadTires => "read_tires",
            Permission::ReadVehicleInfo => "read_vehicle_info",
            Permission::ReadVin => "read_vin",
        }
    }
}

/// Builder of a list of permissions
#[derive(Deserialize, Debug)]
pub struct ScopeBuilder {
    pub permissions: HashSet<Permission>,
    query_value: String,
}

impl ScopeBuilder {
    pub fn new() -> ScopeBuilder {
        ScopeBuilder {
            permissions: HashSet::new(),
            query_value: String::from(""),
        }
    }

    /// Create a ScopeBuilder with ALL the permissions
    pub fn add_permission(mut self, p: Permission) -> Self {
        if !self.permissions.contains(&p) {
            if self.query_value.len() != 0 {
                self.query_value.push_str(" ");
            };

            self.query_value.push_str(p.as_str());
            self.permissions.insert(p);
        }
        self
    }

    /// Create a ScopeBuilder with all available permissions
    pub fn with_all_permissions() -> ScopeBuilder {
        ScopeBuilder {
            permissions: HashSet::new(),
            query_value: String::from(""),
        }
        .add_permission(Permission::ReadEngineOil)
        .add_permission(Permission::ReadBattery)
        .add_permission(Permission::ReadCharge)
        .add_permission(Permission::ControlCharge)
        .add_permission(Permission::ReadThermometer)
        .add_permission(Permission::ReadFuel)
        .add_permission(Permission::ReadLocation)
        .add_permission(Permission::ControlSecurity)
        .add_permission(Permission::ReadOdometer)
        .add_permission(Permission::ReadTires)
        .add_permission(Permission::ReadVehicleInfo)
        .add_permission(Permission::ReadVin)
    }
}

#[test]
fn test_getting_scope_url_params_string() {
    let permissions = ScopeBuilder::new()
        .add_permission(Permission::ReadEngineOil)
        .add_permission(Permission::ReadFuel)
        .add_permission(Permission::ReadVin);

    let expecting = "read_engine_oil read_fuel read_vin";
    assert_eq!(&permissions.query_value, expecting);
}
