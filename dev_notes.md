# TODO List

## CURRENT WORK

- admin ui component to show blacklisted IPs
- maybe functionality to manually blacklist IPs?

## TODO v0.17

- admin ui component for the new ApiKeys
- add event for a user password reset
- rauthy-notify crate for matrix + slack notifiers
- latest app version check via github api
- show current app version in admin ui
- add event for new app version available

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- add all default claims for users https://openid.net/specs/openid-connect-core-1_0.html#ScopeClaims
- double check against https://openid.net/specs/openid-connect-core-1_0.html that everything is implemented correctly one more time
- benchmarks and performance tuning

## Stage 3 - Possible nice to haves

- auto-encrypted backups + backups to remote locations (ssh, nfs, s3, ...) -> postponed - should be applied to sqlite only
since postgres has pg_backrest and a lot of well established tooling anyway
- oidc-client (google, github, ...)
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
