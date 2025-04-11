# TODO List

## CURRENT WORK

- greatly reduce the allowed clock skew during token validation and make it configurable
- add `HIGHEST_COMPATIBLE_VERSION` to DB version check

### SCIM

- [ ] content type must always be `application/scim+json`
- [ ] impl core schemas https://www.rfc-editor.org/rfc/rfc7643 - check if exists already on crates.io
- [ ] For authentication, we want to use static, upfront communicated `Bearer` tokens. These are typically opaque
  to the client, but will have an expiry on the Service Provider side, like e.g. for aws.
- [ ] Consider an option like "generate Bearer token" for authentication. The idea is, that Rauthy could generate e.g.
  a `client_credentials` token for the specific client which could be used for authentication as well. This would keep
  maintenance down. However, most Service Providers probably won't support that anywhere else than on the `/Self`
  endpoint, which we don't even want to use with Rauthy.
- [ ] do NOT impl the service provider side of SCIM - Rauthy would be the client in that case
  service provider MAY be partly implemented into the `rauthy-client` though
- [ ] check how much work auto-discovery would be and if it makes sense to put in the time
- [ ] We can maybe ignore any auto discovery to reduce the complexity by a lot. Instead, rely on default SCIM v2
  schemas. In case of the endpoint URLs, a user would only have to enter 2 of them, while the specific ones could be
  derived.
    - base url for `/Users`
    - base url for `/Groups

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- Migrate `src/models/src/events/ip_blacklist_handler.rs` persistence from thread local HashMap to distributed Hiqlite
- `amr` is not correctly set to `mfa` using the `device_code` flow, even when the user has used a passkey
- check if `REFRESH_TOKEN_GRACE_TIME` can be dropped with Hiqlite
- randomize default admin user id on prod init + set email to `BOOTSTRAP_ADMIN_EMAIL` before password info logging
- "known host cookie" with connection between accounts and IPs to send out warnings in case
  of a login on a new device
- credential stuffing detection
- find a nice way to always expose the swagger UI for rauthy admins only without config
  current issue: when the session cookie is a non-host cookie with path restriction -> not working
  probably move from `/docs/v1` to `/auth/v1/docs`
- prettify the UI
- maybe get a nicer logo

### `rauthy-client` TODO's

- when implementing userinfo lookup, add an fn to validate the `at_hash` as well

## Stage 3 - Possible nice to haves

- impl experimental `dilithium` alg for token signing to become quantum safe
- custom event listener template to build own implementation?
