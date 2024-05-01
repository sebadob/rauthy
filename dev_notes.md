# TODO List

## CURRENT WORK

## TODO before v0.23.0

- add `at_hash` claim to the ID token
- fix button label misplacement on chrome + ugly tab bar
- sessions view needs pagination
- `refresh_token` handler in rauthy-client for `device_code` grant flow
- check possibility of `no_std` for rauthy-client + device_code

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- update the book with all the new features
- benchmarks and performance tuning
- maybe get a nicer logo

### Notes for performance optimizations

- all the `get_`s on the `Client` will probably be good with returning slices instead of real Strings
  -> less memory allocations

## Stage 3 - Possible nice to haves

- respect `display=popup` and / or `display=touch` on `/authorize`
- impl experimental `dilithium` alg for token signing to become quantum safe
- 'rauthy-migrate' project to help migrating to rauthy? probably when doing benchmarks anyway and use it
  for dummy data?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
