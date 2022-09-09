# Contributing

## Running Tests

You will need [geckodriver](https://github.com/mozilla/geckodriver/releases) (and subsequently, Firefox) to run tests.

One way to install geckodriver is to use npm - https://www.npmjs.com/package/geckodriver

The tests make requests to the Smartcar API, so you'll need to create an application on Smartcar and get your client id and client secret. You'll also need to add the testing redirect URI to your application.

1. Create an application on the [developer dashboard](https://dashboard.smartcar.com)
2. Add `https://example.com/auth` as a redirect URI
3. Pass your client credentials as environment variables:

```
export E2E_SMARTCAR_CLIENT_ID='<Your client id>'
export E2E_SMARTCAR_CLIENT_SECRET='<Your client secret>'
export E2E_SMARTCAR_REDIRECT_URI='<Your redirect URI>'
```

4. Run:

```
geckodriver --port 4444& cargo test -- --nocapture
```
