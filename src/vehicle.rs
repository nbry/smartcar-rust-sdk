use reqwest::{Response, StatusCode};
use serde_json::{json, Value};

use crate::error::{Error, SmartcarError};
use crate::helpers;
use crate::request::{self, HttpVerb};
use crate::response::batch::{build_batch_request_body, Batch};
use crate::response::meta;
use crate::response::meta::Meta;
use crate::response::{Action, ApplicationPermissions};

use crate::response::{
    BatteryCapacity, BatteryLevel, ChargingStatus, EngineOilLife, FuelTank, Location, Odometer,
    TirePressure, VehicleAttributes, Vin,
};

#[derive(Debug)]
pub enum UnitSystem {
    Imperial,
    Metric,
}

impl UnitSystem {
    fn as_str(&self) -> &'static str {
        match self {
            UnitSystem::Imperial => "imperial",
            UnitSystem::Metric => "metric",
        }
    }
}

/// A vehicle instance, for making requests to Smartcar API
#[derive(Debug)]
pub struct Vehicle {
    pub id: String,
    pub access_token: String,
    pub unit_system: UnitSystem,
}

impl Vehicle {
    pub fn new(vehicle_id: &str, access_token: &str) -> Vehicle {
        Vehicle {
            id: vehicle_id.to_owned(),
            access_token: access_token.to_owned(),
            unit_system: UnitSystem::Metric,
        }
    }

    /// Returns a list of the permissions that have been granted to your application
    /// in relation to this vehicle
    ///
    /// [Get Application Permissions](https://smartcar.com/api#get-application-permissions)
    pub async fn permissions(&self) -> Result<(ApplicationPermissions, Meta), Error> {
        let path = "/permissions";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<ApplicationPermissions>().await?;

        Ok((data, meta))
    }

    /// Returns the remaining life span of a vehicle’s engine oil.
    ///
    /// [Engine Oil](https://smartcar.com/api#get-engine-oil-life)
    pub async fn engine_oil(&self) -> Result<(EngineOilLife, Meta), Error> {
        let path = "/engine/oil";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<EngineOilLife>().await?;

        Ok((data, meta))
    }

    /// Returns the total capacity of an electric vehicle's battery.
    ///
    /// [EV Battery Capacity](https://smartcar.com/api#get-ev-battery-capacity)
    pub async fn battery_capacity(&self) -> Result<(BatteryCapacity, Meta), Error> {
        let path = "/battery/capacity";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<BatteryCapacity>().await?;

        Ok((data, meta))
    }

    /// Returns the state of charge (SOC) and the remaining range of an electric vehicle's battery.
    ///
    /// [EV Battery Level](https://smartcar.com/api#get-ev-battery-level)
    pub async fn battery_level(&self) -> Result<(BatteryLevel, Meta), Error> {
        let path = "/battery";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<BatteryLevel>().await?;

        Ok((data, meta))
    }

    /// Returns the current charge status of an electric vehicle.
    ///
    /// [EV Charging Status](https://smartcar.com/api#get-ev-charging-status)
    pub async fn charging_status(&self) -> Result<(ChargingStatus, Meta), Error> {
        let path = "/charge";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<ChargingStatus>().await?;

        Ok((data, meta))
    }

    /// Returns the status of the fuel remaining in the vehicle’s gas tank.
    /// Note: The fuel tank API is only available for vehicles sold in the United States.
    ///
    /// [Fuel Tank](https://smartcar.com/api#get-fuel-tank)
    pub async fn fuel_tank(&self) -> Result<(FuelTank, Meta), Error> {
        let path = "/fuel";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<FuelTank>().await?;

        Ok((data, meta))
    }

    /// Returns the last known location of the vehicle in geographic coordinates.
    ///
    /// [Location](https://smartcar.com/api#get-location)
    pub async fn location(&self) -> Result<(Location, Meta), Error> {
        let path = "/location";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<Location>().await?;

        Ok((data, meta))
    }

    /// Returns the vehicle’s last known odometer reading.
    ///
    /// [Odometer](https://smartcar.com/api#get-odometer)
    pub async fn odometer(&self) -> Result<(Odometer, Meta), Error> {
        let path = "/odometer";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<Odometer>().await?;

        Ok((data, meta))
    }

    /// Returns the air pressure of each of the vehicle’s tires.
    ///
    /// [Tire Pressure](https://smartcar.com/api#get-tire-pressure)
    pub async fn tire_pressure(&self) -> Result<(TirePressure, Meta), Error> {
        let path = "/tires/pressure";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<TirePressure>().await?;

        Ok((data, meta))
    }

    /// Returns a single vehicle object, containing identifying information.
    ///
    /// [Vehicle Attributes](https://smartcar.com/api#get-vehicle-attributes)
    pub async fn attributes(&self) -> Result<(VehicleAttributes, Meta), Error> {
        let path = "/";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<VehicleAttributes>().await?;

        Ok((data, meta))
    }

    /// Returns the vehicle’s manufacturer identifier.
    ///
    /// [VIN](https://github.com/smartcar/java-sdk)
    pub async fn vin(&self) -> Result<(Vin, Meta), Error> {
        let path = "/vin";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<Vin>().await?;

        Ok((data, meta))
    }

    /// Lock the vehicle.
    ///
    /// [Lock/Unlock](https://smartcar.com/api#post-lockunlock)
    pub async fn lock(&self) -> Result<(Action, Meta), Error> {
        let path = "/security";
        let req_body = json!({ "action": "LOCK"});
        let (res, meta) = self.post_request(path, req_body).await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    /// Unlock the vehicle.
    ///
    /// [Lock/Unlock Doors](https://smartcar.com/api#post-lockunlock)
    pub async fn unlock(&self) -> Result<(Action, Meta), Error> {
        let path = "/securiy";
        let req_body = json!({ "action": "UNLOCK"});
        let (res, meta) = self.post_request(path, req_body).await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    /// Start charging an electric vehicle.
    ///
    /// [Start/Stop Charge](https://smartcar.com/api#post-ev-startstop-charge)
    pub async fn start_charge(&self) -> Result<(Action, Meta), Error> {
        let path = "/charge";
        let req_body = json!({ "action": "START"});
        let (res, meta) = self.post_request(path, req_body).await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    /// Stop charging an electric vehicle.
    ///
    /// [Start/Stop Charge](https://smartcar.com/api#post-ev-startstop-charge)
    pub async fn stop_charge(&self) -> Result<(Action, Meta), Error> {
        let path = "/charge";
        let req_body = json!({ "action": "STOP"});
        let (res, meta) = self.post_request(path, req_body).await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    /// Returns a list of responses from multiple Smartcar endpoints, all combined into a single request.
    ///
    /// [Batch Request](https://smartcar.com/api#post-batch-request)
    pub async fn batch(&self, paths: Vec<String>) -> Result<(Batch, Meta), Error> {
        let path = "/batch";
        let req_body = build_batch_request_body(paths)?;
        let (res, meta) = self.post_request(path, req_body).await?;
        let data = res.json::<Batch>().await?;

        Ok((data, meta))
    }

    async fn get_request(
        &self,
        path: &str,
        override_path: bool,
    ) -> Result<(Response, Meta), Error> {
        let url = if override_path {
            format!(
                "{api_url}/v2.0{path}",
                api_url = helpers::get_api_url(),
                path = path
            )
        } else {
            format!(
                "{api_url}/v2.0/vehicles/{id}{path}",
                api_url = helpers::get_api_url(),
                id = self.id,
                path = path
            )
        };

        let res = reqwest::Client::new()
            .get(url)
            .header(
                "Authorization",
                request::get_bearer_token_header(self.access_token.as_str()),
            )
            .header("Sc-Unit-System", self.unit_system.as_str())
            .send()
            .await?;

        if res.status() != StatusCode::OK {
            let sc_err = res.json::<SmartcarError>().await?;
            return Err(Error::SmartcarError(sc_err));
        }

        let meta = meta::generate_meta_from_headers(res.headers());

        Ok((res, meta))
    }

    async fn post_request(&self, path: &str, body: Value) -> Result<(Response, Meta), Error> {
        let url = format!(
            "{api_url}/v2.0/vehicles/{id}{path}",
            api_url = helpers::get_api_url(),
            id = self.id,
            path = path
        );

        let res = reqwest::Client::new()
            .post(url)
            .header(
                "Authorization",
                request::get_bearer_token_header(self.access_token.as_str()),
            )
            .header("Sc-Unit-System", self.unit_system.as_str())
            .json::<Value>(&body)
            .send()
            .await?;

        let meta = meta::generate_meta_from_headers(res.headers());

        if res.status() != StatusCode::OK {
            let sc_err = res.json::<SmartcarError>().await?;
            return Err(Error::SmartcarError(sc_err));
        }

        Ok((res, meta))
    }
}
