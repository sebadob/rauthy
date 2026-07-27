# Device Authorization Grant Example

To make this example work (without any changes to `main.rs`), create client on your `localhost`
Rauthy:

- id: `device`
- non-confidential
- redirect URI `http://localhost:3000`
- enable `device_code` flow
- enable `S256` PKCE
