# TODO List

## CURRENT WORK

## TODO before v0.16.0

- add a new table that keeps track about when password expiry / reset emails were sent out to avoid duplicates
- update the book with all the new features

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- NATS events stream or maybe internal one?
- add an 'ip blacklist' feature?
- add all default claims for users https://openid.net/specs/openid-connect-core-1_0.html#ScopeClaims
- double check against https://openid.net/specs/openid-connect-core-1_0.html that everything is implemented correctly one more time
- benchmarks and performance tuning

## Stage 3 - Possible nice to haves

- auto-encrypted backups + backups to remote locations (ssh, nfs, s3, ...) -> postponed - should be applied to sqlite only
since postgres has pg_backrest and a lot of well established tooling anyway
- oidc-client (google, github, ...)
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
