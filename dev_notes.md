# TODO List

## CURRENT WORK

TODO before v0.30:

- add just recipe that archives a pre-built static HTML folder for FreeBSD like systems
- update book with the "new" K8s sts setup to show how to do an HA deployment
- double check weird backup push error on staging

## Stage 1 - essentials

[x] finished

## Stage 2 - features - do before v1.0.0

- `amr` is not correctly set to `mfa` using the `device_code` flow, even when the user has used a passkey
- check if `REFRESH_TOKEN_GRACE_TIME` can be dropped with Hiqlite
- "known host cookie" with connection between accounts and IPs to send out warnings in case
  of a login on a new device

## Stage 3 - Possible nice to haves

- impl experimental `dilithium` alg for token signing to become quantum safe
- custom event listener template to build own implementation?
