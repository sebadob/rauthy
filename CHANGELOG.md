# Changelog

## v0.19.1

This is a small bugfix and compatibility release regarding password reset E-Mails.

The main reason for this release are problems with the Password Reset via E-Mail when users
are using Microsoft (the only service provider where this problems can be replicated 100% of the time)
and / or Outlook. These users were unable to use password reset links at all.
The reason is a "Feature" from Microsoft. They fully scan the user's E-Mails and even follow all links
inside it. The problem is, that the binding cookie from Rauthy will go to the Microsoft servers instead
of the user, making is unusable and basically invalidating everything before the user has any chance to
use the link properly.

The usage of this config variable is **highly discouraged,** and you should **avoid it, if you can**.
However, big enterprises are moving slowly (and often not at all). This new config variable can be used
as a last resort, to make it usable by giving up some security.

```
# This value may be set to 'true' to disable the binding cookie checking
# when a user uses the password reset link from an E-Mail.
#
# When using such a link, you will get a so called binding cookie. This
# happens on the very first usage of such a reset link. From that moment on,
# you will only be able to access the password reset form with this very 
# device and browser. This is just another security mechanism and prevents
# someone else who might be passively sniffing network traffic to extract 
# the (unencrypted) URI from the header and just use it, before the user 
# has a change to fill out the form. This is a mechanism to prevent against
# account takeovers during a password reset.
#
# The problem however are companies (e.g. Microsoft) who scan their customers
# E-Mails and even follow links and so on. They call it a "feature". The
# problem is, that their servers get this binding cookie and the user will be
# unable to use this link himself. The usage of this config option is highly
# discouraged, but since everything moves very slow in big enterprises and
# you cannot change your E-Mail provider quickly, you can use it do just make
# it work for the moment and deal with it later.
#
# default: false
#UNSAFE_NO_RESET_BINDING=false
```

### Changes

- implement `UNSAFE_NO_RESET_BINDING` like mentioned above
[1f4a146](https://github.com/sebadob/rauthy/commit/1f4a1462697e85e068edd6bbd3f670f9d1ed985b)
- prettify the expiry timestamp in some E-Mails
[1173fa0](https://github.com/sebadob/rauthy/commit/1173fa0f5ac517c7797c34e1240cbd37cb54dae6)

### Bugfixes
- It was possible to get an "Unauthorized Session" error during a password reset, if it has been
initiated by an admin and / or from another browser.
[e5d1d9d](https://github.com/sebadob/rauthy/commit/e5d1d9dd30452fdf5c33cc8e1cfac9670a514c74)
- Correctly set `ML_LT_PWD_FIRST` - set the default value in minutes (like documented) instead
of seconds. New default is `ML_LT_PWD_FIRST=4320`
[e9d1b56](https://github.com/sebadob/rauthy/commit/e9d1b5627809825241fcb0dbea4935f76d1334f1)

## v0.19.0

### Solid OIDC Support

This is the main new feature for this release.

With the now accepted `RSA` signatures for DPoP tokens, the ephemeral, dynamic clients and
the basic serving of `webid` documents for each user, Rauthy should now fully support Solid OIDC.
This feature just needs some more real world testing with already existing applications though.

These 3 new features are all opt-in, because a default deployment of Rauthy will most probably
not use them at all. There is a whole new section in the [Config](https://sebadob.github.io/rauthy/config/config.html)
called `EPHEMERAL CLIENTS` where you can configure these things. The 3 main variables you need
to set are:

```
# Can be set to 'true' to allow the dynamic client lookup via URLs as
# 'client_id's during authorization_code flow initiation.
# default: false
ENABLE_EPHEMERAL_CLIENTS=true

# Can be set to 'true' to enable WebID functionality like needed
# for things like Solid OIDC.
# default: false
ENABLE_WEB_ID=true

# If set to 'true', 'solid' will be added to the 'aud' claim from the ID token
# for ephemeral clients.
# default: false
ENABLE_SOLID_AUD=true
```

Afterward, the only "manual" thing you need to do is to add a custom scope called `webid`
once via the Admin UI.

### `EVENT_MATRIX_ERROR_NO_PANIC`

This new config variable solves a possible chicken and egg problem, if you use a self-hosted
Matrix server and Rauthy as its OIDC provider at the same time. If both services are offline,
for instance because of a server reboot, you would not be able to start them.

- The Matrix Server would panic because it cannot connect to and verify Rauthy
- Rauthy would panic because it cannot connect to Matrix

Setting this variable to `true` solves this issue and Rauthy would only log an error in that
case instead of panicking. The panic is the preferred behavior though, because this makes 
100% sure that Rauthy will actually be able to send out notification to configured endpoints.

### Features

- ~20% smaller binary size by stripping unnecessary symbols
[680d5e5](https://github.com/sebadob/rauthy/commit/680d5e5926481947324d8e5868a9c4d903ead05a)
- Accept `DPoP` tokens with `RSA` validations
[daade41](https://github.com/sebadob/rauthy/commit/daade41a4ff22980d41e54570462eef783607766)
- Dynamically build up and serve custom scopes in the `/.well-known/openid-configuration`
[904cf09](https://github.com/sebadob/rauthy/commit/904cf090a1f3070a33dbbac8c503b7190dd6ee47)
- A much nicer way of generating both DEV and PROD TLS certificates by using [Nioca](https://github.com/sebadob/nioca)
has been integrated into the project itself, as well as the 
[Rauthy Book](https://sebadob.github.io/rauthy/config/tls.html)
[463bf8a](https://github.com/sebadob/rauthy/commit/463bf8a40bf71e588a0449d647714acc96c68f83)
[a14beda](https://github.com/sebadob/rauthy/commit/a14beda84942ecb17ac56d15b52e4668ebb12b41)
- Implement opt-in ephemeral clients
[52c84c2](https://github.com/sebadob/rauthy/commit/52c84c2343447698978be4e25e5f537aad0070e0)
[617908b](https://github.com/sebadob/rauthy/commit/617908bada3ba3e16d352e797e25c2d62eb512a6)
- Implement opt-in basic `webid` document serving
[bca77f5](https://github.com/sebadob/rauthy/commit/bca77f5612a2a374d8e34ca8a717d94832328e7f)
[1e32f6f](https://github.com/sebadob/rauthy/commit/1e32f6f93dd09aacabd6ea8596d028818691624e)
[79cb836](https://github.com/sebadob/rauthy/commit/79cb83622fe508b3d296e9a6a71f5d6761a6b83a)
[55433f4](https://github.com/sebadob/rauthy/commit/55433f4c614b7660dad32ad2321ff86367d8892e)
[3cdf81c](https://github.com/sebadob/rauthy/commit/3cdf81c03cf9b78177412069264fa76f4d770ecc)
- For developers, a new [CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md)
guide has been added to get people started quickly
[7c38142](https://github.com/sebadob/rauthy/commit/7c381428f74210a2dc56a5d995ab76485a3686ad)
[411393f](https://github.com/sebadob/rauthy/commit/411393faab2d4f0242ea6fc1414d501c1260d50c)
- Add a new config variable `EVENT_MATRIX_ERROR_NO_PANIC` to only throw an error instead of
panic on Matrix connection errors
[4fc3382](https://github.com/sebadob/rauthy/commit/4fc3382929e65780fb20a78994233357423f0aab)
- Not really a bug nor a feature, but the "App Version Update" watcher now remembers a
sent notification for an update and will only notify after a restart again.
[be19735](https://github.com/sebadob/rauthy/commit/be197355437d0338041cc3206421ec638ca938d7)

### Bugfixes

- In a HA deployment, the new integrated health watcher from v0.17.0 could return false positives
[93d75d5](https://github.com/sebadob/rauthy/commit/93d75d5d97e92d20a54f4781cea0f0b186b1098d)
[9bbaeb2](https://github.com/sebadob/rauthy/commit/9bbaeb2f0b582838398547ddb477b7f8ab537a30)
- In v0.18.0 a bug has been introduced because of internal JWKS optimizations. This produced
cache errors when trying to deserialize cached JWKS after multiple requests.
[3808423](https://github.com/sebadob/rauthy/commit/3808423c8c13c06cdd82f6d97a9ef01486561a79)

## v0.18.0

This is a rather small release.
The main reason it is coming so early is the license change.

### License Change To Apache 2.0

With this release, the license of Rauthy is changed from the AGPLv3 to an Apache 2.0.
The Apache is way more permissive and make the integration with other open source projects and software a lot easier.

### DPoP Token Support (Experimental)

The first steps towards DPoP Token support have been made.
It is marked as experimental though, because the other authentication methods have been tested and verified with
various real world applications already. This is not the case for DPoP yet.
Additionally, the only supported alg for DPoP proofs is EdDSA for now. The main reason being that I am using Jetbrains
IDE's and the Rust plugin for both IDEA and RustRover are currently broken in conjunction with the `rsa` crate
(and some others) which makes writing code with them a nightmare. RSA support is prepared as much as possible
though and I hope they will fix this bug soon, so it can be included.

If you have or use a DPoP application, I would really appreciate testing with Rauthy and to get some feedback, so I
can make the whole DPoP flow more resilient as well.

Please note that Authorization Code binding to a DPoP key is also not yet supported, only the `/token` endpoint accepts
and validates the `DPoP` header for now.

### Changes

- Experimental DPoP support
[803e312](https://github.com/sebadob/rauthy/commit/803e312194b67a20e475e685bd1fecf69d8a98fa)
[51dc320](https://github.com/sebadob/rauthy/commit/51dc320291b125cee5c56e97eda54a12149caf33)
[4329303](https://github.com/sebadob/rauthy/commit/43293030bf575175706357dda7ebed0beb6684dd)
[29dce66](https://github.com/sebadob/rauthy/commit/29dce66aa167e789fab689fe680d07201b1126c7)

### Bugfixes

- Typos have been changed in docs and config
[51dc320](https://github.com/sebadob/rauthy/commit/51dc320291b125cee5c56e97eda54a12149caf33)
- Listen Scheme was not properly set when only HTTP was selected exclusively
[c002fbe](https://github.com/sebadob/rauthy/commit/c002fbed506e9dae2024e6e3ea5af33b7429cf11)
- Resource links in default error HTML template did not work properly in all locations
[5965d9a](https://github.com/sebadob/rauthy/commit/5965d9aa1f7d7144786a2f6d73ce966570da5467)

### New Contributors

[twistedfall](https://github.com/twistedfall)

## v0.17.0

This is a pretty huge update with a lot of new features.

### New Features

#### Support for `linux/arm64`

With the release of v0.17.0, Rauthy's container images are now multi-platform.

Both a `linux/amd64` and a `linux/arm64` are supported. This means you can "just use it" now on Raspberry Pi and
others, or on Ampere architecture from Cloud providers without the need to compile it yourself.

#### Events and Auditing

Rauthy now produces events in all different kinds of situations. These can be used for auditing, monitoring, and so on.
You can configure quite a lot for them in the new `EVENTS / AUDIT` section in the
[Rauthy Config](https://sebadob.github.io/rauthy/config/config.html).

These events are persisted in the database, and they can be fetched in real time via a new Server Sent Events(SSE)
endpoint `/auth/v1/events/stream`. There is a new UI component in the Admin UI that uses the same events stream.
In case of a HA deployment, Rauthy will use one additional DB connection (all the time) from the connection pool
to distribute these events via pg listen / notify to the other members. This makes a much simpler deployment and
there is no real need to deploy additional resources like Nats or something like that. This keeps the setup easier
and therefore more fault-tolerant.

> You should at least set `EVENT_EMAIL` now, if you update from an older version.

#### Slack and Matrix Integrations

The new Events can be sent to a Slack Webhook or Matrix Server.

The Slack integration uses the simple (legacy) Slack Webhooks and can be configured with `EVENT_SLACK_WEBHOOK`:

```
# The Webhook for Slack Notifications.
# If left empty, no messages will be sent to Slack.
#EVENT_SLACK_WEBHOOK=
```

The Matrix integration can connect to a Matrix server and room. This setup requires you to provide a few more
variables:

```
# Matrix variables for event notifications.
# `EVENT_MATRIX_USER_ID` and `EVENT_MATRIX_ROOM_ID` are mandatory.
# Depending on your Matrix setup, additionally one of
# `EVENT_MATRIX_ACCESS_TOKEN` or `EVENT_MATRIX_USER_PASSWORD` is needed.
# If you log in to Matrix with User + Password, you may use `EVENT_MATRIX_USER_PASSWORD`.
# If you log in via OIDC SSO (or just want to use a session token you can revoke),
# you should provide `EVENT_MATRIX_ACCESS_TOKEN`.
# If both are given, the `EVENT_MATRIX_ACCESS_TOKEN` will be preferred.
#
# If left empty, no messages will be sent to Slack.
# Format: `@<user_id>:<server address>`
#EVENT_MATRIX_USER_ID=
# Format: `!<random string>:<server address>`
#EVENT_MATRIX_ROOM_ID=
#EVENT_MATRIX_ACCESS_TOKEN=
#EVENT_MATRIX_USER_PASSWORD=
# Optional path to a PEM Root CA certificate file for the Matrix client.
#EVENT_MATRIX_ROOT_CA_PATH=tls/root.cert.pem
# May be set to disable the TLS validation for the Matrix client.
# default: false
#EVENT_MATRIX_DANGER_DISABLE_TLS_VALIDATION=false
```

You can configure the minimum event level which would trigger it to be sent:

```
# The notification level for events. Works the same way as a logging level. For instance:
# 'notice' means send out a notifications for all events with the info level or higher.
# Possible values:
# - info
# - notice
# - warning
# - critical
#
# default: 'warning'
EVENT_NOTIFY_LEVEL_EMAIL=warning
# default: 'notice'
EVENT_NOTIFY_LEVEL_MATRIX=notice
# default: 'notice'
EVENT_NOTIFY_LEVEL_SLACK=notice
```

#### Increasing Login Timeouts

Up until version 0.16, a failed login would extend the time the client needed to wait for the result
artificially until it ended up in the region of the median time to log in successfully.
This was already a good thing to do to prevent username enumeration.
However, this has been improved a lot now.

When a client does too many invalid logins, the time he needs to wait until he may do another try
increases with each failed attempt. The important thing here is, that this is not bound to a user,
but instead to the clients IP.  
This makes sure, that an attacker cannot just lock a users account by doing invalid logins and therefore
kind of DoS the user. Additionally, Rauthy can detect Brute-Force or DoS attempts independently of
a users account.

There are certain thresholds at 7, 10, 15, 20, 25 invalid logins, when a clients IP will get fully
blacklisted (explained below) for a certain amount of time. This is a good DoS and even DDoS prevention.


#### Ip Blacklist Middleware

This is a new HTTP middleware which checks the clients IP against an internal blacklist.

This middleware is the very first thing that is being executed and just returns an HTML page
to a blacklisted client with the information about the blacklisting and the expiry.  
This blacklist is in-memory only to be as fast as possible to actually be able to handle brute
force and DoS attacks in the best way possible while consuming the least amount of resources
to do this.

Currently, IP's may get blacklisted in two ways:
- Automatically when exceeding the above-mentioned thresholds for invalid logins in a row
- Manually via the Admin UI

Blacklisted IP's always have an expiry and will get removed from the blacklist automatically.
Both actions will trigger one of the new Rauthy Events and send out notifications.

#### JWKS Auto-Rotate Scheduler

This is a simple new cron job which rotates the JSON Web Key Set (JWKS) automatically for
enhanced security, just in case one of the keys may get leaked at some point.

By default, it runs every first day of the month. This can be adjusted in the config:
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

#### Fully Reworked Authentication Middleware

The authentication and authorization system has been fully reworked and improved.

The new middleware and way of checking the client's access rights in each endpoint is way
less error-prone than before. The whole process has been much simplified which indirectly
improves the security:
- CSRF Tokens are now checked automatically if the request method is any other than a `GET`
- `Bearer` Tokens are not allowed anymore to access the Admin API
- A new `ApiKey` token type has been added (explained below)
- Only a single authn/authz struct is needed to validate each endpoint
- The old permission extractor middleware was removed which also increases the performance a bit

#### New API-Key Type

This new API-Key type may be used, if you need to access Rauthy API from other applications.

Beforehand, you needed to create a "user" for an application, if you wanted to access the API,
which is kind of counter-intuitive and cumbersome.  
These new API-Keys can be used to handle this task now. These are static keys with an
optional expiry date and fine-grained access rights. You should only give them permissions
to the resources you actually need to further improve your backend security.

They can be easily created, configured and revoked / deleted in the Admin UI at any time.

> IMPORTANt: The API now cannot be accessed anymore with Bearer tokens! If you used this
> method until now, you need to switch to the new API Keys

#### OIDC Client FORCE_MFA feature

In the configuration for each individual OIDC client, you can find a new `FORCE MFA` switch.
It this new option is activated for a client, it will only issue authentication codes for
those users, that have at least one Passkey registered.  
This makes it possible to force MFA for all your different applications from Rauthy directly
without the need to check for the `amr` claim in the ID token and do or configure all of
this manually downstream. Most of the time, you may not even have control over the client
itself, and you are basically screwed, if the client does not have its own "force mfa integration".

> CAUTION: This mentioned in the UI as well, but when you check this new force mfa option,
> it can only force MFA for the `authorization_code` flow of course! If you use other flows,
> there just is no MFA that could be checked / forced.

#### Rauthy Version Checker

Since we do have an Events system now, there is a new scheduled cron job, which checks the
latest available Rauthy Version.

This Job runs once every 8 hours and does a single poll to the Github Releases API. It looks
for the latest available Rauthy Version that is not a prerelease or anything unstable.
If it finds a version higher than the currently running one, a new Event will be generated.
Additionally,
you will see the current Rauthy Version in the UI now and a small indicator just next to it,
if there is a stable update available.

### Changes

- Support for `linux/arm64`
[2abb071](https://github.com/sebadob/rauthy/commit/2abb071afd6e9379fa3deca233c649bf62d33032)
- New events and auditing
[758dda6](https://github.com/sebadob/rauthy/commit/758dda631734c0c8e5baddf79ff2b0aa67947929)
[488f9de](https://github.com/sebadob/rauthy/commit/488f9de03653c5eb2c673644deb188599763afbb)
[7b95acc](https://github.com/sebadob/rauthy/commit/7b95acc22470bc49af6cd1812d87a414d7b4176b)
[34d8888](https://github.com/sebadob/rauthy/commit/34d8888faa3ee6740d633814bf29920d2c2856a6)
[f70f0b2](https://github.com/sebadob/rauthy/commit/f70f0b2a20e9e3fc53958959238b9f8136b5c9f0)
[5f0c9c9](https://github.com/sebadob/rauthy/commit/5f0c9c979c0ba3cb4d3a75cd78a899eee7d13464)
[a9af494](https://github.com/sebadob/rauthy/commit/a9af494bba788e462bb22eb31131d19b5ffaeaed)
[797dad5](https://github.com/sebadob/rauthy/commit/797dad564ff190b8739393c0405486b8f55b057e)
[b338f26](https://github.com/sebadob/rauthy/commit/b338f2613e9d19581677915c5ceb1996653709d7)
- `rauthy-notify` crate has been added which implements the above-mentioned Slack and
Matrix integrations for Event notifications.
[8767389](https://github.com/sebadob/rauthy/commit/8767389dafe3dc392910135d8cfc7f6a63bf3cd5)
- Increasing login timeouts and delays after invalid logins
[7f7a675](https://github.com/sebadob/rauthy/commit/7f7a675102f21aabe9f4cd2a5eef95d4947134d4)
[5d19d2d](https://github.com/sebadob/rauthy/commit/5d19d2d6d4434b2192eb3c8860eb8111ffdb8d79)
- IpBlacklist Middleware
[d69845e](https://github.com/sebadob/rauthy/commit/d69845ed772e1454e8ac902fdffb253416cbac14)
- IPBlacklist Admin UI Component
[c76c208](https://github.com/sebadob/rauthy/commit/c76c20871a303cbf6cfc587483113cd544d65424)
- JWKS Auto-Rotate
[cd087eb](https://github.com/sebadob/rauthy/commit/cd087eb8b475c9b8ae409e7a34bc880e76f2ee69)
- New Authentication Middleware
[a097a5d](https://github.com/sebadob/rauthy/commit/a097a5d2d0d75940cc192b2b1a86f59e5a732a88)
- ApiKey Admin UI Component
[53ffe49](https://github.com/sebadob/rauthy/commit/53ffe4938a8f5a2fd6d30188ac0a819f3eae2f96)
- OIDC Client `FORCE_MFA`
[3efdcce](https://github.com/sebadob/rauthy/commit/3efdcceb2277b2f8b28572485cdd8137115d8b66)
- Rauthy Version Checker
[aea7794](https://github.com/sebadob/rauthy/commit/aea77942b2488a26d12e7d191582996637079918)
[41b4c9c](https://github.com/sebadob/rauthy/commit/41b4c9c19c061f36f3ed5fa97d75ddcf803e0614)
- Show a Link to the accounts page after a password reset if the redirection does not work
[ace4daf](https://github.com/sebadob/rauthy/commit/ace4daf180b09f0cc8677bf4cf85e792496f9ee0)
- Send out E-Mail change confirmations E-Mails to both old and new address when an admin changes the address
[8e97e31](https://github.com/sebadob/rauthy/commit/8e97e310a6369bd5150d087cd5cd402d8edc221e)
[97197db](https://github.com/sebadob/rauthy/commit/97197dbcdf3268647a3269c5cf5648176f0000d7)
- Allow CORS requests to the `.well-known` endpoints to make the oidc config lookup from an external UI possible
[b57656f](https://github.com/sebadob/rauthy/commit/b57656ffaf7e822527d2d8c9d74b7c948193c220)
- include the `alg` in the JWKS response for the `openid-configuration`
[c9073cb](https://github.com/sebadob/rauthy/commit/c9073cb473be04092e326c95ca7a1b3502379f40)
- The E-Mail HTML templates have been optically adjusted a bit to make them "prettier"
[926de6e](https://github.com/sebadob/rauthy/commit/926de6e7348a8294fe284500cb41381a56a5cd2b)

### Bugfixes

- A User may have not been updated correctly in the cache when the E-Mail was changed.
[8d9cdce](https://github.com/sebadob/rauthy/commit/8d9cdce61992ee59e381876b71b18168e7e3ce31)
- With v0.16, it was possible to not be able to switch back to a password account type from passkey,
when it was a password account before already which did update its password in the past and therefore
would have entries in the DB for `last_recent_passwords` if you had the password policy correctly.
[7a965a2](https://github.com/sebadob/rauthy/commit/7a965a276b2a127057dfd589afde23cf5ec981d1)
- When you were using a password manager that filled out the username 'again' in the login form,
after the additional password request input showed up, it could reset the form on some browser.
[09d1d3a](https://github.com/sebadob/rauthy/commit/09d1d3a7446d5f636b69046c8066d69b09537411)

### Removed

- The `ADMIN_ACCESS_SESSION_ONLY` config variable was removed. This was obsolete now with
the introduction of the new ApiKey type.
[b28d8ba](https://github.com/sebadob/rauthy/commit/b28d8baa50f778867647535cf204cf87a896968b)

## v0.16.1

This is a small bugfix release.  
If you had an active and valid session from a v0.15.0, did an update to v0.16.0 and your session was still valid,
it did not have valid information about the peer IP. This is mandatory for a new feature introduced with v0.16.0
though and the warning logging in that case had an unwrap for the remote IP (which can never be null from v0.16.0 on),
which then would panic.

This is a tiny bugfix release that just gets rid of the possible panic and prints `UNKNOWN` into the logs instead.

### Changes

- print `UNKNOWN` into the logs for non-readable / -existing peer IPs
[6dfd0f4](https://github.com/sebadob/rauthy/commit/6dfd0f4299b90c03d9b5a2fb4106d72f153146af)

## v0.16.0

### Breaking

This version does modify the database and is therefore not backwards compatible with any previous version.
If you need to downgrade vom v0.15 and above, you will only be able to do this via by applying a DB Backup.

### New Features

#### User Expiry

It is now possible to limit the lifetime of a user.  
You can set an optional expiry date and time for each user. This enables temporary access, for instance in
a support case where an external person needs access for a limited time.

Once a user has expired, a few things will happen:
- The user will not be able to log in anymore.
- As soon as the scheduler notices the expiry, it will automatically invalidate all possibly existing
sessions and refresh tokens for this user. How often the scheduler will run can be configured with the
`SCHED_USER_EXP_MINS` variable. The default is 'every 60 minutes' to have a good balance between security
and resource usage. However, if you want this to be very strict, you can adjust this down to something like
'5 minutes' for instance.
- If configured, expired users can be cleaned up automatically after the configured time.
By default, expired users will not be cleaned up automatically. You can enable this feature with the
´SCHED_USER_EXP_DELETE_MINS` variable.

#### `WEBAUTHN_NO_PASSWORD_EXPIRY`

With this new config variable, you can define, if users with at least one valid registered passkey will
have expiring passwords (depending on the current password policy), or not.  
By default, these users do not need to renew their passwords like it is defined in the password policy.

#### Peer IP's for sessions -> `SESSION_VALIDATE_IP`

When a new session is being created, the peer / remote IP will be extracted and saved with the session
information. This peer IP can be checked with each access and the session can be rejected, if this IP
has changed, which will force the user to do a new login.

This will of course happen if a user is "on the road" and uses different wireless networks on the way,
but it prevents a session hijack and usage from another machine, if an attacker has full access to the
victims machine and even can steal the encrypted session cookie and(!) the csrf token saved inside the
local storage. This is very unlikely, since the attacker would need to have full access to the machine
anyway already, but it is just another security mechanism.

If this IP should be validated each time can be configured with the new `SESSION_VALIDATE_IP` variable.
By default, peer IP's will be validated and a different IP for an existing session will be rejected.

#### Prometheus metrics exporter

Rauthy starts up a second HTTP Server for prometheus metrics endpoint and (optional) SwaggerUI.
By default, the SwaggerUI from the `Docs` link in the Admin UI will not work anymore, unless you
specify the SwaggerUI via config to be publicly available. This just reduces the possible attack
surface by default.

New config variables are:

```
# To enable or disable the additional HTTP server to expose the /metrics endpoint
# default: true
#METRICS_ENABLE=true

# The IP address to listen on for the /metrics endpoint.
# You do not want to expose your metrics on a publicly reachable endpoint!
# default: 0.0.0.0
#METRICS_ADDR=0.0.0.0

# The post to listen on for the /metrics endpoint.
# You do not want to expose your metrics on a publicly reachable endpoint!
# default: 9090
#METRICS_PORT=9090

# If the Swagger UI should be served together with the /metrics route on the internal server.
# It it then reachable via:
# http://METRICS_ADDR:METRICS_PORT/docs/v1/swagger-ui/
# (default: true)
#SWAGGER_UI_INTERNAL=true

# If the Swagger UI should be served externally as well. This makes the link in the Admin UI work.
#
# CAUTION: The Swagger UI is open and does not require any login to be seen!
# Rauthy is open source, which means anyone could just download it and see on their own,
# but it may be a security concern to just expose less information.
# (default: false)
#SWAGGER_UI_EXTERNAL=false
```

#### User Verification status for Passkeys

For all registered passkeys, the User Verification (UV) state is now being saved and optionally checked.
You can see the status for each device with the new fingerprint icon behind its name in the UI.

New config variable:

```
# This feature can be set to 'true' to force User verification during the Webauthn ceremony.
# UV will be true, if the user does not only need to verify its presence by touching the key,
# but by also providing proof that he knows (or is) some secret via a PIN or biometric key for
# instance. With UV, we have a true MFA scenario where UV == false (user presence only) would
# be a 2FA scenario (with password).
#
# Be careful with this option, since Android and some special combinations of OS + browser to
# not support UV yet.
# (default: false)
#WEBAUTHN_FORCE_UV=true
```

#### Passkey Only Accounts

This is the biggest new feature for this release. It allows user accounts to be "passkey only".

A passkey only account does not have a password. It works only with registered passkeys with
forced additional User Verification (UV).

Take a look at the updated documentation for further information:  
[Passkey Only Accounts]https://sebadob.github.io/rauthy/config/fido.html#passkey-only-accounts


#### New E-Mail verification flow

If an already existing user decides to change the E-Mail linked to the account, a new verification
flow will be started:

- A user changes the E-Mail in the Account view.
- The E-Mail will not be updated immediately, but:
    - A verification mail will be sent to the new address with an expiring magic link.
    - After the user clicked the link in the mail, the new address will be verified.
- Once a user verifies the new E-Mail:
    - The address will finally be updated in the users profile.
    - Information E-Mails about the change will be sent to the old and the new address

#### `EMAIL_SUB_PREFIX` config variable

This new variable allows you to add an E-Mail subject prefix to each mail that Rauthy sends out.
This makes it easier for external users to identify the email, what it is about and what it is
doing, in case the name 'Rauthy' does not mean anything to them.

```
# Will be used as the prefix for the E-Mail subject for each E-Mail that will be sent out to a client.
# This can be used to further customize your deployment.
# default: "Rauthy IAM"
EMAIL_SUB_PREFIX="Rauthy IAM"
```

#### New nicely looking error page template

In a few scenarios, for instance wrong client information for the authorization code flow or
a non-existing or expired magic link, Rauthy now does not return the generic JSON error response,
but actually a translated HTML page which informs the user in a nicer looking way about the problem.
This provides a way better user experience especially for all Magic Link related requests.

#### Rauhty DB Version check

This is an additional internal check which compares the version of the DB during startup and 
the App version of Rauthy itself. This makes it possible to have way more stable and secure
migrations between versions in the future and helps prevent user error during upgrades.

### Changes

- legacy MFA app pieces and DB columns have been cleaned up
[dce148a](https://github.com/sebadob/rauthy/commit/dce148a2d665836ec733b8d0904e9d3892c40b2f)
[423db7a](https://github.com/sebadob/rauthy/commit/423db7a66c412ce9d04ec317cd0a237e278653bb)
- user expiry feature
[bab6bfc](https://github.com/sebadob/rauthy/commit/bab6bfcecd62e44d23b9527009904c12a04e567a)
[e63d1ce](https://github.com/sebadob/rauthy/commit/e63d1ce746dd27ed78acebb688d44fbf7a0b50dc)
[566fff1](https://github.com/sebadob/rauthy/commit/566fff12e0a3e8ee2b867ecfd590cbe0ef24378d)
- `WEBAUTHN_NO_PASSWORD_EXPIRY` config variable
[7e16b6e](https://github.com/sebadob/rauthy/commit/7e16b6e7c5e5038c653f55807e6aeed35e7536cb)
- `SESSION_VALIDATE_IP` config variable
[828bcd2](https://github.com/sebadob/rauthy/commit/828bcd285b844d846f23e22e488a372dba0c59ff)
[26924ff](https://github.com/sebadob/rauthy/commit/26924ffbc2054abc537bba4ef6cd98b50ad594c3)
- `/metrics` endpoint
[085d412](https://github.com/sebadob/rauthy/commit/085d412e51c3f77449c39449206d296b710c1d9b)
- `WEBAUTHN_FORCE_UV` config variable
- Passkey Only Accounts
[6c2406c](https://github.com/sebadob/rauthy/commit/6c2406c4f261ec0e784df6ee50efd837a86160f7)
- New E-Mail verification flow
[260169d](https://github.com/sebadob/rauthy/commit/260169d289731c44289cd6fce1c642c6d26786ae)
- New nicely looking error page template
[0e476ab](https://github.com/sebadob/rauthy/commit/0e476ab0ec8728450d83a315240dd870ac34cddf)
- Rust v1.73 update
[43f5b61](https://github.com/sebadob/rauthy/commit/43f5b61c86d563cefaf2ea2ee832e3317bcfcfb7)
- `EMAIL_SUB_PREFIX` config variable
[af85839](https://github.com/sebadob/rauthy/commit/af858398d61b2bea347dabcd4b7adf6a8c60f86d)
- Rauthy DB Version check
[d2d9271](https://github.com/sebadob/rauthy/commit/d2d92716b93a9ed1e6dc02ce7906f0741044d464)
- Cleanup schedulers in HA_MODE run on leader only
[4881dd8](https://github.com/sebadob/rauthy/commit/4881dd83a9b7d298fc998c635face7feb55549ff)
- Updated documentation and book
[5bbaae9](https://github.com/sebadob/rauthy/commit/5bbaae9c24ce49dec0c10908d351257519279c9a)
- Updated dependencies
[0f11923](https://github.com/sebadob/rauthy/commit/0f1192354c805832d8f252d3204f624deb9e6561)

## v0.15.0

### Breaking

This version does modify the database and is therefore not backwards compatible with any previous version.
If you need to downgrade vom v0.15 and above, you will only be able to do this via by applying a DB Backup.

### Changes

This release is all about new Passkey Features.

- A user is not limited to just 2 keys anymore
- During registration, you can (and must) provide a name for the passkey, which helps you identify and distinguish
your keys, when you register multiple ones.
- The `exclude_credentials` feature is now properly used and working. This makes sure, that you cannot register the
same Passkey multiple times.
- The Passkeys / MFA section in the Admin UI has been split from the User Password section to be more convenient to use

Commits:  
- New Passkey Features  
[d317d90](https://github.com/sebadob/rauthy/commit/d317d902f66613b743955117dbede3d52efa74b6)
[cd5d086](https://github.com/sebadob/rauthy/commit/cd5d086e0b38d8d906d3ce698ed9860bec3d52ab)
[61464bf](https://github.com/sebadob/rauthy/commit/61464bfcc69921fae0c826fe8c3bac73e290a242)
[e70c5a7](https://github.com/sebadob/rauthy/commit/e70c5a7bbf8f40f90a9d8d3d6570736e5322a0c5)
[bc75610](https://github.com/sebadob/rauthy/commit/bc75610d7aa4cfc35abf1e0ee67af3b4dc2eea82)
[49e9630](https://github.com/sebadob/rauthy/commit/49e9630bb3ffc25be1523492ef47d2fc66dc041c)
- New config option `WEBAUTHN_FORCE_UV` to optionally reject Passkeys without user verification
[c35ecc0](https://github.com/sebadob/rauthy/commit/c35ecc090b222196cf1ac095aa2b340edbeec0f5)
- Better `/token` endpoint debugging capabilities
[87f7969](https://github.com/sebadob/rauthy/commit/87f79695b86f9b7199dbad244c61e2eb1a0c6bcb)

## v0.14.5

This is the last v0.14 release.  
The next v0.15 will be an "in-between-release" which will do some migration preparations for Webauthn
/ FIDO 2 updates and features coming in the near future.

- Removed duplicate `sub` claims from JWT ID Tokens
[a35db33](https://github.com/sebadob/rauthy/commit/a35db330ff7c6ee680a7d834f08a3db077e08073)
- Small UI improvements:
    - Show loading indicator when doing a password change
    - The Loading animation was changes from JS to a CSS animation
[abd0a06](https://github.com/sebadob/rauthy/commit/abd0a06280de4fedef9028f142b6e844bf132d80)
- Upgrades to actix-web 4.4 + rustls 0.21 (and all other minor upgrades)
[070a453](https://github.com/sebadob/rauthy/commit/070a453aaa584ff8d024284de91477626fe5ea6c)

## v0.14.4

This release mostly finishes the translation / i18n part of rauthy for now and adds some other
smaller improvements.  
Container Images will be published with ghcr.io as well from now on. Since I am on the free plan
here, storage is limited and too old versions will be deleted at some point in the future.
However, I will keep pushing all of them to docker hub as well, where you then should be able
to find older versions too. ghcr.io is just preferred, because it is not so hardly rate limited
than the docker hub free tier is.

- Added translations for E-Mails
[11544ac](https://github.com/sebadob/rauthy/commit/11544ac46fcddeb53a511b8ad702b1ad2868148e)
- Made all UI parts work on mobile (except for the Admin UI itself)
[a4f31f2](https://github.com/sebadob/rauthy/commit/a4f31f22396b5767c5a2c20e0253171910296447)
[4ee3540](https://github.com/sebadob/rauthy/commit/4ee3540d9b32e6f0e2fbd5cf5eabcd7736179da8)
- Images will be published on Github Container Registry as well from now on
[cc15ea9](https://github.com/sebadob/rauthy/commit/cc15ea9cb5d7c0bd6d3cad1fea908e656f488e50)
- All dependencies have been updates in various places. This just keeps everything up to date
and fixed some potential security issues in third party libraries

## v0.14.3

- UI: UX Improvements to Webauthn Login when the user lets the request time out
[7683133](https://github.com/sebadob/rauthy/commit/76831338e88a36cc3166039493318c38cc7a1e49)
- UI: i18n for password reset page
[27e620e](https://github.com/sebadob/rauthy/commit/27e620edb34d803259a000d837513920905d2332)
- Keep track of users chosen language in the database
[7517693](https://github.com/sebadob/rauthy/commit/7517693ddec4f5e0215fcced9dde6322e665ff10)
- Make user language editable in the admin ui
[77886a9](https://github.com/sebadob/rauthy/commit/77886a958655b721e96c092df739627d0d5d9172)
[1061fc2](https://github.com/sebadob/rauthy/commit/1061fc212bb684b153ae99f1439ea312091e32cd)
- Update the users language in different places:
    - Language switch in the Account page
    - Fetch users chosen language from User Registration
    - Selector from Registration in Admin UI
[5ade849](https://github.com/sebadob/rauthy/commit/5ade849dd5a0b139c73a61b8fefde03eff3036bf)

## v0.14.2

- Fix for the new LangSelector component on mobile view
- Add default translations (english) for the PasswordPolicy component for cases when it is used
in a non-translated context
[2f8a627](https://github.com/sebadob/rauthy/commit/2f8a6270f97df075d52507c0aa4e6850e5ef8edc)

## v0.14.1

Bugfix release for the Dockerfiles and Pagination in some places

- Split the Dockerfiles into separate files because the `ARG` introduced access rights problems
[25e3918](https://github.com/sebadob/rauthy/commit/25e39189f95d21e84e24f6d21670709a7ee1effd)
- Small bugfix for the pagination component in some places
[317dbad](https://github.com/sebadob/rauthy/commit/317dbadb0391b78aca0cdadc65dff79d9bf74717)

## v0.14.0

This release is mostly about UI / UX improvements and some smaller bugfixes.

- UI: Client side pagination added
[60a499a](https://github.com/sebadob/rauthy/commit/60a499aee0ff071937a03a78759e447f0d477c90)
- Browsers' native language detection
[884f599](https://github.com/sebadob/rauthy/commit/884f5995b71b8faf6ebcacf4331fdda6ccd78d57)
- `sqlx` v0.7 migration
[7c7a380](https://github.com/sebadob/rauthy/commit/7c7a380bdac520df7c29d0d0f4b3b7d3d48be943)
- Docker container image split
[adb3971](https://github.com/sebadob/rauthy/commit/adb397139627a4e3b3f72682457bdfd24b6ce9f4)
- Target database validation before startup
[a68c652](https://github.com/sebadob/rauthy/commit/a68c6527a68b970221f3c48982cabc418aa81d39)
- UI: I18n (english and german currently) for: Index, Login, Logout, Account, User Registration
[99e454e](https://github.com/sebadob/rauthy/commit/99e454ee459ac041c6975df99d481f0145cf7fa4)
[dd2e9ae](https://github.com/sebadob/rauthy/commit/dd2e9ae579444359dc04db76bc1e13d3d0753fe6)
[7b401f6](https://github.com/sebadob/rauthy/commit/7b401f6f0639c053b0e9475121f9ec814f80ef65)
- UI: Custom component to overwrite the browsers' native language
[4208fdb](https://github.com/sebadob/rauthy/commit/4208fdb9904044f9c5b9ddfb5eb2c42c1264481a)
- Some Readme and Docs updates
[e2ebef9](https://github.com/sebadob/rauthy/commit/e2ebef9c72d0f212ac5a4b39241b6d5d486bd8b0)
[d0a71d6](https://github.com/sebadob/rauthy/commit/d0a71d641c3c829b33191cc2c0ff04e5f7d27017)
- The `sub` claim was added to the ID token and will contain the Users UID
[6b0a8b0](https://github.com/sebadob/rauthy/commit/6b0a8b0679484c5515a068911810159a5a39e07d)

## v0.13.3

- UI: small visual bugfixes and improvements in different places
[459bdbd](https://github.com/sebadob/rauthy/commit/459bdbd55ca60bdb0076908131c569a4dc653086)
[57a5600](https://github.com/sebadob/rauthy/commit/57a56000f6ffecf46bd1d202a3bea5a2ded4985f)
- UI: All navigation routes can be reached via their own link now. This means a refresh of
the page does not return to the default anymore
[4999995](https://github.com/sebadob/rauthy/commit/49999950ac1ade24e433e911df84c99256a7f4d0)
[7f0ac0b](https://github.com/sebadob/rauthy/commit/7f0ac0b0d1cf1e2c53881c4a4e010ce43cc2ec11)
[cadaa40](https://github.com/sebadob/rauthy/commit/cadaa407efa9b70b5159e6ec42b5151f8ef79997)
- UI: added an index to the users table to prevent a rendering bug after changes
[e35ffbe](https://github.com/sebadob/rauthy/commit/e35ffbe9cb4e14785c61249d141895c1a7fb4921)

## v0.13.2

- General code and project cleanup
[4531ae9](https://github.com/sebadob/rauthy/commit/4531ae93d453429a54198211b7d122dada452ae4)
[782bb9a](https://github.com/sebadob/rauthy/commit/782bb9adbbb12f77232b1820e7dd05265c0fdf00)
[0c5ad02](https://github.com/sebadob/rauthy/commit/0c5ad02e369935b01aac46988a2242c859737e24)
[e453142](https://github.com/sebadob/rauthy/commit/e45314269234612a3eec046073e988e260a7ca31)
[85fbafe](https://github.com/sebadob/rauthy/commit/85fbafe5ef6b8f124af6af1508b6e2bab067a8ff)
- Created a `justfile` for easier development handling
[4aa5b99](https://github.com/sebadob/rauthy/commit/4aa5b9993897e43dfc765eb2849172bc087ea34c)
[1489efe](https://github.com/sebadob/rauthy/commit/1489efe139c0a0c79169f47ba4fc964cdc6b6e3e)
- UI: fixed some visual bugs and improved the rendering with larger default browser fonts
[45334fd](https://github.com/sebadob/rauthy/commit/45334fd65049f2950dae3a2bc28c5667c275aa1d)

## v0.13.1

This is just a small bugfix release.

- UI Bugfix: Client flow updates were not applied via UI
[6fe8fbc](https://github.com/sebadob/rauthy/commit/6fe8fbc1440498ea126a5aee5bed9dfe34e367d4)

## v0.13.0

- Improved container security: Rauthy is based off a Scratch container image by default now. This improved the security
quite a lot, since you cannot even get a shell into the container anymore, and it reduced the image size by another
~4MB.  
This makes it difficult however if you need to debug something, for instance when you use a SQLite deployment. For this reason, you can append `-debug` to a tag
and you will get an Alpine based version just like before.
[1a7e79d](https://github.com/sebadob/rauthy/commit/1a7e79dc96d27d8d180d1e4394644c8851cbdf70)
- More stable HA deployment: In some specific K8s HA deployments, the default HTTP2 keep-alive's from
[redhac](https://github.com/sebadob/redhac) were not good enough and we got broken pipes in some environments which
caused the leader to change often. This has been fixed in [redhac-0.6.0](https://github.com/sebadob/redhac/releases/tag/v0.6.0)
too, which at the same time makes Rauthy HA really stable now.
- The client branding section in the UI has better responsiveness for smaller screens
[dfaa23a](https://github.com/sebadob/rauthy/commit/dfaa23a30ccf77da2b29654c7dd3b41a4ca78168)
- For a HA deployment, cache modifications are now using proper HA cache functions. These default back to the single
instance functions in non-HA mode since [redhac-0.6.0](https://github.com/sebadob/redhac/releases/tag/v0.6.0)
[7dae043](https://github.com/sebadob/rauthy/commit/7dae043d7b42724adad85b5ed54f1dcd9d143d27)
- All static UI files are now precompressed with gzip and brotli to use even fewer resources
[10ad51a](https://github.com/sebadob/rauthy/commit/10ad51a296c5a7596b34f9c726fe87480b6ec42c)
- CSP script-src unsafe-inline was removed in favor of custom nonce's
[7de918d](https://github.com/sebadob/rauthy/commit/7de918d601007d2807701a096d6403bf2b3274c9)
- UI migrated to Svelte 4
[21f73ab](https://github.com/sebadob/rauthy/commit/21f73abfb0332be3fc391b9d108655a0cd5a3cec)

## v0.12.0

Rauthy goes open source
