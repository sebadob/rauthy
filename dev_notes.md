# TODO List

## CURRENT WORK

## Documentation TODO

- breaking: only a single container from now on
- `HealthResponse` response has been changed with Hiqlite -> breaking change
- database backup config has been changed slightly
- restore from backup has changed slightly
- write a small guide on how to migrate from existing sqlite to hiqlite

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- events view wide -> button height 30px -> error with overflow
- input validation in account view broken for first / last name -> backend requests 2 chars +
- find a nice way to simply always expose the swagger UI for rauthy admins only without config
  current issue: when the session cookie is a non-host cookie with path restriction -> not working
  probably move from `/docs/v1` to `/auth/v1/docs`
- prettify the UI
- update the book with all the new features
- maybe get a nicer logo

### `rauthy-client` TODO's

- when implementing userinfo lookup, add an fn to validate the `at_hash` as well

## Stage 3 - Possible nice to haves

- impl experimental `dilithium` alg for token signing to become quantum safe
- custom event listener template to build own implementation?
