//! Learn the flow of Smartcar with a basic CLI program that prints
//! a test (i.e. fake) Tesla's attributes, odometer, charge state,
//! battery level, tire pressure, and VIN
//!
//! HOW TO RUN THIS EXAMPLE
//!
//! 1. Login and go to your Smartcar dashboard
//! 2. Get your client id, secret, and set your redirect URI
//!     - `https://example.com/auth` is a good default redirect URI
//! 3. Export them as environment variables:
//!
//! ```
//! export SMARTCAR_CLIENT_ID='<your client id>'
//! export SMARTCAR_CLIENT_SECRET='<your client secret>'
//! export SMARTCAR_REDIRECT_URI='your redirect uri'
//! ```
//!
//! 4. Run the example:
//!
//! ```
//! cargo run --example=learn-by-cli
//! ```

use colored::{ColoredString, Colorize};
use std::{io, time::Duration};

use smartcar::*;

use auth_client::{AuthClient, AuthUrlOptionsBuilder};
use response::*;
use vehicle::Vehicle;

const DELAY: u64 = 300;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let auth_client = AuthClient::from_env(true);
    let auth_url_options = AuthUrlOptionsBuilder::new().set_force_prompt(true);
    let scope = ScopeBuilder::new()
        .add_permission(Permission::ReadVehicleInfo)
        .add_permission(Permission::ReadOdometer)
        .add_permission(Permission::ReadVin)
        .add_permission(Permission::ReadCharge)
        .add_permission(Permission::ReadBattery)
        .add_permission(Permission::ReadTires);

    // Generate URL for your user to go through Smartcar Connect
    // For this example, the user is you!
    let auth_url = auth_client.get_auth_url(&scope, Some(&auth_url_options));
    _print_instructions(&auth_url).await;

    let mut auth_code = String::new();

    io::stdin()
        .read_line(&mut auth_code)
        .ok()
        .expect("Expected the query `code`");

    println!("\nYou entered: {}", auth_code.blue());

    // Exchange the authorization code (which represents a user's consent)
    // for an access struct w/access tokens.
    let (access, _) = auth_client
        .exchange_code(auth_code.trim())
        .await?;

    // Use the access token to get a list of vehicles (ids)
    let (ids, _) = smartcar::get_vehicles(&access, None, None).await?;

    // Using the access token, start sending requests to the first vehicle in the list
    let vehicle = Vehicle::new(&ids.vehicles[0], &access.access_token);

    let (attributes, _) = vehicle.attributes().await?;
    let (odometer, _) = vehicle.odometer().await?;
    let (vin, _) = vehicle.vin().await?;
    let (battery_level, _) = vehicle.battery_level().await?;
    let (charging_status, _) = vehicle.charging_status().await?;

    _retro_narrative(attributes, odometer, vin, battery_level, charging_status).await;

    Ok(())
}

///////////////////////////////////////////////////////////
// The functions below are only for for print statements //
// They are not necessarily related to the use and       //
// functionality of the smartcar sdk                     //
///////////////////////////////////////////////////////////

async fn _print_instructions(auth_url: &str) {
    let mut message = "\nSMARTCAR RUST SDK - GETTING STARTED - CLI"
        .yellow()
        .bold();
    println!("{message}");

    println!(
        "\nIn this example, you you will be playing both the roles of the application developer"
    );
    println!("AND the vehicle owner (i.e the user).");

    println!("\nYou will begin by going through Smartcar Connect.");
    println!("Smartcar Connect is the authorization flow that your users will go through.");
    println!("Through Connect, your users can can grant your application permission(s)");
    println!("to make requests to their vehicle.");

    println!("\nYou are going to request permission to get the attributes,");
    println!("odometer reading, vin, charge state, and battery level of a Tesla!");

    message = "\nP.S. Be a good sport and actually pick a Tesla :)".yellow();
    println!("{message}");

    println!("\nWe are going to be using test mode with mock cars. While in test mode,");
    println!("when prompted, you can choose any car and in enter fake credentials");
    message = "e.g. blah@blah.com, pass1234".white();
    println!("{message}");

    println!("\nAfter going through Smartcar Connect, you will be redirected to your");
    println!("REDIRECT URI with an auth code (i.e. with query of `code`).");
    println!("For example, if your redirect URI is:");
    message = "http://fake-redirect-uri.com/".cyan();
    println!("{message}");

    println!("\n...upon permission approval, it will be called like this:");
    message = "http://fake-redirect-uri.com/callback?code=12345-abcde".cyan();
    println!("{message}");

    println!("\nIn this example is, your auth `code` is:");
    message = "12345-abcde".purple();
    println!("{message}");

    println!("\nPaste the following URL in a browser to proceed with the Smartcar Connect flow:");
    let a = auth_url.clone().green();
    println!("\n{a}");

    message = "After you finish, paste your code below:"
        .red()
        .bold();
    println!("\n{message}");
}

async fn _retro_narrative(
    attributes: VehicleAttributes,
    odometer: Odometer,
    vin: Vin,
    battery_level: BatteryLevel,
    charging_status: ChargingStatus,
) {
    async fn _print_and_delay(msg: &ColoredString) {
        println!("{}", msg);
        tokio::time::sleep(Duration::from_millis(DELAY)).await;
    }

    let mut _m = String::new();
    _m = "Successfully exchanged the code for access tokens".to_string();
    _print_and_delay(&_m.white()).await;

    _m = "\nSuccessfully got the list of vehicle ids".to_string();
    _print_and_delay(&_m.white()).await;

    _m = "\nUsing the first vehicle".to_string();
    _print_and_delay(&_m.white()).await;

    _m = format!(
        "\nGot vehicle's id, make, model, and year!\n\n{:#?}",
        attributes
    );
    _print_and_delay(&_m.purple()).await;

    _m = format!("\nGot the vehicle's odometer reading!\n\n{:#?}", odometer);
    _print_and_delay(&_m.red()).await;

    _m = format!("\nGot the vehicle's vin!\n\n{:#?}", vin);
    _print_and_delay(&_m.yellow()).await;

    _m = format!("\nGot the vehicle's battery!\n\n{:#?}", battery_level);
    _print_and_delay(&_m.green()).await;

    _m = format!("\nGot the vehicle's charge!\n\n{:#?}", charging_status);
    _print_and_delay(&_m.blue()).await;

    println!("\n\nEND");
}
