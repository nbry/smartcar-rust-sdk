use serde::Deserialize;

use crate::request::QueryString;

/// # Smartcar Permission
///
/// A permission that your application is requesting
/// access to during SmartcarConnect
#[derive(Deserialize, Debug)]
pub enum Permission {
    ReadEngineOil,   // Read vehicle engine oil health
    ReadBattery,     // Read EV battery's capacity and state of charge
    ReadCharge,      // Know whether vehicle is charging
    ControlCharge,   // Start or stop your vehicle's charging
    ReadThermometer, // Read temperatures from inside and outside the vehicle
    ReadFuel,        // Read fuel tank level
    ReadLocation,    // Access location
    ControlSecurity, // Lock or unlock your vehicle
    ReadOdometer,    // Retrieve total distance traveled
    ReadTires,       // Read vehicle tire pressure
    ReadVehicleInfo, // Know make, model, and year
    ReadVin,         // Read VIN
}

/// # Permissions builder
#[derive(Deserialize, Debug)]
pub struct Permissions {
    permissions: Vec<Permission>,
}

/// Builder for adding permissions to your app
impl Permissions {
    /// Create a new Permissions builder
    pub fn new() -> Permissions {
        Permissions {
            permissions: Vec::new(),
        }
    }

    pub fn add(mut self, permission: Permission) -> Self {
        self.permissions.push(permission);
        self
    }

    pub fn add_all(self) -> Self {
        self.add(Permission::ReadEngineOil)
            .add(Permission::ReadBattery)
            .add(Permission::ReadCharge)
            .add(Permission::ControlCharge)
            .add(Permission::ReadThermometer)
            .add(Permission::ReadFuel)
            .add(Permission::ReadLocation)
            .add(Permission::ControlSecurity)
            .add(Permission::ReadOdometer)
            .add(Permission::ReadTires)
            .add(Permission::ReadVehicleInfo)
            .add(Permission::ReadVin)
    }
}

fn get_query_param_for_permission(permission: &Permission) -> &str {
    match permission {
        Permission::ReadEngineOil => "read_engine_oil",
        Permission::ReadBattery => "read_battery",
        Permission::ReadCharge => "read_charge",
        Permission::ControlCharge => "control_charge",
        Permission::ReadThermometer => "read_thermometer",
        Permission::ReadFuel => "read_fuel",
        Permission::ReadLocation => "read_location",
        Permission::ControlSecurity => "control_security",
        Permission::ReadOdometer => "read_odometer",
        Permission::ReadTires => "read_tires",
        Permission::ReadVehicleInfo => "read_vehicle_info",
        Permission::ReadVin => "read_vin",
    }
}

impl QueryString for Permissions {
    /// Build the URL query param for `scope`
    fn query_string(&self) -> String {
        let mut space_separated_permissions = String::from("");

        self.permissions.iter().for_each(|permission| {
            let param = get_query_param_for_permission(permission);
            space_separated_permissions.push_str(" ");
            space_separated_permissions.push_str(param);
        });

        format!("&scope={}", space_separated_permissions.trim())
    }
}

#[test]
fn test_getting_scope_url_params_string() {
    let scope_query_param = Permissions::new()
        .add(Permission::ReadEngineOil)
        .add(Permission::ReadFuel)
        .add(Permission::ReadVin)
        .query_string();

    let expected = String::from("&scope=read_engine_oil read_fuel read_vin");
    assert_eq!(scope_query_param, expected)
}
