use reqwest::{Response, StatusCode};
use serde_json::{json, Value};

use crate::error::{Error, SmartcarError};
use crate::helpers;
use crate::request;
use crate::response::batch::{build_batch_request_body, Batch};
use crate::response::meta;
use crate::response::meta::Meta;
use crate::response::{Action, ApplicationPermissions};

use crate::response::{
    BatteryCapacity, BatteryLevel, ChargingStatus, EngineOilLife, FuelTank, Location, Odometer,
    TirePressure, User, VehicleAttributes, Vin,
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

    pub async fn permissions(&self) -> Result<(ApplicationPermissions, Meta), Error> {
        let path = "/permissions";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<ApplicationPermissions>().await?;

        Ok((data, meta))
    }

    pub async fn engine_oil(&self) -> Result<(EngineOilLife, Meta), Error> {
        let path = "/engine/oil";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<EngineOilLife>().await?;

        Ok((data, meta))
    }

    pub async fn battery_capacity(&self) -> Result<(BatteryCapacity, Meta), Error> {
        let path = "/battery/capacity";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<BatteryCapacity>().await?;

        Ok((data, meta))
    }

    pub async fn battery_level(&self) -> Result<(BatteryLevel, Meta), Error> {
        let path = "/battery";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<BatteryLevel>().await?;

        Ok((data, meta))
    }

    pub async fn charging_status(&self) -> Result<(ChargingStatus, Meta), Error> {
        let path = "/charge";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<ChargingStatus>().await?;

        Ok((data, meta))
    }

    pub async fn fuel_tank(&self) -> Result<(FuelTank, Meta), Error> {
        let path = "/fuel";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<FuelTank>().await?;

        Ok((data, meta))
    }

    pub async fn location(&self) -> Result<(Location, Meta), Error> {
        let path = "/location";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<Location>().await?;

        Ok((data, meta))
    }

    pub async fn odometer(&self) -> Result<(Odometer, Meta), Error> {
        let path = "/odometer";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<Odometer>().await?;

        Ok((data, meta))
    }

    pub async fn tire_pressure(&self) -> Result<(TirePressure, Meta), Error> {
        let path = "/tires/pressure";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<TirePressure>().await?;

        Ok((data, meta))
    }

    pub async fn user(&self) -> Result<(User, Meta), Error> {
        let path = "/user";
        let (res, meta) = self.get_request(path, true).await?;
        let data = res.json::<User>().await?;

        Ok((data, meta))
    }

    pub async fn attributes(&self) -> Result<(VehicleAttributes, Meta), Error> {
        let path = "/";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<VehicleAttributes>().await?;

        Ok((data, meta))
    }

    pub async fn vin(&self) -> Result<(Vin, Meta), Error> {
        let path = "/vin";
        let (res, meta) = self.get_request(path, false).await?;
        let data = res.json::<Vin>().await?;

        Ok((data, meta))
    }

    pub async fn lock(&self) -> Result<(Action, Meta), Error> {
        let path = "/security";
        let req_body = json!({ "action": "LOCK"});
        let (res, meta) = self.post_request(path, req_body).await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    pub async fn unlock(&self) -> Result<(Action, Meta), Error> {
        let path = "/securiy";
        let req_body = json!({ "action": "UNLOCK"});
        let (res, meta) = self.post_request(path, req_body).await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    pub async fn start_charge(&self) -> Result<(Action, Meta), Error> {
        let path = "/charge";
        let req_body = json!({ "action": "START"});
        let (res, meta) = self.post_request(path, req_body).await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

    pub async fn stop_charge(&self) -> Result<(Action, Meta), Error> {
        let path = "/charge";
        let req_body = json!({ "action": "STOP"});
        let (res, meta) = self.post_request(path, req_body).await?;
        let data = res.json::<Action>().await?;

        Ok((data, meta))
    }

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
