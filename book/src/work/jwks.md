# JSON Web Keys

JSON Web Keys (JWKs) are used to sign JWT tokens. Multiple JWKs form a JSON Web Key Set (JWKS). These are private
/ public key pairs Rauthy generates internally in a cryptographically secure way. All of them are generated at
once, one for each algorithm Rauthy supports signing tokens with, which are

- RS256
- RS384
- RS512
- EdDSA

The **RSA** algorithms exist for compatibility. The `RS256` is the only mandatory algorithm by the OIDC RFC and
the `RS384`and `RS512` basically come for free, when you implement `RS256`. However, these algorithms, produce
pretty big signatures and are very slow to generate.

Rauthy does not support for **ECDSA** keys, because they are the worst option in this scenario in my opinion.
`ECDSA` keys have the advantage that they produce way smaller signatures than `RSA` keys and can be generated pretty
fast, but are slow at token validations. Tokens need to be validated with each single request, so you want to this
to be as fast as possible (without sacrificing security of course).

The best option is **EdDSA**, which uses `ed25519` keys. It is the fastest option at signing and validation, fast to
generate and produces the smallest signatures and therefore total token size. These are the default when you create
a new client, but some applications do not support them. If you have trouble logging in because of a problem with the
signature, try to change it to `RS256`.

## Key Rotation

As long as your private key's do not leak, you technically do not need to rotate keys and generate new ones.
However, it is good practice to do this regularly. Usually, you don't need to care about this. Rauthy rotates its JWKS
**automatically** with default settings. This will happen on each 1. of the month at 03:30 in the morning.  
You can change this behavior with the following setting:

```
# JWKS auto rotate cronjob. This will (by default) rotate all JWKs every
# 1. day of the month. If you need smaller intervals, you may adjust this
# value. For security reasons, you cannot fully disable it.
# In a HA deployment, this job will only be executed on the current cache
# leader at that time.
# Format: "sec min hour day_of_month month day_of_week year"
# default: "0 30 3 1 * * *"
JWK_AUTOROTATE_CRON="0 30 3 1 * * *"
```

If you however had a secret's leak or something like this, you can of course rotate **manually**. You just need to log
in to the Admin UI:  
`Config` -> `JWKS` -> `Rotate Keys`

```admonish info
When Rauthy does a key rotation, the old keys will not be deleted immediately, because they will be needed for some
additional time to verify existing tokens. 

A rotation generates new keys which will then be used for any upcoming token signatures. The old keys will be 
auto-deleted after 30 days, you don't need to care about cleanup.
```

## Rotation Event

Rauthy creates a new event when keys are rotated. The default level is `notice`. If you want to change this,
you can do so with:

```
# The level for the generated Event after the JWKS has been rotated
# default: notice
EVENT_LEVEL_JWKS_ROTATE=notice
```