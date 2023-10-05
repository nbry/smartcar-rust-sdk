use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Json;
use axum::{routing::get, Router};
use serde::Deserialize;
use serde_json::json;

use smartcar::*;

use auth_client::{AuthClient, AuthUrlOptionsBuilder};
use response::{Meta, VehicleAttributes};
use vehicle::Vehicle;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // This route demonstrates the Smartcar Conenct flow that your user
        // will go through. For this example, you'll be going through it yourself.
        .route("/login", get(login))
        // This route captures the redirect after your user finishes the Smartcar Connect flow.
        // If the user grants permission to your app, it will contain a query `code`
        .route("/callback", get(callback));

    // Run the server on localhost 3000
    println!("\nStep 0: Running on localhost, port 3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Helper for creating an Auth Client instance with your credentials
fn get_auth_client() -> AuthClient {
    AuthClient::new(
        "REPLACE_WITH_YOUR_SMARTCAR_CLIENT_ID",
        "REPLACE_WITH_YOUR_SMARTCAR_CLIENT_SECRET",
        "REPLACE_WITH_YOUR_SMARTCAR_REDIRECT_URI.COM",
        true,
    )
}

/// Smartcar Connect Flow
async fn login() -> Redirect {
    let auth_client = get_auth_client();
    println!(
        "\nStep 1:  Create an AuthClient struct with your client credentials:\n{:#?}\n",
        auth_client
    );

    // Here we are adding the read_vehicle_info permission so we can get
    // the make, model, and year of the vehicle. In other words, we are asking
    // the vehicle owner for permission to get these attributes.
    let scope = ScopeBuilder::new().add_permissions([Permission::ReadVehicleInfo]);

    // Here we build the options for creating the auth url.
    // This particular option forces the approval prompt page to show up.
    // For educational purposes, let's force it to show up all the time.
    let auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);

    let auth_url = auth_client.get_auth_url(&scope, Some(&auth_url_options));
    println!("Step 2: Generating the auth url that your user will go to");
    println!(
        "\nResult: Generated auth URL. Redirecting to:\n\n{}",
        auth_url
    );

    Redirect::to(&auth_url)
}

/// The potential query values in the redirect to /callback
/// after user goes through Smartcar Connect
#[derive(Deserialize)]
struct Callback {
    code: Option<String>,
    error: Option<String>,
}

// Handle Smartcar callback with auth code. To run this example, setup your
// redirect URI in your Smartcar account dashboard to include http://localhost:3000/callback
async fn callback(q: Query<Callback>) -> impl IntoResponse {
    println!("\nStep 3: Completed in browser, i.e. User finished connect flow");
    println!("\nStep 4a: Redirecting to /callback");

    // If user denies you access, you'll see this
    if q.error.is_some() {
        return (
            StatusCode::EXPECTATION_FAILED,
            Json(json!("User delined during Smartcar Connect")),
        );
    };

    // This is the authorization code that represents the user’s consent
    // granting you (in this example) permission to read their vehicle's attributes
    // This code must be exchanged for an access token to start making requests to the vehicle.
    let code = &q.code.to_owned().unwrap();
    println!("\nResult: `code` query present in /callback:\n{}", code);

    match get_attributes_handler(code).await {
        Err(_) => (
            StatusCode::EXPECTATION_FAILED,
            Json(json!("attributes request failed")),
        ),
        Ok((attributes, _)) => {
            (
                StatusCode::OK,
                Json(json!(attributes)), // please help me make this better... lol
            )
        }
    }
}

async fn get_attributes_handler(
    auth_code: &str,
) -> Result<(VehicleAttributes, Meta), smartcar::error::Error> {
    let client = get_auth_client();

    let (access, _) = client.exchange_code(auth_code).await?;
    println!(
        "\nStep 4b: Exchange code for an Access struct with tokens:\n{:#?}\n",
        access
    );

    let (vehicle_ids, _) = smartcar::get_vehicles(&access, None, None).await?;
    println!(
        "\nStep 5: Use access token to get the user's vehicles (i.e. a list of vehicle ids):\n{:#?}",
        vehicle_ids
    );

    let vehicle = Vehicle::new(&vehicle_ids.vehicles[0], &access.access_token);
    println!(
        "\nStep 6: Use any id from the Vehicles (plural) instance to make a single Vehicle struct:\n{:#?}",
        vehicle
    );

    println!("\nStep 7: Send a request to get the vehicle's attributes");

    let (attributes, meta) = vehicle.attributes().await?;
    println!(
        "\nResult: Got the vehicle's id, make, model, and year:\n{:#?}",
        attributes
    );

    println!(
        "\nAlso got information about the request itself:\n{:#?}",
        meta
    );

    println!("\nTHE END");

    Ok((attributes, meta))
}
