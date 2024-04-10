# TODO List

## CURRENT WORK

## TODO before v0.22

- maybe migrate the old client logos handling to the new optimized webp flow?
- document "Login with Github" guide

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- update the book with all the new features
- add `at_hash` claim to the ID token
- remove `offline_access` everywhere, because its overhead to manage and not really beneficial with webauthn?
- benchmarks and performance tuning
- maybe get a nicer logo

## Stage 3 - Possible nice to haves

- respect `display=popup` and / or `display=touch` on `/authorize`
- impl experimental `dilithium` alg for token signing to become quantum safe
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
