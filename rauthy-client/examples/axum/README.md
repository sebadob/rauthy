# Basic Axum Example

This example shows the basic usage with `axum` + `rauthy-client`.

The config is set up to work with a local Rauthy instance running in `DEV_MODE`. If you want to fully test
the example, you should clone Rauthys repo and `just run` it. Then log in with the default credentials
`admin@localhost / 123SuperSafe`.

Then, to make it work, create a non-confidential client with the id `test` and set:

- Redirect URI: http://localhost:3000/callback
- Post Logout Redirect URI: http://localhost:3000
- Backchannel Logout URI: http://localhost:3000/logout

Then you can start the example with `cargo run` and open the UI on `localhost:3000`.

If you have any other setup, keep in mind that in case of Backchannel Logouts, Rauthy needs to be able to reach
the client directly.

## SCIM

This example also shows a very simple SCIM implementation. You can test it if you naviagte to

```
http://localhost:3000/data
```

And then activate SCIM for the `test` client via Rauthy's Admin UI.

The `SCIM Base URI` is `http://localhost:3000/scim/v2`. Rauthy will derive all the other endpoints based on this
URI like defined in the RFC. The bearer token is `123SuperSafeSCIM` as set in the `RauthyConfig` in `main.rs`.

The simple test page will automatically refresh every 2 seconds and you should see all User / Group data being updated
in real-time if you have set everything up correctly.