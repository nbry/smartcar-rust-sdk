use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait SmartcarResponse {
    fn new() -> Self;
}

pub type Meta = HashMap<String, String>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Access {
    vin: String,
    // meta: Meta,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vin {
    vin: String,
    // meta: Meta,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Odometer {
    odometer: f32,
    // meta: Meta,
}
