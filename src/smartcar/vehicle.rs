pub enum ApiVersion {
    One,
    Two,
}

pub enum MetricSystem {
    Imperial,
    Metric,
}

pub struct Vehicle {
    pub id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub api_version: ApiVersion,
    pub unit_system: MetricSystem,
}

impl Vehicle {
    pub fn vin() {}
    pub fn charge() {}
    pub fn battery() {}
    pub fn battery_capacity() {}
    pub fn fuel() {}
    pub fn tire_pressure() {}
    pub fn engine_oil() {}
    pub fn odometer() {}
    pub fn location() {}
    pub fn permissions() {}
    pub fn door_action() {}
    pub fn charge_action() {}
}
