# Changelog

## 0.26.2

### Bugfix

This patch reverts an unintended change to the `user:group` inside the container images.
This will fix issues with migrations from existing deployments using SQLite with manually managed
volume access rights.

v0.26.0 changed from `scratch` to `gcr.io/distroless/cc-debian12:nonroot` as the base image for the final deployment.
The distroless image however sets a user of `65532` by default, while it always has been `10001:10001` before.
The affected versions are

- `0.26.0`
- `0.26.1`

Starting from this release (`0.26.2`), the user inside the container will be the same one as before:

`10001:10001`

[839724001710cb095f39ff7df6be00708a01801a](https://github.com/sebadob/rauthy/pull/596/commits/839724001710cb095f39ff7df6be00708a01801a)

## 0.26.1

### Changes

#### Upstream Auth Provider Query Params

Some upstream auth providers need custom query params appended to their authorization endpoint URL.
Rauthy will now accept URLs in the auth provider config with pre-defined query params, as long as they
don't interfere with OIDC default params.

[7dee26a](https://github.com/sebadob/rauthy/commit/7dee26af0ab757cd80395652fe03f82ffbc2c8bc)

#### Option Log Fmt as JSON

To make automatic parsing of logs possible (to some extent), you now have the ability to change the logging output from
text to json with the following new config variable:

```
# You can change the log output format to JSON, if you set:
# `LOG_FMT=json`.
# Keep in mind, that some logs will include escaped values,
# for instance when `Text` already logs a JSON in debug level.
# Some other logs like an Event for instance will be formatted 
# as Text anyway. If you need to auto-parse events, please consider 
# using an API token and listen ot them actively.
# default: text
#LOG_FMT=text
```

[1ef1353](https://github.com/sebadob/rauthy/commit/1ef1353162655fb5dc54396b02c73ef80ff0506a)

### Bugfix

- With relaxing requirements for password resets for new users, a bug has been introduced that would prevent
  a user from registering an only-passkey account when doing the very first "password reset".
  [de2cfea](https://github.com/sebadob/rauthy/commit/de2cfea107cff4fb98fc81be692d0b83cf597398)

## 0.26.0

### Breaking

#### Deprecated API Routes Removal

The following API routes have been deprecated in the last version and have now been fully removed:

- `/oidc/tokenInfo`
- `/oidc/rotateJwk`

#### Cache Config

The whole `CACHE` section in the config has been changed:

```
#####################################
############## CACHE ################
#####################################

# Can be set to 'k8s' to try to split off the node id from the hostname
# when Hiqlite is running as a StatefulSet inside Kubernetes.
# Will be ignored if `HQL_NODE_ID_FROM=k8s`
#HQL_NODE_ID_FROM=k8s

# The node id must exist in the nodes and there must always be
# at least a node with ID 1
HQL_NODE_ID=1

# All cluster member nodes.
# To make setting the env var easy, the values are separated by `\s`
# while nodes are separated by `\n`
# in the following format:
#
# id addr_raft addr_api
# id addr_raft addr_api
# id addr_raft addr_api
#
# 2 nodes must be separated by 2 `\n`
HQL_NODES="
1 localhost:8100 localhost:8200
"

# If set to `true`, all SQL statements will be logged for debugging
# purposes.
# default: false
#HQL_LOG_STATEMENTS=false

# If given, these keys / certificates will be used to establish
# TLS connections between nodes.
#HQL_TLS_RAFT_KEY=tls/key.pem
#HQL_TLS_RAFT_CERT=tls/cert-chain.pem
#HQL_TLS_RAFT_DANGER_TLS_NO_VERIFY=true

#HQL_TLS_API_KEY=tls/key.pem
#HQL_TLS_API_CERT=tls/cert-chain.pem
#HQL_TLS_API_DANGER_TLS_NO_VERIFY=true

# Secrets for Raft internal authentication as well as for the API.
# These must be at least 16 characters long and you should provide
# different ones for both variables.
HQL_SECRET_RAFT=SuperSecureSecret1337
HQL_SECRET_API=SuperSecureSecret1337

# You can either parse `ENC_KEYS` and `ENC_KEY_ACTIVE` from the
# environment with setting this value to `env`, or parse them from
# a file on disk with `file:path/to/enc/keys/file`
# default: env
#HQL_ENC_KEYS_FROM=env
```

[0919767](https://github.com/sebadob/rauthy/commit/09197670e6491f83a8b739c0f195d4b842abe771)

#### `/auth/v1/health` Response Change

The response for `/auth/v1/health` has been changed.

If you did not care about the response body, there is nothing to do for you. The body itself returns different values
now:

```rust
struct HealthResponse {
    db_healthy: bool,
    cache_healthy: bool,
}
```

[0919767](https://github.com/sebadob/rauthy/commit/09197670e6491f83a8b739c0f195d4b842abe771)

### Changes

#### ZH-Hans Translations

Translations for `ZH-Hans` have been added to Rauthy. These exist in all places other than the Admin UI, just like the
existing ones already.

[ec6c2c3](https://github.com/sebadob/rauthy/commit/ec6c2c3bb4e8b41fa0cd2a60ccc4043d051c17a5)  
[fcba3c7](https://github.com/sebadob/rauthy/commit/fcba3c7cd7bce7e15d911c0f9d7f55f852e7c424)

#### Support for deep-linking client apps like Tauri

Up until v0.25, it was not possible to set the `Allowed Origin` for a client in a way that Rauthy would allow access
for instance from inside a Tauri app. The reason is that Tauri (and most probably others) do not set an HTTP / HTTPS
scheme in the `Origin` header, but something like `tauri://`.

Rauthy has now support for such situations with adjusted validation for the Origin values and a new config variable
to allow specific, additional `Origin` schemes:

```
# To bring support for applications using deep-linking, you can set custom URL
# schemes to be accepted when present in the `Origin` header. For instance, a
# Tauri app would set `tauri://` instead of `https://`.
#
# Provide the value as a space separated list of Strings, like for instance:
# "tauri myapp"
ADDITIONAL_ALLOWED_ORIGIN_SCHEMES="tauri myapp"
```

[d52f76c](https://github.com/sebadob/rauthy/commit/d52f76c71b350a08bb67080e620b87bf55d00389)

#### More stable health checks in HA

For HA deployments, the `/health` checks are more stable now.  
The quorum is also checked, which will detect network segmentations. To achieve this and still make it possible to use
the health check in situations like Kubernetes rollouts, a delay has been added, which will simply always return `true`
after a fresh app start. This initial delay make it possible to use the endpoint inside Kubernetes and will not prevent
from scheduling the other nodes. This solves a chicken-and-egg problem.

You usually do not need to care about it, but this value can of course be configured:

```
# Defines the time in seconds after which the `/health` endpoint 
# includes HA quorum checks. The initial delay solves problems 
# like Kubernetes StatefulSet starts that include the health 
# endpoint in the scheduling routine. In these cases, the scheduler 
# will not start other Pods if the first does not become healthy.
# 
# This is a chicken-and-egg problem which the delay solves.
# There is usually no need to adjust this value.
#
# default: 30
#HEALTH_CHECK_DELAY_SECS=30
```

[5d1ddca](https://github.com/sebadob/rauthy/commit/5d1ddcac2222b77f9a38baf93b44f896f5ba7933)

#### Migration to `ruma`

To send out Matrix notifications, Rauthy was using the `matrix-sdk` up until now. This crate however comes with a huge
list of dependencies and at the same time pushes too few updates. I had quite a few issues with it in the past because
it was blocking me from updating other dependencies.

To solve this issue, I decided to drop `matrix-sdk` in favor of `ruma`, which it is using under the hood anyway. With
`ruma`, I needed to do a bit more work myself since it's more low level, but at the same time I was able to reduce the
list of total dependencies Rauthy has by ~90 crates.

This made it possible to finally bump other dependencies and to start the internal switch
from [redhac](https://github.com/sebadob/redhac) to [Hiqlite](https://github.com/sebadob/hiqlite) for caching.

**IMPORTANT:**  
If you are using a self-hosted homeserver or anything else than the official `matrix.org` servers for Matrix event
notifications, you must set a newly introduced config variable:

```
# URL of your Matrix server.
# default: https://matrix.org
#EVENT_MATRIX_SERVER_URL=https://matrix.org
```

[0b50376](https://github.com/sebadob/rauthy/commit/0b5037610d475e7ad6fc0a8bf3b851330088cab1)

#### Internal Migration from `redhac` to `hiqlite`

The internal cache layer has been migrated from [redhac](https://github.com/sebadob/redhac)
to [Hiqlite](https://github.com/sebadob/hiqlite).

A few weeks ago, I started rewriting the whole persistence layer from scratch in a separate project. `redhac` is working
fine, but it has some issues I wanted to get rid of.

- its network layer is way too complicated which makes it very hard to maintain
- there is no "sync from other nodes" functionality, which is not a problem on its own, but leads to the following
- for security reasons, the whole cache is invalidated when a node has a temporary network issue
- it is very sensitive to even short term network issues and leader changes happen too often for my taste

I started the [Hiqlite](https://github.com/sebadob/hiqlite) project some time ago to get rid of these things and have
additional features. It is outsourced to make it generally usable in other contexts as well.

This first step will also make it possible to only have a single container image in the future without the need to
decide between Postgres and SQLite via the tag.

[0919767](https://github.com/sebadob/rauthy/commit/09197670e6491f83a8b739c0f195d4b842abe771)

#### Local Development

The way the container images are built, the builder for the images is built and also the whole `justfile` have been
changed quite a bit. This will not concern you if you are not working with the code.

The way of wrapping and executing everything inside a container, even during local dev, became tedious to maintain,
especially for different architectures and I wanted to get rid of the burden of maintenance, because it did not provide
that many benefits. Postgres and Mailcrab will of course still run in containers, but the code itself for backend and
frontend will be built and executed locally.

The reason I started doing all of this inside containers beforehand was to not need a few additional tool installed
locally to make everything work, but the high maintenance was not worth it in the end. This change now reduced the
size of the Rauthy builder image from 2x ~4.5GB down to 1x ~1.9GB, which already is a big improvement. Additionally,
you don't even need to download the builder image at all when you are not creating a production build, while beforehand
you always needed the builder image in any case.

To encounter the necessary dev tools installation and first time setup, I instead added a new `just` recipe called
`setup` which will do everything necessary, as long as you have the prerequisites available (which you needed before
as well anyway, apart from `npm`). This has been updated in the
[CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md).

### Bugfix

- The `refresh_token` grant type on the `/token` endpoint did not set the original `auth_time` for the `id_token`, but
  instead calculated it from `now()` each time.
  [aa6e07d](https://github.com/sebadob/rauthy/commit/aa6e07db8822e72e28329b0ecea52e6113851d4a)

## v0.25.0

### Changes

#### Token Introspection

The introspection endpoint has been fixed in case of the encoding like mentioned in bugfixes.  
Additionally, authorization has been added to this endpoint. It will now make sure that the request also includes
an `AUTHORIZATION` header with either a valid `Bearer JwtToken` or `Basic ClientId:ClientSecret` to prevent
token scanning.

The way of authorization on this endpoint is not really standardized, so you may run into issues with your client
application. If so, you can disable the authentication on this endpoint with

```
# Can be set to `true` to disable authorization on `/oidc/introspect`.
# This should usually never be done, but since the auth on that endpoint is not
# really standardized, you may run into issues with your client app. If so, 
# please open an issue about it.
# default: false
DANGER_DISABLE_INTROSPECT_AUTH=true
```

[2e84ceb](https://github.com/sebadob/rauthy/commit/2e84ceb062c677e863f5ad524c7fe8b2af21449b)
[7087a59](https://github.com/sebadob/rauthy/commit/7087a5998f5c687c6b7bd90a0771451ddec9068e)

#### API Routes Normalization

In preparation for a clean v1.0.0, some older API routes have been fixed regarding their casing and naming.
The "current" or old routes and names will be available for exactly one release and will be phased out afterward
to have a smooth migration, just in case someone uses these renamed routes.

- `/oidc/tokenInfo` -> `/oidc/introspect`
- `/oidc/rotateJwk` -> `/oidc/rotate_jwk`

Since I don't like `kebab-case`, most API routes are written in `snake_case`, with 2 exceptions that follow RFC namings:

- `openid-configuration`
- `web-identity`

All the `*info` routes like `userinfo` or `sessioninfo` are not `kebab_case` on purpose, just to match other IdPs and
RFCs a bit more.

There is not a single `camelCase` anymore in the API routes to avoid confusion and issues in situations where you could
for instance mistake an uppercase `I` as a lowercase `l`. The current `camelCase` endpoints only exist for a smoother
migration and will be phased out with the next bigger release.

[107f148](https://github.com/sebadob/rauthy/commit/107f14807760e56a1671e587fa9b08284589f932)

#### Config Read

The current behavior of reading in config variables was not working as intended.

Rauthy reads the `rauthy.cfg` as a file first and the environment variables afterward. This makes it possible to
configure it in any way you like and even mix and match.  
However, the idea was that any existing variables in the environment should overwrite config variables and therefore
have the higher priority. This was exactly the other way around up until `v0.24.1` and has been fixed now.

How Rauthy parses config variables now correctly:

1. read `rauthy.cfg`
2. read env var
3. all existing env vars will overwrite existing vars from `rauthy.cfg` and therefore have the higher priority

[28b2457](https://github.com/sebadob/rauthy/commit/28b2457a53bf31163e94a363f2009b811e1b0b76)

### Bugfixes

- The token introspection endpoint was only accepting requests with `Json` data, when it should have instead been
  with `Form` data.

## 0.24.1

The last weeks were mostly for updating the documentation and including all the new features that came to Rauthy in
the last months. Some small things are still missing, but it's almost there.

Apart from that, this is an important update because it fixes some security issues in external dependencies.

### Security

Security issues in external crates have been fixed:

- moderate [matrix-sdk-crypto](https://github.com/sebadob/rauthy/security/dependabot/54)
- moderate [openssl](https://github.com/sebadob/rauthy/security/dependabot/55)
- low [vodozemac](https://github.com/sebadob/rauthy/security/dependabot/53)

### Changes

#### `S3_DANGER_ACCEPT_INVALID_CERTS` renamed

The config var `S3_DANGER_ACCEPT_INVALID_CERTS` has been renamed to `S3_DANGER_ALLOW_INSECURE`. This is not a breaking
change right now, because for now Rauthy will accept both versions to not introduce a breaking change, but the
deprecated values will be removed after v0.24.

#### S3 Compatibility

Quite a few internal dependencies have been updated to the latest versions (where it made sense).

One of them was my own [cryptr](https://github.com/sebadob/cryptr). This was using the `rusty-s3` crate beforehand,
which is a nice one when working with S3 storages, but had 2 issues. One of them is that it is using pre-signed URLs.
That is not a flaw in the first place, just a design decision to become network agnostic. The other one was that it
signed the URL in a way that would make the request not compatible with [Garage](https://garagehq.deuxfleurs.fr/).  
I migrated `cryptr` to my own [s3-simple](https://github.com/sebadob/s3-simple) which solves these issues.

This update brings compatibility with the `garage` s3 storage for Rauthy's S3 backup feature.

[f1eab35](https://github.com/sebadob/rauthy/commit/f1eab35dcbf195ed38d11756e1df69f42e05e7e0)

### Bugfixes

- Fetching the favicon (and possibly other images) was forbidden because of the new CSRF middleware from some weeks
  ago.
  [76cd728](https://github.com/sebadob/rauthy/commit/76cd7281fcd1493c9f0cbb208c3fa7ef93814422)
- The UI and the backend had a difference in input validation for `given_name` and `family_name` which could make
  some buttons in the UI get stuck. This has been fixed and the validation for these 2 is the same everywhere and at
  least 1 single character is required now.
  [19d512a](https://github.com/sebadob/rauthy/commit/19d512ad6ea930467f51d7b704252d3edee7ef1c)

## v0.24.0

Many thousands of lines have been refactored internally to provide better maintainability in the future.
These are not mentioned separately, since they did not introduce anything new. Apart from this, there are only small
changes, but one of them is an important breaking change.

### Breaking

#### `TRUSTED_PROXIES` Config Variable

The new config variable `TRUSTED_PROXIES` introduces a breaking change in some cases.  
If you are running Rauthy with either `PROXY_MODE=true` or with a set `PEER_IP_HEADER_NAME` value, you must add the
`TRUSTED_PROXIES` to your existing config before updating.

This value specifies trusted proxies in the above situation. The reason is that Rauthy extracts the client IP from
the HTTP headers, which could be spoofed if they are used without validating the source. This was not a security issue,
but gave an attacker the ability to blacklist or rate-limit IPs that do not belong to him.

When `PROXY_MODE=true` or set `PEER_IP_HEADER_NAME`, Rauthy will now only accept direct connections from IPs specified
with `TRUSTED_PROXIES` and block all other requests. You can provide a list of CIDRs to have full flexibility for your
deployment.

```
# A `\n` separated list of trusted proxy CIDRs.
# When `PROXY_MODE=true` or `PEER_IP_HEADER_NAME` is set,
# these are mandatory to be able to extract the real client
# IP properly and safely to prevent IP header spoofing.
# All requests with a different source will be blocked.
#TRUSTED_PROXIES="
#192.168.14.0/24
#10.0.0.0/8
#"
```

**Note:**  
Keep in mind, that you must include IPs for direct health checks like for instance inside Kubernetes here,
if they are not being sent via a trusted proxy.

[e1ae491](https://github.com/sebadob/rauthy/commit/e1ae49164f9753e3ec57cb4b6e2aed8614227bce)

### Features

#### User Registration Domain Blacklisting

If you are using an open user registration without domain restriction, you now have the possibility to blacklist
certain E-Mail provider domains. Even if your registration endpoint allows registrations, this blacklist will be
checked and deny requests with these domains.  
This is mainly useful if you want to prevent malicious E-Mail providers from registering and spamming your database.

```
# If `OPEN_USER_REG=true`, you can blacklist certain domains
# on the open registration endpoint.
# Provide the domains as a `\n` separated list.
#USER_REG_DOMAIN_BLACKLIST="
#example.com
#evil.net
#"
```

[824b109](https://github.com/sebadob/rauthy/commit/824b109e081f608a9b950483b3da90cde288f6eb)

### Changes

Even though it was not needed so far, the OIDC userinfo endpoint now has a proper `POST` handler in addition to the
existing `GET` to comply with the RFC.  
[05a8793](https://github.com/sebadob/rauthy/commit/05a8793f013f864db1855ae0ee1c848ec36b9254)

### Bugfixes

- The upstream crate `curve25519-dalek` had a moderate timing variability security issue
  [8bb4069](https://github.com/sebadob/rauthy/commit/8bb4069e5f22439b08f9f5a51059561007b90af3)

## v0.23.5

### Upstream IdP Locale Fix

This patch fixes a regression from fixing the special characters encoding in upstream IdP JWT tokens. A panic was
possible when the upstream IdP did not include a `locale` in the `id_token`.  
[ea24e7e](https://github.com/sebadob/rauthy/commit/ea24e7e73f883446a30d7c4f8e38837148da9732)  
[481c9b3](https://github.com/sebadob/rauthy/commit/481c9b36d3304a2100c2c4906013224c8a5a934f)

## v0.23.4

This is a tiny update, but brings an important bugfix for upstream IdPs.

### Bugfix

A bug has been fixed in case an upstream IdP included special characters inside Strings in the returned JWT token after
a successful user login flow.  
Since JWT tokens should use UNICODE encoding in these cases, it is not possible to do zero-copy deserialization into
Rust UTF8 string slices in that case. This has been fixed in a way, that only when there are existing special
characters,
Rauthy will now do the additional, necessary String allocations for the deserialization process.  
This should fix current issues when logging in via an upstream IdP with special characters inside the E-Mail address for
instance.  
[aa97cb8](https://github.com/sebadob/rauthy/commit/aa97cb8ba2100f540d48e98aa597c11963c84be3)

Apart from that, there were some minor UX improvements for the Admin UI providers setup page like earlier client side
checking of variables and preventing form submission when some required ones were missing.  
[9a227c9](https://github.com/sebadob/rauthy/commit/9a227c9297ec326191b3f83eaa45a9f88d8270fb)  
[c89fb7f](https://github.com/sebadob/rauthy/commit/c89fb7fbacb002a202a8022d8318362ee4d6db73)

## v0.23.3

### Changes

#### Documentation Updates

Updated sections in the documentation for:

- Choose database in Getting Started
- Started a new page for production setup
- Info on Android passkey status
- Encryption section
- Getting Started with Kubernetes

[9f85c77](https://github.com/sebadob/rauthy/commit/9f85c776666a492eb910c3730676e9fc9c23865d)

#### More strict origin checking

More strict checking and validation for `allowed_origins` has been implemented, when you configure clients. Before,
the regex only checked for the input to be a valid URI, which is not strict enough for validation an origin.  
This should improve the UX and prevents hard to debug bugs, when someone enters an invalid origin.

At the same time, a better visual separation has been added to the Origins / URI section in the UI when configuring
clients.

[55704f3](https://github.com/sebadob/rauthy/commit/55704f3cd3a5dcf04e5796a66ab4aba48b8c70dd)  
[d993d42](https://github.com/sebadob/rauthy/commit/d993d420fae628b069ac3857dfc1e69d812b16f7)  
[8d4e455](https://github.com/sebadob/rauthy/commit/8d4e455aa9455418aa8fc90612b707da1d72ce57)

#### Small performance optimizations

Small improvements have been made in a lot of places, which resulted in less memory allocations.  
[9144f2a](https://github.com/sebadob/rauthy/commit/9144f2afad3d1052bc4a927d0cbeaf24c6ea5589)

#### POST `/authorize` simplification

The logic on POST `/authorize` has been simplified internally. The code grew to an over-complicated state with new
features coming in all the time until a point where it was hard to follow. This has been simplified.
This makes the software better maintainable in the future.  
[af0db9d](https://github.com/sebadob/rauthy/commit/af0db9d0cbe560de327db60e33b61ccf32e33776)

### Bugfix

- add all `/fed_cm/` routes as exceptions to the new CSRF protection middleware
  [360ce46](https://github.com/sebadob/rauthy/commit/360ce46c19bad81ee60de817f3b3f74f0dd3c408)
- upstream auth provider templates could get stuck in the UI when switching between them
  [d2b928a](https://github.com/sebadob/rauthy/commit/d2b928ad9d34302f0ef2af7830d19cadaf784d5a)
- when a problem with an upstream provider occurs on `/callback`, you will now see the detailed error in the UI
  [8041c95](https://github.com/sebadob/rauthy/commit/8041c95b57292ac83330c7613698398857864e30)

## v0.23.2

This release brings some very minor features and bugfixes.

### Changes

#### New CSRF protection middleware

CSRF protection was there already without any issues.  
However, a new middleware has been added to the whole routing stack in addition to the existing checks. This provides
another defense in depth. The advantage of the new middleware is, that this can be enforced all the way in the future
after enough testing in parallel.  
If this works fine without any issues, we might get rid of the current way of doing it and only use the new middleware,
which is easier to maintain and to work with.

To not break any existing deployments and make sure I did not forget route exceptions for the new middleware, you can
set it to warn-only mode for this minor release. This option will be removed in future releases though and should only
be a temporary solution:

```
# If set to true, a violation inside the CSRF protection middleware based
# on Sec-* headers will block invalid requests. Usually you always want this
# enabled. You may only set it to false during the first testing phase if you
# experience any issues with an already existing Rauthy deployment.
# In future releases, it will not be possible the disable these blocks.
# default: true
#SEC_HEADER_BLOCK=true
```

[97fedf1](https://github.com/sebadob/rauthy/commit/97fedf189d54afe9c0d03dbf241ea59248806900)

#### Experimental FedCM support

This is not really considered a new feature, but Rauthy now has experimental support for FedCM in its current state.
This is opt-in and disabled by default. You should not attempt to use it in production because the FedCM implementation
itself still has a few bumps and sharp edges.  
The only reason the experimental support is there is to help smooth out these things and hopefully have FedCM as a
really nice addition. It does not really bring any new possibilities to the table, but it would improve the UX quite a
bit, if it hopefully turns out great.

```
#####################################
############## FED CM ###############
#####################################

## CAUTION: The FedCM is highly experimental at this point!
## Do not attempt to use it in production because it is subject to change
## in the future! The spec is currently a draft and under active development.

# Set to `true` to enable the experimental FedCM.
# default: false
#EXPERIMENTAL_FED_CM_ENABLE=false

# Session lifetime for FedCM in seconds - the session can not be extended
# beyond this time and a new login will be forced.
# default: 2592000
#SESSION_LIFETIME_FED_CM=2592000

# Session timeout for FedCM in seconds
# When a new token / login is requested before this timeout hits the limit,
# the user will be authenticated without prompting for the credentials again.
# This is the value which can extend the session, until it hits its maximum
# lifetime set with _FED_CM.
# default: 259200
#SESSION_TIMEOUT_FED_CM=259200
```

[4689e54](https://github.com/sebadob/rauthy/commit/4689e54b6193cf62877a75650d74e3bc29e5c8e7)

#### Relaxed validation on ephemeral `client_id`s

The input validation for ephemeral `client_id`s has been relaxed. This now makes it possible to test them with OIDC
playgrounds, which typically generate pretty long testing URLs, which were being rejected for their length beforehand.
Rauthy now accepts URLs of up to 256 characters as `client_id`s.  
[62405bb](https://github.com/sebadob/rauthy/commit/62405bbad7230f2e3af864b004081a90ab505f6f)

#### Bumped Argon2ID defaults

The default values for the Argon2ID hashing algorithm have been bumped up quite a bit. Rauthy's goal is to be as secure
as possible by default. The old values were quite a bit above the OWASP recommendation, but still way too low imho.
The values will of course still need tuning and adjustment to the target architecture / deployment, but they provide a
way better starting point and can be considered really secure even if not adjusted.

The new defaults are:

```
# M_COST should never be below 32768 in production
ARGON2_M_COST=131072
# T_COST should never be below 1 in production
ARGON2_T_COST=4
# P_COST should never be below 2 in production
ARGON2_P_COST=8
```

[5145889](https://github.com/sebadob/rauthy/commit/514588937db574b2f6beea8a41dc80062f1faad4)
[feee23e](https://github.com/sebadob/rauthy/commit/feee23ee1809bd50416370ccd975e9577a7192e6)

### Bugfixes

- Ephemeral client's now work properly with the `/userinfo` endpoint in strict-validation mode. Their validation is
  simply being skipped at that point, because it does not make much sense to do an `enabled` check at that point.
  [90b0367](https://github.com/sebadob/rauthy/commit/90b03677ffd3b99372ac9496540449b302af66d5)
- A small bug appeared in the UI after you have added new custom user attributes. Instead of resetting the input
  values to empty strings after the registration, they were set to undefined.
  [ab77595](https://github.com/sebadob/rauthy/commit/ab775958dec70eee3b2915fe2faa8b4a9816ec2e)
- Because of a bug in the account overview UI, it was not possible to link an already existing account to an upstream
  IdP after the registration.
  [22751ee](https://github.com/sebadob/rauthy/commit/22751ee6e9b31361d2ee5047a8c8795518d30745)

## v0.23.1

### Features

#### Global Cookie Encryption

All Rauthy cookies (except for the locale) are now encrypted globally inside the whole app by default.  
This is just another defense in depth. The AEAD algorithm makes sure, that you can't tamper with the cookie values,
even if you would try to do it manually.

[4fdb3f2](https://github.com/sebadob/rauthy/commit/4fdb3f262f128fe803d1542418c06eff08bf210c)

#### Easier extraction of CSRF tokens with external Frontend

If you are in the situation where you run Rauthy behind a reverse proxy on the exact same origin with another app,
and you want to build custom user facing UI parts, you had to retrieve the original HTML for `/authorize` or the
password reset to extract the CSRF token from the HTML content.  
Doing this in tests is fine, but very tedious and wasteful for a production deployment.

For this reason, there are now 2 new possibilities:

- POST `/oidc/session` endpoint to create a session in `Init` state, which will return the cookie and the
  correct CSRF token in a json body
- the password reset link returns a json with a CSRF token instead of an HTML document, if you request it
  with a `Accept: application/json` header

[c37e1f5](https://github.com/sebadob/rauthy/commit/c37e1f5bc27ebd679c4424ef568a8bae82c523bc)

### Bugfix

- the password expiry reminder E-Mail had a wrong a link to the account page, a left over from older versions
  with `.html` appended
  [d728317](https://github.com/sebadob/rauthy/commit/d728317ef31e20a117a0ca2a903e767e34c556d4)

## v0.23.0

This release does the first preparations to prepare a future v1.0.0 release.  
Quite a few values have been cleaned up or improved.

### Breaking

#### `rauthy-client` compatibility

If you are using the [rauthy-client](https://crates.io/crates/rauthy-client), you should upgrade to `v0.4.0` before
upgrade Rauthy to `v0.23.0`. Any older client version will not understand the new grant type for the OAuth2
Device Authorization grant.

#### Removal of `UNSAFE_NO_RESET_BINDING` in favor of `PASSWORD_RESET_COOKIE_BINDING`

The config variable `UNSAFE_NO_RESET_BINDING` has been removed in favor of `PASSWORD_RESET_COOKIE_BINDING`.
The logic for this security feature has been reversed. The default behavior until now was to block subsequent
requests to the password reset form if they provided an invalid binding cookie. This created issues for people
that were using evil E-Mail providers. These would scan their users E-Mails and use links inside them.
This link usage however made it impossible for "the real user" to use the link properly, because it has been
used already by its provider.  
In some cases, this hurts the UX more than it is a benefit to the security, so this feature is now an opt-in
hardening instead of opt-out evil provider error fixing.  
Additionally, to improve the UX even further, the additional E-Mail input form has been removed from the password
reset page as well. The security benefits of this were rather small compared to the UX degradation.
#365
[1af7b92](https://github.com/sebadob/rauthy/commit/1af7b92204a99de4883154055bb3081dc196d759)

#### Removal of `OFFLINE_TOKEN_LIFETIME` config var

`OFFLINE_TOKEN_LIFETIME` has been removed from the config. This variable has been deprecated since a lof
of versions now. The `offline_access` scope was not even allowed via the UI for a long time now, so these offline
tokens were never issued anyway.  
The "new" mechanism Rauthy uses with the switch in the Admin UI to issue / allow refresh tokens for a client
is much more clear, since the `offline_access` scope produces a lot of confusion for people new to OIDC.
From the name, it simply makes no sense that you need to activate `offline_access` to get a refresh token.
Having an option named "allow refresh tokens" is just so much better.
[71db7fe](https://github.com/sebadob/rauthy/commit/71db7fef18568a599f30cae6e494bba40cb33e7d)

#### Change in `GET /clients/{id}/secret`

If you used the endpoint for retrieving a client secret with an API key before, you need to change the method.
The endpoint works exactly the same, but the method has been changed from a `GET` to a `POST` to request and validate
the additional CSRF token from the Admin UI.  
[72f077f](https://github.com/sebadob/rauthy/commit/72f077f462e4b28624d101510cfb50b64700e425)

#### Removal of the `Refresh Token` switch in Admin UI

The `Refresh Token` switch for a client config in the Admin UI has been removed.  
The old behavior was misleading and unintuitive, I just got rid of that switch.

If you want to use the refresh flow with a client, the only thing you need to do is to allow the `refresh_token` flow.
You needed to do this before anyway, but in addition enable the switch further down below. So this is not really a
breaking change, but could lead to confusion, if this switch is just gone.  
[2ece6ed](https://github.com/sebadob/rauthy/commit/2ece6ed6da214b353cc7c9adfbe7904c0f2f6bce)

### Features

#### OAuth 2.0 Device Authorization Grant

This release brings support for the OAuth 2.0 Device Authorization Grant.  
On top of the default RFC spec, we have some additional features like optional rate limiting and being able to
do the flow with confidential clients as well. The [rauthy-client](https://crates.io/crates/rauthy-client) has the
basics implemented as well for fetching tokens via the `device_code` flow. An automatic refresh token handler is
on the TODO list though. A small
[example](https://github.com/sebadob/rauthy/blob/main/rauthy-client/examples/device-code/src/main.rs) exists as well.  
You will find new sections in the account and admin -> user view, where you can see all linked devices, can give
them a friendly name and revoke refresh tokens, if they exist.
[544bebe](https://github.com/sebadob/rauthy/commit/544bebe162797870401ae60ad98dfb8cb6ecae92)
[8d028bf](https://github.com/sebadob/rauthy/commit/8d028bf6273819395d946bc13f2215ec6289a8b6)
[e8077ce](https://github.com/sebadob/rauthy/commit/e8077ce2f6c6c21d0e83ba531efe2bf5ef1c6d84)
[62d41bc](https://github.com/sebadob/rauthy/commit/62d41bc2c1eb0e9b6b85f1c4528b333f8d6fb97e)
[51a50ac](https://github.com/sebadob/rauthy/commit/51a50ac20118a87dd2914006e858bf159bcf4675)
[9352b3c](https://github.com/sebadob/rauthy/commit/9352b3c73885f4467e209c1d797116216562c256)

#### Dynamic Server Side Search + Pagination

Until now, the Admin UI used client side searching and pagination. This is fine for most endpoints, but
the users can grow quite large depending on the instance while all other endpoints will return rather small
"GET all" data.  
To keep big Rauthy instances with many thousands of users fast and responsive, you can set a threshold for
the total users count at which Rauthy will dynamically switch from client side to server side pagination
and searching for the Admin UI's Users and Sessions page.

```
# Dynamic server side pagination threshold
# If the total users count exceeds this value, Rauthy will dynamically
# change search and pagination for users in the Admin UI from client
# side to server side to not have a degradation in performance.
# default: 1000
SSP_THRESHOLD=1000
```

For smaller instances, keeping it client side will make the UI a bit more responsive and snappy.
For higher user counts, you should switch to do this on the server though to keep the UI fast and not
send huge payloads each time.

[b4dead3](https://github.com/sebadob/rauthy/commit/b4dead36169cc284c97af5a982cc33fb8a0be02b)
[9f87af3](https://github.com/sebadob/rauthy/commit/9f87af3dfb49b48300b885bf406f852579470193)
[e6d39d1](https://github.com/sebadob/rauthy/commit/e6d39d1e1118e18aeb020fbbb477a944fcd1467a)

#### UX Improvement on Login

The login form now contains a "Home" icon which will appear, if a `client_uri` is registered for the current
client. A user may click this and be redirected to the client, if a login is not desired for whatever reason.
Additionally, if the user registration is configured to be open, a link to the user registration will be shown
at the bottom as well.
[b03349c](https://github.com/sebadob/rauthy/commit/b03349c9d3f998aaecd3e4177c7b62bda067bf8b)
[b03349c](https://github.com/sebadob/rauthy/commit/b03349c9d3f998aaecd3e4177c7b62bda067bf8b)

#### Unlink Account from Provider

A new button has been introduced to the account view of federated accounts.  
You can now "Unlink" an account from an upstream provider, if you have set it up with at least
a password or passkey before.

[8b1d9a8](https://github.com/sebadob/rauthy/commit/8b1d9a882b0d4b059f3ed884deaacfcdeb109856)

#### Link Existing Account to Provider

This is the counterpart to the unlink feature from above.
This makes it possible to link an already existing, unlinked user account to an upstream auth provider.
The only condition is a matching `email` claim after successful login. Apart from that, there are quite a few things
going on behind the scenes and you must trigger this provider link from an authorized, valid session from inside your
user account view. This is necessary to prevent account takeovers if an upstream provider has been hacked in some way.

[fdc683c](https://github.com/sebadob/rauthy/commit/fdc683cec0181e03bb86da1e42fff213715718f0)

#### Bootstrap default Admin in production

You can set environment variables either via `rauthy.cfg`, `.env` or as just an env var during
initial setup in production. This makes it possible to create an admin account with the very first
database setup with a custom E-Mail + Password, instead of the default `admin@localhost.de` with
a random password, which you need to pull from the logs. A single API Key may be bootstrapped as well.

```
#####################################
############# BOOSTRAP ##############
#####################################

# If set, the email of the default admin will be changed
# during the initialization of an empty production database.
BOOTSTRAP_ADMIN_EMAIL="alfred@batcave.io"

# If set, this plain text password will be used for the
# initial admin password instead of generating a random
# password.
#BOOTSTRAP_ADMIN_PASSWORD_PLAIN="123SuperSafe"

# If set, this will take the argon2id hashed password
# during the initialization of an empty production database.
# If both BOOTSTRAP_ADMIN_PASSWORD_PLAIN and
# BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID are set, the hashed version
# will always be prioritized.
BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID='$argon2id$v=19$m=32768,t=3,p=2$mK+3taI5mnA+Gx8OjjKn5Q$XsOmyvt9fr0V7Dghhv3D0aTe/FjF36BfNS5QlxOPep0'

# You can provide an API Key during the initial prod database
# bootstrap. This key must match the format and pass validation.
# You need to provide it as a base64 encoded JSON in the format:
#
# ```
# struct ApiKeyRequest {
#     /// Validation: `^[a-zA-Z0-9_-/]{2,24}$`
#     name: String,
#     /// Unix timestamp in seconds in the future (max year 2099)
#     exp: Option<i64>,
#     access: Vec<ApiKeyAccess>,
# }
#
# struct ApiKeyAccess {
#     group: AccessGroup,
#     access_rights: Vec<AccessRights>,
# }
#
# enum AccessGroup {
#     Blacklist,
#     Clients,
#     Events,
#     Generic,
#     Groups,
#     Roles,
#     Secrets,
#     Sessions,
#     Scopes,
#     UserAttributes,
#     Users,
# }
#
# #[serde(rename_all = "lowercase")]
# enum AccessRights {
#     Read,
#     Create,
#     Update,
#     Delete,
# }
# ```
#
# You can use the `api_key_example.json` from `/` as
# an example. Afterwards, just `base64 api_key_example.json | tr -d '\n'`
#BOOTSTRAP_API_KEY="ewogICJuYW1lIjogImJvb3RzdHJhcCIsCiAgImV4cCI6IDE3MzU1OTk2MDAsCiAgImFjY2VzcyI6IFsKICAgIHsKICAgICAgImdyb3VwIjogIkNsaWVudHMiLAogICAgICAiYWNjZXNzX3JpZ2h0cyI6IFsKICAgICAgICAicmVhZCIsCiAgICAgICAgImNyZWF0ZSIsCiAgICAgICAgInVwZGF0ZSIsCiAgICAgICAgImRlbGV0ZSIKICAgICAgXQogICAgfSwKICAgIHsKICAgICAgImdyb3VwIjogIlJvbGVzIiwKICAgICAgImFjY2Vzc19yaWdodHMiOiBbCiAgICAgICAgInJlYWQiLAogICAgICAgICJjcmVhdGUiLAogICAgICAgICJ1cGRhdGUiLAogICAgICAgICJkZWxldGUiCiAgICAgIF0KICAgIH0sCiAgICB7CiAgICAgICJncm91cCI6ICJHcm91cHMiLAogICAgICAiYWNjZXNzX3JpZ2h0cyI6IFsKICAgICAgICAicmVhZCIsCiAgICAgICAgImNyZWF0ZSIsCiAgICAgICAgInVwZGF0ZSIsCiAgICAgICAgImRlbGV0ZSIKICAgICAgXQogICAgfQogIF0KfQ=="

# The secret for the above defined bootstrap API Key.
# This must be at least 64 alphanumeric characters long.
# You will be able to use that key afterwards with setting
# the `Authorization` header:
#
# `Authorization: API-Key <your_key_name_from_above>$<this_secret>`
#BOOTSTRAP_API_KEY_SECRET=
```

[1a7d9e4](https://github.com/sebadob/rauthy/commit/1a7d9e40aad551a44648fe39e24c05d36a621fab)

#### New config var `USERINFO_STRICT`

You can now set a new config variable called `USERINFO_STRICT`. If set so true, Rauthy will do additional
validations on the `/userinfo` endpoint and actually revoke (even otherwise still valid) access tokens,
when any user / client / device it has been issued for has been deleted, expired or disabled. The non-strict
mode will simply make sure the token is valid and that the user still exists. The additional validations
will consume more resources because they need 1-2 additional database lookups but will provide more strict
validation and possible earlier token revocation. If you don't need it that strict, and you are resource
constrained, set it to `false`.  
[198e7f9](https://github.com/sebadob/rauthy/commit/198e7f957c32fef5f0f786b145408f7d625f20ce)

#### `at_hash` in `id_token`

The Rauthy `id_token` now contains the access token hash `at_hash` claim. This is needed for additional
downstream validation, if a client provides both tokens and they are not coming from Rauthy directly.
With the additional validation of the `at_hash` claim, clients can be 100% sure, that a given `id_token`
belongs to a specific `access_token` and has not been swapped out.  
[d506865](https://github.com/sebadob/rauthy/commit/d506865898e61fce45e5cf4c754ad4300bd37161)

#### Better roles, groups and scopes names

The allowed names for roles, groups and scopes have been adjusted. Rauthy allows names of up to 64 characters
now and containing `:` or `*`. This will make it possible to define custom scopes with names like
`urn:matrix:client:api:guest` or `urn:matrix:client:api:*`.

[a5982d9](https://github.com/sebadob/rauthy/commit/a5982d91f37a2f2917ed4215dc6ded216dc0fd69)
[50d0214](https://github.com/sebadob/rauthy/commit/50d021440eb50473977ec851a46c0bc979bbd12b)

#### Configurable Cookie Security

Depending on your final deployment, you may want to change the way Rauthy's set's its cookies, for instance if you
want to create your own UI endpoints but still want to be able to communicate with the API.

The default cookie setting has been changed in a way that all cookies will have the `__Host-` prefix now, which provides
the highest level of security. There might be cases where you don't want this and rather have the path restriction to
`/auth` from before, for instance when you host an additional app on the same origin behind a reverse proxy, that should
not be able to read Rauthy's cookies.

And finally, for all Safari users, since Safari does not consider `localhost` to be secure when testing, you can even
set insecure cookies for testing purposes.

```
# You can set different security levels for Rauthy's cookies.
# The safest option would be 'host', but may not be desirable when
# you host an application on the same origin behind a reverse proxy.
# In this case you might want to restrict to 'secure', which will then
# take the COOKIE_PATH from below into account.
# The last option is 'danger-insecure' which really should never be used
# unless you are just testing on localhost and you are using Safari.
#COOKIE_MODE=host

# If set to 'true', Rauthy will bind the cookie to the `/auth` path.
# You may want to change this only for very specific reasons and if
# you are in such a situation, where you need this, you will know it.
# Otherwise don't change this value.
# default: true
#COOKIE_SET_PATH=true
```

[e697389](https://github.com/sebadob/rauthy/commit/e697389d409dace9cdf866988e833efcd79d4529)

#### Auto-Blacklisting of suspicious requests

Rauthy can now auto-blacklist IP's that do suspicious requests, like for instance:

- /.ssh/
- /.kube/config
- /backup.zip
- /wp-admin/

... and so on.  
Rauthy has a "catch all" API route handler on `/` which looks for these by default.  
By default, IPs from such requests will be blacklisted for 24 hours, but you can of course configure this.

```
# The "catch all" route handler on `/` will compare the request path
# against a hardcoded list of common scan targets from bots and attackers.
# If the path matches any of these targets, the IP will be blacklisted
# preemptively for the set time in minutes.
# You can disable it with setting it to `0`.
# default: 1440
SUSPICIOUS_REQUESTS_BLACKLIST=1440

# This will emit a log with level of warning if a request to `/` has
# been made that has not been caught by any of the usual routes and
# and handlers. Apart from a request to just `/` which will end in
# a redirect to `/auth/v1`, all additional path's will be logged.
# This can help to improve the internal suspicious blocklist in the
# future.
# default: false
SUSPICIOUS_REQUESTS_LOG=flase
```

#### Changes for `/auth/v1/whoami`

The whoami endpoint has been changed. It does not return all headers anymore, because this could possibly leak sensitive
headers in some environments, especially with the new auth headers feature in some situations.  
Instead, it only returns the peer IP that Rauthy extracted for this request. This can be very helpful if you need to
configure the extraction, for instance when you are behind a reverse proxy or CDN.  
[758b31c](https://github.com/sebadob/rauthy/commit/758b31cb5dc2277a0cc3ec31f15b5de90ff00ea7)

#### New sorting options for users

Users in the Admin UI can now be sorted by their `created_at` or `last_login` timestamp.  
Users that never have logged in will always be at the end of the list, since this value might be `undefined`.  
[4c41d64](https://github.com/sebadob/rauthy/commit/4c41d64f570dbbe94dcb8b681ac31608ed492652)

### Bugfixes

- The button for requesting a password reset from inside a federated account view has been
  disabled when it should not be, and therefore did not send out requests.
  [39e585d](https://github.com/sebadob/rauthy/commit/39e585d1d53a2490b273ba5c33b864ec0d7835d5)
- A really hard to reproduce bug where the backend complained about a not-possible mapping
  from postgres `INT4` to Rust `i64` as been fixed. This came with the advantage of hacing
  a few more compile-time checked queries for the `users` table.
  [1740177](https://github.com/sebadob/rauthy/commit/174017736d62d2237a5f00b9d0508bca0a57c8b0)
- A fix for the `/users/register` endpoint in the OpenAPI documentation has been fixed, which
  was referencing the wrong request body
  [463e424](https://github.com/sebadob/rauthy/commit/463e42409d14d84b59a1d3606fa53ca2ecee2b86)
- The page title for a password reset now shows "New Account" if this is a fresh setup and only
  "Password Reset" when it actually is a reset
  [84bbdf7](https://github.com/sebadob/rauthy/commit/84bbdf7bc464e5869285225e446cb56e17f53583)
- The "User Registration" header on the page for an open user registration as only showing up,
  when the domain was restricted.
  [fc3417e](https://github.com/sebadob/rauthy/commit/fc3417e04451a552bc89c2437c11cc2b019867a0)
- Button labels were misplaced on chrome based browsers
  [901eb55](https://github.com/sebadob/rauthy/commit/901eb55c3e980c5340ace0b67941dba447da0671)
- `/authorize` for logins had a bit too strict validation for the user password, which had a chance that
  a new password a user just set, would be rejected because of some invalid special chars not being allowed
  [9bb0a72](https://github.com/sebadob/rauthy/commit/9bb0a72fe2e3cc87e000b6db36c84fcf2d255bf5)
- when resources in the Admin UI have been re-fetched, for instance because of a user deletion, the search input
  has not been emptied
  [033db25](https://github.com/sebadob/rauthy/commit/033db25db695d2565bc4adcdfe07a77125c2a9a5)
- the deprecated `x-xss-protection` header has been removed
  [5008438](https://github.com/sebadob/rauthy/commit/50084385df887f4782bbe9224e63bc60719600fd)

## 0.22.1

### Security

This version fixes a [potential DoS in rustls](https://rustsec.org/advisories/RUSTSEC-2024-0336.html) which has
been found yesterday.  
[f4d65a6](https://github.com/sebadob/rauthy/commit/f4d65a6b056183f914075d6047384e2a7a4f0329)

### Features

#### Dedicated `/forward_auth` + Trusted Authn/Authz Headers

In addition to the `/userinfo` endpoint specified in the OIDC spec, Rauthy implements an additional endpoint
specifically for ForwardAuth situations. You can find it at `/auth/v1/oidc/forward_auth` and it can be configured
to append optional Trusted Header with User Information for downstream applications, that do not support OIDC
on their own.

The HeaderNames can be configured to match your environment.
Please keep in mind, that you should only use these, if you legacy application does not support OIDC natively,
because Auth Headers come with a lot of pitfalls, when your environment is not configured properly.

```
# You can enable authn/authz headers which would be added to the response
# of the `/auth/v1/forward_auth` endpoint. With  `AUTH_HEADERS_ENABLE=true`,
# the headers below will be added to authenticated requests. These could
# be used on legacy downstream applications, that don't support OIDC on
# their own.
# However, be careful when using this, since this kind of authn/authz has
# a lot of pitfalls out of the scope of Rauthy.
AUTH_HEADERS_ENABLE=true

# Configure the header names being used for the different values.
# You can change them to your needs, if you cannot easily change your
# downstream apps.
# default: x-forwarded-user
AUTH_HEADER_USER=x-forwarded-user
# default: x-forwarded-user-roles
AUTH_HEADER_ROLES=x-forwarded-user-roles
# default: x-forwarded-user-groups
AUTH_HEADER_GROUPS=x-forwarded-user-groups
# default: x-forwarded-user-email
AUTH_HEADER_EMAIL=x-forwarded-user-email
# default: x-forwarded-user-email-verified
AUTH_HEADER_EMAIL_VERIFIED=x-forwarded-user-email-verified
# default: x-forwarded-user-family-name
AUTH_HEADER_FAMILY_NAME=x-forwarded-user-family-name
# default: x-forwarded-user-given-name
AUTH_HEADER_GIVEN_NAME=x-forwarded-user-given-name
# default: x-forwarded-user-mfa
AUTH_HEADER_MFA=x-forwarded-user-mfa
```

[7d5a44a](https://github.com/sebadob/rauthy/commit/7d5a44a2ff29ec87458c6bf6b979cc4750491391)

### Bugfixes

- allow CORS requests for the GET PoW and the user sign up endpoint's to make it possible to build a custom UI without
  having a server side. At the same time, the method for requesting a PoW **has been changed from `GET` to `POST`**.
  This change has been done because even though only in-memory, a request would create data in the backend, which should
  never be done by a `GET`.
  Technically, this is a breaking change, but since it has only been available from the Rauthy UI itself because of the
  CORS header setting, I decided to only bump the patch, not the minor version.
  [e4d935f](https://github.com/sebadob/rauthy/commit/e4d935f7b51459031a37fb2ec2eb9952bc278f2e)

## v0.22.0

### Breaking

There is one breaking change, which could not have been avoided.  
Because of a complete rewrite of the logic how custom client logos (uploaded via
`Admin UI -> Clients -> Client Config -> Branding`), you will loose custom logos uploaded in the past for a client.
The reason is pretty simple. Just take a look at `Auto Image Optimization` below.

Apart from this, quite a few small internal improvements have been made to make life easier for developers and new
contributors. These changes are not listed in the release notes.

### Changes

#### Upstream Auth Providers

Rauthy v0.22.0 brings (beta) support for upstream authentication providers.  
This is a huge thing. It will basically allow you to set up things like *Sign In with Github* into Rauthy. You could use
your Github account for signup and login, and manage custom groups, scopes, and so on for the users on Rauthy. This
simplifies the whole onboarding and login for normal users a lot.

You can add as many auth providers as you like. They are not statically configured, but actually configurable via the
Admin UI. A user account can only be bound to one auth provider though for security reasons. Additionally, when a user
already exists inside Rauthy's DB, was not linked to an upstream provider and then tries a login but produces an email
conflict, the login will be rejected. It must be handled this way, because Rauthy can not know for sure, if the upstream
email was actually been verified. If this is not the case, simply accepting this login could lead to account takeover,
which is why this will not allow the user to login in that case.  
The only absolutely mandatory information, that Rauthy needs from an upstream provider, is an `email` claim in either
the `id_token` or as response from the userinfo endpoint. If it cannot find any `name` / `given_name` / `family_name`,
it will simply insert `N/A` as values there. The user will get a warning on his next values update to provide that
information.

The supported features (so far) are:

- auto OpenID Connect metadata discovery
- accept invalid TLS certs for upstream clients, for instance inside self-hosted environments
- provide a root certificate for an upstream client for the same reason as above
- choose a template for the config (currently Google and Github exist)
- fully customized endpoint configuration if the provider does not support auto-lookup
- optional mfa claim mapping by providing a json parse regex:  
  If the upstream provider returns information about if the user has actually done at least a 2FA sign in, Rauthy can
  extract this information dynamically from the returned JSON. For instance, Rauthy itself will add an `amr` claim to
  the `id_token` and you can find a value with `mfa` inside it, if the user has done an MFA login.  
  Github returns this information as well (which has been added to the template).
- optional `rauthy_admin` claim mapping:
  If you want to allow full rauthy admin access for a user depending on some value returned by the upstream provider,
  you can do a mapping just like for the mfa claim above.
- upload a logo for upstream providers
  Rauthy does not (and never will do) an automatic logo download from a provider, because this logo will be shown on the
  login page and must be trusted. However, if Rauthy would download any arbitrary logo from a provider, this could
  lead to code injection into the login page. This is why you need to manually upload a logo after configuration.

**Note:**  
If you are testing this feature, please provide some feedback in [#166](https://github.com/sebadob/rauthy/issues/166)
in any case - if you have errors or not. It would be nice to know about providers that do work already and those, that
might need some adoptions. All OIDC providers should work already, because for these we can rely on standards and RFCs,
but all others might produce some edge cases and I simply cannot test all of them myself.  
If we have new providers we know of, that need special values, these values would be helpful as well, because Rauthy
could provide a template in the UI for these in the future, so please let me know.

#### Auto Image Optimization

The whole logic how images are handled has been rewritten.
Up until v0.21.1, custom client logos have been taken as a Javascript `data:` url because of easier handling.
This means however, that we needed to allow `data:` sources in the CSP for `img-src`, which can be a security issue and
should be avoided if possible.

This whole handling and logic has been rewritten. The CSP hardening has been finalized by removing the `data:` allowance
for `img-src`. You can still upload SVG / JPG / PNG images under the client branding (and for the new auth providers).
In the backend, Rauthy will actually parse the image data, convert the images to the optimized `webp` format, scale
the original down and save 2 different versions of it. The first version will be saved internally to fit into 128x128px
for possible later use, the second one even smaller. The smaller version will be the one actually being displayed on
the login page for Clients and Auth Providers.  
This optimization reduces the payload sent to clients during the login by a lot, if the image has not been manually
optimized beforehand. Client Logos will typically be in the range of ~5kB now while the Auth Providers ones will usually
be less than 1kB.

[6ccc541](https://github.com/sebadob/rauthy/commit/6ccc541b0e6d155024aa9ba4d832dcb01f135528)

#### Custom Header Name for extracting Client IPs

With the name config variable `PEER_IP_HEADER_NAME`, you can specify a custom header name which will be used for
extracting the clients IP address. For instance, if you are running Rauthy behind a Cloudflare proxy, you will usually
only see the IP of the proxy itself in the `X-FORWARDED-FOR` header. However, cloudflare adds a custom header called
`CF-Connecting-IP` to the request, which then shows the IP you are looking for.    
Since it is very important for rate limiting and blacklisting that Rauthy knows the clients IP, this can now be
customized.

```
# Can be set to extract the remote client peer IP from a custom header name
# instead of the default mechanisms. This is needed when you are running
# behind a proxy which does not set the `X-REAL-IP` or `X-FORWARDED-FOR` headers
# correctly, or for instance when you proxy your requests through a CDN like
# Cloudflare, which adds custom headers in this case.
# For instance, if your requests are proxied through cloudflare, your would
# set `CF-Connecting-IP`.
PEER_IP_HEADER_NAME="CF-Connecting-IP"
```

[a56c05e](https://github.com/sebadob/rauthy/commit/a56c05e560c4b89f88f34240cb79a29e1e8a746c)

#### Additional Client Data: `contacts` and `client_uri`

For each client, you can now specify contacts and a URI, where the application is hosted. These values might be shown
to users during login in the future. For Rauthy itself, the values will be set with each restart in the internal
anti lockout rule. You can specify the contact via a new config variable:

```
# This contact information will be added to the `rauthy`client
# within the anti lockout rule with each new restart.
RAUTHY_ADMIN_EMAIL="admin@localhost.de"
```

[6ccc541](https://github.com/sebadob/rauthy/commit/6ccc541b0e6d155024aa9ba4d832dcb01f135528)

#### Custom `redirect_uri` During User Registration

If you want to initiate a user registration from a downstream app, you might not want your users to be redirected
to their Rauthy Account page after they have initially set the password. To encounter this, you can redirect them
to the registration page and append a `?redirect_uri=https%3A%2F%2Frauthy.example.com` query param. This will be
saved in the backend state and the user will be redirected to this URL instead of their account after they have set
their password.

#### Password E-Mail Tempalte Overwrites

You can not overwrite the template i18n translations for the NewPassword and ResetPassword E-Mail templates.  
There is a whole nwe section in the config and it can be easily done with environment variables:

```
#####################################
############ TEMPLATES ##############
#####################################

# You can overwrite some default email templating values here.
# If you want to modify the basic templates themselves, this is
# currently only possible with a custom build from source.
# The content however can mostly be set here.
# If the below values are not set, the default will be taken.

# New Password E-Mail
#TPL_EN_PASSWORD_NEW_SUBJECT="New Password"
#TPL_EN_PASSWORD_NEW_HEADER="New password for"
#TPL_EN_PASSWORD_NEW_TEXT=""
#TPL_EN_PASSWORD_NEW_CLICK_LINK="Click the link below to get forwarded to the password form."
#TPL_EN_PASSWORD_NEW_VALIDITY="This link is only valid for a short period of time for security reasons."
#TPL_EN_PASSWORD_NEW_EXPIRES="Link expires:"
#TPL_EN_PASSWORD_NEW_BUTTON="Set Password"
#TPL_EN_PASSWORD_NEW_FOOTER=""

#TPL_DE_PASSWORD_NEW_SUBJECT="Passwort Reset angefordert"
#TPL_DE_PASSWORD_NEW_HEADER="Passwort Reset angefordert für"
#TPL_DE_PASSWORD_NEW_TEXT=""
#TPL_DE_PASSWORD_NEW_CLICK_LINK="Klicken Sie auf den unten stehenden Link für den Passwort Reset."
#TPL_DE_PASSWORD_NEW_VALIDITY="Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig."
#TPL_DE_PASSWORD_NEW_EXPIRES="Link gültig bis:"
#TPL_DE_PASSWORD_NEW_BUTTON="Passwort Setzen"
#TPL_DE_PASSWORD_NEW_FOOTER=""

# Password Reset E-Mail
#TPL_EN_RESET_SUBJECT="Neues Passwort"
#TPL_EN_RESET_HEADER="Neues Passwort für"
#TPL_EN_RESET_TEXT=""
#TPL_EN_RESET_CLICK_LINK="Klicken Sie auf den unten stehenden Link um ein neues Passwort zu setzen."
#TPL_EN_RESET_VALIDITY="This link is only valid for a short period of time for security reasons."
#TPL_EN_RESET_EXPIRES="Link expires:"
#TPL_EN_RESET_BUTTON="Reset Password"
#TPL_EN_RESET_FOOTER=""

#TPL_DE_RESET_SUBJECT="Passwort Reset angefordert"
#TPL_DE_RESET_HEADER="Passwort Reset angefordert für"
#TPL_DE_RESET_TEXT=""
#TPL_DE_RESET_CLICK_LINK="Klicken Sie auf den unten stehenden Link für den Passwort Reset."
#TPL_DE_RESET_VALIDITY="Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig."
#TPL_DE_RESET_EXPIRES="Link gültig bis:"
#TPL_DE_RESET_BUTTON="Passwort Zurücksetzen"
#TPL_DE_RESET_FOOTER=""
```

### Bugfix

- UI: when a client name has been removed and saved, the input could show `undefined` in some cases
  [2600005](https://github.com/sebadob/rauthy/commit/2600005be81649083103051a5bfc7b7ec49c9c3c)
- The default path to TLS certificates inside the container image has been fixed in the deploy cfg template.
  This makes it possible now to start the container for testing with TLS without explicitly specifying the path
  manually.
  [3a04dc0](https://github.com/sebadob/rauthy/commit/3a04dc02a878263cec2d841553747c78a41b7c4a)
- The early Passkey implementations of the Bitwarden browser extension seem to have not provided all correct values,
  which made Rauthy complain because of not RFC-compliant requests during Passkey sign in. This error cannot really be
  reproduced. However, Rauthy tries to show more error information to the user in such a case.
  [b7f94ff](https://github.com/sebadob/rauthy/commit/b7f94ff8ad3cd9aad61baf3710e4f9788c498ca6)
- Don't use the reset password template text for "new-password emails"
  [45b4160](https://github.com/sebadob/rauthy/commit/45b41604b1e0c0c9af423be65021953116b88150)

## v0.21.1

### Changes

- host `.well-known additionally` on root `/`
  [3c594f4](https://github.com/sebadob/rauthy/commit/3c594f4628c8d301ed1556d5dd3ea5a004afdcac)

### Bugfix

- Correctly show the `registration_endpoint` for dynamic client registration in the `openid-configuration`
  if it is enabled.
  [424fdd1](https://github.com/sebadob/rauthy/commit/424fdd10e57639c1d60dde61f551462e67ff4934)

## v0.21.0

### Breaking

The access token's `sub` claim had the email as value beforehand. This was actually a bug.  
The `sub` of access token and id token must be the exact same value. `sub` now correctly contains the user ID,
which is 100% stable, even if the user changes his email address.  
This means, if you used the `sub` from the access token before to get the users email, you need to pay attention now.
The `uid` from the access token has been dropped, because this value is now in `sub`. Additionally, since many
applications need the email anyway, it makes sense to have it inside the access token. For this purpose, if `email`
is in the requested `scope`, it will be mapped to the `email` claim in the access token.

### Features

#### OpenID Core Compatibility

Rauthy should now be compliant with the mandatory part of the OIDC spec.
A lot of additional things were already implemented many versions ago.
The missing thing was respecting some additional params during GET `/authorize`.

#### OpenID Connect Dynamic Client Registration

Rauthy now supports Dynamic Client registration as
defined [here](https://openid.net/specs/openid-connect-registration-1_0.html).

Dynamic clients will always get a random ID, starting with `dyn$`, followed by a random alphanumeric string,
so you can distinguish easily between them in the Admin UI.  
Whenever a dynamic client does a `PUT` on its own modification endpoint with the `registration_token` it
received from the registration, the `client_secret` and the `registration_token` will be rotated and the
response will contain new ones, even if no other value has been modified. This is the only "safe" way to
rotate secrets for dynamic clients in a fully automated manner. The secret expiration is not set on purpose,
because this could easily cause troubles, if not implemented properly on the client side.  
If you have a
badly implemented client that does not catch the secret rotation and only if you cannot fix this on the
client side, maybe because it's not under your control, you may deactivate the auto rotation with
`DYN_CLIENT_SECRET_AUTO_ROTATE=false`. Keep in mind, that this reduces the security level of all dynamic
clients.

Bot and spam protection is built-in as well in the best way I could think of. This is disabled, if you set
the registration endpoint to need a `DYN_CLIENT_REG_TOKEN`. Even though this option exists for completeness,
it does not really make sense to me though. If you need to communicate a token beforehand, you could just
register the client directly. Dynamic clients are a tiny bit less performant than static ones, because we
need one extra database round trip on successful token creation to make the spam protection work.  
However, if you do not set a `DYN_CLIENT_REG_TOKEN`, the registration endpoint would be just open to anyone.
To me, this is the only configuration for dynamic client registration, that makes sense, because only that
is truly dynamic. The problem then are of course bots and spammers, because they can easily fill your
database with junk clients. To counter this, Rauthy includes two mechanisms:

- hard rate limiting - After a dynamic client has been registered, another one can only be registered
  after 60 seconds (default, can be set with `DYN_CLIENT_RATE_LIMIT_SEC`) from the same public IP.
- auto-cleanup of unused clients - All clients, that have been registered but never used, will be deleted
  automatically 60 minutes after the registration (default, can be set with `DYN_CLIENT_CLEANUP_MINUTES`).

There is a whole new section in the config:

```
#####################################
########## DYNAMIC CLIENTS ##########
#####################################

# If set to `true`, dynamic client registration will be enabled.
# Only activate this, if you really need it and you know, what
# you are doing. The dynamic client registration without further
# restriction will allow anyone to register new clients, even
# bots and spammers, and this may create security issues, if not
# handled properly and your users just login blindly to any client
# they get redirected to.
# default: false
#ENABLE_DYN_CLIENT_REG=false

# If specified, this secret token will be expected during
# dynamic client registrations to be given as a
# `Bearer <DYN_CLIENT_REG_TOKEN>` token. Needs to be communicated
# in advance.
# default: <empty>
#DYN_CLIENT_REG_TOKEN=

# The default token lifetime in seconds for a dynamic client,
# that will be set during the registration.
# This value can be modified manually after registration via
# the Admin UI like for any other client.
# default: 1800
#DYN_CLIENT_DEFAULT_TOKEN_LIFETIME=1800

# If set to 'true', client secret and registration token will be
# automatically rotated each time a dynamic client updates itself
# via the PUT endpoint. This is the only way that secret rotation
# could be automated safely.
# However, this is not mandatory by RFC and it may lead to errors,
# if the dynamic clients are not implemented properly to check for
# and update their secrets after they have done a request.
# If you get into secret-problems with dynamic clients, you should
# update the client to check for new secrets, if this is under your
# control. If you cannot do anything about it, you might set this
# value to 'false' to disable secret rotation.
# default: true
#DYN_CLIENT_SECRET_AUTO_ROTATE=true

# This scheduler will be running in the background, if
# `ENABLE_DYN_CLIENT_REG=true`. It will auto-delete dynamic clients,
# that have been registered and not been used in the following
# `DYN_CLIENT_CLEANUP_THRES` hours.
# Since a dynamic client should be used right away, this should never
# be a problem with "real" clients, that are not bots or spammers.
#
# The interval is specified in minutes.
# default: 60
#DYN_CLIENT_CLEANUP_INTERVAL=60

# The threshold for newly registered dynamic clients cleanup, if
# not being used within this timeframe. This is a helper to keep
# the database clean, if you are not using any `DYN_CLIENT_REG_TOKEN`.
# The threshold should be specified in minutes. Any client, that has
# not been used within this time after the registration will be
# automatically deleted.
#
# Note: This scheduler will only run, if you have not set any
# `DYN_CLIENT_REG_TOKEN`.
#
# default: 60
#DYN_CLIENT_CLEANUP_MINUTES=60

# The rate-limiter timeout for dynamic client registration.
# This is the timeout in seconds which will prevent an IP from
# registering another dynamic client, if no `DYN_CLIENT_REG_TOKEN`
# is set. With a `DYN_CLIENT_REG_TOKEN`, the rate-limiter will not
# be applied.
# default: 60
#DYN_CLIENT_RATE_LIMIT_SEC=60
```

#### Better UX with respecting `login_hint`

This is a small UX improvement in some situations. If a downstream client needs a user to log in, and it knows
the users E-Mail address somehow, maybe because of an external initial registration, It may append the correct
value with appending the `login_hint` to the login redirect. If this is present, the login UI will pre-fill the
E-Mail input field with the given value, which make it one less step for the user to log in.

### Changes

- The `/userinfo` endpoint now correctly respects the `scope` claim from withing the given `Bearer` token
  and provides more information. Depending on the `scope`, it will show the additional user values that were
  introduced with v0.20
  [49dd553](https://github.com/sebadob/rauthy/commit/49dd553e1df072f6a0db3b1cfaa130f7146aaf25)
- respect `max_age` during GET `/authorize` and add `auth_time` to the ID token
  [9ca6970](https://github.com/sebadob/rauthy/commit/9ca697091eba2a6297c289162f426a92b28385f4)
- correctly work with `prompt=none` and `prompt=login` during GET `/authorize`
  [9964fa4](https://github.com/sebadob/rauthy/commit/9964fa4aed7fdb6428b518a13dc18cc2761606f3)
- Make it possible to use an insecure SMTP connection
  [ef46414](https://github.com/sebadob/rauthy/commit/ef464145d5ed7f8ffe97fc24667a74485f94c2f1)
- Implement OpenID Connect Dynamic Client Registration
  [b48552e](https://github.com/sebadob/rauthy/commit/b48552e79f2a3aca0c5cefcc25ef7d9f7c21c6d4)
  [12179c9](https://github.com/sebadob/rauthy/commit/12179c9898126e5e78a80a3b49df6ca5a501ff81)
- respect `login_hint` during GET `/authorize`
  [963644c](https://github.com/sebadob/rauthy/commit/963644c36466c5eb9d0ad4d2411198ea71753d59)

### Bugfixes

- Fix the link to the latest version release notes in the UI, if an update is available
  [e66e496](https://github.com/sebadob/rauthy/commit/e66e4962631620ad762a80181e085d3e477ad8d4)
- Fix the access token `sub` claim (see breaking changes above)
  [29dbe26](https://github.com/sebadob/rauthy/commit/29dbe26a5c8f76d9a61931078811192ac2fb782d)
- Fix a short route fallback flash during Admin UI logout
  [6787261](https://github.com/sebadob/rauthy/commit/67872618949bc3b13e95d144d9c434f97174b395)

## v0.20.1

This is a small bugfix release.  
The temp migrations which exist for v0.20 only to migrate existing database secrets to
[cryptr](https://github.com/sebadob/cryptr) were causing a crash at startup for a fresh installation. This is the only
thing that has been fixed
with this version. They are now simply ignored and a warning is logged into the console at the very first startup.

## v0.20.0

### Breaking

This update is not backwards-compatible with any previous version. It will modify the database under the hood
which makes it incompatible with any previous version. If you need to downgrade for whatever reason, you will
only be able to do this by applying a database backup from an older version.
Testing has been done and everything was fine in tests. However, if you are using Rauthy in production, I recommend
taking a database backup, since any version <= v0.19 will not be working with a v0.20+ database.

### IMPORTANT Upgrade Notes

If you are upgrading from any earlier version, there is a manual action you need to perform, before you can
start v0.20.0. If this has not been done, it will simply panic early and not start up. Nothing will get damaged.

The internal encryption of certain values has been changed. Rauthy now uses [cryptr](https://github.com/sebadob/cryptr)
to handle these things,
like mentioned below as well.

However, to make working with encryption keys easier and provide higher entropy, the format has changed.
You need to convert your currently used `ENC_KEYS` to the new format:

#### Option 1: Use `cryptr` CLI

**1. Install cryptr - https://github.com/sebadob/cryptr**

If you have Rust available on your system, just execute:

  ```
  cargo install cryptr --features cli --locked
  ```

Otherwise, pre-built binaries do exist:

Linux: https://github.com/sebadob/cryptr/raw/main/out/cryptr_0.2.2

Windows: https://github.com/sebadob/cryptr/raw/main/out/cryptr_0.2.2.exe

**2. Execute:**

  ```
  cryptr keys convert legacy-string
  ```

**3. Paste your current ENC_KEYS into the command line.**

For instance, if you have

  ```
  ENC_KEYS="bVCyTsGaggVy5yqQ/S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4 q6u26onRvXVG4427/3CEC8RJWBcMkrBMkRXgx65AmJsNTghSA"
  ```

in your config, paste

  ```
  bVCyTsGaggVy5yqQ/S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4 q6u26onRvXVG4427/3CEC8RJWBcMkrBMkRXgx65AmJsNTghSA
  ```

If you provide your ENC_KEYS via a Kubernetes secret, you need to do a base64 decode first.
For instance, if your secret looks something like this

  ```
  ENC_KEYS: YlZDeVRzR2FnZ1Z5NXlxUS9TOW43b0NlbjUzeFNKTHpjc21mZG5CRHZOcnFRNjNyNCBxNnUyNm9uUnZYVkc0NDI3LzNDRUM4UkpXQmNNa3JCTWtSWGd4NjVBbUpzTlRnaFNB
  ```

Then decode via shell or any tool your like:

  ```
  echo -n YlZDeVRzR2FnZ1Z5NXlxUS9TOW43b0NlbjUzeFNKTHpjc21mZG5CRHZOcnFRNjNyNCBxNnUyNm9uUnZYVkc0NDI3LzNDRUM4UkpXQmNNa3JCTWtSWGd4NjVBbUpzTlRnaFNB | base64 -d
  ```

... and paste the decoded value into cryptr

**4. cryptr will output the correct format for either usage in config or as kubernetes secret again**

**5. Paste the new format into your Rauthy config / secret and restart.**

#### Option 2: Manual

Rauthy expects the `ENC_KEYS` now base64 encoded, and instead of separated by whitespace it expects them to
be separated by `\n` instead.  
If you don't want to use `cryptr` you need to convert your current keys manually.

For instance, if you have

```
ENC_KEYS="bVCyTsGaggVy5yqQ/S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4 q6u26onRvXVG4427/3CEC8RJWBcMkrBMkRXgx65AmJsNTghSA"
```

in your config, you need to convert the enc key itself, the value after the `/`, to base64, and then separate
them with `\n`.

For instance, to convert `bVCyTsGaggVy5yqQ/S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4`, split off the enc key part
`S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4` and encode it with base64:

```
echo -n 'S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4' | base64
```

Then combine the result with the key id again to:

```
bVCyTsGaggVy5yqQ/UzluN29DZW41M3hTSkx6Y3NtZmRuQkR2TnJxUTYzcjQ=
```

Do this for every key you have. The `ENC_KEYS` should then look like this in the end:

```
ENC_KEYS="
bVCyTsGaggVy5yqQ/UzluN29DZW41M3hTSkx6Y3NtZmRuQkR2TnJxUTYzcjQ=
q6u26onRvXVG4427/M0NFQzhSSldCY01rckJNa1JYZ3g2NUFtSnNOVGdoU0E=
"
```

**Important:**  
Make sure to not add any newline characters or spaces when copying values around when doing the bas64 encoding!

### Encrypted SQLite backups to S3 storage

Rauthy can now push encrypted SQLite backups to a configured S3 bucket.
The local backups to `data/backups/` do still exist. If configured, Rauthy will now push backups from SQLite
to an S3 storage and encrypt them on the fly. All this happens with the help
of [cryptr](https://github.com/sebadob/cryptr)
which is a new crate of mine. Resource usage is minimal, even if the SQLite file would be multiple GB's big.
The whole operation is done with streaming.

### Auto-Restore SQLite backups

Rauthy can now automatically restore SQLite backups either from a backup inside `data/backups/` locally, or
fetch an encrypted backup from an S3 bucket. You only need to set the new `RESTORE_BACKUP` environment variable
at startup and Rauthy will do the rest. No manually copying files around.  
For instance, a local backup can be restored with setting `RESTORE_BACKUP=file:rauthy-backup-1703243039` and an
S3 backup with `RESTORE_BACKUP=s3:rauthy-0.20.0-1703243039.cryptr`.

### Test S3 config at startup

To not show unexpected behavior at runtime, Rauthy will initialize and test a configured S3 connection
at startup. If anything is not configured correctly, it will panic early. This way, when Rauthy starts
and the tests are successful, you know it will be working during the backup process at night as well, and
it will not crash and throw errors all night long, if you just had a typo somewhere.

### Migration to `spow`

The old (very naive) Proof-of-Work (PoW) mechanism for bot and spam protection has been migrated to make use
of the [spow](https://github.com/sebadob/spow) crate, which is another new project of mine.
With this implementation, the difficulty for PoW's a client must solve can be scaled up almost infinitely,
while the time is takes to verify a PoW on the server side will always be `O(1)`, no matter hoch high the
difficulty was. `spow` uses a modified version of the popular Hashcat PoW algorithm, which is also being used
in the Bitcoin blockchain.

### Separate users cache

A typical Rauthy deployment will have a finite amount of clients, roles, groups, scopes, and so on.
The only thing that might scale endlessly are the users. Because of this, the users are now being cached
inside their own separate cache, which can be configured and customized to fit the deployment's needs.
You can now set the upper limit and the lifespan for cached user's. This is one of the first upcoming
optimizations, since Rauthy gets closer to the first v1.0.0 release:

### E-Mails as lowercase only

Up until now, it was possible to register the same E-Mail address multiple times with using uppercase characters.
E-Mail is case-insensitive by definition though. This version does a migration of all currently existing E-Mail
addresses
in the database to lowercase only characters. From that point on, it will always convert any address to lowercase only
characters to avoid confusion and conflicts.  
This means, if you currently have the same address in your database with different casing, you need to resolve this
issue manually. The migration function will throw an error in the console at startup, if it finds such a conflict.

```
# The max cache size for users. If you can afford it memory-wise, make it possible to fit
# all active users inside the cache.
# The cache size you provide here should roughly match the amount of users you want to be able
# to cache actively. Depending on your setup (WebIDs, custom attributes, ...), this number
# will be multiplied internally  by 3 or 4 to create multiple cache entries for each user.
# default: 100
CACHE_USERS_SIZE=100

# The lifespan of the users cache in seconds. Cache eviction on updates will be handled automatically.
# default: 28800
CACHE_USERS_LIFESPAN=28800
```

### Additional claims available in ID tokens

The scope `profile` now additionally adds the following claims to the ID token (if they exist for the user):

- `locale`
- `birthdate`

The new scope `address` adds:

- `address` in JSON format

  The new scope `phone` adds:
- `phone`

### Changes

- new POST `/events` API endpoint which serves archived events
  [d5d4b01](https://github.com/sebadob/rauthy/commit/d5d4b0145768982f20ebc1dbbfc568a73d7937bd)
- new admin UI section to fetch and filter archived events.
  [ece73bb](https://github.com/sebadob/rauthy/commit/ece73bb38878d8d189d52855845c63fa729cae2a)
- backend + frontend dependencies have been updated to the latest versions everywhere
- The internal encryption handling has been changed to a new project of mine
  called [cryptr](https://github.com/sebadob/cryptr).  
  This makes the whole value encryption way easier, more stable and future-proof, because values have their own
  tiny header data with the minimal amount of information needed. It not only simplifies encryption key rotations,
  but also even encryption algorithm encryptions really easy in the future.
  [d6c224e](https://github.com/sebadob/rauthy/commit/d6c224e98198c155d7df83c25edc5c97ab590d2a)
  [c3df3ce](https://github.com/sebadob/rauthy/commit/c3df3cedbdff4a2a9dd592aac65ae21e5cd67385)
- Push encrypted SQLite backups to S3 storage
  [fa0e496](https://github.com/sebadob/rauthy/commit/fa0e496956769f995e22bc6e388a8cd2a88a34d3)
- S3 connection and config test at startup
  [701c785](https://github.com/sebadob/rauthy/commit/701c7851dd872c337e89227d56a3e46f2a05ac3e)
- Auto-Restore SQLite backups either from file or S3
  [65bbfea](https://github.com/sebadob/rauthy/commit/65bbfea5a1a3b23735b82f3eb05a415ce7c51013)
- Migrate to [spow](https://github.com/sebadob/spow)
  [ff579f6](https://github.com/sebadob/rauthy/commit/ff579f60414cb529d727ae27fd83e9506ad770d5)
- Pre-Compute CSP's for all HTML content at build-time and get rid of the per-request nonce computation
  [8fd2c99](https://github.com/sebadob/rauthy/commit/8fd2c99d25aea2f307e0197f6f91a585b4408dce)
- `noindex, nofollow` globally via headers and meta tag -> Rauthy as an Auth provider should never be indexed
  [38a2a52](https://github.com/sebadob/rauthy/commit/38a2a52fe6530cf4efdedfe96d2b3041959fcd3d)
- push users into their own, separate, configurable cache
  [3137927](https://github.com/sebadob/rauthy/commit/31379278440ec6ddaf1a2288ba3950ab60994963)
- Convert to lowercase E-Mail addresses, always, everywhere
  [a137e96](https://github.com/sebadob/rauthy/commit/a137e963d2c409749b65240ebd9f5b0587c96938)
  [2467227](https://github.com/sebadob/rauthy/commit/24672277e58694d9b23ce12da932ba515eb8674e)
- add additional user values matching OIDC default claims
  [fca0c13](https://github.com/sebadob/rauthy/commit/fca0c1306624bdffa112ad8239e381064cb0b843)
- add `address` and `phone` default OIDC scopes and additional values for `profile`
  [3d497a2](https://github.com/sebadob/rauthy/commit/3d497a2fb7d91952d82d6d0efaea891a8088f523)

### Bugfixes

- A visual bugfix appeared on Apple systems because of the slightly bigger font size. This made
  the live events look a bit ugly and characters jumping in a line where they should never end up.
  [3b56b50](https://github.com/sebadob/rauthy/commit/3b56b50f4f24b7707c522934f9c03714703c64ad)
- An incorrect URL has been returned for the `end_session_endpoint` in the OIDC metadata
  [3caabc9](https://github.com/sebadob/rauthy/commit/3caabc98b24893ecadcd2f9219783a202c12f730)
- Make the `ItemTiles` UI componend used for roles, groups, and so on, wrap nicely on smaller screens
  [6f83e4a](https://github.com/sebadob/rauthy/commit/6f83e4a917ffd4eaec9b543b66170dc5ea76ed6e)
- Show the corresponding E-Mail address for `UserPasswordReset` and `UserEmailChange` events in the UI
  [7dc4794](https://github.com/sebadob/rauthy/commit/7dc47945ec3fdd335ed486465f98cdbb4734d653)

## v0.19.2

### Changes

- Invalidate all user sessions after a password reset to have a more uniform flow and better UX
  [570dea6](https://github.com/sebadob/rauthy/commit/570dea6379d1bba061a8b7acee64d9e36cf52733)
- Add an additional foreign key constraint on the `user_attr_values` table to cascade rows
  on user deletion for enhanced stability during user deletions
  [1dc730c](https://github.com/sebadob/rauthy/commit/1dc730c78c7f4e0a9c6b0b1b3dab8a35a4893b47)

### Bugfixes

- Fix a bug when an existing user with already registered passkeys would not be able
  to use the password reset functionality correctly when opened in a fully fresh browser
  [24af03c](https://github.com/sebadob/rauthy/commit/24af03c0365faad2108516bba3174d208e52d616)
- Fix cache evictions of existing user sessions after a user has been deleted while having
  an active session
  [ed76418](https://github.com/sebadob/rauthy/commit/ed764180fff5ed1f4aa6f497b0fc2362412db7d7)
- Fix some default values in the reference config docs not having the correct default documented
  [a7101b2](https://github.com/sebadob/rauthy/commit/a7101b25d72ced35e7bc22c7a084816ce7fa7635)

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
  This makes it difficult however if you need to debug something, for instance when you use a SQLite deployment. For
  this reason, you can append `-debug` to a tag
  and you will get an Alpine based version just like before.
  [1a7e79d](https://github.com/sebadob/rauthy/commit/1a7e79dc96d27d8d180d1e4394644c8851cbdf70)
- More stable HA deployment: In some specific K8s HA deployments, the default HTTP2 keep-alive's from
  [redhac](https://github.com/sebadob/redhac) were not good enough and we got broken pipes in some environments which
  caused the leader to change often. This has been fixed
  in [redhac-0.6.0](https://github.com/sebadob/redhac/releases/tag/v0.6.0)
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
