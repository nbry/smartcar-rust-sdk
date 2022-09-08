use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Json;
use axum::{routing::get, Router};
use serde::Deserialize;
use serde_json::json;

use smartcar;
use smartcar::response::{Meta, VehicleAttributes};
use smartcar::vehicle::Vehicle;
use smartcar::{Permission, ScopeBuilder};

/// Helper for creating an Auth Client instance with your credentials
fn get_auth_client() -> smartcar::auth_client::AuthClient {
    smartcar::auth_client::AuthClient::new(
        "REPLACE-WITH-YOUR-SMARTCAR-CLIENT-ID",
        "REPLACE-WITH-YOUR-SMARTCAR-CLIENT-SECRET",
        "REPLACE-WITH-YOUR-SMARTCAR-REDIRECT-URI.COM",
        true,
    )
}

/// Redirect to Smartcar Connect
async fn login() -> Redirect {
    let scope = ScopeBuilder::new().add_permission(Permission::ReadVehicleInfo);
    let link = get_auth_client().get_auth_url(&scope, None);

    println!("URL your user will go to:\n\n{}", link);
    Redirect::to(&link)
}

/// The potential query codes after user goes through Smartcar Connect
#[derive(Deserialize)]
struct Callback {
    code: Option<String>,
    error: Option<String>,
}

// Handle Smartcar callback with auth code
#[axum_macros::debug_handler]
async fn callback(q: Query<Callback>) -> impl IntoResponse {
    if let Some(_) = &q.error {
        return (
            StatusCode::EXPECTATION_FAILED,
            Json(json!("User declined auth")),
        );
    };

    let code = &q.code.to_owned().unwrap();
    let res = get_attributes_handler(code.as_str()).await;

    // If user denies you access, you'll see this
    let (attributes, meta) = match res {
        Err(_) => {
            return (
                StatusCode::EXPECTATION_FAILED,
                Json(json!("User delined during Smartcar Connect")),
            )
        }
        Ok((a, m)) => (a, m),
    };

    println!("Information about the request itself:\n\n{:#?}", meta);
    println!("Vehicle's id, make, model, year:\n\n{:#?}", attributes);

    (
        StatusCode::OK,
        Json(json!(attributes)), // please help me make this better... lol
    )
}

async fn get_attributes_handler(
    auth_code: &str,
) -> Result<(VehicleAttributes, Meta), smartcar::error::Error> {
    let client = get_auth_client();

    // Exchange auth code for an access struct (that has tokens)
    let (access, _) = client.exchange_code(auth_code).await?;

    // Get the user's vehicles
    let (vehicle_ids, _) = smartcar::get_vehicles(&access, None, None).await?;

    // For the sake of this example, just use the first vehicle
    let vehicle = Vehicle::new(&vehicle_ids.vehicles[0], &access.access_token);

    // Get the vehicle's attributes (make/model/year)
    let (attributes, meta) = vehicle.attributes().await?;

    Ok((attributes, meta))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
