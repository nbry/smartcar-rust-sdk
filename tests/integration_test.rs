use serial_test::serial;
use smartcar::{
    auth_client::{AuthClient, AuthUrlOptionsBuilder},
    get_user, get_vehicles,
    vehicle::Vehicle,
    CompatibilityOptions, ScopeBuilder,
};

use crate::helpers::{get_creds_from_env, run_connect_flow};

mod helpers;

#[tokio::test]
#[serial]
async fn full_e2e_bev() -> Result<(), Box<dyn std::error::Error>> {
    let (client_id, client_secret, redirect_uri) = get_creds_from_env();
    let scope = ScopeBuilder::with_all_permissions();

    // SET UP AUTH CLIENT
    let ac = AuthClient::new(
        client_id.as_str(),
        client_secret.as_str(),
        redirect_uri.as_str(),
        true,
    );
    let get_auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);

    // GET ACCESS TOKEN
    let url = ac.get_auth_url(&scope, Some(&get_auth_url_options));
    let code = run_connect_flow(url.as_str(), "TESLA", "4444").await?;
    let (access, _) = ac.exchange_code(code.as_str()).await?;
    let access_token = access.access_token.as_str();

    let (user, _) = get_user(&access).await?;
    println!("got user id: {:#?}", user);

    // GET VEHICLES
    let (vehicles, _) = get_vehicles(&access, None, None).await?;
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

    let vin = v.vin().await?;
    println!("vin: {:#?}", vin);

    let compatibility_opts = CompatibilityOptions {
        client_id: Some(client_id),
        client_secret: Some(client_secret),
        flags: None,
    };

    let compatiblity =
        smartcar::get_compatibility(vin.0.vin.as_str(), &scope, "US", Some(compatibility_opts))
            .await?;

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

    let fuel_tank = v.fuel_tank().await;
    assert!(fuel_tank.is_err());

    println!("compatiblity: {:#?}", compatiblity);

    Ok(())
}

#[tokio::test]
#[serial]
async fn full_e2e_ice() -> Result<(), Box<dyn std::error::Error>> {
    println!("===================");
    let (client_id, client_secret, redirect_uri) = get_creds_from_env();
    let scope = ScopeBuilder::with_all_permissions();

    // SET UP AUTH CLIENT
    let ac = AuthClient::new(
        client_id.as_str(),
        client_secret.as_str(),
        redirect_uri.as_str(),
        true,
    );
    let get_auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);

    // GET TOKENS
    let url = ac.get_auth_url(&scope, Some(&get_auth_url_options));
    let code = run_connect_flow(url.as_str(), "BUICK", "4444").await?;
    let (access, _) = ac.exchange_code(code.as_str()).await?;

    // TRY A REFRESH TOKEN EXCHANGE
    let refresh_token = access.refresh_token.as_str();
    let (new_access, _) = ac.exchange_refresh_token(refresh_token).await?;
    let access_token = new_access.access_token.as_str();

    // GET VEHICLES
    let (vehicles, _) = get_vehicles(&access, None, None).await?;
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
