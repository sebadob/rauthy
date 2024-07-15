# TODO List

## CURRENT WORK

## Documentation TODO

- JWKS rotation
- secrets migration
- custom scopes
- ephemeral clients
- mention i18n possibilities
- cookie behavior with password + passkey accounts
- E-Mail templates
- Session setup / timeouts
- proxy mode
- Swagger UI (no external by default)

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- prettify the UI
- update the book with all the new features
- maybe get a nicer logo

### `rauthy-client` TODO's

- when implementing userinfo lookup, add an fn to validate the `at_hash` as well

## Stage 3 - Possible nice to haves

- impl experimental `dilithium` alg for token signing to become quantum safe
- 'rauthy-migrate' project to help migrating to rauthy? probably when doing benchmarks anyway and use it
  for dummy data?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
