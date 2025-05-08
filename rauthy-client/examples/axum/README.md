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
