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
    permissions: Vec<String>,
    paging: Paging,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EngineOilLife {
    life_remaining: f32,
}

#[derive(Deserialize, Debug)]
pub struct BatteryCapacity {
    capacity: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatteryLevel {
    percent_remaining: f32,
    range: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStatus {
    is_plugged_in: bool,
    state: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuelTank {
    range: f32,
    percent_remaining: f32,
    amount_remaining: f32,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize, Debug)]
pub struct Odometer {
    distance: f32,
}

#[derive(Deserialize, Debug)]
pub struct Paging {
    count: i32,
    offset: i32,
}

#[derive(Deserialize, Debug)]
pub struct Vehicles {
    pub vehicles: Vec<String>,
    pub paging: Paging,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TirePressure {
    front_left: f32,
    front_right: f32,
    back_left: f32,
    back_right: f32,
}

#[derive(Deserialize, Debug)]
pub struct User {
    id: String,
}

#[derive(Deserialize, Debug)]
pub struct VehicleAttributes {
    id: String,
    make: String,
    model: String,
    year: i32,
}

#[derive(Deserialize, Debug)]
pub struct Vin {
    vin: String,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    message: String,
    status: String,
}
