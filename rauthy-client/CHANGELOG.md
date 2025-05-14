# Changelog

## v0.8.0

### OIDC Backchannel Logout

There is a new feature `backchannel-logout` which enables compatibility with Backchannel Logout. You will be able to
accept and validate `LogoutToken`s and perform a Backchannel Logout.
The [axum Example](https://github.com/sebadob/rauthy/tree/main/rauthy-client/examples/axum) shows how to use it.

### SCIM

With the feature `scim` enabled, all SCIM-related types are included. The SCIM impl is only partial, but covers every
case / combination that Rauthy might send to a client. The above
mentioned [axum Example](https://github.com/sebadob/rauthy/tree/main/rauthy-client/examples/axum) shows an example
integration.

> Both new implementations (Backchannel Logout / SCIM) also bring API extractors for `axum` out of the box. There are
> not `actix-web` extractors yet.

## v0.7.0

### Breaking

The function signature for `OidcProviderConfig::setup_from_config()` now expects the `redirect_uri` as a `String`
instead of `&str`.

### Changes

External dependencies have been bumped, and the MSRV to `1.85.0`. `ring`, which is a pretty heavy-weight dependency, has
been dropped in favor of `sha2`. `ring` was used only for S256 PKCE hashes.

### Bugfix

- The `redirect_uri` during the `/token` request has been URL encoded, which should not be the case and could lead to
  errors like `invalid redirect_uri`

## v0.6.1

Fixes the `OptionalFromRequestParts` trait impl for `axum-0.8` in a way that it makes the example work
again as expected.

## v0.6.0

Brings compatibility for `axum-0.8` by bumping `axum` and `axum_extra` dependencies.

## v0.5.0

This version bumps internal dependencies and
remediates [CVE-2024-12224](https://rustsec.org/advisories/RUSTSEC-2024-0421). The way the vulnerable `idna` crate has
been used can not lead to any security issues though.

The MSRV is also bumped to `1.71.1`.

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
