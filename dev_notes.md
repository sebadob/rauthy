# TODO List

## CURRENT WORK

## TODO before v0.22

- validate `db_migrate` is formed correctly after all the new migrations
  sqlite -> postgres -> sqlite again
- maybe migrate the old client logos handling to the new optimized webp flow?
- add `contact` and `application_url` to the `clients` table for future features
- document "Login with Github" guide
- specifically set cache headers on client logos

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- update the book with all the new features
- add `at_hash` claim to the ID token
- remove `offline_access` everywhere, because its overhead to manage and not really beneficial with webauthn?
- benchmarks and performance tuning
- maybe get a nicer logo

### Notes for performance optimizations

- all the `get_`s on the `Client` will probably be good with returning slices instead of real Strings -> less memory
  allocations

## Stage 3 - Possible nice to haves

- respect `display=popup` and / or `display=touch` on `/authorize`
- impl experimental `dilithium` alg for token signing to become quantum safe
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
