use serde::{Deserialize, Serialize};

pub mod batch;
pub mod meta;

#[derive(Debug, Deserialize, Serialize)]
pub struct Access {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationPermissions {
    pub permissions: Vec<String>,
    pub paging: Paging,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineOilLife {
    pub life_remaining: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BatteryCapacity {
    pub capacity: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryLevel {
    pub percent_remaining: f32,
    pub range: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStatus {
    pub is_plugged_in: bool,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuelTank {
    pub range: f32,
    pub percent_remaining: f32,
    pub amount_remaining: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Odometer {
    pub distance: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paging {
    pub count: i32,
    pub offset: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vehicles {
    pub vehicles: Vec<String>,
    pub paging: Paging,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TirePressure {
    pub front_left: f32,
    pub front_right: f32,
    pub back_left: f32,
    pub back_right: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VehicleAttributes {
    pub id: String,
    pub make: String,
    pub model: String,
    pub year: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vin {
    pub vin: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub message: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Disconnect {
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Capability {
    pub permission: String,
    pub endpoint: String,
    pub capable: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Compatibility {
    pub compatible: bool,
    pub reason: Option<String>,
    pubcapabilities: Vec<Capability>,
}
