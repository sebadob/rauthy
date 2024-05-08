# Changelog

## v0.4.0

### BREAKING

The `rauthy_client::init()` has been renamed to `rauthy_client::init_with()` and a new `rauthy_client::init()`
has been added which just uses safe defaults.

### Changes

- renamed `rauthy_client::init()` to `rauthy_client::init_with()` and added a new `rauthy_client::init()`
  with safe defaults
- added a new feature `device-code` which will make it possible to use the Device Authorization Grant flow
  with this client
- added a new feature `userinfo` which will provide an additional `fetch_userinfo()` fn for the `PrincipalOidc`

## v0.3.0

- Changes the `RauthyConfig` to now accept `ClaimMapping` for `admin_claim` and `user_claim`.
  This provides a lot more flexibility how user roles can be mapped without prior interaction on
  the server side.

## v0.2.0

- deprecated `uid` claim has been removed from the `id_token` and `uid` is not correctly mapped to `sub`
- internal API improvements to avoid future mistakes during maintenance
- dependency updates

## v0.1.0

Initial Version
