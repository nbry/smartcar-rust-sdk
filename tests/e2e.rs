use serial_test::serial;
use smartcar::{
    auth_client::{AuthClient, AuthUrlOptionsBuilder},
    get_user, get_vehicles,
    request::HttpVerb,
    vehicle::Vehicle,
    CompatibilityOptions, DeleteConnectionsFilters, Permission, ScopeBuilder,
};

use crate::helpers::{get_creds_from_env, run_connect_flow};

mod helpers;

#[tokio::test]
#[serial]
async fn full_e2e_bev() -> Result<(), Box<dyn std::error::Error>> {
    let (client_id, client_secret, redirect_uri, amt) = get_creds_from_env();
    let scope = ScopeBuilder::with_all_permissions();

    // SET UP AUTH CLIENT
    let ac = AuthClient::new(&client_id, &client_secret, &redirect_uri, true);
    let get_auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);

    // GET ACCESS TOKEN
    let url = ac.get_auth_url(&scope, Some(&get_auth_url_options));
    let code = run_connect_flow(&url, "TESLA", "4444").await?;
    let (access, _) = ac.exchange_code(&code).await?;
    let access_token = &access.access_token;

    // GET VEHICLES (and isolate one vehicle for testing)
    let (_, _) = get_user(&access).await?;
    let (vehicles, _) = get_vehicles(&access, None, None).await?;
    let v = Vehicle::new(&vehicles.vehicles[0], access_token);

    // Method Calls
    let attributes = v.attributes().await?;
    println!("attributes: {:#?}", attributes);

    let battery_capacity = v.battery_capacity().await?;
    println!("battery_capacity: {:#?}", battery_capacity);

    let battery_level = v.battery_level().await?;
    println!("battery_level: {:#?}", battery_level);

    let charge_limit = v.charge_limit().await?;
    println!("charge_limit: {:#?}", charge_limit);

    let charging_status = v.charging_status().await?;
    println!("charging status: {:#?}", charging_status);

    let fuel_tank = v.fuel_tank().await;
    assert!(fuel_tank.is_err());

    let location = v.location().await?;
    println!("location: {:#?}", location);

    let lock = v.lock().await?;
    println!("lock: {:#?}", lock);

    let lock_status = v.lock_status().await?;
    println!("lock_status: {:#?}", lock_status);

    let odometer = v.odometer().await?;
    println!("odometer: {:#?}", odometer);

    let permissions = v.permissions().await?;
    println!("permissions: {:#?}", permissions);

    let start_charge = v.start_charge().await?;
    println!("start_charge: {:#?}", start_charge);

    let tire_pressure = v.tire_pressure().await?;
    println!("tire_pressure: {:#?}", tire_pressure);

    let vin = v.vin().await?;
    println!("vin: {:#?}", vin);

    // Make Specific Endpoint Test - With public `request` method
    let (compass, _meta) = v
        .request("/tesla/compass", HttpVerb::Get, None, None)
        .await?;
    let compass_body = compass.text().await?;
    println!("compass_body: {:#?}", compass_body);

    let batch = v
        .batch(vec![
            "/odometer".to_string(),
            "/charge".to_string(),
            "/fuel".to_string(), // should error
        ])
        .await?;
    println!("batch: {:#?}", batch);

    // Compatibility
    let compatibility_opts = CompatibilityOptions {
        client_id: Some(client_id),
        client_secret: Some(client_secret),
        flags: None,
    };
    let compatibility_scope =
        ScopeBuilder::new().add_permissions(vec![Permission::ReadBattery, Permission::ReadFuel]);
    let compatiblity = smartcar::get_compatibility(
        &vin.0.vin,
        &compatibility_scope,
        "US",
        Some(compatibility_opts),
    )
    .await?;
    println!("compatibility: {:#?}", compatiblity);

    // Vehicle Management
    smartcar::get_connections(amt.as_str(), None, None).await?;
    let delete_connections_filter = DeleteConnectionsFilters {
        vehicle_id: Some(v.id),
        user_id: None,
    };
    let delete_connections =
        smartcar::delete_connections(amt.as_str(), Some(delete_connections_filter)).await?;
    println!("delete_connections: {:#?}", delete_connections);

    Ok(())
}

#[tokio::test]
#[serial]
async fn full_e2e_ice() -> Result<(), Box<dyn std::error::Error>> {
    println!("===================");
    let (client_id, client_secret, redirect_uri, _) = get_creds_from_env();
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
