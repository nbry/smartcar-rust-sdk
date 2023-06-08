use serial_test::serial;
use smartcar::{
    auth_client::{AuthClient, AuthUrlOptionsBuilder},
    get_user,
    get_vehicles,
    request::HttpVerb,
    vehicle::Vehicle,
    // CompatibilityOptions,
    ScopeBuilder,
};

use crate::helpers::{get_creds_from_env, run_connect_flow};

mod helpers;

#[tokio::test]
#[serial]
async fn full_e2e_bev() -> Result<(), Box<dyn std::error::Error>> {
    let (client_id, client_secret, redirect_uri) = get_creds_from_env();
    let scope = ScopeBuilder::with_all_permissions();

    // SET UP AUTH CLIENT
    let ac = AuthClient::new(&client_id, &client_secret, &redirect_uri, true);
    let get_auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);

    // GET ACCESS TOKEN
    let url = ac.get_auth_url(&scope, Some(&get_auth_url_options));
    let code = run_connect_flow(&url, "TESLA", "4444").await?;
    let (access, _) = ac.exchange_code(&code).await?;
    let access_token = &access.access_token;

    let (_, _) = get_user(&access).await?;
    let (vehicles, _) = get_vehicles(&access, None, None).await?;
    let v = Vehicle::new(&vehicles.vehicles[0], access_token);

    let permissions = v.permissions().await?;
    println!("permissions: {:#?}", permissions);

    let start_charge = v.start_charge().await?;
    println!("start_charge: {:#?}", start_charge);

    let lock = v.lock().await?;
    println!("lock: {:#?}", lock);

    let vin = v.vin().await?;
    println!("vin: {:#?}", vin);

    let attributes = v.attributes().await?;
    println!("attributes: {:#?}", attributes);

    let battery_capacity = v.battery_capacity().await?;
    println!("battery capacity: {:#?}", battery_capacity);

    let battery_level = v.battery_level().await?;
    println!("battery level: {:#?}", battery_level);

    let charging_status = v.charging_status().await?;
    println!("charging status: {:#?}", charging_status);

    let charge_limit = v.charge_limit().await?;
    println!("charge limit: {:#?}", charge_limit);

    let location = v.location().await?;
    println!("location: {:#?}", location);

    let odometer = v.odometer().await?;
    println!("odometer: {:#?}", odometer);

    let tire_pressure = v.tire_pressure().await?;
    println!("tire pressure: {:#?}", tire_pressure);

    let fuel_tank = v.fuel_tank().await;
    assert!(fuel_tank.is_err());

    // Using general purpose request to get brand specific endpoint
    let compass = v
        .request("/tesla/compass", HttpVerb::Get, None, None)
        .await;
    println!("compass: {:#?}", compass);

    let batch = v
        .batch(vec![
            "/odometer".to_string(),
            "/charge".to_string(),
            "/fuel".to_string(), // should error
        ])
        .await?;
    println!("batch: {:#?}", batch);

    // let compatibility_opts = CompatibilityOptions {
    //     client_id: Some(client_id),
    //     client_secret: Some(client_secret),
    //     flags: None,
    // };

    // let compatiblity =
    //     smartcar::get_compatibility(&vin.0.vin, &scope, "US", Some(compatibility_opts)).await?;
    // println!("compatiblity: {:#?}", compatiblity);

    Ok(())
}

#[tokio::test]
#[serial]
async fn full_e2e_ice() -> Result<(), Box<dyn std::error::Error>> {
    println!("===================");
    let (client_id, client_secret, redirect_uri) = get_creds_from_env();
    let scope = ScopeBuilder::with_all_permissions();

    // SET UP AUTH CLIENT
    let ac = AuthClient::new(&client_id, &client_secret, &redirect_uri, true);
    let get_auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);

    // GET TOKENS
    let url = ac.get_auth_url(&scope, Some(&get_auth_url_options));
    let code = run_connect_flow(&url, "BUICK", "4444").await?;
    let (access, _) = ac.exchange_code(&code).await?;

    // TRY A REFRESH TOKEN EXCHANGE
    let refresh_token = &access.refresh_token;
    let (new_access, _) = ac.exchange_refresh_token(refresh_token).await?;
    let access_token = new_access.access_token;

    // GET VEHICLES & ISOLATE FIRST VEHICLE
    let (vehicles, _) = get_vehicles(&access, None, None).await?;
    let v = Vehicle::new(&vehicles.vehicles[1], &access_token);
    // println!("using first vehicle: {:#?}", v);

    let engine_oil = v.engine_oil().await?;
    println!("engine_oil: {:#?}", engine_oil);

    let fuel_tank = v.fuel_tank().await?;
    println!("fuel_tank: {:#?}", fuel_tank);

    Ok(())
}
