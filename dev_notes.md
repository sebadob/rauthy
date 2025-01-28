# TODO List

## CURRENT WORK

- `amr` is not correctly set to `mfa` using the `device_code` flow, even when the user has used a passkey

### Missing i18n (zh/ko) for Svelte 5 Migration:

- `t.authorize.expectingPasskey`
- `t.authorize.requestExpired`
- `t.common.close`
- `t.common.changeTheme`
- `t.common.selectI18n`
- `t.common.month`
- `t.common.months`
- `t.common.weekDaysShort`
- `t.common.year`

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

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
