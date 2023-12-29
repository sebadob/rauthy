# TODO List

## CURRENT WORK

## Leftover TODO's for v0.20.0

- UI: fix ItemTiles component flex-wrap
- UI: new version event shows `NewRauthyAdmin` instead of `NewRauthyVersion`
- UI: event `UserPasswordReset` could additionally show the corresponding E-Mail
- UI: event `UserEmailChange` could additionally show the corresponding E-Mail
- UI: event `NewRauthyVersion` should wrap the release link in `<a>`

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- impl oidc metadata `check_session_iframe`
- impl leftovers on `/userinfo` endpoint
- admin ui: template button for client branding: default-light + default-dark ?
- double check against https://openid.net/specs/openid-connect-core-1_0.html that everything is implemented correctly one more time
- benchmarks and performance tuning
- maybe get a nicer logo

## Stage 3 - Possible nice to haves

- impl experimental `dilithium` alg for token signing to become quantum safe 
- oidc-client (google, github, ...)
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
