# Internal development notes

## Build

To build the project, a few things have to be done:
- `cd frontend`
- `npm install`
- `./build.sh`
- `cd ..`
- Rauthy Dev Build: `cargo build` - Prod Build: `cargo build --release`

## Testing

### Install Dependencies

TODO

### Execute tests:

start the backend:

`cargo run test`

in another terminal:

`cargo test`

## TODO List

### CURRENT WORK

### Stage 1 - essentials

[x] finished

### Stage 2 - features - do before v1.0.0

- cleanup
- benchmarks and performance tuning
- double check against https://openid.net/specs/openid-connect-core-1_0.html that everything is implemented correctly one more time

### Stage 3 - Possible nice to haves

- add temporary users
- auto-encrypted backups + backups to remote locations (ssh, nfs, s3, ...) -> postponed - should be applied to sqlite only
since postgres has pg_backrest and a lot of well established tooling anyway
- when a user changes his email address, set email to not verified again and send a validation email
- add all default claims for users https://openid.net/specs/openid-connect-core-1_0.html#ScopeClaims
- oidc-client (google, github, ...)
- 'rauthy-migrate' project to help migrating to rauthy?
- add tracing-actix-web + opentelemetry
- NATS events stream or maybe internal one?
