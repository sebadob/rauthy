# TODO List

## CURRENT WORK

## TODO before v0.22

- update matrix SDK dependencies to be able to update all deps to the latest versions
  -> maybe drop matrix-sdk in favor of ruma-client because of huge dependencies and the full sdk is
  not needed at all? we actually only need the client to post events
- BUG: wrong path in the default Dockerfile which points to the DEV TLS certificates
  -> Fix has been pushed - test and validate with next nightly or beta image build
- create and test a "Login with Github" flow and add documentation about it in the book
- after matrix-sdk has been fixed / updated, update all deps

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
