# rauthy-client

Minimal and safe by default client library for the [Rauthy](https://github.com/sebadob/rauthy)
project.

You can, of course, use any generic OIDC client with [Rauthy](https://github.com/sebadob/rauthy).
However, the idea of this crate is to provide the simplest possible production ready setup, with the
least amount of overhead and secure default values, if you only
use [Rauthy](https://github.com/sebadob/rauthy) anyway.

You can find examples for `actix-web`, `axum` or a fully generic framework / application in the
[Examples](https://github.com/sebadob/rauthy/tree/main/rauthy-client/examples) directory. If you
want examples for advanced things like backchannel logout or SCIM, take a look at the `axum`
example.

## Features

- `actix-web` to provide extractors and `actix`-specific implementations
- `axum` to provide extractors and `axum`-specific implementations
- `backchannel-logout` for backchannel logout support
- `device-code` for `device_code` authorization flow support
- `qrcode` only makes sense in combination with `device-code` to generate QR codes for the auth
  grant link
- `scim` for SCIM v2 support
- `userinfo` to provide additional functions to live-validate tokens against the `/userinfo`
  endpoint
- `rsa` to be able to verify RSA-signed tokens (by default, only `EdDSA` is supported)
