//! This module includes the the vehicle struct. This struct contains methods
//! for getting data from and send comands to a vehicle.

use serde_json::json;

use crate::error::Error;
use crate::helpers::get_api_url;
use crate::request::{get_bearer_token_header, HttpVerb, SmartcarRequestBuilder};
use crate::response::batch::build_batch_request_body;
use crate::response::{Action, ApplicationPermissions, Status, Subscribe};
use crate::response::{
    Batch, BatteryCapacity, BatteryLevel, ChargingStatus, EngineOilLife, FuelTank, Location, Meta,
    Odometer, TirePressure, VehicleAttributes, Vin,
};

#[derive(Debug)]
pub enum UnitSystem {
    Imperial,
    Metric,
}

#[derive(Debug)]
pub struct Vehicle {
    pub id: String,
    pub access_token: String,
    pub unit_system: UnitSystem,
}

impl Vehicle {
    /// Initializes a new Vehicle to use for making requests to the Smartcar API.
    pub fn new(vehicle_id: &str, access_token: &str) -> Vehicle {
        Vehicle {
            id: vehicle_id.to_owned(),
            access_token: access_token.to_owned(),
            unit_system: UnitSystem::Metric,
        }
    }

    fn request(&self, path: &str, verb: HttpVerb) -> SmartcarRequestBuilder {
        let url = format!(
            "{api_url}/v2.0/vehicles/{id}{path}",
            api_url = get_api_url(),
            id = self.id,
            path = path
        );

        SmartcarRequestBuilder::new(url, verb).add_header(
            "Authorization",
            &get_bearer_token_header(&self.access_token),
        )
    }

    /// Returns a list of the permissions that have been granted to your application
    /// in relation to this vehicle
    ///
    /// [Get Application Permissions](https://smartcar.com/api#get-application-permissions)
    pub async fn permissions(&self) -> Result<(ApplicationPermissions, Meta), Error> {
        let path = "/permissions";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<ApplicationPermissions>().await?;

        Ok((data, meta))
    }

    /// Returns the remaining life span of a vehicle’s engine oil.
    ///
    /// [Engine Oil](https://smartcar.com/api#get-engine-oil-life)
    pub async fn engine_oil(&self) -> Result<(EngineOilLife, Meta), Error> {
        let path = "/engine/oil";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<EngineOilLife>().await?;

        Ok((data, meta))
    }

    /// Returns the total capacity of an electric vehicle's battery.
    ///
    /// [EV Battery Capacity](https://smartcar.com/api#get-ev-battery-capacity)
    pub async fn battery_capacity(&self) -> Result<(BatteryCapacity, Meta), Error> {
        let path = "/battery/capacity";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<BatteryCapacity>().await?;

        Ok((data, meta))
    }

    /// Returns the state of charge (SOC) and the remaining range of an electric vehicle's battery.
    ///
    /// [EV Battery Level](https://smartcar.com/api#get-ev-battery-level)
    pub async fn battery_level(&self) -> Result<(BatteryLevel, Meta), Error> {
        let path = "/battery";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<BatteryLevel>().await?;

        Ok((data, meta))
    }

    /// Returns the current charge status of an electric vehicle.
    ///
    /// [EV Charging Status](https://smartcar.com/api#get-ev-charging-status)
    pub async fn charging_status(&self) -> Result<(ChargingStatus, Meta), Error> {
        let path = "/charge";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<ChargingStatus>().await?;

        Ok((data, meta))
    }

    /// Returns the status of the fuel remaining in the vehicle’s gas tank.
    /// Note: The fuel tank API is only available for vehicles sold in the United States.
    ///
    /// [Fuel Tank](https://smartcar.com/api#get-fuel-tank)
    pub async fn fuel_tank(&self) -> Result<(FuelTank, Meta), Error> {
        let path = "/fuel";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<FuelTank>().await?;

        Ok((data, meta))
    }

    /// Returns the last known location of the vehicle in geographic coordinates.
    ///
    /// [Location](https://smartcar.com/api#get-location)
    pub async fn location(&self) -> Result<(Location, Meta), Error> {
        let path = "/location";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<Location>().await?;

        Ok((data, meta))
    }

    /// Returns the vehicle’s last known odometer reading.
    ///
    /// [Odometer](https://smartcar.com/api#get-odometer)
    pub async fn odometer(&self) -> Result<(Odometer, Meta), Error> {
        let path = "/odometer";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<Odometer>().await?;

        Ok((data, meta))
    }

    /// Returns the air pressure of each of the vehicle’s tires.
    ///
    /// [Tire Pressure](https://smartcar.com/api#get-tire-pressure)
    pub async fn tire_pressure(&self) -> Result<(TirePressure, Meta), Error> {
        let path = "/tires/pressure";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<TirePressure>().await?;

        Ok((data, meta))
    }

    /// Returns a single vehicle object, containing identifying information.
    ///
    /// [Vehicle Attributes](https://smartcar.com/api#get-vehicle-attributes)
    pub async fn attributes(&self) -> Result<(VehicleAttributes, Meta), Error> {
        let path = "/";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<VehicleAttributes>().await?;

        Ok((data, meta))
    }

    /// Returns the vehicle’s manufacturer identifier.
    ///
    /// [VIN](https://github.com/smartcar/java-sdk)
    pub async fn vin(&self) -> Result<(Vin, Meta), Error> {
        let path = "/vin";
        let (res, meta) = self.request(path, HttpVerb::GET).send().await?;
        let data = res.json::<Vin>().await?;

        Ok((data, meta))
    }

    /// Lock the vehicle.
    ///
    /// [Lock/Unlock](https://smartcar.com/api#post-lockunlock)
    pub async fn lock(&self) -> Result<(Action, Meta), Error> {
        let path = "/security";
        let req_body = json!({ "action": "LOCK"});
        let (res, meta) = self
            .request(path, HttpVerb::POST)
            .add_body(req_body)
            .send()
            .await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    /// Unlock the vehicle.
    ///
    /// [Lock/Unlock Doors](https://smartcar.com/api#post-lockunlock)
    pub async fn unlock(&self) -> Result<(Action, Meta), Error> {
        let path = "/securiy";
        let req_body = json!({ "action": "UNLOCK"});
        let (res, meta) = self
            .request(path, HttpVerb::POST)
            .add_body(req_body)
            .send()
            .await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    /// Start charging an electric vehicle.
    ///
    /// [Start/Stop Charge](https://smartcar.com/api#post-ev-startstop-charge)
    pub async fn start_charge(&self) -> Result<(Action, Meta), Error> {
        let path = "/charge";
        let req_body = json!({ "action": "START"});
        let (res, meta) = self
            .request(path, HttpVerb::POST)
            .add_body(req_body)
            .send()
            .await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    /// Stop charging an electric vehicle.
    ///
    /// [Start/Stop Charge](https://smartcar.com/api#post-ev-startstop-charge)
    pub async fn stop_charge(&self) -> Result<(Action, Meta), Error> {
        let path = "/charge";
        let req_body = json!({ "action": "STOP"});
        let (res, meta) = self
            .request(path, HttpVerb::POST)
            .add_body(req_body)
            .send()
            .await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    /// Returns a list of responses from multiple Smartcar endpoints, all combined into a single request.
    ///
    /// [Batch Request](https://smartcar.com/api#post-batch-request)
    pub async fn batch(&self, paths: Vec<String>) -> Result<(Batch, Meta), Error> {
        let path = "/batch";
        let req_body = build_batch_request_body(paths)?;
        let (res, meta) = self
            .request(path, HttpVerb::POST)
            .add_body(req_body)
            .send()
            .await?;
        let data = res.json::<Batch>().await?;

        Ok((data, meta))
    }

    /// Revoke access for the current requesting application.
    ///
    /// [Disconnect](https://smartcar.com/api#delete-disconnect)
    pub async fn disconnect(&self) -> Result<(Status, Meta), Error> {
        let path = "/application";
        let (res, meta) = self
            .request(path, HttpVerb::DELETE)
            .send()
            .await?;
        let data = res.json::<Status>().await?;

        Ok((data, meta))
    }

    /// Subscribe a vehicle to a webhook
    ///
    /// [Subsribe to Webhook](https://smartcar.com/api#post-subscribe)
    pub async fn subscribe(&self, webhook_id: &str) -> Result<(Subscribe, Meta), Error> {
        let path = format!("/webhooks/{}", webhook_id);
        let (res, meta) = self.request(&path, HttpVerb::POST).send().await?;
        let data = res.json::<Subscribe>().await?;

        Ok((data, meta))
    }

    /// Unsubscribe a vehicle to a webhook
    ///
    /// # Fields
    /// - `amt` - The Application Management Token found on Smartcar Dashbaord
    /// - `webhook_id` - The id of the webhook, found in your dashboard
    ///
    /// [Unsubscribe from Webhook](https://smartcar.com/api#delete-unsubscribe)
    pub async fn unsubscribe(
        &self,
        amt: &str,
        webhook_id: &str,
    ) -> Result<(Subscribe, Meta), Error> {
        let url = format!(
            "{api_url}/v2.0/vehicles/{id}/webhooks/{webhook_id}",
            api_url = get_api_url(),
            id = self.id,
            webhook_id = webhook_id
        );

        // Different bearer token requires a request built from scratch,
        let (res, meta) = SmartcarRequestBuilder::new(url, HttpVerb::DELETE)
            .add_header("Authorization", &get_bearer_token_header(amt))
            .send()
            .await?;
        let data = res.json::<Subscribe>().await?;

        Ok((data, meta))
    }
}
