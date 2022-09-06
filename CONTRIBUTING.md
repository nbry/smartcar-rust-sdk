# Contributing

## Running Tests

You will need [geckodriver](https://github.com/mozilla/geckodriver/releases) (and subsequently, Firefox) to run tests.

The tests make requests to the Smartcar API, so you'll need to create an application on Smartcar and get your client id and client secret. You'll also need to add the testing redirect URI to your application.

1. Create an application on the [developer dashboard](https://dashboard.smartcar.com)
2. Add `https://example.com/auth` as a redirect URI
3. Pass the client id and client secret to the tests as environment variables

```
export E2E_SMARTCAR_CLIENT_ID='<Your client id>'
export E2E_SMARTCAR_CLIENT_SECRET='<Your client secret>'
export E2E_SMARTCAR_REDIRECT_URI='<Your redirect URI>'
```

4. Open another shell and run geckodriver on port 4444

5. `cargo test -- --nocapture` (for print statements)

