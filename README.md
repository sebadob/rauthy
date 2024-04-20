# Rauthy

**CAUTION:**  
There has not been any third party security audit for this project.  
Use this software at your own risk!

**INFO:**  
This project is currently pre v1.0, which means, even though it is not expected, breaking changes might come
with new versions.

## What it is

Rauthy is an OpenID Connect (OIDC) Provider and Single Sign-On solution written in Rust.

**Secure by default**  
It tries to be as secure as possible by default while still providing all the options needed to be compatible with
older systems. For instance, if you create a new OIDC client, it activates `ed25519` as the default algorithm for
token signing and S256 PKCE flow. This will not work with old clients, which do not support it, but you can of course
deactivate this to your liking.

**MFA and Passwordless Login**

1. Option: Password + Security Key (without User Verification):  
   Rauthy provides FIDO 2 / Webauthn login flows. If you once logged in on a new client with your username + password,
   you
   will get an encrypted cookie which will allow you to log in without a password from that moment on. You only need to
   have a FIDO compliant Passkey being registered for your account.
2. Option: Passkey-Only Accounts:  
   Since v0.16, Rauthy supports Passkey-Only-Accounts. You basically just provide your E-Mail address and log in with
   your
   FIDO 2 Passkey. Your account will not even have / need a password. This login flow is restricted though to only those
   passkeys, that can provide User Verification (UV) to always have at least 2FA security.

**Fast and efficient**  
The main goal was to provide an SSO solution like Keycloak and others while using a way lower footprint
and being more efficient with resources. For instance, Rauthy can easily run a fully blown SSO provider on just a
Raspberry Pi. It makes extensive use of caching to be as fast as possible in cases where your database is further
away or just a bit slower, because it is maybe running on an SBC from an SD card. Most things are even cached
for several hours (config options will come in the future) and special care has been taken into account in case of cache
eviction and invalidation.<br />
A Rauthy deployment with the embedded SQLite, filled caches and a small set of clients and users configured typically
only uses **between 17 and 22 MB of memory**! This is pretty awesome when comparing it to other existing solutions
out there. If a password from a login is hashed, the memory consumption will of course go up way higher than this
depending on your configured Argon2ID parameters, which you got fully under control. If you use it with an external
Postgres, the memory consumption of Rauthy itself will even be a bit lower, since it does not need to care about SQLite.
<br />
For achieving this speed and efficiency, some additional design tradeoffs werde made. For instance, some things you
configure statically via config file and not dynamically via UI, while most of them are configured once and then never
touched again.

**Highly Available**  
Even though it makes extensive use of caching, you can run it in HA mode. It uses its own embedded distributed HA cache
called [redhac](https://crates.io/crates/redhac), which cares about cache eviction on remote hosts.
You can choose between a *SQLite* for single instance deployments and a *Postgres*, if you need HA. MySQL support might
come in the future.

**Nice UI**  
Unlike many other options, `rauthy` does have a nice Admin UI which can be used to basically do almost any operation you
might need to administrate the whole application and its users. Some Screenshots and further introduction will follow
in the future.

**Client Branding**  
You have a simple way to create some kind of branding or stylized look for the Login page for each client.  
The whole color theme can be changed and each client can have its own custom logo.  
Additionally, if you modify the branding for the default `rauthy` client, it will not only change the look for the Login
page, but also for the Account and Admin page.

**Events and Auditing**  
Since v0.17, Rauthy implements an Event-System. Events are generated in all kinds of scenarios. They can be sent via
E-Mail, Matrix or Slack, depending on the severity and the configured level. You will see them in the Admin UI in real
time, or you can subscribe to the events stream and externally handle them depending on your own business logic.

**Brute-Force and basic DoS protection**  
Rauthy has brute force and basic DoS protection for the login endpoint. Your timeout will be artificially delayed after
enough invalid logins. It does auto-balacklist IP's that exceeded too many invalid logins, with automatic
expiry of the blacklisting. You can, if you like, manually blacklist certain IP's as well via the Admin UI.

**Already in production**  
Rauthy is already being used in production, and it works with all typical OIDC clients (so far). It was just not an
open source project for quite some time.  
Keycloak was a rough inspiration in certain places and if something is working with Keycloak, it does with `rauthy` too
(again, so far).

### Features List

- [x] Fully working OIDC provider
- [x] SQLite or Postgres as database
- [x] Fast and efficient with minimal footprint
- [x] Highly configurable
- [x] Secure default values
- [x] True passwordless accounts with E-Mail + Magic Link + Passkey
- [x] Account conversions between traditional password and Passkey only
- [x] Supports DPoP tokens for decentralized login flows
- [x] Supports ephemeral, dynamic clients for decentralized login flows
- [x] Can serve a basic `webid` document for decentralized logins
- [x] OpenID Connect Dynamic Client Registration
- [x] Upstream Authentication Providers (Login with ...)
- [x] Admin UI
- [x] Account UI for each user with self-service
- [x] Simple per client branding for the login page
- [x] All End-User facing sites support i18n server-side translation
- [x] Custom roles
- [x] Custom groups
- [x] Custom scopes
- [x] Custom user attributes
- [x] User attribute binding to custom scopes
- [x] Configurable password policy
- [x] Dedicated `forward_auth` endpoint, in addition to the existing userinfo,
  with support for configurable trusted auth headers
- [x] Admin API Keys with fine-grained access rights
- [x] Events and alerting system
- [x] Optional event notifications via: E-Mail, Matrix, Slack
- [x] Configurable E-Mail templates for NewPassword + ResetPassword events
- [x] Optional event persistence
- [x] Admin UI component for archived / persisted events
- [x] Optional Force MFA for the Admin UI
- [x] Optional Force MFA for each individual client
- [x] Additional encryption inside the database for the most critical entries
- [x] Automatic database backups with configurable retention and auto-cleanup (SQLite only)
- [x] auto-encrypted backups (SQLite)
- [x] pushing SQLite backups to S3 storage
- [x] auto-restore SQLite backups from file and s3
- [x] High-Availability
- [x] HA cache layer with its own (optional) mTLS connection
- [x] Username enumeration prevention
- [x] Login / Password hashing rate limiting
- [x] Session client peer IP binding
- [x] IP blacklisting feature
- [x] Auto-IP blacklisting for login endpoints
- [x] Argon2ID with config helper UI utility
- [x] Housekeeping schedulers and cron jobs
- [x] JSON Web Key Set (JWKS) autorotation feature
- [x] Optional open user registration
- [x] Optional user registration domain restriction
- [x] App version update checker
- [x] Prometheus `/metrics` endpoint on separate port
- [x] SwaggerUI documentation
- [x] No-Setup migrations between different databases (Yes, even between SQLite and Postgres)

### Features TODO

This is a non-exhaustive list of currently open TODO's

- [ ] Benchmarks and Optimizations
- [ ] Maybe get a nicer Rauthy Logo
- [ ] experimental implementation of [dilithium](https://pq-crystals.org/dilithium/) singing algorithm to become quantum
  safe
- [ ] maybe something like a `rauthy-migrate` project to make migrating an existing user's DB easier
- [ ] UI overhaul to make it "prettier" in certain places

## Getting Started

Either just take a look at the [Rauthy Book](https://sebadob.github.io/rauthy/), or start directly by taking a look at
the application yourself with docker on your localhost:

```
docker run --rm -p 8080:8080 ghcr.io/sebadob/rauthy:0.22.0-lite
```

## Contributing

If you want to contribute to this repository, please take a look at
[CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md)
