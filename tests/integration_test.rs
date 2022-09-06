use serial_test::serial;
use smartcar::{
    auth_client::{auth_url_options::AuthUrlOptionsBuilder, AuthClient},
    get_vehicles,
    permission::Permissions,
    vehicle::Vehicle,
};

mod helpers;

#[tokio::test]
#[serial]
async fn full_e2e_bev() -> Result<(), Box<dyn std::error::Error>> {
    let (client_id, client_secret, redirect_uri) = helpers::get_creds_from_env();
    let scope = Permissions::new().add_all();

    // SET UP AUTH CLIENT
    let ac = AuthClient::new(client_id, client_secret, redirect_uri, true);
    let get_auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);

    // GET ACCESS TOKEN
    let url = ac.get_auth_url(scope, get_auth_url_options);
    let code = helpers::run_connect_flow(url.as_str(), "TESLA", "4444").await?;
    let access = ac.exchange_code(code.as_str()).await?;
    let access_token = access.access_token.as_str();

    // GET VEHICLES
    let vehicles = get_vehicles(&access, None, None).await?;
    println!("got vehicle ids: {:#?}", vehicles);

    //  USE API ENDPOINTS ON ONE VEHICLE
    let id_of_first_vehicle = vehicles.vehicles[0].as_str();
    let v = Vehicle::new(id_of_first_vehicle, access_token);
    println!("using first vehicle: {:#?}", v);

    let permissions = v.permissions().await?;
    println!("permissions: {:#?}", permissions);

    let start_charge = v.start_charge().await?;
    println!("start_charge: {:#?}", start_charge);

    let lock = v.lock().await?;
    println!("lock: {:#?}", lock);

    let batch = v
        .batch(vec![
            "/odometer".to_string(),
            "/charge".to_string(),
            "/fuel".to_string(), // should error
        ])
        .await?;
    println!("batch: {:#?}", batch);

    let attributes = v.attributes().await?;
    println!("attributes: {:#?}", attributes);

    let battery_capacity = v.battery_capacity().await?;
    println!("battery capacity: {:#?}", battery_capacity);

    let battery_level = v.battery_level().await?;
    println!("battery level: {:#?}", battery_level);

    let charging_status = v.charging_status().await?;
    println!("charging status: {:#?}", charging_status);

    let location = v.location().await?;
    println!("location: {:#?}", location);

    let odometer = v.odometer().await?;
    println!("odometer: {:#?}", odometer);

    let tire_pressure = v.tire_pressure().await?;
    println!("tire pressure: {:#?}", tire_pressure);

    let vin = v.vin().await?;
    println!("vin: {:#?}", vin);

    let fuel_tank = v.fuel_tank().await;
    assert!(fuel_tank.is_err());

    Ok(())
}

#[tokio::test]
#[serial]
async fn full_e2e_ice() -> Result<(), Box<dyn std::error::Error>> {
    println!("===================");
    let (client_id, client_secret, redirect_uri) = helpers::get_creds_from_env();
    let permissions = Permissions::new().add_all();

    // SET UP AUTH CLIENT
    let ac = AuthClient::new(client_id, client_secret, redirect_uri, true);
    let get_auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);

    // GET ACCESS TOKEN
    let url = ac.get_auth_url(permissions, get_auth_url_options);
    let code = helpers::run_connect_flow(url.as_str(), "BUICK", "4444").await?;
    let access = ac.exchange_code(code.as_str()).await?;
    let access_token = access.access_token.as_str();

    // GET VEHICLES
    let vehicles = get_vehicles(&access, None, None).await?;
    println!("got vehicle ids: {:#?}", vehicles);

    // USE API ENDPOINTS ON ONE VEHICLE
    let id_of_first_vehicle = vehicles.vehicles[1].as_str();
    let v = Vehicle::new(id_of_first_vehicle, access_token);
    println!("using first vehicle: {:#?}", v);

    let fuel_tank = v.fuel_tank().await?;
    println!("fuel_tank: {:#?}", fuel_tank);

    let engine_oil = v.engine_oil().await?;
    println!("engine_oil: {:#?}", engine_oil);

    Ok(())
}
