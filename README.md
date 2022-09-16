# Smartcar Rust SDK

[![Crate](https://img.shields.io/crates/v/smartcar.svg)](https://crates.io/crates/smartcar)
[![Documentation](https://docs.rs/smartcar/badge.svg)](https://docs.rs/smartcar)
[![Tests](https://github.com/nbry/smartcar-rust-sdk/actions/workflows/tests.yml/badge.svg)](https://github.com/nbry/smartcar-rust-sdk/actions/workflows/tests.yml)

Rust library crate for sending requests to Smartcar API

## Overview

[Smartcar API](https://smartcar.com/docs) lets you read vehicle data and send commands to vehicles (lock, unlock) using HTTP requests.

To make requests to a vehicle from a web or mobile application, the end user must connect their vehicle using [Smartcar Connect](https://smartcar.com/docs/api#smartcar-connect). This flow follows the OAuth spec and will return a `code` which can be used to obtain an access token from Smartcar.

The Smartcar Rust SDK provides methods to:

1. Generate the link to redirect to Connect.
2. Make a request to Smartcar with the `code` obtained from Connect to obtain an access and refresh token
3. Make requests to the Smartcar API to read vehicle data and send commands to vehicles using the access token obtained in step 2.

Before integrating with Smartcar's SDK, you'll need to register an application in the [Smartcar Developer portal](https://developer.smartcar.com). If you do not have access to the dashboard, please [request access](https://smartcar.com/subscribe).

Note that the Rust SDK only supports version 2.0 of Smartcar API.

## Installation

Add this to your `Cargo.toml`:

```
[dependencies]
smartcar = "0.1.7"
```

## Flow

1. Create a new `AuthClient` struct with your `client_id`, `client_secret`, and `redirect_uri`.
2. Redirect the user to Smartcar Connect using `<AuthClient>.get_auth_url` with required `scope`.
3. The user will login and then accept or deny your `scope`'s permissions.
4. If the user accepted your permissions:

	a. Handle the get request to your `redirect_uri`. It will have a query `code`, which represents the user's consent.

	b. Use `<AuthClient>.exchange_code` with this code to obtain an `Access` struct. This struct contains your tokens: `access_token` (lasting 3 hours) and `refresh_token` (lasting 60 days) *.

5. Use `get_vehicles` to get a `Vehicles` struct that has all the the ids of the owner's vehicles.
6. Create a new `Vehicle` (singular) struct using an `id` from the previous response and the `access_token` from Step 4.
7. Start making requests to the Smartcar API!

---

*\* In order to make subsequent requests, you will need to save this the tokens in the Access struct somewhere.*

*\*\* When your access token expires, use `<AuthClient>.exchange_refresh_token` on your `refresh_token` to get a fresh set.*

## Getting Started


Let's see a basic use case of `smartcar` using the [axum web framework](https://github.com/tokio-rs/axum). In this example, we will set up a simple server running on localhost 3000 to run through the flow described above, in order to get the make, model, and year of a vehicle.

See the code in [./example/getting-started.rs](https://github.com/nbry/smartcar-rust-sdk/blob/main/examples/getting-started.rs).

*For a much simpler example without a web framework integration, check out [./example/getting-started-cli.rs](https://github.com/nbry/smartcar-rust-sdk/blob/main/examples/getting-started-cli.rs).*

### Requirements

- [Rust/cargo](https://www.rust-lang.org/tools/install)
- A browser

### How to run this example

1. Clone this repo `cd` into the directory.
2. Set up a new redirect URI in your Smartcar dashboard.
	- Add `http://localhost:3000/callback`
3. Find `get_auth_client` in [./example/getting-started.rs](https://github.com/nbry/smartcar-rust-sdk/blob/main/examples/getting-started.rs) and replace the fake credentials with your actual client credentials from your dashboard.
	- The fake credentials are prefixed with `"REPLACE_WITH_YOUR_..."`.
4. Run the example by using the cargo run with the example flag*:

```
cargo run --example=getting-started
```

5. In a browser, go `http://locahost:3000/login` to see the Smartcar Connect flow. This example runs connect in Test Mode, which uses randomized data and fake cars.
	- Normally, your users will be the one going through this flow. In this example, you will be going through it yourself.
	- Choose any make and type in a fake email/password
	- e.g. username: `"blah@blah.com"`, password: `"blah"`

6. After logging in and approving permissions, you should get a JSON response with the vehicle's make, model, year, and id.

Follow along with the print statements in your terminal to see the steps!

---

*\* example/getting-started.rs has print statements that correspond to the 7-step Flow above. To minimize noise, the code below does not include the print statements.

#### Getting started (with axum web framework)

```rust
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
    // Flow - Step 1
    let auth_client = get_auth_client();

    // Here we are adding the read_vehicle_info permission so we can get
    // the make, model, and year of the vehicle. In other words, we are asking
    // the vehicle owner for permission to get these attributes.
    let scope = ScopeBuilder::new().add_permission(Permission::ReadVehicleInfo);

    // Here we build the options for creating the auth url.
    // The following option forces the "approval" page to always show up.
    let auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);

    // Flow - Step 2
    let auth_url = auth_client.get_auth_url(&scope, Some(&auth_url_options));

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
    // Flow - Step 3 completed, starting 4a

    // If user denies you access, you'll see this
    if let Some(_) = &q.error {
        return (
            StatusCode::EXPECTATION_FAILED,
            Json(json!("User delined during Smartcar Connect")),
        );
    };

    // This is the authorization code that represents the user’s consent,
    // granting you permission to read their vehicle's attributes
    // This code must be exchanged for an access token to start making requests to the vehicle.
    let code = &q.code.to_owned().unwrap();

    match get_attributes_handler(&code).await {
        Err(_) => {
            return (
                StatusCode::EXPECTATION_FAILED,
                Json(json!("attributes request failed")),
            )
        }
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

    // Flow - Step 4b
    let (access, _) = client.exchange_code(auth_code).await?;

    // Flow - Step 5
    let (vehicle_ids, _) = smartcar::get_vehicles(&access, None, None).await?;

    // Flow - Step 6
    let vehicle = Vehicle::new(&vehicle_ids.vehicles[0], &access.access_token);

    // Flow - Step 7
    let (attributes, meta) = vehicle.attributes().await?;

    Ok((attributes, meta))
}
```

