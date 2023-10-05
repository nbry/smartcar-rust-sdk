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

## Adding a new vehicle endpoint

1. Check if we have a new permission associated with the endpoint
  - Add permission to the public enum `Permission` in ./lib.rs
  - Add permission to the vector in `ScopeBuilder.with_all_permissions`
2. Create a new response struct in src/response.response.rs
3. If this is a GET request, add the response struct to `SmartcarResponseBody` enum so it can be batched
4. Create the vehicle struct method in src/vehicle.rs

## Adding a make-specific endpoint

To use make-specific requests, users should be using the public `vehicle.request` function and manually use or serialize the response.
Therefore, we really only need to check if there is a new permission related to the make-specific endpoint.

1. Check if we have a new permission associated with the endpoint
  - Add permission to the public enum `Permission` in ./lib.rs
  - Add permission to the vector in `ScopeBuilder.with_all_permissions`

