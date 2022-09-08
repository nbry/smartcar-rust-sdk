//! These structs are representations of the response body
//! after sending a request
//! to Smartcar API

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::SmartcarError;

pub mod batch;
pub mod meta;

/// Contains tokens for authenticating API requests
///
/// This is the struct representation for the response body of
/// **POST** `https://auth.smartcar.com/oauth/token`
///
/// Note that this is path for either exchanging an auth code OR a refresh token
/// [More info on Authorization](https://smartcar.com/docs/api/#authorization)
#[derive(Debug, Deserialize, Serialize)]
pub struct Access {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub token_type: String,
}

/// Contains a list of permissions that have been granted to your
/// application in relation to the vehicle
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/permissions`
#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationPermissions {
    pub permissions: Vec<String>,
    pub paging: Paging,
}

/// Contains the remaining life span of a vehicle’s engine oil
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/engine/oil`
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineOilLife {
    pub life_remaining: f32,
}

/// Contains the total capacity of an electric vehicle's battery
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/battery/capacity`
#[derive(Debug, Deserialize, Serialize)]
pub struct BatteryCapacity {
    pub capacity: f32,
}

/// Contains the state of charge (SOC) and the remaining range of an electric vehicle's battery
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/battery`
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryLevel {
    pub percent_remaining: f32,
    pub range: f32,
}

/// Contains the current charge status of an electric vehicle
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/charge`
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStatus {
    pub is_plugged_in: bool,
    pub state: String,
}

/// Contains the status of the fuel remaining in the vehicle’s gas tank
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/fuel`
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuelTank {
    pub range: f32,
    pub percent_remaining: f32,
    pub amount_remaining: f32,
}

/// Contains the last known location of the vehicle in geographic coordinates
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/location`
#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}

/// Contains the vehicle's last known odometer reading
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/odometer`
#[derive(Debug, Deserialize, Serialize)]
pub struct Odometer {
    pub distance: f32,
}

/// Contains a paged list of all vehicles connected to the application for the current authorized user
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles`
#[derive(Debug, Deserialize, Serialize)]
pub struct Vehicles {
    pub vehicles: Vec<String>,
    pub paging: Paging,
}

/// Contains metadata about the current a list of elements, including
/// the total number of elements for the entire query and
/// the current start index of the returned list.
#[derive(Debug, Deserialize, Serialize)]
pub struct Paging {
    pub count: i32,
    pub offset: i32,
}

/// Contains the the air pressure of each of the vehicle’s tires
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/tires/pressure`
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TirePressure {
    pub front_left: f32,
    pub front_right: f32,
    pub back_left: f32,
    pub back_right: f32,
}

/// Contains the vehicle’s manufacturer identifier
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}/vin`
#[derive(Debug, Deserialize, Serialize)]
pub struct Vin {
    pub vin: String,
}

/// Contains identifying information about a vehicle
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/vehicles/{id}`
#[derive(Debug, Deserialize, Serialize)]
pub struct VehicleAttributes {
    pub id: String,
    pub make: String,
    pub model: String,
    pub year: i32,
}

/// Contains the id of the vehicle owner who granted access to your application
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/user`
#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
}

/// Contains the status after sending a command to the vehicle
///
/// Commands include:
/// - Lock/Unlock Doors
/// **POST** `https://api.smartcar.com/v2.0/security`
///
/// - Stop/Start Charge
/// **POST** `https://api.smartcar.com/v2.0/charge`
#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub message: String,
    pub status: String,
}

/// Contains the status after sending a DELETE request.
///
/// This includes:
/// - disconnecting a vehicle from an application
/// - unsubscribing a vehicle from a webhook
///
/// This is the struct representation for the response body of
/// **DELETE** `https://api.smartcar.com/v2.0/vehicles/{id}/application` or
/// **DELETE** `https://api.smartcar.com/v2.0/vehicles/{id}/webhooks/{webhookId}` or
#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub status: String,
}

/// Contains the information about a webhook upon subscribing a vehicle to one
///
/// This is the struct representation for the response body of
/// **POST** https://api.smartcar.com/v2.0/vehicles/{id}/webhooks/{webhookId}`
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscribe {
    pub webhook_id: String,
    pub vehicle_id: String,
}

/// Contains information about whether the vehicle is capable of
/// a smartcar endpoint. This struct as as part of the full Compatibility
/// API response, and is thus nested in the Compatibility struct.
#[derive(Debug, Deserialize, Serialize)]
pub struct Capability {
    pub permission: String,
    pub endpoint: String,
    pub capable: bool,
    pub reason: Option<String>,
}

/// Contains information about whether a car is compatible with Smartcar API and
/// if it is capable of the endpoints that your application needs.
///
/// This is the struct representation for the response body of
/// **GET** `https://api.smartcar.com/v2.0/compatibility?vin={vin}&scope={scope}&country={country}`
#[derive(Debug, Deserialize, Serialize)]
pub struct Compatibility {
    pub compatible: bool,
    pub reason: Option<String>,
    pub capabilities: Vec<Capability>,
}

/// Contains the Smartcar headers from a response
///
/// [More info on Smartcar Response Headers](https://smartcar.com/docs/api/#response-headers)
#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    #[serde(rename = "sc-data-age")]
    pub data_age: Option<DateTime<Utc>>,

    #[serde(rename = "sc-request-id")]
    pub request_id: Option<String>,

    #[serde(rename = "sc-unit-system")]
    pub unit_system: Option<String>,
}

/// The response body of a single endpoint in a batch request
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SmartcarResponseBody {
    ApplicationPermissions(ApplicationPermissions),
    EngineOilLife(EngineOilLife),
    BatteryCapacity(BatteryCapacity),
    BatteryLevel(BatteryLevel),
    ChargingStatus(ChargingStatus),
    FuelTank(FuelTank),
    Location(Location),
    Odometer(Odometer),
    TirePressure(TirePressure),
    VehicleAttributes(VehicleAttributes),
    Vin(Vin),
    SmartcarError(SmartcarError),
}

/// Contains the response body AND metadata of a single endpoint in a batch request
///
/// e.g. If you sent a batch request, requesting for endpoints `/odometer`, `/charge`, AND `/vin`,
/// This struct represents the singular response to any of those requests.
///
/// Therefore, there will be three of these BatchResponse structs in the response field
/// of the Batch struct. One for Odometer, one for Charge, and one for Vin.
///
/// [More info on batch](https://smartcar.com/api#post-batch-request)
#[derive(Debug, Deserialize, Serialize)]
pub struct BatchResponse {
    pub path: String,
    pub body: SmartcarResponseBody,
    pub code: i32,
    pub headers: Option<Meta>,
}

/// Contains the list of responses for multiple Smartcar Endpoints after
/// sending a batch request
///
/// This is the struct representation for the response body of
/// **POST** `https://api.smartcar.com/v2.0/vehicles/{id}/batch`
#[derive(Debug, Deserialize, Serialize)]
pub struct Batch {
    pub responses: Vec<BatchResponse>,
}
