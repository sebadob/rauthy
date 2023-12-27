# TODO List

## CURRENT WORK

## Leftover TODO's for v0.20.0

- add additional user values for self-modify in the accounts view
- add more default claims for users https://openid.net/specs/openid-connect-core-1_0.html#ScopeClaims
- add DB migration for the new user values

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- admin ui: template button for client branding: default-light + default-dark ?
- double check against https://openid.net/specs/openid-connect-core-1_0.html that everything is implemented correctly one more time
- benchmarks and performance tuning
- maybe get a nicer logo

## Stage 3 - Possible nice to haves

- impl experimental `dilithium` alg for token signing to become quantum safe 
- oidc-client (google, github, ...)
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
