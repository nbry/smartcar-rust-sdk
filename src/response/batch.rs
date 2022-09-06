use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Error, SmartcarError};

use super::{
    meta::Meta, ApplicationPermissions, BatteryCapacity, BatteryLevel, ChargingStatus,
    EngineOilLife, FuelTank, Location, Odometer, TirePressure, User, VehicleAttributes, Vin,
};

#[derive(Serialize, Debug)]
pub(crate) struct BatchRequestPath {
    pub(crate) path: String,
}

#[derive(Serialize, Debug)]
pub(crate) struct BatchRequestBody {
    pub(crate) requests: Vec<BatchRequestPath>,
}

impl BatchRequestBody {
    pub fn add(&mut self, path: String) {
        self.requests.push(BatchRequestPath { path });
    }
}

pub(crate) fn build_batch_request_body(paths: Vec<String>) -> Result<Value, Error> {
    let mut batch_request_body = BatchRequestBody {
        requests: Vec::new(),
    };
    paths
        .iter()
        .for_each(|path| batch_request_body.add(path.to_string()));

    Ok(serde_json::to_value(&batch_request_body)?)
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BatchResponseBody {
    ApplicationPermissions(ApplicationPermissions),
    EngineOilLife(EngineOilLife),
    BatteryCapacity(BatteryCapacity),
    BatteryLevel(BatteryLevel),
    ChargingStatus(ChargingStatus),
    FuelTank(FuelTank),
    Location(Location),
    Odometer(Odometer),
    TirePressure(TirePressure),
    User(User),
    VehicleAttributes(VehicleAttributes),
    Vin(Vin),
    SmartcarError(SmartcarError),
}

#[derive(Debug, Deserialize)]
pub struct BatchResponse {
    path: String,
    body: BatchResponseBody,
    code: i32,
    headers: Option<Meta>,
}

#[derive(Debug, Deserialize)]
pub struct Batch {
    responses: Vec<BatchResponse>,
}
