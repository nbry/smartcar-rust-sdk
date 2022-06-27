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

/// Construct a String with space separated scopes to
/// pass as the URL query param 'scope'
pub fn get_scope_url_param(scopes: Vec<Permission>) -> String {
    let mut scope_url_param = "".to_string();
    scopes
        .iter()
        .for_each(|scope| {
            let p = get_url_param(scope);
            scope_url_param = format!("{} {}", scope_url_param, p)
        });

    scope_url_param
        .trim()
        .to_string()
}

fn get_url_param(scope: &Permission) -> &str {
    match scope {
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
    let vec_of_scope_enums = vec![
        Permission::ReadEngineOil,
        Permission::ReadFuel,
        Permission::ReadVin,
    ];

    let result = get_scope_url_param(vec_of_scope_enums);
    let expected = String::from("read_engine_oil read_fuel read_vin");
    assert_eq!(result, expected)
}
