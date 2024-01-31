# TODO List

## CURRENT WORK

### Dynamic Client Registration TODO

https://openid.net/specs/openid-connect-registration-1_0.html

- [x] define new POST request registration object by RFC
- [x] implement registration endpoint
- [x] config variable for token-protected dynamic registration
- [x] DB migrations and create `clients_dyn` table with all necessary information 
- [x] implement a GET endpoint specific for dynamic clients in the correct format by RFC
- [x] issue `registration_token`s 
- [x] implement a PUT endpoint for clients to self-modify
- [x] add `ClientDyn` to secret migrations task to properly migrate `registration_token`s
- [x] check for dynamic client during final token creation and efficiently update `last_used` in `clients_dyn`
- [x] implement an efficient way to auto-delete unused clients to prevent spam and bots
- [x] make sure ClientDyn cache is cleaned up properly on Client::delete()
- [x] config var for an auto-cleanup cron job for dyn clients
- [x] some kind of rate-limiting for an open dyn client reg endpoint
- [x] not cache the result -> only used in Admin UI and may grow very big because of dyn client reg

## TODO next

- respect `login_hint` in the authorize ui
- add `at_hash` claim to the ID token
- does it make sense for Rauthy to impl `acr_values` ?
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
