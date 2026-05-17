# Changelog

## v0.14.1

After the new custom JWT validation, the token expiration was not checked in `v0.14.0` when no
allowed clock skew was specified. This was fixed in this version. If you use `v0.14.0`, update
immediately. This version also sets a (for now) hardcoded allowed clock skew of 5 seconds for the
validation.

## v0.14.0

This update comes with a few but easy to fix **breaking changes**.

### Breaking

While fixing a public key decode error for RSA keys, the JWT token dependency broke minor versions
(again, unfortunately). Because of this, a part of the custom JWT token implementation from Rauthy
was copied over and adjusted. This was a performance boost, and it's more light-weight, but it came
with breaking changes:

- Quite a few claims like `iat`, `nbf`, `exp`, ..., are not on the base level anymore, but can be
  found in `.common` for the given claims. This reduces maintenance overhead and error
  possibilities, since all tokens share these claims anyway. There is no need to define them
  multiple times anymore.
- `PrincipalOidc.expires_at_ts` is not an `Option<_>` anymore and was changed to an `i64` to match
  the behavior of other crates when dealing with Unix timestamps.
- When using `actix-web`, the `enc_key` for some functions now must be `Bytes` instead of `&[u8]`

The default `EdDSA` algorithm that Rauthy uses is superior to RSA in almost all cases. The new
implementation only validates these keys by default. If you want (or need) to use RSA, you can
enable the `rsa` feature.

The Rust edition was bumped to `2024`, which also came with an MSRV bump to `1.88.0`.

> NOTE: This change added the `rsa` dependency for RSA validation (`rsa` feature only). This shows
> an error on `cargo audit` being vulnerable to the Marvin Attack. This is NOT the case how it's
> used in this crate. Technically, the latest RC of the crate should not be vulnerable anymore, but
> it's not listed yet, since it's not stable. However, the Marvin Attack is NOT possible when it's
> used for validating tokens. To abuse this vulnerability, you would need to do a TLS handshake over
> a public network, which we never do here. I could have used non-Rust crates to achieve the same
> goal, but they come with other issues.

### Bugfix

- When you were using RSA-signed tokens before, the validation would fail because of a missing
  base64-decode step before building the public key.

## v0.13.0

This release makes it possible to have a interrupt-free migration to Rauthy v0.35+, which changed
the default Issuer. It now has a trailing `/` to bring more compatibility for some scenarios, where
a client constructs URLs in a way without checking for trailing `/`. This is e.g. the case when a
client like the Matrix Javascript SDK constructs the `openid-configuration` lookup URL with the
default JS URL constructor in combination with a custom path. To make this work, the isser must have
a trailing `/`.

With this version, the `rauthy-client` will accept both versions, and it will check if you provided
an issuer with trailing `/` or not, and append one if necessary. This means you should upgrade
clients to this version BEFORE upgrading Rauthy to v0.35+, which will make the transition smooth and
without any interrupt or config changes. You can even convert your config to the trailing `/` issuer
beforehand.

### Breaking

The claim name in `JwtIdClaims` when the `phone` scope was requests did not match the OIDC RFC. It
was called `phone` when it should have been `phone_number`.

Apart from that, the HTTP client does not embed TLS certificates anymore, but instead uses the
platform verifier to reduce binary size in most situations.

## v0.12.0

The `timezone` is now available for `ScimUser`, when used in combination with Rauthy v0.34.4+. Apart
from that, there was a bump in MSRV to `1.87.0`.

## v0.11.0

### Breaking

The `preferred_username` does not exist in the `access_token` and therefore not in the
`OidcPrincipal` anymore. It now can be found in the `IdClaims` only when the `profile` scope was
requested during login. This matches the latest changes for Rauthy since the `preferred_username`
got configurable by users on their own. In addition, you will now have access to `zoneinfo` if the
user has a timezone other than `UTC` configured.

## v0.10.0

- A new `ScimToken` has been added which can be used in SCIM endpoints to have an easy validation of
  the token without any manual work. It comes with an `axum` extractor and the usage is shown in the
  `axum` example. It is necessary on endpoints without any `Scim*` extractors like `GET /users/{id}`
  for instance.
- Some SCIM types also had changed values and some `Option<_>` values have been changed into
  non-optional, even if they are optional by the SCIM RFC. The reason they are made mandatory is,
  because Rauthy will reject request when some specific values do not exist. Other values like
  `_ref` in some situations have been removed completely, because Rauthy does not care about them at
  all, and they are unnecessary overhead.
- The `LogoutToken.sub` claims was not set properly as well in some situations and the value was
  missing completely.
- The `picture` claim that Rauthy sends by now has been added to `JwtIdClaims`.

## v0.9.0

### Breaking

The type of the `postal_code` in the `id_token` -> address claims has been changed from `i32` to
`String`. This has been changed with Rauthy `v0.30` for more compatibility.

## v0.8.0

### OIDC Backchannel Logout

There is a new feature `backchannel-logout` which enables compatibility with Backchannel Logout. You
will be able to accept and validate `LogoutToken`s and perform a Backchannel Logout.
The [axum Example](https://github.com/sebadob/rauthy/tree/main/rauthy-client/examples/axum) shows
how to use it.

### SCIM

With the feature `scim` enabled, all SCIM-related types are included. The SCIM impl is only partial,
but covers every case / combination that Rauthy might send to a client. The
above-mentioned [axum Example](https://github.com/sebadob/rauthy/tree/main/rauthy-client/examples/axum)
shows an example integration.

> Both new implementations (Backchannel Logout / SCIM) also bring API extractors for `axum` out of
> the box. There are not `actix-web` extractors yet.

## v0.7.0

### Breaking

The function signature for `OidcProviderConfig::setup_from_config()` now expects the `redirect_uri`
as a `String` instead of `&str`.

### Changes

External dependencies have been bumped, and the MSRV to `1.85.0`. `ring`, which is a pretty
heavy-weight dependency, has been dropped in favor of `sha2`. `ring` was used only for S256 PKCE
hashes.

### Bugfix

- The `redirect_uri` during the `/token` request has been URL encoded, which should not be the case
  and could lead to errors like `invalid redirect_uri`

## v0.6.1

Fixes the `OptionalFromRequestParts` trait impl for `axum-0.8` in a way that it makes the example
work again as expected.

## v0.6.0

Brings compatibility for `axum-0.8` by bumping `axum` and `axum_extra` dependencies.

## v0.5.0

This version bumps internal dependencies and
remediates [CVE-2024-12224](https://rustsec.org/advisories/RUSTSEC-2024-0421). The way the
vulnerable `idna` crate has been used can not lead to any security issues though.

The MSRV is also bumped to `1.71.1`.

## v0.4.0

### BREAKING

The `rauthy_client::init()` has been renamed to `rauthy_client::init_with()` and a new
`rauthy_client::init()` has been added which just uses safe defaults.

### Changes

- renamed `rauthy_client::init()` to `rauthy_client::init_with()` and added a new
  `rauthy_client::init()` with safe defaults
- added a new feature `device-code` which will make it possible to use the Device Authorization
  Grant flow with this client
- added a new feature `userinfo` which will provide an additional `fetch_userinfo()` fn for the
  `PrincipalOidc`

## v0.3.0

- Changes the `RauthyConfig` to now accept `ClaimMapping` for `admin_claim` and `user_claim`. This
  provides a lot more flexibility how user roles can be mapped without prior interaction on the
  server side.

## v0.2.0

- deprecated `uid` claim has been removed from the `id_token` and `uid` is not correctly mapped to
  `sub`
- internal API improvements to avoid future mistakes during maintenance
- dependency updates

## v0.1.0

Initial Version
