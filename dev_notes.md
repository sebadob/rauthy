# TODO List

## CURRENT WORK

## TODO next

- respect `login_hint` in the authorize ui
- add `at_hash` claim to the ID token
- respect `request_uri` during auth
- fix broken link build in Admin UI if a new version is available

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- impl oidc metadata `check_session_iframe` ?
- remove `offline_access` everywhere, because its overhead to manage and not really beneficial with webauthn?
- admin ui: template button for client branding: default-light + default-dark ?
- double check against https://openid.net/specs/openid-connect-core-1_0.html that everything is implemented correctly
- benchmarks and performance tuning
- maybe get a nicer logo

## Stage 3 - Possible nice to haves

- respect `display=popup` and / or `display=touch` on `/authorize`
- impl experimental `dilithium` alg for token signing to become quantum safe 
- oidc-client (google, github, ...)
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
