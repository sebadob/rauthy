# rauthy-client

Minimal and safe by default client library for the [Rauthy](https://github.com/sebadob/rauthy) project.

You can of course use any generic OIDC client with [Rauthy](https://github.com/sebadob/rauthy).
However, the idea of this crate is to provide the simplest possible production ready setup, with the least amount of
overhead and secure default values, if you only use [Rauthy](https://github.com/sebadob/rauthy) anyway.

This client should work without any issues with other OIDC providers as well, as long as they support the S256 PKCE
flow. However, this was only tested against [Rauthy](https://github.com/sebadob/rauthy).

You can find examples for `actix-web`, `axum` or a fully generic framework / application in the
[Examples](https://github.com/sebadob/rauthy/tree/main/rauthy-client/examples) directory.
