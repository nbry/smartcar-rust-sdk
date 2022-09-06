use serde::Deserialize;

pub mod batch;
pub mod meta;

#[derive(Deserialize, Debug)]
pub struct Access {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationPermissions {
    pub permissions: Vec<String>,
    pub paging: Paging,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EngineOilLife {
    pub life_remaining: f32,
}

#[derive(Deserialize, Debug)]
pub struct BatteryCapacity {
    pub capacity: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatteryLevel {
    pub percent_remaining: f32,
    pub range: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStatus {
    pub is_plugged_in: bool,
    pub state: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuelTank {
    pub range: f32,
    pub percent_remaining: f32,
    pub amount_remaining: f32,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Deserialize, Debug)]
pub struct Odometer {
    pub distance: f32,
}

#[derive(Deserialize, Debug)]
pub struct Paging {
    pub count: i32,
    pub offset: i32,
}

#[derive(Deserialize, Debug)]
pub struct Vehicles {
    pub vehicles: Vec<String>,
    pub paging: Paging,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TirePressure {
    pub front_left: f32,
    pub front_right: f32,
    pub back_left: f32,
    pub back_right: f32,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct VehicleAttributes {
    pub id: String,
    pub make: String,
    pub model: String,
    pub year: i32,
}

#[derive(Deserialize, Debug)]
pub struct Vin {
    pub vin: String,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    pub message: String,
    pub status: String,
}
