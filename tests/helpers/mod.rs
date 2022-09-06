use fantoccini::{ClientBuilder, Locator};

use url::Url;

pub(crate) async fn run_connect_flow(
    auth_url: &str,
    make: &str,
    port: &str,
) -> Result<String, fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect(format!("http://localhost:{}", port).as_str())
        .await
        .expect("failed to connect to WebDriver");

    c.goto(auth_url).await?;

    // Preamble
    c.find(Locator::Css("button#continue-button"))
        .await?
        .click()
        .await?;
    println!("connect - continue button pressed");

    // Brand Select
    let brand_button = format!("button.brand-selector-button[data-make='{}']", make);
    c.find(Locator::Css(brand_button.as_str()))
        .await?
        .click()
        .await?;
    println!("connect - brand selected: {}", make);

    // Log in
    c.find(Locator::Css("#username"))
        .await?
        .send_keys("test2@test.com")
        .await?;
    println!("connect - username typed");

    c.find(Locator::Css("#password"))
        .await?
        .send_keys("test-password")
        .await?;
    println!("connect - password typed");

    c.find(Locator::Css("#sign-in-button"))
        .await?
        .click()
        .await?;
    println!("connect - sign in button pressed");

    // Permissions approval
    c.find(Locator::Css("#approval-button"))
        .await?
        .click()
        .await?;
    println!("connect - approval button pressed");

    // Capture url and get "CODE" query value
    let u = c.current_url().await?;
    c.close().await?;

    let full_url = Url::parse(u.as_str()).expect("redirect uri with auth code");
    println!("connect - got url: {}", full_url);

    let all_queries: Vec<(String, String)> = full_url
        .query_pairs()
        .filter(|(q, _)| q == "code")
        .map(|(q, v)| (q.into_owned(), v.into_owned()))
        .collect();

    let code = &all_queries[0].1;
    println!("connect - code query isolated: {}", code);

    Ok(code.to_owned())
}

pub(crate) fn get_creds_from_env() -> (String, String, String) {
    let client_id = std::env::var("E2E_SMARTCAR_CLIENT_ID")
        .expect("Need E2E_SMARTCAR_CLIENT_ID to run e2e tests");

    let client_secret = std::env::var("E2E_SMARTCAR_CLIENT_SECRET")
        .expect("Need E2E_SMARTCAR_CLIENT_SECRET to run e2e tests");

    let redirect_uri = std::env::var("E2E_SMARTCAR_REDIRECT_URI")
        .expect("Need E2E_SMARTCAR_REDIRECT_URI to run e2e tests");

    (client_id, client_secret, redirect_uri)
}
