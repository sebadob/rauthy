# Basic Axum Example

This example shows the basic usage with `axum` + `rauthy-client`.

The config is set up to work with the default test container as mentioned in the
Readme https://github.com/sebadob/rauthy?tab=readme-ov-file#getting-started . You need to add the env var
`-e BACKCHANNEL_DANGER_ALLOW_HTTP=true` to the startup command though to allow plain HTTP URLs.

To make it work, start the container and create a non-confidential client with the id `test` and set:

- Redirect URI: http://localhost:3000/callback
- Post Logout Redirect URI: http://localhost:3000
- Backchannel Logout URI: http://localhost:3000/logout

Then you can start the example with `cargo run` and open the UI on `localhost:3000`