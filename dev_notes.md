# TODO List

## CURRENT WORK

## TODO before v0.23.0

- accept an optional `name` param during initial `device_code` request to have better readable first
  name for new devices OR redirect to account view and open devices directly?
- cleanup scheduler that looks for expired devices like every 24h

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- sessions view needs pagination + option to revoke a single session
- `USERINFO_STRICT` config option -> strict session / device id checking against DB
- add `at_hash` claim to the ID token
- handle `offline_access` properly again - either decide to fully remove or support it everywhere
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
