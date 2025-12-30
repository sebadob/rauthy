![Rauthy Logo](https://github.com/sebadob/rauthy/blob/a89a8e9712c567551cb2d25b9da8823e35794f0a/logo/rauthy_grey_small.png)

# Rauthy

Rauthy - Single Sign-On Identity & Access Management via OpenID Connect, OAuth 2.0 and PAM

> [!NOTE]
> This application received an independent security audit
> from [Radically Open Security](https://www.radicallyopensecurity.com/)
> ([Frank Plattel](https://github.com/Sp0Q1) and [Morgan Hill](https://github.com/pcwizz)) as part
> of the [NGI Zero Core](https://nlnet.nl/core) funding. There were some findings, that were
> addressed in `v0.32.1`. The full report can be
> found [here](https://raw.githubusercontent.com/sebadob/rauthy/refs/heads/main/assets/security_audit_report_v0.32.pdf).

## What it is

Rauthy is a lightweight and easy to use Identity Provider supporting OpenID Connect, OAuth 2.0 and
PAM. It aims to be simple to both set up and operate, with very secure defaults and lots of config
options, if you need the flexibility. It puts heavy emphasis on Passkeys and a very strong security
in general. The project is written in Rust to be as memory efficient, secure and fast as possible,
and it can run on basically any hardware. If you need Single Sign-On support for IoT or headless CLI
tools, it's got you covered as well.  
You get High-Availability, client branding, UI translation, a nice Admin UI, Events and Auditing,
and many more features. By default, it runs on top of [Hiqlite](https://github.com/sebadob/hiqlite)
and does not depend on an external database (Postgres as an alternative) to make it even simpler to
operate, while scaling up to millions of users easily.

### Secure by default

It tries to be as secure as possible by default while still providing all the options needed to be
compatible with older systems. For instance, if you create a new OIDC client, it activates `ed25519`
as the default algorithm for token signing and S256 PKCE flow. This will not work with clients,
which do not support it, but you can of course deactivate this to your liking.

### MFA and Passwordless Login

**Option 1:**  
Password + Security Key (without User Verification):  
Rauthy provides FIDO 2 / Webauthn login flows. If you once logged in on a new client with your
username + password, you will get an encrypted cookie which will allow you to log in without a
password from that moment on. You only need to have a FIDO compliant Passkey being registered for
your account.

**Option 2:**  
Passkey-Only Accounts:  
Rauthy supports Passkey-Only-Accounts: you basically just provide your E-Mail address and log in
with your FIDO 2 Passkey. Your account will not even have / need a password. This login flow is
restricted though to only those passkeys, that can provide User Verification (UV) to always have at
least 2FA security.

> [!TIP]
> Discoverable credentials are discouraged with Rauthy (for good reason). This means you will need
> to enter your E-Mail for the login (which will be autofilled after the first one), but Rauthy
> passkeys do not use any storage on your device. For instance when you have a Yubikey which can
> store 25 passkeys, it will not use a single slot there even having full support.

### Fast and efficient

The main goal was to provide an SSO solution like Keycloak and others while using a way lower
footprint and being more efficient with resources. For instance, Rauthy can easily run a fully blown
SSO provider on just a Raspberry Pi. It makes extensive use of caching for everything used in the
authentication chain to be as fast as possible. Most things are even cached for several hours and
special care has been taken into account in case of cache eviction and invalidation.

Rauthy comes with two database options:

- with embedded [Hiqlite](https://github.com/sebadob/hiqlite), which is the default setting
- or you can optionally use a Postgres as your database, if you already have an instance running
  anyway.

The resource usage depends a lot on your setup (Hiqlite, Postgres, HA deployment, amount of
users, ...), but for a small set of users, it is usually below 100mb of memory even with the very
aggressive, in-memory caching Rauthy uses, and in some cases even below 50mb.

### Highly Available

Even though it makes extensive use of caching, you can run it in HA
mode. [Hiqlite](https://github.com/sebadob/hiqlite) creates its own embedded HA cache and
persistence layer. Such a deployment is possible with
both [Hiqlite](https://github.com/sebadob/hiqlite) and Postgres.

### Admin UI + User Account Dashboard

Rauthy does have an Admin UI which can be used to basically do almost any operation you might need
to administrate the whole application and its users. There is also an account dashboard for each
individual user, where users will get a basic overview over their account and can self-manage som
values, password, passkeys, and so on.

![Admin UI](https://github.com/sebadob/rauthy/blob/a89a8e9712c567551cb2d25b9da8823e35794f0a/frontend/screenshots/users.png)

![Account Dashboard](https://github.com/sebadob/rauthy/blob/a89a8e9712c567551cb2d25b9da8823e35794f0a/frontend/screenshots/account.png)

### Client Branding

You have a simple way to create a branding or stylized look for the Login page for each client. The
whole color theme can be changed and each client can have its own custom logo. Additionally, if you
modify the branding for the default `rauthy` client, it will not only change the look for the Login
page, but also for the Account and Admin page.

![Client Branding](https://github.com/sebadob/rauthy/blob/c10e9421e65f386718528b15e3d0ace37aff1158/frontend/screenshots/branding.png)

### Events and Auditing

Rauthy comes with an Event- and Alerting-System. Events are generated in all kinds of scenarios.
They can be sent via E-Mail, Matrix or Slack, depending on the severity and the configured level.
You will see them in the Admin UI in real-time, or you can subscribe to the events stream and
externally handle them depending on your own business logic.

### Brute-Force and basic DoS protection

Rauthy has brute-force and basic DoS protection for the login endpoint. The timeout will be
artificially delayed after enough invalid logins. It auto-blacklists IPs that exceeded too many
invalid logins, with automatic expiry of the blacklisting. You can, if you like, manually blacklist
certain IPs as well via the Admin UI.

### IoT Ready

With the possibility to run on devices with very limited resources and having compatibility for the
OAuth Device Authorization Grant `device_code` flow, Rauthy would be a very good choice for IoT
projects. The IdP itself can easily run on a Raspberry Pi and all headless devices can be
authenticated via the `device_code` flow. The `rauthy-client` has everything built-in and ready, if
you want to use Rust on the IoT devices as well. It has not been checked in a `no_std` environment
yet, but the client implementation is pretty simple.

### PAM Logins

OIDC / OAuth covers almost all web apps, and for those that don't have any support, Rauthy comes
with `forward_auth` support. To not need an additional LDAP / AD / something similar for your
backend and workstations, Rauthy comes with its own custom PAM module. It does not just use JWT
Tokens for logging in, but you can actually manage all your Linux hosts, groups and users in
different ways. You have the option to secure local logins to workstations via Yubikey (only USB
Passkeys supported, no QR-code / software keys), and all SSH logins can be done with ephemeral,
auto-expiring passwords, that you can generate via your Account dashboard, if an Admin has created a
PAM user for you. This means you basically have MFA-secured SSH logins without the need for any
modifications or additional software on your local SSH client, and you can use any SSH client from
any machine securely, even if it's not your own.

In addition to the PAM module, you get an NSS module and an NSS proxy that runs on each machine. You
can dynamically log in to any machine an Admin has given you access to. Users and groups are not
added to local files, but will be resolved via the network.

This module is published in a separate repo to avoid licensing issues, since it relies on some GPLv3
dependencies. You can take a look at it
here: [rauthy-pam-nss](https://github.com/sebadob/rauthy-pam-nss).

### Scales to millions of users

Rauthy has no issue handling even millions of users. Everything keeps being fast and responsive,
apart from the search function for users in der Admin UI when you reach the 10+ million users, where
searching usually takes ~3 seconds (depending on your server of course).   
The only limiting factor at that point will be your configuration and needs for password hashing
security. It really depends on how many resources you want to use for hashing (more resources ==
more secure) and how many concurrent logins at the exact same time you need to support.

### Features List

- [x] Fully working OIDC / OAuth 2.0 provider
- [x] PAM logins via custom PAM + NSS modules
- [x] [Hiqlite](https://github.com/sebadob/hiqlite) or Postgres as database
- [x] Fast and efficient with low footprint
- [x] Secure default values
- [x] Highly configurable
- [x] High-Availability
- [x] True passwordless accounts with E-Mail + Magic Link + Passkey
- [x] Dedicated Admin UI
- [x] Account dashboard UI for each user with self-service
- [x] OpenID Connect Dynamic Client Registration
- [x] OpenID Connect RP Initiated Logout
- [x] OpenID Connect Backchannel Logout
- [x] OAuth 2.0 Device Authorization Grant flow
- [x] Upstream Authentication Providers (Login with ...)
- [x] DPoP tokens for decentralized login flows
- [x] Ephemeral, dynamic clients for decentralized login flows
- [x] SCIM v2 for downstream clients
- [x] All End-User facing sites support i18n server-side translation
  with the possibility to add more languages
- [x] Simple per client branding for the login page
- [x] Custom roles
- [x] Custom groups
- [x] Custom scopes
- [x] Custom user attributes
- [x] User attribute binding to custom scopes
- [x] Optional user-editable custom attributes
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
- [x] Automatic database backups with configurable retention and
  auto-cleanup ([Hiqlite](https://github.com/sebadob/hiqlite) only)
- [x] auto-encrypted backups ([Hiqlite](https://github.com/sebadob/hiqlite) only)
- [x] Ability to push [Hiqlite](https://github.com/sebadob/hiqlite) backups to S3 storage
- [x] auto-restore [Hiqlite](https://github.com/sebadob/hiqlite) backups from file or s3
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
- [x] No-Setup migrations between different databases (Yes, even
  between [Hiqlite](https://github.com/sebadob/hiqlite)
  and Postgres)
- [x] Hot-Reload TLS certificates without restart
- [x] Can serve a basic `webid` document
- [x] Experimental FedCM support

## Getting Started

Either just take a look at the [Rauthy Book](https://sebadob.github.io/rauthy/), or start directly
by taking a look at the application yourself with docker on your localhost. Rauthy comes with a
setting for very quick and easy local testing and taking a first look. By setting `LOCAL_TEST=true`,
a demo config is being loaded at startup.

```
docker run -it --rm -e LOCAL_TEST=true -p 8443:8443 ghcr.io/sebadob/rauthy:0.33.4
```

> [!CAUTION]
> Some browsers like Firefox do not allow the registration of Passkeys when using self-signed TLS
> certificates. To be able to do this during testing, you would need to add the generated CA
> certificate to your trust store.

> [!IMPORTANT]
> This command starts an HTTPS server with self-signed certificates.  
> Make sure to add the `https://` scheme if you open the URL manually.

## Support

If you need professional (paid) support for Rauthy, please feel free to contact me at
`mail@sebadob.dev`. Otherwise, please use the Discussions Q&A section. Opened issues, which are
actually just support requests, will most probably not be answered at all. I am working on this
project mostly in my free time.

## Contributing

If you want to contribute to this repository, please take a look at
[CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md)

## Funding

This project was funded through [NGI Zero Core](https://nlnet.nl/core), a fund established
by [NLnet](https://nlnet.nl) with financial support from the European
Commission's [Next Generation Internet](https://ngi.eu) program. Learn more at
the [NLnet project page](https://nlnet.nl/project/Rauthy).

[<img src="https://nlnet.nl/logo/banner.png" alt="NLnet foundation logo" width="20%" />](https://nlnet.nl)  
[<img src="https://nlnet.nl/image/logos/NGI0_tag.svg" alt="NGI Zero Logo" width="20%" />](https://nlnet.nl/core)
