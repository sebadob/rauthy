# TODO List

## CURRENT WORK

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- improve the book with all the new features
- upstream auth providers
- add `at_hash` claim to the ID token
- impl oidc metadata `check_session_iframe` ?
- remove `offline_access` everywhere, because its overhead to manage and not really beneficial with webauthn?
- admin ui: template button for client branding: default-light + default-dark ?
- benchmarks and performance tuning
- maybe get a nicer logo

## Stage 3 - Possible nice to haves

- respect `display=popup` and / or `display=touch` on `/authorize`
- impl experimental `dilithium` alg for token signing to become quantum safe 
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
