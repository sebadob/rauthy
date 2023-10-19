# TODO List

## CURRENT WORK

## TODO v0.17

The 0.17 release will be mostly about auditing and events, for instance things like "3 invalid login attempts from IP XY"

### events

task 'NATS events stream or maybe internal one' from the stage 2 todo

- implement base foundation for sending events internally and just log them to console in first version
- add the configurable option to persist events
- create a new events section in the admin ui to watch for new ones and
start with simple polling every few seconds first because of the HA_MODE problematic with SSE
- implement an SSE endpoint for listening to events in real time
    - for SI deployments with SQLite, this is a no-brainer -> just copy each event to the corresponding tx 
    - for HA deployments:
        - research sqlx + postgres + CDC to avoid additional deployment needs (or maybe just listen / notify? KISS?)
        - if postgres does not work out nicely, think about using a NATS deployment for this task
- switch the UI component to the SSE stream
- add some way of configuring an email (or webhook, slack, ... ?) which gets messages depending on configured event level
 

### other features (some may come with v0.18 depending on amount of work)

- impl ApiKeyEntity in enc keys migrations
- add a way of detecting brute force or DoS attempts from certain IPs
- add an 'ip blacklist' feature
- add 'alg' in well-known jwks
- create an optional config to auto-blacklist IPs that have been detected doing brute force or DoS
  think about the bigger picture here, maybe do this in 2 stages, like short block after 5 bad logins, 24h block after 10, ...
- admin ui component to show blacklisted IPs
- maybe functionality to manually blacklist IPs?

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- maybe add a 'Static Admin API Token' feature: Create static service API tokens with very specific access rights to
the API for certain automated tasks from the outside. (?)
- add all default claims for users https://openid.net/specs/openid-connect-core-1_0.html#ScopeClaims
- double check against https://openid.net/specs/openid-connect-core-1_0.html that everything is implemented correctly one more time
- benchmarks and performance tuning

## Stage 3 - Possible nice to haves

- auto-encrypted backups + backups to remote locations (ssh, nfs, s3, ...) -> postponed - should be applied to sqlite only
since postgres has pg_backrest and a lot of well established tooling anyway
- oidc-client (google, github, ...)
- 'rauthy-migrate' project to help migrating to rauthy?
- custom event listener template to build own implementation? -> only if NATS will be implemented maybe?
