# TODO List

## CURRENT WORK

## TODO before v0.23.0

- link to upstream provider with existing account
- sessions view needs pagination
- check out the possibility to include SCIM

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- prettify the UI
- update the book with all the new features
- benchmarks and performance tuning
- maybe get a nicer logo

### Notes for performance optimizations

- all the `get_`s on the `Client` will probably be good with returning slices instead of real Strings
  -> less memory allocations

### `rauthy-client` TODO's

- check possibility of `no_std` with `device_code` flow -> embedded devices
- automatic `refresh_token` handler for `device_code` grant flow
- when implementing userinfo lookup, add an fn to validate the `at_hash` as well

## Stage 3 - Possible nice to haves

- respect `display=popup` and / or `display=touch` on `/authorize`
- impl experimental `dilithium` alg for token signing to become quantum safe
- 'rauthy-migrate' project to help migrating to rauthy? probably when doing benchmarks anyway and use it
  for dummy data?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
