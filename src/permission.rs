use serde::Deserialize;

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

    pub(crate) fn query_value(&self) -> String {
        let mut query_value = String::from("");

        for p in self.permissions.iter() {
            query_value.push_str(get_query_param_for_permission(p));
            query_value.push_str(" ");
        }

        query_value.trim().to_string()
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

#[test]
fn test_getting_scope_url_params_string() {
    let permissions = Permissions::new()
        .add(Permission::ReadEngineOil)
        .add(Permission::ReadFuel)
        .add(Permission::ReadVin);

    let expecting = "read_engine_oil read_fuel read_vin";
    assert_eq!(permissions.query_value(), expecting);
}
