# TODO List

## CURRENT WORK

- integrate `cryptr` and migrate all old ENC_KEYS in:
    - [ ] db_migrate 
    - [ ] api_keys 
    - [ ] jwk 
    - [ ] clients
- adopt Webauthn cookies to cryptr and check compatibility
- move auth::rotate_jwks into Jwk Entity and adopt encryption

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- auto-apply DB backups from S3 (SQLite)
- auto-encrypted backups + backups to remote locations (ssh, nfs, s3, ...) -> postponed - should be applied to sqlite only
since postgres has pg_backrest and a lot of well established tooling anyway
- admin ui: template button for client branding: default-light + default-dark ?
- add all default claims for users https://openid.net/specs/openid-connect-core-1_0.html#ScopeClaims
- double check against https://openid.net/specs/openid-connect-core-1_0.html that everything is implemented correctly one more time
- benchmarks and performance tuning
- maybe get a nicer logo

## Stage 3 - Possible nice to haves

- impl experimental `dilithium` alg for token signing to become quantum safe 
- oidc-client (google, github, ...)
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
