# TODO List

## CURRENT WORK

- greatly reduce the allowed clock skew during token validation and make it configurable
- add `HIGHEST_COMPATIBLE_VERSION` to DB version check

### SCIM

- impl retry scheduler for failed tasks, just like for backchannel logout

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

## Stage 3 - Possible nice to haves

- impl experimental `dilithium` alg for token signing to become quantum safe
- custom event listener template to build own implementation?
