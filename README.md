# Smartcar Rust SDK

[![Crate](https://img.shields.io/crates/v/smartcar.svg)](https://crates.io/crates/smartcar)
[![Documentation](https://docs.rs/smartcar/badge.svg)](https://docs.rs/smartcar)

Rust crate for Smartcar API

## Overview

[Smartcar API](https://smartcar.com/docs) lets you read vehicle data and send commands to vehicles (lock, unlock) using HTTP requests.

To make requests to a vehicle from a web or mobile application, the end user must connect their vehicle using [Smartcar Connect](https://smartcar.com/docs/api#smartcar-connect). This flow follows the OAuth spec and will return a `code` which can be used to obtain an access token from Smartcar.

The Smartcar Rust SDK provides methods to:

1. Generate the link to redirect to Connect.
2. Make a request to Smartcar with the `code` obtained from Connect to obtain an
   access and refresh token
3. Make requests to the Smartcar API to read vehicle data and send commands to
   vehicles using the access token obtained in step 2.

Before integrating with Smartcar's SDK, you'll need to register an application in the [Smartcar Developer portal](https://developer.smartcar.com). If you do not have access to the dashboard, please [request access](https://smartcar.com/subscribe).

## Installation

Add this to your `Cargo.toml`:

```
[dependencies]
smartcar = "0.1.0"
```

## Flow

- Create a new `AuthClient` object with your `client_id`, `client_secret`,
  `redirectUri`.
- Redirect the user to Smartcar Connect using `<AuthClient>.get_auth_url` with required `scope` or with one
  of our frontend SDKs.
- The user will login, and then accept or deny your `scope`'s permissions.
- Handle the get request to your `redirect_uri`.
  - If the user accepted your permissions:
    - Use `<AuthClient>.exchange_code` with this code to obtain an access struct.
		This struct contains an access token (lasting 2 hours) and a refresh token (lasting 60 days).
	  - (Save this access struct)
- Get the user's vehicles with `get_vehicles`.
- Create a new `vehicle` struct using an `d` from the previous response,
  and the `access_token`.
- Make requests to the Smartcar API.
- Use `<AuthClient>.exchange_refresh_token` on your saved `refresh_token` to retrieve a new token
  when your `accessToken` expires.

## Getting Started

```
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

```
