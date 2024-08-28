# Rauthy

**CAUTION:**  
There has not been any third party security audit for this project.
Use this software at your own risk!

**INFO:**  
This project is currently pre v1.0, which means, even though it is not expected, breaking changes might come
with new versions.

## What it is

Rauthy is an OpenID Connect (OIDC) Provider and Single Sign-On solution written in Rust.

### Secure by default

It tries to be as secure as possible by default while still providing all the options needed to be compatible with
older systems. For instance, if you create a new OIDC client, it activates `ed25519` as the default algorithm for
token signing and S256 PKCE flow. This will not work with old clients, which do not support it, but you can of course
deactivate this to your liking.

### MFA and Passwordless Login

**Option 1:**  
Password + Security Key (without User Verification):  
Rauthy provides FIDO 2 / Webauthn login flows. If you once logged in on a new client with your username + password,
you
will get an encrypted cookie which will allow you to log in without a password from that moment on. You only need to
have a FIDO compliant Passkey being registered for your account.

**Option 2:**  
Passkey-Only Accounts:  
Rauthy supports Passkey-Only-Accounts: you basically just provide your E-Mail address and log in with
your FIDO 2 Passkey. Your account will not even have / need a password. This login flow is restricted though to only
those passkeys, that can provide User Verification (UV) to always have at least 2FA security.

**Note:**  
Discoverable credentials are discouraged with Rauthy. This means you will need to enter your E-Mail for the login
(which will be auto-filled after the first one), but Rauthy passkeys do not use any storage on your device. For instance
when you have a Yubikey which can store 25 passkeys, it will not use a single slot there even having full support.

### Fast and efficient

The main goal was to provide an SSO solution like Keycloak and others while using a way lower footprint
and being more efficient with resources. For instance, Rauthy can easily run a fully blown SSO provider on just a
Raspberry Pi. It makes extensive use of caching to be as fast as possible in cases where your database is further
away or just a bit slower, because it is maybe running on an SBC from an SD card or in the cloud with the lowest storage
bandwidth. Most things are even cached for several hours and special care has been taken into account in case of cache
eviction and invalidation.

A Rauthy deployment with the embedded SQLite, filled caches and a small set of clients and users configured typically
only uses **between 20 and 25 MB of memory**! This is pretty awesome when comparing it to other existing solutions
out there. If a password from a login is hashed, the memory consumption will of course go up way higher than this
depending on your configured Argon2ID parameters, which you got fully under control.

For achieving the speed and efficiency, some additional design tradeoffs were made. For instance, some things you
configure statically via config file and not dynamically via UI, while most of them are configured once and then never
touched again.

### Highly Available

Even though it makes extensive use of caching, you can run it in HA mode. It uses its own embedded distributed HA cache
called [redhac](https://crates.io/crates/redhac), which cares about cache eviction on remote hosts.
You can choose between a *SQLite* for single instance deployments and a *Postgres*, if you need HA. MySQL support might
come in the future.

### Admin UI + User Account Dashboard

Unlike many other options, `rauthy` does have an Admin UI which can be used to basically do almost any operation you
might need to administrate the whole application and its users. There is also an account dashboard for each individual
user, where users will get a basic overview over their account and can self-manage som values, password, passkeys, and
so on.  
Some Screenshots and further introduction will follow in the future.

### Client Branding

You have a simple way to create some kind of branding or stylized look for the Login page for each client.
The whole color theme can be changed and each client can have its own custom logo.
Additionally, if you modify the branding for the default `rauthy` client, it will not only change the look for the Login
page, but also for the Account and Admin page.

### Events and Auditing

Rauthy comes with an Event and Alerting-System. Events are generated in all kinds of scenarios. They can be sent via
E-Mail, Matrix or Slack, depending on the severity and the configured level. You will see them in the Admin UI in real
time, or you can subscribe to the events stream and externally handle them depending on your own business logic.

### Brute-Force and basic DoS protection

Rauthy has brute force and basic DoS protection for the login endpoint. Your timeout will be artificially delayed after
enough invalid logins. It does auto-balacklist IP's that exceeded too many invalid logins, with automatic
expiry of the blacklisting. You can, if you like, manually blacklist certain IP's as well via the Admin UI.

### IoT Ready

With the possibility to run on devices with very limited resources and having compatibility for the OAuth Device
Authorization Grant `device_code` flow, Rauthy would be a very good choice for IoT projects. The IdP itself can easily
run on a Raspberry Pi and all headless devices can be authenticated via the `device_code` flow. The `rauthy-client`
has everything built-in and ready, if you want to use Rust on the IoT devices as well. It has not been checked in a
`no_std` environment yet, but the client implementation is pretty simple.

### Scales to millions of users

Benchmarks for v1.0.0 have not been done yet, but after some first basic tests and generating a lot of dummy data, I
can confirm that Rauthy has no issues handling millions of users. The first very basic tests have been done with SQLite
and ~11 million users. All parts and functions kept being fast and responsive with the only exception that the
user-search in the admin UI was slowed down with such a high user count. It took ~2-3 seconds at that point to get a
result, which should be no issue at all so far (Postgres tests have not been done yet).
The only limiting factor at that point will be your configuration and needs for password hashing security. It really
depends on how many resources you want to use for hashing (more resources == more secure) and how many concurrent logins
at the exact same time you need to support.

### Already in production

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
- [x] Dedicated Admin UI
- [x] Account dashboard UI for each user with self-service
- [x] OpenID Connect Dynamic Client Registration
- [x] OAuth 2.0 Device Authorization Grant flow
- [x] Upstream Authentication Providers (Login with ...)
- [x] Supports DPoP tokens for decentralized login flows
- [x] Supports ephemeral, dynamic clients for decentralized login flows
- [x] All End-User facing sites support i18n server-side translation
  with the possibility to add more languages
- [x] Simple per client branding for the login page
- [x] Custom roles
- [x] Custom groups
- [x] Custom scopes
- [x] Custom user attributes
- [x] User attribute binding to custom scopes
- [x] Configurable password policy
- [x] Admin API Keys with fine-grained access rights
- [x] Events and alerting system
- [x] Optional event persistence
- [x] Dedicated `forward_auth` endpoint, in addition to the existing userinfo,
  with support for configurable trusted auth headers
- [x] Optional event notifications via: E-Mail, Matrix, Slack
- [x] Optional Force MFA for the Admin UI
- [x] Optional Force MFA for each individual client
- [x] Additional encryption inside the database for the most critical entries
- [x] Automatic database backups with configurable retention and auto-cleanup (SQLite only)
- [x] auto-encrypted backups (SQLite)
- [x] Ability to push SQLite backups to S3 storage
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
- [x] Account conversions between traditional password and Passkey only
- [x] Optional open user registration
- [x] Optional user registration domain restriction
- [x] App version update checker
- [x] SwaggerUI documentation
- [x] Configurable E-Mail templates for NewPassword + ResetPassword events
- [x] Prometheus `/metrics` endpoint on separate port
- [x] No-Setup migrations between different databases (Yes, even between SQLite and Postgres)
- [x] Can serve a basic `webid` document
- [x] Experimental FedCM support

### Features TODO

This is a non-exhaustive list of currently open TODO's

- [ ] Maybe get a nicer Rauthy Logo
- [ ] experimental implementation of [dilithium](https://pq-crystals.org/dilithium/) singing algorithm to become quantum
  safe
- [ ] maybe something like a `rauthy-migrate` project to make migrating an existing user's DB easier
- [ ] UI overhaul to make it "prettier" in certain places

## Getting Started

Either just take a look at the [Rauthy Book](https://sebadob.github.io/rauthy/), or start directly by taking a look at
the application yourself with docker on your localhost. Rauthy has pretty strict cookie settings and not all
browsers treat `localhost` as being secure, therefore you should allow insecure cookies for testing locally:

```
docker run --rm -e COOKIE_MODE=danger-insecure -p 8080:8080 ghcr.io/sebadob/rauthy:0.25.0-lite
```

## Contributing

If you want to contribute to this repository, please take a look at
[CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md)
