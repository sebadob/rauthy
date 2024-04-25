# TODO List

## CURRENT WORK

https://datatracker.ietf.org/doc/html/rfc8628

oauth2 device auth flow for IoT and embedded devices:

- [ ] API endpoint for initiating a device flow
- [ ] API endpoints for fetching all information about an existing one
- [ ] UI to show to the user for investigation + accept
- [ ] implement the logic either into `rauthy-client` or maybe an independent new crate, if one does not
  yet exist for the rust ecosystem
- [ ] create a fully working example with `rauthy` + `rauthy-client` on how to use it with a CLI tool

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- add `at_hash` claim to the ID token
- update the book with all the new features
- handle `offline_access` properly again - either decide to fully remove or support it everywhere
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
