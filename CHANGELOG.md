# Changelog

## UNRELEASED

### Changes

#### Less login location check noise

The logic behind the "login from new location" emails was reworked. Up until now, the location was
compared via `user_id` + `ip`. However, if you IP changes quite often, you will receive many of
these emails with no real benefit.

To counter this, you will now get a new Cookie during `GET /authorize`. This cookie will live for 5
years and contain a random String with a "Browser ID". If Rauthy can find such a Browser ID during
login, the index for the location check will be `user_id` + `browser_id`, which will not generate
new warning emails, if your IP changes (as long as you don't delete cookies all the time). The IP
will only be used as a fallback in situations, where this Browser ID cannot be found in a cookie.

> This change needed a rework of the database table. To make this possible, the old table with login
> locations will be dropped and a new one with a proper compound key will be created. This means you
> will get new login location emails with this version after the very first login, even when your IP
> has not changed.

[#1335](https://github.com/sebadob/rauthy/pull/1335)

#### Token Revocation

Only because of compatibility issues with Matrix "next-gen auth", Rauthy now provided a token
revocation endpoint, which is also published via `/.well-known/openid-configuration`.

```toml
[access]
# Revoke JWT access + refresh tokens if a user does a dedicated logout
# from the account dashboard via the logout button, or when a
# (backchannel) logout is being triggered from a client.
# In most situations, you want your tokens to be able to live longer
# than a session on Rauthy, especially refresh tokens, or access tokens
# issues to headless devices via the `device_code` flow.
#
# !!! CAUTION !!!
# JWT token revocation is a myth and doesn't exist. You can only
# blacklist, but not revoke them, even though that's the official term
# used in the RFC. Token revocation does only work, when a client
# validates tokens against the `/introspection` or `/userinfo`
# endpoints, but they will still be valid, if a client just validates
# via public keys!
#
# The default value of `false` makes the most sense, because token
# revocation is NOT reliable on its own! It only works with the
# "correct" implementation on the client side, which also involves
# a lot more resource and traffic usage!
#
# If a session is being force-deleted or a user is force-logged-out
# via the AdminUI, tokens will always be revoked, which makes the
# most sense.
#
# default: false
# overwritten by: TOKEN_REVOKE_ON_LOGOUT
token_revoke_on_logout = false

# If set to true, tokens issued via the `device_code` flow will be
# revoked during `/logout`.
# These tokens are usually used on headless IoT devices or for CLIs
# on remote servers, and logging these back in can be a lot more
# work than the default logins via browser.
#
# This value will only be respected, if `token_revoke_on_logout` is
# set to `true` as well. Otherwise, tokens will only be revoked
# via the dedicated `recovation_endpoint` or forced logout from an
# admin.
#
# default: false
# overwritten by: TOKEN_REVOKE_DEVICE_TOKENS
token_revoke_device_tokens = false
```

> **Sidenote:**
>
> JWT Token Revocation is a myth! It does not exist. You cannot revoke JWT tokens by design. Once
> they are created and signed, there is no way to revoke them. The only thing you can do is to
> blacklist them! Rauthy now keeps track of each single `access_token` via its `jti`, and checks
> the database during `/introspect` and `/userinfo`.
>
> **BUT**:  
> Token Revocation does only work, when the tokens are checked against Rauthys API! If a client
> validates them via public keys, it is IMPOSSIBLE to "revoke" them. They will be valid until the
> `exp` claim is in the past, which makes the whole term "token revocation" complete nonsense (for
> JWT tokens, which are the default in almost all cases these days).
>
> **DO NOT RELY ON TOKEN REVOCATION!**  
> This feature only exists for compatibility!

[#1328](https://github.com/sebadob/rauthy/pull/1328)

#### Custom Root CA for SMTP

You can now provide a custom Root CA certificate for SMTP connections.

```toml
[email]
# You can provide a custom Root Certificate in PEM format here,
# which then will be used for SMTP connections.
#
# default: not set
# overwritten by: SMTP_ROOT_CA
root_ca = """
-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----
"""
```

[#1336](https://github.com/sebadob/rauthy/pull/1336)

#### "Password Expired" during login

When you missed the E-Mail notification about a soon expiring password, and you did not renew it in
advance, you will now get a warning inside the login form, which notifies you about an expired
password.

Beforehand, you would only see "invalid credentials", even though they were correct, only expired.
If now the password has expired and the given one matched the expired password, a new reset email
will be sent out and you will see the proper message in the login form.

[#1337](https://github.com/sebadob/rauthy/pull/1337)

#### `rauthy` Client Name customizable

Usually, if you update anything for the `rauthy` client via the Admin UI, all values will be
reverted with the next restart as part of the anti-lockout rule. However, the Client Name is only
a UX improvement. If you update it now, it will excluded from the anti-lockout rule and kept between
restarts.

[#1309](https://github.com/sebadob/rauthy/pull/1309)

#### Additional characters for roles / scopes

Roles and scopes are allowed to have `.` and UPPERCASE characters from now on. This makes it
possible to use namespaced scopes, and the uppercase characters provide compatibility for Element
Web / Matrix "next-gen auth".

[#1313](https://github.com/sebadob/rauthy/pull/1313)
[#1315](https://github.com/sebadob/rauthy/pull/1315)

#### `client_id` as `sub`

When you retrieve an `access_token` via `client_credentials` flow, the `sub` claim would be `null`,
because it should only contain End-User IDs by RFC. For some weird reason, there are applications
out there expecing the `sub` claim set to the `client_id` in such cases, even though this
information could be taken from either `azp` or `aud` claims.

You now have the possibility to set the `sub` to the `client_id` in such cases. Because it's against
the RFC and there is a (very tiny) change of misuse, it is opt-in:

```toml
[access]
# The `sub` claim should only contain End-User IDs. Therefore, it will
# be `null` for the `client_credentials` flow. However, some applications
# need the `sub` claim to contain the `client_id` in such a case for
# whatever reason (it exists in `azp` / `aud` already anyway). You can
# enable this setting here.
#
# CAUTION:
# Theoretically, it is possible to create a client with an Id that
# matches a user ID (if one with no uppercase chars was generated).
# If this is the case, it is NOT possible to distinguish between
# a user and a client token in such a scenario!
# The chance that an ID for a user without an uppercase char exists
# is almost 0, but it exists.
#
# default: false
# overwritten by: CLIENT_CREDENTIALS_MAP_SUB
client_credentials_map_sub = false
```

[#1334](https://github.com/sebadob/rauthy/pull/1334)

#### I18n - Ukrainian Translations

Ukrainian Translations are now available for Email, Common UI and Admin UI.

[#1307](https://github.com/sebadob/rauthy/pull/1307)

### Bugfix

- When trying to use Dynamic Clients via an external UI (like e.g. Matrix Element), some CORS
  preflight checks were failing because of missing headers.
  [#1312](https://github.com/sebadob/rauthy/pull/1312)
  [#1314](https://github.com/sebadob/rauthy/pull/1314)
- The PKCE requirement for Dynamic Clients was inverted and not set as expected for public clients.
  [#1316](https://github.com/sebadob/rauthy/pull/1316)
- `jemallocator` is incompatible with `openbsd`.
  [#1332](https://github.com/sebadob/rauthy/pull/1332)
- The `@` character was missing in the validation regex for URLs.
  [#1330](https://github.com/sebadob/rauthy/pull/1330)
  [#1340](https://github.com/sebadob/rauthy/pull/1340)

## v0.33.4

### Changes

#### `timezone` for SCIM

The `ScimUser` now also contains the `timezone` (with `profile` scope), which was added in v0.33.0.

[#1292](https://github.com/sebadob/rauthy/pull/1292)

#### `picture` + `zoneinfo` via `/userinfo`

While fixing the `preferred_username` bug on the `/userinfo` endpoint, the `picture` and `zoneinfo`
were added to the response. There was also a slight rework of the logic to get rid of a few
unnecessary memory allocations.

[#1291](https://github.com/sebadob/rauthy/pull/1291)

### Bugfix

- The `preferred_username` was not set correctly on the `/userinfo` endpoint. It still had the old
  behavior (pre v0.33) and it always contained the `email`.
  [#1291](https://github.com/sebadob/rauthy/pull/1291)
- With the added support for 16k / 64k kernels on `arm64`, a conflict for the `MALLOC_CONF` was
  discovered. To resolve it, the value is not set during build time anymore, but was added as an
  `ENV` var with a default value to the final release container. It can be overwritten as it was
  the case before.
  [#1293](https://github.com/sebadob/rauthy/pull/1293)

## v0.33.3

### Changes

#### Default kernel page size on arm64

Support for kernel page sizes up to 64K on `arm64` was added. This makes it possible to run on e.g.
Raspberry Pi 5's default kernel, and other 64k kernels. This might increase the memory usage on 4k
`arm64` devices a tiny bit, but should improve efficiency and performance on 64k kernels.

This is for `arm64` only, since `x86_64` has fixed 4k page sizes.

[#1288](https://github.com/sebadob/rauthy/pull/1288)

## v0.33.2

### Changes

#### Updated I18n

Translations for `zh` / `zhhans` have been updated, and `zh` is available for the Admin UI now as
well.

[#1278](https://github.com/sebadob/rauthy/pull/1278)
[#1279](https://github.com/sebadob/rauthy/pull/1279)

#### Hiqlite trigger backup via API

While fixing the "no local backup with no S3 config" bug mentioned below
in [Hiqlite](https://github.com/sebadob/hiqlite), a small feature was added. It is now possible to
trigger the creation of a backup manually via direct API call, in addition to using the Rauthy Admin
UI or relying on the cron job. It is a simple `POST` request to the Hiqlite API (port 8200 by
default) using the API secret, like so:

```bash
curl -i -XPOST -H 'X-API-SECRET: SuperSecureSecret1337' localhost:8200/backup
```

### Bugfixes

- [Hiqlite](https://github.com/sebadob/hiqlite) had a bug where the backup cron job was not started
  if no S3 config was given. It was bumped to the latest version which includes a fix.
- The UI had some smaller CSS issues here and there after the recent updates.
  [#1283](https://github.com/sebadob/rauthy/pull/1283)

## v0.33.1

### Bugfix

- The new feature "Rauthy Logo as client fallback" from `v0.33.0` was working fine for the branding
  page, but not during `/authorize`.
  [#1276](https://github.com/sebadob/rauthy/pull/1276)

## v0.33.0

### Breaking

#### JWKS rotate / cleanup scheduler

The JWKS rotate scheduler was not started on some of the last versions when it should have. For
this reason, depending on which version you were running and for how long, the cleanup scheduler
might clean up "too much". It is advisable to trigger a manual JWKS rotation before doing the
upgrade:

Admin UI -> Config -> JWKS -> Rotate Keys

At least if you see errors after the upgrade with something like "kid not found" when trying to
fetch public keys for validation, you need to rotate manually once.

#### `preferred_username` in Tokens

The `preferred_username` was always added to both `access_token` and `id_token` and it always
contained the same value as the `email` claim. This it NOT the case anymore! This value is
configurable now. To match the OIDC spec, it will never be added to the `access_token` anymore, and
only exist in the `id_token` if the client requested the `profile` scope during login. The value of
this claim depends on your configuration. For more details, check the "`preferred_username` and
`tz`" changes below.

Because of these changes, the `email` will not show up as the `username` in the response from the
OAuth2 `/introspect` endpoint as well.

> Note: If you are using the `rauthy-client`, make sure to upgrade it to `0.11` beforehand.

#### User Request and Response API data

The user values are much more configurable now (see in changes below). At the same time, the
`given_name` is now always optional in responses from the API. The necessary values during user
registration, if you have an open endpoint and use direct API requests from somewhere else, have
changed as well. They now also depend on your configuration.

If you don't change anything in the new `[user_values]` section, you will not experience any
breaking changes for direct API requests.

### Changes

#### `AuthorizedKeys` for PAM users

If a user is linked to an existing PAM user, and the config allows it, users can upload their own
public keys. A server can then make use of the `AuthorizedKeysCommand` via the `sshd_config`
and resolve these public keys dynamically:

```
AuthorizedKeysCommand
   Specifies a program to be used to look up the user's
   public keys.  The program must be owned by root, not
   writable by group or others and specified by an absolute
   path.  Arguments to AuthorizedKeysCommand accept the
   tokens described in the “TOKENS” section.  If no arguments
   are specified then the username of the target user is
   used.

   The program should produce on standard output zero or more
   lines of authorized_keys output (see “AUTHORIZED_KEYS” in
   sshd(8)).  AuthorizedKeysCommand is tried after the usual
   AuthorizedKeysFile files and will not be executed if a
   matching key is found there.  By default, no
   AuthorizedKeysCommand is run.
```

[rauthy-pam-nss](https://github.com/sebadob/rauthy-pam-nss) was updated and can work with this new
feature. You need to update to `v0.2.0` for compatibility.

You will have the following new config options:

```toml
[pam.authorized_keys]

# If set to `true`, a user with a linked PAM user can upload
# public SSH keys via the account dashboard. This is disabled
# by default, because the auto-expiring PAM user passwords are
# the safer option.
#
# default: true
# overwritten by: PAM_SSH_AUTHORIZED_KEYS_ENABLE
authorized_keys_enable = true

# By default, even though these are "public" keys, the endpoint
# to retrieve them quires authentication. This will be a `basic"
# `Authentication` header in the form of `host_id:host_secret` of
# any valid PAM host configured on Rauthy.
# If you set it to `false`, the endpoint will be publicly available.
# This is fine in the sense that you cannot leak any keys (they are
# public keys anyway), but the endpoint could be abused for username
# enumeration. Depending on the `include_comments` settings below,
# you might even leak some more information that is not strictly
# sensitive, but could be abused in some other way.
#
# default: true
# overwritten by: PAM_SSH_AUTH_REQUIRED
auth_required = true

# By default, SSH keys that have expired because of
# `forced_key_expiry_days` below will be added to an internal
# blacklist. This blacklist will be checked upon key add to
# make sure keys were actually rotated and that not an old key
# is added again.
#
# default: true
# overwritten by: PAM_SSH_BLACKLIST_SSH_KEYS
blacklist_used_keys = true

# Configure the days after which blacklisted SSH keys will be
# cleaned up.
#
# default: 730
# overwritten by: PAM_SSH_BLACKLIST_CLEANUP_DAYS
blacklist_cleanup_days = 730

# You can include comments in the public response for the
# `authorized_keys` for each user. This can be helpful for
# debugging, but should generally be disabled to not
# disclose any possibly somewhat "internal" information.
#
# default: true
# overwritten by: PAM_SSH_INCLUDE_COMMENTS
include_comments = true

# You can enforce an SSH key expiry in days. After this time,
# users must generate new keys. This enforces a key rotation
# with is usually overlooked especially for SSH keys.
# Set to `0` to disable the forced expiry.
#
# default: 365
# overwritten by: PAM_SSH_KEY_EXP_DAYS
forced_key_expiry_days = 365
```

[#1249](https://github.com/sebadob/rauthy/pull/1249)
[#1250](https://github.com/sebadob/rauthy/pull/1250)
[#1253](https://github.com/sebadob/rauthy/pull/1253)

#### `preferred_username` and `tz`

The custom user values have been expanded. Each user can now provide a `preferred_username` and a
`tz` (timezone) via the account dashboard. The default timezone will always be UTC, just like it was
up until this version. The `preferred_username` behavior depends on some new configuration values.
In addition to that, the requirements of all other already existing values has more config options
as well. Everything that is `required` will also be requested during the initial registration, if
you have an open registration endpoint.

Because we have these new values, they will also show up in the `id_token` if the `profile` scope
was requested. Until now, the `preferred_username` was always existing and simply set to the
`email`. However, this has the potential to produce issues in downstream clients, if they don't
handle the `preferred_username` properly and require some specific value (which they really should
not ...). If the user has anything else than `UTC` or `Etc/UTC` configured as timezone, the
`zoneinfo` claim will be added to the `id_token` as well.

CAUTION: If your client does not request the `profile` scope during login, the `preferred_username`
will NOT be set to the `email` like it was the case up until this version!

These are the new config options:

```toml
[user_values]

# In this section, you can configure the requirements for different
# user values to adjust them to your needs. The `preferred_username`
# as a special value provide some additional options.
# The `email` is and always will be mandatory.
#
# A value of `hidden` will only hide these values for normal users
# in the account dashboard. An admin will still see all values.
#
# You can set one of the following values:
# - required
# - optional
# - hidden

# default: 'required'
given_name = 'required'
# default: 'optional'
family_name = 'optional'
# default: 'optional'
birthdate = 'optional'
# default: 'optional'
street = 'optional'
# default: 'optional'
zip = 'optional'
# default: 'optional'
city = 'optional'
# default: 'optional'
country = 'optional'
# default: 'optional'
phone = 'optional'
# default: 'optional'
tz = 'optional'

[user_values.preferred_username]

# If the `preferred_username` is not set for a given user, the
# `email` will be used as a fallback. This can happen, if it is
# not set to `required`, or if you had it optional before and
# then changed it, while the user may have not updated it yet
# according to the new policy.
#
# one of: required, optional, hidden
# default: 'optional'
preferred_username = 'optional'

# The `preferred_username` is an unstable claim by the OIDC RFC.
# This means it MUST NOT be trusted to be unique, be a stable
# map / uid for a user, or anything like that. It is "just
# another value" and should be treated like that.
#
# However, `preferred_username`s from Rauthy will always be
# guaranteed to be unique. You can define if these usernames
# are immutable once they are set, which is the default, or if
# users can change them freely at any time.
#
# default: true
immutable = true

# Provide an array of blacklisted names.
#
# CAUTION: Provide all these names as lowercase! The value
#  submitted via API will be converted to lowercase and
#  then compared to each entry in this list.
#
# default: ['admin', 'administrator', 'root']
blacklist = ['admin', 'administrator', 'root']

# You can define the validation regex / pattern.
#
# The `pattern_html` it will be sent to the frontend as a
# String value dynamically. It must be formatted in a way,
# that it will work as a
# [`pattern` attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/pattern)
# after the conversion. If you are unsure if it works, check
# your developer tools console. You will see an error log
# if the conversion fails.
# NOTE: These are NOT Javascript regexes!
#
# By default, the validation matches the Linux username regex,
# but you may want to increase the minimum characters for
# instance.
#
# default: '^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$'
regex_rust = '^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$'
# default: '^[a-z][a-z0-9_\-]{1,61}$'
pattern_html = '^[a-z][a-z0-9_\-]{1,61}$'

# If a user does not have a `preferred_username`, the `email`
# can be used as a fallback value for the id token.
#
# default: true
email_fallback = true
```

#### Terms of Service

It is now possible to add Terms of Service (ToS) to Rauthy. These can be found in the Admin UI under
`Config` -> `ToS`. I am not a lawyer, but I would say the implementation is legally "safe". If you
have an open registration, the latest existing ToS is being shown to the user and the registration
can only be completed after an accept. The other situation is an update to the ToS for existing
users. To have it legally correct, a user must accept in the middle of a login flow.

After ToS have been added, they are immutable in every regard, which is another important thing for
legal reasons. But, you can always add a new version, which users then have to accept. It is also
possible to enable an optional transition time for new ToS. For instance, if you have Rauthy in
front of an application that contains user data, you can make it possible for users to at least get
their data and download it or whatever, even if they don't want to accept the new ToS. Only after
the transition time is over, it becomes mandatory to accept updated ToS.

It is also possible to check the accept status for each user via the same page on the Admin UI.

While working in this feature, some major refactoring has been made for the code logic of the login
flow. The goal was to simplify everything and also make it easier to maintain in the future, because
the ToS added some addition complexity.

You can do most things via the Admin UI, but there is a single new config value:

```toml
[tos]

# The timeout in seconds for a user to accept update ToS during the
# login flow.
# The initial lifetime of an AuthCode after a successful authentication
# will be extended by the `accept_timeout`. This gives the user a bit
# more time to read through updates ToS and avoids an AuthCode expiry
# if it takes a bit longer. This is mainly a UX improvement. After the
# ToS have been accepted, the original AuthCode will be re-saved with
# the actual lifetime to not weaken the security in these cases.
#
# CAUTION: Even though you can extend the lifetime on Rauthys side, you
# can run into issues with logins on the client side. For legal reasons,
# accepting updated ToS must happen after a successful login but before
# providing any access. Login flows are not only time-limited on Rauthys
# side, but most often also on the client side. This means if it takes
# too long to read and accept update ToS, the user may run into an auth
# error and do the login again.
#
# default: 900
# overwritten by: TOS_ACCEPT_TIMEOUT
accept_timeout = 900
```

[#1221](https://github.com/sebadob/rauthy/pull/1221)

#### Send custom E-Mail

It is now possible to send custom E-Mails to users and filtered user groups. This is important for
instance when you are planning a bigger maintenance window, or maybe you have a deadline for s
specific client / user group when you enforce MFA-secured logins, and so on.

You can now find a simple editor in the Admin UI -> Users overview in the navigation. You can decide
to send out E-Mail to all users, or filter them by

- in group
- not in group
- has role
- has not role

It is also possible to not send out the mails directly, but schedule them to a specific date and
time. There is no such thing as embedding images or sending attachments though.

You will have the following new config options:

```toml
[email.jobs]

# This section cares about email sending to users, which can
# be done via the Admin UIs user page. These settings only
# apply for custom emails sent via UI. All automatic mails like
# a new user registration will be sent immediately.

# If an open email job has not been updated for more than
# `orphaned_seconds` seconds, it will be considered as orphaned.
# In this case, the current cluster leader can pick up this
# job and start after the last successful email sent.
#
# default: 300
# overwritten by: EMAIL_JOBS_ORPHANED_SECONDS
orphaned_seconds = 300

# The interval in seconds at which the scheduler for orphaned
# or scheduled jobs should run and check. Smaller values
# increase precision for scheduled jobs with sacrificing a bit
# higher resource usage.
#
# default: 300
# overwritten by: EMAIL_JOBS_SCHED_SECONDS
scheduler_interval_seconds = 300

# Configures the batch size and delay between batches of users
# for sending custom emails. The batch size configures the
# batch of users being retrieved from the DB at once. This means,
# if you have a filter on your email targets, the total amount
# of emails sent can be lower of course. Users are filtered
# on the client side to take the load off the DB.
#
# The default is pretty conservative to not have CPU and memory
# spikes if there is a huge amount of users, and to not overwhelm
# the SMTP server or reach rate limits.
# Depending on the speed of your SMTP server, the conservative
# default will handle ~5000 users in 1 hour. Even if it can
# take a higher load, be careful with sending too quickly to not
# trigger spam filters. Only increase throughput if needed.
#
# Note: If any error comes up during a batch, some users from this
# very batch may get duplicate emails when it is retried after
# being marked as orphaned.
#
# default: 3
# overwritten by: EMAIL_JOBS_BATCH_SIZE
batch_size = 3
#
# Delay in ms between email batches. If you set this to 0,
# Rauthy will send out emails as fast as possible. This
# should be avoided, especially for high user counts.
#
# default: 2000
# overwritten by: EMAIL_JOBS_BATCH_DELAY_MS
batch_delay_ms = 2000
```

[#1247](https://github.com/sebadob/rauthy/pull/1247)

#### UI Improvements

The UI received some updates and improvements.

[#1230](https://github.com/sebadob/rauthy/pull/1230)
[#1268](https://github.com/sebadob/rauthy/pull/1268)
[#1269](https://github.com/sebadob/rauthy/pull/1269)

#### Customize Timestamp formatting in E-Mails

You can now customize how the timestamp in E-Mails will be formatted. In combination with the new
`tz` value for users (see above), timestamps can now be formatted very specific for each user to
avoid confusion with UTC.

These are the new config options:

```toml
[email.tz_fmt]

# The formatting of timestamps in emails can be configured
# depending on the users' language.
#
# You can generally use all options from
# https://docs.rs/chrono/0.4.42/chrono/format/strftime/index.html
#
# default: '%d.%m.%Y %T (%Z)'
# overwritten by: TZ_FMT_DE
de = '%d.%m.%Y %T (%Z)'
# default: '%m/%d/%Y %T (%Z)'
# overwritten by: TZ_FMT_EN
en = '%m/%d/%Y %T (%Z)'
# default: '%Y-%m-%d %T (%Z)'
# overwritten by: TZ_FMT_KO
ko = '%Y-%m-%d %T (%Z)'
# default: '%d.%m.%Y %T (%Z)'
# overwritten by: TZ_FMT_NO
no = '%d.%m.%Y %T (%Z)'
# default: '%d-%m-%Y %T (%Z)'
# overwritten by: TZ_FMT_ZHHANS
zhhans = '%d-%m-%Y %T (%Z)'

# If a user has no timezone set, you can configure a
# fallback. This is useful for instance when you run a
# regional deployment.
#
# default: 'UTC'
# overwritten by: TZ_FALLBACK
tz_fallback = 'UTC'
```

[#1246](https://github.com/sebadob/rauthy/pull/1246)

#### User self-delete

Users can now be allowed to self-delete their accounts. By default, it is disabled, because when
you are using e.g. SCIM, a user deletion can trigger quite a few events in other clients as well,
and it might delete data that you need to clean up (or archive for legal reasons) before you can
fully delete a user. So, it's opt-in, and it probably makes the most sense when you have an open
registration as well.

```toml
[user_delete]

# You can enable user self-deletion via the Account Dashboard.
# It is disabled by default, because especially if you use things like
# SCIM, the deletion of a user might trigger a series of events which
# will delete other important data as well, that might be linked to a
# user account, and you want to clean up manually before a user is being
# fully deleted.
#
# default: false
# overwritten by: USER_ENABLE_SELF_DELETE
enable_self_delete = false
```

[#1267](https://github.com/sebadob/rauthy/pull/1267)

#### Relax input validation for Client URIs

The input validation for different URIs when configuring clients via the Admin UI has been relaxed.
This makes it possible to e.g have an allowed `redirect_uri` or `http://localhost:*` to work with
dynamic callback ports.

[#1243](https://github.com/sebadob/rauthy/pull/1243)

#### Rauthy Logo as client fallback

If you have a custom logo for the Rauthy client, the same logo will automatically be used as the
fallback for all other clients that do not have a custom one on their own. This is an addition of
the `rauthy` client branding as a fallback in these cases from an earlier version.

[#1242](https://github.com/sebadob/rauthy/pull/1242)

#### `skip_okp=true` for GET `/oidc/certs`

As a workaround for some buggy OIDC client implementations like e.g. Cloudflare Zero Trust, you can
now add `skip_okp=true` as a query param to the JWKS URI. If set to `true`, it will strip all `OKP`
keys from the response. The URI will then look like this:

```
https://iam.example.com/auth/v1/oidc/certs?skip_okp=true
```

[#1263](https://github.com/sebadob/rauthy/pull/1263)

#### HA Stability Improvements

The work was done in [hiqlite](https://github.com/sebadob/hiqlite), but with the updates, Rauthys
stability in HA deployments was improved quite a bit.

The `cluster.shutdown_delay_millis` config option was removed. It is not necessary to set it
manually anymore. Instead, more automatic detection is being applied and a necessary delay to smooth
out rolling releases or make sure the readiness of a container is being caught is added without the
need for additional config.

Apart from that, lots of improvements have been made to rolling releases and how WebSocket
re-connects and node startups are being handled in general. There is a new `/ready` endpoint on the
public API as well. It can be used in e.g. Kubernetes to smooth out rolling releases and detect a
pod shutdown before it becomes unable to handle Raft requests. To do so, it is important however to
not have too high `periodSeconds`, and the `headless` service needs to `publishNotReadyAddresses`
ports before ready, like so:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: rauthy-headless
spec:
  type: ClusterIP
  clusterIP: None
  # Make sure to only publish them on the headless service 
  # and NOT the one you are using via your reverse proxy!
  publishNotReadyAddresses: true
  sessionAffinity: None
  selector:
    app: rauthy
  ports:
    - name: hiqlite-raft
      protocol: TCP
      port: 8100
      targetPort: 8100
    - name: hiqlite-api
      protocol: TCP
      port: 8200
      targetPort: 8200
```

Then you can make use of the new readiness check in the `StatefulSet`:

```yaml
readinessProbe:
  httpGet:
    scheme: HTTP
    # Hiqlite API port
    port: 8200
    path: /ready
  initialDelaySeconds: 5
  # Do NOT increase this period, because otherwise K8s may not catch
  # a shutting down pod fast enough and may keep routing requests to
  # it while is will be unable to handle them properly because of
  # the shutdown.
  periodSeconds: 3
  # We may get a single failure during leader switches
  failureThreshold: 2
livenessProbe:
  httpGet:
    scheme: HTTP
    # Rauthy API port
    port: 8080
    path: /auth/v1/health
  initialDelaySeconds: 60
  periodSeconds: 30
  # We may get a single failure during leader switches
  failureThreshold: 2
```

Apart from that, the `hiqlite-wal` had a bug where the `last_purged_log_id` was overwritten with
`None` during a log truncation, even if it had a value from a log purge before. If the node
restarted before another log purge fixed it, it would result in an error during startup. The new
version includes a check + fix, if you start up an instance with a data set that currently has this
issue.

> NOTE: Rauthys shutdown in HA deployments can take up to 30 seconds when done gracefully, depending
> on the config and the current state the cluster is in. Some container runtimes may force-kill a
> container after only a few seconds by default. Make sure to adjust that timeout.

### Bugfix

- With a bigger internal code migration and cleanup some time ago, a few housekeeping schedulers
  got lost and were not started anymore.
  [#1247](https://github.com/sebadob/rauthy/pull/1247)
- The UI for the `device_code` flow had a wrong value for the `user_code_length` inserted via
  HTML `<template>`s.
  [#1258](https://github.com/sebadob/rauthy/pull/1258)
- A query param was missing in the SQL for cleaning up old JWKs.
  [#1265](https://github.com/sebadob/rauthy/pull/1265)

## v0.32.6

### Bugfix

- In `hiqlite` as an external dependency, it was possible to get into a situation where the WAL
  cleanup of not anymore needed logs / files was not working as expected. This could lead to an
  endlessly filling up volume. This version bumps `hiqlite` to the latest, fixed version. With the
  next snapshot creation for existing instances (every 10k Raft logs), all old WAL files will be
  cleaned up.
  [#1233](https://github.com/sebadob/rauthy/pull/1233)

## v0.32.5

### Bugfix

- It was not possible to send out an additional `Password Reset E-Mail` for users via the Admin UI
  because of a missing value from the frontend. The loading spinner would run forever and the E-Mail
  would not be sent.
  [#1213](https://github.com/sebadob/rauthy/pull/1213)

## v0.32.4

### Bugfix / Security

Under certain config conditions and client / user setup, and if you were using the Login Group
Prefix limitation for a client, it was possible that this restriction was ignored during session
refreshes, when logging in to a restricted client. The additional check was missing during
refreshes, like when you still had a valid session on Rauthy, because you logged into something else
beforehand, and you then wanted to log in to this restricted client within the session timeout
window.

[#1208](https://github.com/sebadob/rauthy/pull/1208)

## v0.32.3

### Bugfix

- When deleting a client,there was a last case that could make the deletion fail under specific
  circumstances.
  [#1198](https://github.com/sebadob/rauthy/pull/1198)

## v0.32.2

### Security

Some external dependencies have been bumped because of security issues. However, these either only
affected development servers, or were false-positives, because the fixed versions were being used
already, just not set in the lock files.

### Changes

#### Additional Events

For improved auditing, Rauthy now emits the additional event types `LoginNewLocation` and
`TokenIssued`.

The `LoginNewLocation` will be emitted when ever a user does a login from a new / unknown location.
The `TokenIssued` even will be triggered after each JWT token creation. Especially the `TokenIssued`
can become spammy, if you have a huge amount of users. Because of this, you can disable the
generation of these events. You get some new config variables:

```toml
[events]
# Can be set to `false` to disable events being generated
# when a new token was issued. These events improve your
# auditing, but they can also be considered spam if you
# have a huge amount of users and logins.
#
# default: true
# overwritten_by: EVENT_GENERATE_TOKEN_ISSUED
generate_token_issued = true

# The level for the generated Event after a login from
# a new location for a user.
#
# default: notice
# overwritten by: EVENT_LEVEL_NEW_LOGIN_LOCATION
level_new_login_location = 'notice'

# The level for the generated Event after a new JWT
# Token was issued.
#
# default: info
# overwritten by: EVENT_LEVEL_TOKEN_ISSUED
level_token_issued = 'info'
```

[#1192](https://github.com/sebadob/rauthy/pull/1192)

#### Translations for Norwegian Bokmål

Rauthy now has Norwegian Bokmål (nb) for Admin and User UI translations.

[#1180](https://github.com/sebadob/rauthy/pull/1180)

### Bugfix

- It was possible to get into a race condition during client deletion under some conditions.
  [#1190](https://github.com/sebadob/rauthy/pull/1190)
- API Keys were missing `PAM` access a rights group.
  [#1191](https://github.com/sebadob/rauthy/pull/1191)

## v0.32.1

### Security

- Make sure a `page_size` of `0` will not be accepted when doing server-side searches for users or
  sessions to prevent a possible division by `0`. This did not lead to a `panic` because of a
  conversion into `f64`, but definitely to an unexpected answer to the client from the calculation
  for the next page. The max value for it is now also limited and instead of `u16::MAX`, the max is
  `server.ssp_threshold`.
  [#1165](https://github.com/sebadob/rauthy/pull/1165)
- Make the secret comparison for OIDC clients and PAM hosts constant time.
  [#1167](https://github.com/sebadob/rauthy/pull/1167)
- Fix logic bug in SVG sanitization for user pictures and allow direct SVG upload via the Account
  Dashboard. Also add a specially hardened CSP to the client and user picture endpoints.
  [#1168](https://github.com/sebadob/rauthy/pull/1168)
  [#1169](https://github.com/sebadob/rauthy/pull/1169)
- Remove a reachable `unwrap()` in the Cache GET handler, which was possible if a request is being
  cancelled before awaiting the answer from the cache. This was fixed in `hiqlite-0.10.1` as a
  dependency.
  [#1177](https://github.com/sebadob/rauthy/pull/1177)

### Changes

#### User Name validation

`'` is now an allowed character in user `user.given_name` and `user.family_name`.

[#1171](https://github.com/sebadob/rauthy/pull/1171)

#### Self-Signed TLS certificates

As part of the repo cleanup before an upcoming `v1.0.0`, the static DEV TLS certificates were
removed from the repo. Rauthy can now generate self-signed certificates (with a proper CA for more
in-depth testing) on its own. A CA with a lifetime of 10 years will be generated and saved (
encrypted) into the database. This CA will be used to create self-signed TLS certificates for the
HTTPS server, and all nodes of an HA cluster will make use of it.

```toml
[tls]
# If set to `true`, Rauthy will generate self-signed TLS certs and copy
# them into `tls/self_signed_cert.pem` and `tls/self_signed_key.pem`.
# It will also IGNORE any `cert_path` / `key_path`.
#
# CAUTION: If set to `true`, it will delete existing files:
# - `tls/self_signed_cert.pem`
# - `tls/self_signed_key.pem`
#
# This should only be used for testing and never in production!
#
# default: false
# overwritten by: TLS_GENERATE_SELF_SIGNED
generate_self_signed = true
```

[#1173](https://github.com/sebadob/rauthy/pull/1173)
[#1174](https://github.com/sebadob/rauthy/pull/1174)

## v0.32.0

### Changes

#### PAM + NSS Modules

OIDC / OAuth covers almost all web apps, and for those that don't have any support, Rauthy comes
with `forward_auth` support. To not need an additional LDAP / AD / something similar for your
backend and workstations, Rauthy now has its own custom PAM module. It does not just use JWT Tokens
for logging in, but you can actually manage all your Linux hosts, groups and users in different
ways. You have the option to secure local logins to workstations via Yubikey (only USB Passkeys
supported, no QR-code / software keys), and all SSH logins can be done with ephemeral, auto-expiring
passwords, that you can generate via your Account dashboard, if an Admin has created a PAM user for
you. This means you get MFA-secured SSH logins without the need for any modifications or additional
software on your local SSH client, and you can use any SSH client from any machine securely, even if
it's not your own.

In addition to the PAM module, you there is an NSS module and an NSS proxy that run on each machine.
You can dynamically log in to any machine an Admin has given you access to. Users and groups are not
added to local files, but will be resolved via the network.

This module is published in a separate repo to avoid licensing issues, since it relies on some GPLv3
dependencies. You can take a look at it
here: [rauthy-pam-nss](https://github.com/sebadob/rauthy-pam-nss). It has been tested with local
terminal and window manager logins, as well es via SSH, but it should work basically anywhere where
you can authenticate via PAM.

A more detailed documentation was added
to [The Book](https://sebadob.github.io/rauthy/work/pam.html) already.

```toml
[pam]
# The length of newly generated PAM remote passwords via the
# account dashboard. The default is fine as long as you can copy
# & paste them. You may want to reduce the length here if you e.g.
# occasionally generate them on mobile and need to type them
# manually into some terminal.
#
# default: 24
# overwritten by: PAM_REMOTE_PASSWORD_LEN
remote_password_len = 24

# The TTL for newly generated PAM remote passwords in seconds.
# The default gives you plenty of time to open a few sessions in
# some terminals and maybe switch to `root` on some remote machines,
# while still expiring quick enough to be secure.
#
# default: 120
# overwritten by: PAM_REMOTE_PASSWORD_TTL
remote_password_ttl = 120
```

> Even though a lot of testing was done, I am pretty sure there are still some issues or maybe
> SELinux warnings that
> need to be fixed, but it should be absolutely usable. Everything is secured, but it may still lack
> a good UX here and
> there.

[#1101](https://github.com/sebadob/rauthy/pull/1101)
[#1132](https://github.com/sebadob/rauthy/pull/1132)
[#1134](https://github.com/sebadob/rauthy/pull/1134)
[#1142](https://github.com/sebadob/rauthy/pull/1142)

#### SMTP: XOAUTH2 + Microsoft Graph

Because Microsoft does really stupid things, and they disable SMTP basic auth in september, you can
now also connect via SMTP XOAUTH2, or if even that is an issue for you, you can use the Microsoft
365 native Graph API. Both options require you to provide all necessary values to fetch an OAuth
token via `client_credentials` flow and the SMTP connection may be re-opened when the token expires.

Both of these options are worse than the default. Against all misleading blog posts about it, they
provide absolutely no advantages in security, and you should not use them, if you can stick with the
default. Both options have an overhead of fetching, storing and refreshing the token, and re-opening
the connections over and over. Especially the Graph API is pretty slow and it should be your last
resort.

There are some new config options available in the `email` section:

```toml
[email]
# You usually do not need to change this value. The 'default'
# will connect via SMTP PLAIN or LOGIN, which works in almost
# all scenarios. However, you can change it to `xoauth2` to
# connect via XOAUTH2. In addition, there is also `microsoft_graph`.
# It is a custom implementaion that does not use SMTP anymore,
# but instead the Microsoft Azure Graph API. You should not use
# it, unless you have no other choice or a very good reason.
#
# If you specify another value than ´default`, you must provide
# all the additional `x_oauth_*` values bwloe.
#
# possible values: 'default', 'xoauth2', 'microsoft_graph'
# default: 'default'
# overwritten by: SMTP_CONN_MODE
smtp_conn_mode = 'default'

# These values must be given if `auth_xoauth2 = true`.
# They will be used for a `client_credentials` request
# to the `xoauth_url` to retrieve a token, that then
# will be used for authentication via SMTP XOAUTH2.
#
# overwritten by: SMTP_XOAUTH2_URL
xoauth_url = ''
# overwritten by: SMTP_XOAUTH2_CLIENT_ID
xoauth_client_id = ''
# overwritten by: SMTP_XOAUTH2_CLIENT_SECRET
xoauth_client_secret = ''
# overwritten by: SMTP_XOAUTH2_SCOPE
xoauth_scope = ''

# Only needed if `smtp_conn_mode = 'microsoft_graph'`:
# https://learn.microsoft.com/en-us/graph/api/user-sendmail?view=graph-rest-1.0&tabs=http
#
# default: not set
# overwritten by: SMTP_MICROSOFT_GRAPH_URI
microsoft_graph_uri = ''
```

[#1136](https://github.com/sebadob/rauthy/pull/1136)

#### Auth Providers Onboarding + Link

Upstream Auth Providers have 2 new options:

- auto-onboarding
- auto-link user

The `auto-onboarding` should be self-explanatory. If a user does not exist yet inside Rauthys DB
after a successful auth provider login, it will automatically be created. This was the default up
until now. However, this allows anyone with an account on the upstream provider to log in, and even
though such a user would not have any access rights, this may be unwanted. To "fix" this, you can
now decide to disable auto-onboarding.

To still provide a good UX for new users, the additional `auto-link user` option will allow an Admin
to create an fresh account, that has no password or anything else set. A user can decide to not
create a password for Rauthy, but log in via the upstream provider directly and if this option is
set and the user is not linked to any other provider yet, it will auto-link this user to the
upstream provider on login.

> **CAUTION:** This option will show you a warning in the Admin UI as well. If you set
`auto-link user` and your
> upstream provider does NOT VALIDATE E-Mail addresses 100% correctly, and allows a user to set an
> address that belongs
> to someone else, this option can lead to account takeover! Do NOT use it if you cannot fully trust
> the validation
> process of the upstream provider!

[#1153](https://github.com/sebadob/rauthy/pull/1153)

#### EncKeyID Validation

The validation regex for EncKeyIDs has been relaxed a tiny bit. It was changed from
`^[a-zA-Z0-9]{2,20}$` to
`^[a-zA-Z0-9:_-]{2,20}$`.

### Bugfix

- The default value for `rp_origin` during `LOCAL_TEST=true` was set to a value that would not work
  out of the box.
  [#1138](https://github.com/sebadob/rauthy/pull/1138)
- Overwriting ENV vars would not be parsed, if the underlying section did not exist at all in the
  config file. This was not an issue on its own, because there was nothing to "overwrite" in the
  first place, but probably unexpected behavior.
  [#1147](https://github.com/sebadob/rauthy/pull/1147)
- If a given `redirect_uri` for a client already contained query params, the final URI was not built
  correctly.
  [#1148](https://github.com/sebadob/rauthy/pull/1148)
- `session.validate_ip` was not respected during `/forward_auth`.
  [#1149](https://github.com/sebadob/rauthy/pull/1149)

## v0.31.3

### Bugfix

- The key creation of RSA keys for the RS256 signing algorithm had a typo and generated 2028 bit
  keys instead of 2048. This tiny difference is not really a huge issue in terms of security, but it
  made some clients fail to validate the tokens, because they expected 2048. That bug came in with
  the big JWT rework in v0.30 and the Custom JWT implementation. If you currently have issues with
  some client that uses RS256, you will get a new pair of keys via:
  `Admin UI -> Config -> JWKS -> Rotate Keys`
  [#1124](https://github.com/sebadob/rauthy/pull/1124)
- The automatic Session Refresh action on the Login UI stopped working under some conditions. By
  default, as long as you have a valid session (and other config + request variables are met), you
  should not be prompted for another login and the UI should refresh you session automatically and
  log you in. The effect trigger logic in the UI was reworked slightly, and it fixed the issue in my
  test setup. If someone still has problems, please open an issue about it.
  [#1128](https://github.com/sebadob/rauthy/pull/1128)

## v0.31.2

### Bugfix

- It was possible that a "Login from new location" notification was sent out for Passkey-only
  accounts, if someone knew all the right values incl the username, but was not able to get past the
  MFA / Passkey challenge. This would have been a false-positive.
  [#1105](https://github.com/sebadob/rauthy/pull/1105)
- After Rauthy received an accessibility review from [HAN](https://www.han.nl/) as part of the NGI
  funding, it was declared as "fairly accessible" already, but there were still some things that
  have been improved after the review.
  [#1099](https://github.com/sebadob/rauthy/issues/1099)
- When using the `MIGRATE_DB_FROM` feature, some of the latest changes and features were missing in
  migration queries.
  [#1116](https://github.com/sebadob/rauthy/issues/1116)

## v0.31.1

### Bugfix

- The Login Location check during the `code` flow has been done on `POST /token` instead of
  `/authorize` and it therefore caught the OIDC client app instead of the user browser.
  [#1097](https://github.com/sebadob/rauthy/pull/1097)

## v0.31.0

### Breaking

#### Empty `User-Agent`s rejected

While implementing Geoblocking (see below), a check for the `User-Agent` has been added. If this is
empty, requests will be rejected. If you are doing something via API-Keys, make sure your clients
send a proper `User-Agent` header. This rejection does not provide any security at all, it just
catches a few bots out of the box.

### Changes

#### Remembered Login Locations

Rauthy will now remember Login locations / IPs for each user and send out an E-Mail notification, if
a login from a new IP / location was done. This E-Mail will also contain a link to revoke all
Sessions and Refresh tokens for this user, which will also trigger a backchannel logout to all
configured clients, in case an invalid login was done for a user. Users should be able to handle
these situations without the need of an Admin in this case.

[#1072](https://github.com/sebadob/rauthy/pull/1072)

#### IP-Geoblocking

Rauthy can now block requests depending on the origin country of the peer IP address. You can either
provide a custom header value, which is inserted for instance by a WAF or CDN, like in case of
Cloudflare the `CF-IPCountry` header, or you can opt-in and provide a Maxmind AccountID + License
for either the free GeoLite databases, or a paid version of it. These GeoLite databases are
published under the Creative Commons License, you only need to create a free account to generate a
License. Rauthy will download the configured DB and update it regularly.

This IpGeo information will also be used for the *Remembered Login Locations* feature above. The
country and depending on the chosen DB type also the city will be added to the E-Mail notice.

```toml
[geolocation]
# If you have a configured and working Geolocation setup, you can
# define if un-resolvable IP addresses should be blocked. By default,
# if a Country cannot be found for a certain IP address, it will be
# allowed anyway. This is necessary to allow private network connections
# for instance. Only if you run a public Rauthy instance, and you will
# guaranteed always connect with a public Peer IP, you might want to
# set this to `true`.
#
# default: false
# overwritten by: GEO_BLOCK_UNKONW
block_unknown = false

# If you have a WAF or CDN which injects a geoloaction header
# with the country code, provide the name here. For instance,
# in case of Cloudflare, this would be 'CF-IPCountry'.
#
# This header will only be accepted, if Rauthy runs in proxy_mode,
# and the source IP is a trusted proxy, to prevent spoofing.
#
# default: not set
# overwritten by: GEO_COUNTRY_HEADER
country_header = 'CF-IPCountry'

# You can black- or whitelist countries, if you have a configured
# and working Geolocation, either via `country_header` or a
# Maxmind database.
#
# The `country_list_type` can be either `whitelist` or `blacklist`,
# and it will specify the behavior of the `country_list`.
# For instance, if you have `country_list_type = 'whitelist'` and
# `country_list = ['DE', 'FR']`, only access from Germany and France
# will be allowed.
#
# The `whitelist` type is a `default-deny`, while `blacklist` is
# `default-allow`.
#
# If `country_list_type` is not set at all, Geoblocking will be
# disabled.
#
# default: not set
country_list_type = 'whitelist'
# default: not set
country_list = []

# If you don't have a header with a country code, you can
# also provide a Maxmind account. Rauthy will then download
# the 'GeoLite2 Country' database regularly and use it for
# geolocating IPs.
#
# The GeoLite databases from Maxmind are free and published
# under the Creative Commons License. You can also provide
# an Enterprise database, which will have more accurate data.
# Check the `maxmind_db_type` below.
#
# default: not set
# overwritten by: GEO_MAXMIND_ACC_ID
maxmind_account_id = ''
# overwritten by: GEO_MAXMIND_LICENSE
maxmind_license_key = ''

# If `maxmind_account_id` and `maxmind_license_key`, this
# will be the directory being used for DB download and storage.
#
# default: 'data'
# overwritten by: GEO_MAXMIND_DIR
maxmind_db_dir = 'data'

# By default, the `GeoLite2-Country` database from Maxmind is
# being used. The IP Geolocation databases are loaded fully into
# memory at startup to speedup lookups. The size therefore makes
# a big difference, not only for lookup speed, but also in terms
# of memory usage. The Country DB adds ~10MB of memory overhead,
# while the City DB is around 65MB.
#
# Possible Values (case-sensitive):
# - GeoLite2-Country
# - GeoLite2-City
#
# If you have access to paid Maxmind databases, you can add the
# db_type in a way that it resolves to a valid download link.
# The link will be created with the following template:
# `https://download.maxmind.com/geoip/databases/{maxmind_db_type}/download?suffix=tar.gz`
#
# default: 'GeoLite2-Country'
# overwritten by: GEO_MAXMIND_DB_TYPE
maxmind_db_type = 'GeoLite2-Country'

# If you configured a `maxmind_account_id` + `maxmind_license_key`,
# you can change the time when the DB update job runs. By default,
# it runs every night at 05:00. It will check if y new version of
# the MaxMind DB is available and if so, download it.
#
# Accepts cron syntax:
# "sec min hour day_of_month month day_of_week year"
#
# default: "0 0 5 * * * *"
# overwritten by: GEO_MAXMIND_UPDATE_CRON
maxmind_update_cron = "0 0 5 * * * *"
```

> While IP Geo data is very helpful and can boost your security, it is fully optional. Everything
> will work if you don't provide any of the options. You just won't have Geo data in the "Login from
> new location" notifications and other places, and you will of course not be able to block requests
> depending on the origin country.

[#1077](https://github.com/sebadob/rauthy/pull/1077)

#### TLS Hot-Reload

Rauthy can now hot-reload TLS Key + Certificates. If started with `server.scheme` set to any `https`
value and TLS certificates are used, Rauthy will watch for file changes on `tls.cert_path` +
`tls.key_path` and will do a hot-reload of the TLS configuration if anything changes. This is a real
hot-reload, meaning there is no restarting the server, and it does it without any interruption in
service.

[#1056](https://github.com/sebadob/rauthy/pull/1056)

#### OIDC-backed Forward-Auth

In addition to the already existing, very simple `/forward_auth` endpoint, which has limited
compatibility, Rauthy now provides a very much advanced version of it. This new version is not a
replacement of the old approach, but an addition.

The already existing endpoint is very simple: It expects a valid JWT token to be present in the
`Authorization` header, parses and validates it, and if it's valid, it returns an HTTP 200 and a 401
otherwise. Depending on `auth_headers.enable`, it will also append the Forward-Auth headers to the
request, which the reverse proxy could inject into the request sent to the downstream client.

The new version is much improved. It does not work with stateless JWT tokens, but it binds to the
Rauthy session. This makes it possible to revoke access as any time. It can also do proper CSRF
checks, validates the client and user configuration, and it can make everything work without any
modification to the client. On auth success, it behaves in the same way as the already existing
endpoint. On invalid though, it will redirect to Rauthys Login, which then again will do another
redirect to a Callback UI and therefore trigger a complete OIDC flow. On the callback page, Rauthy
can now set fully-secured session cookies and do others things like check the `Sec-Fetch-Site`
header. This is the most secure it can get, without modifications to the client. The callback page
is exposed by Rauthy itself, and can be "injected" into the client app at your reverse proxy level,
which makes all of this as secure as possible.

You can do this new Forward-Auth for any client, as long as it's configured properly. Rauthy is
quite a bit more strict about the correct client config upfront. This makes it possible to have a
few additional safety hooks which will help you prevent unwanted, invalid reverse proxy config,
which can happen very quickly for complex setups.

> CAUTION: Even though this is probably the most secure you can get with Forward-Auth, it should
> still only be the last resort, and you should always prefer a native OIDC client implementation,
> if it exists! If you screw up the reverse proxy config, or if an attacker can find a way around
> your reverse proxy and skip it, all your security will be gone immediately.
>
> The Rauthy book will be updated in the upcoming days and provide a bit more documentation about
> the setup.

To help during Forward-Auth setup and making sure you got it right in your environment, the
`/auth/v1/whoami` endpoint has received an update as well. You can now set
`access.whoami_headers = true` or use `WHOAMI_HEADERS`. This will make the `/whoami` endpoint not
only return the extracted "real IP", but it will also return all request headers it received.
This will help you make sure your setup is working correctly, if you use `auth_headers`. By default,
this is set to `false`. Depending on your internal network setup, this could expose sensitive
headers, if you inject any. It will not return values for `Cookie` and Rauthys own CSRF token
headers, but all others return will show their raw values.

[#1053](https://github.com/sebadob/rauthy/pull/1053)

#### Backups via Admin UI

If Rauthy is running with Hiqlite as the database, you can now view and download existing backups
via the `Admin UI -> Config -> Backups` section. It shows local backup files and the ones on S3
storage, if it's configured. You also get the option to trigger manual backups with a new button,
which gets rid of the issue of updating the cron task to "a few minutes in the future" to trigger a
backup on demand.

> If you run a HA cluster and want to download local backups via the Admin UI, you will most
> probably run into errors when downloading older ones, that have been created before this version,
> if running behind a load balancer. The timestamp part of the filename was dynamically set by each
> node independently before and only new backups from this version on will have the exact same
> filename on all nodes.

[#1079](https://github.com/sebadob/rauthy/pull/1079)

#### Re-Authenticate for MFA keys modifications

To be able to modify MFA / Passkeys in any way, a user now needs to re-authenticate. This
re-authentication will open a 2-minute window that allows modifications for MFA keys, like adding
new ones and deleting existing ones.

[#1068](https://github.com/sebadob/rauthy/pull/1068)

#### Load config from Vault

The ability to load the config file from a Vault source has been added. To do this, you need to
provide the ENV var `USE_VAULT_CONFIG=true` and a `vault.toml` file, that contains the necessary
information on how to connect and/or override the settings with ENV vars.

```toml
[vault]

# The full address to your vault including scheme and port
#
# default: <empty>
# overwritten by: VAULT_ADDR
addr = ''

# The token that rauthy uses to access the secret
#
# default: <empty>
# overwritten by: VAULT_TOKEN
token = ''

# Secret engine mount point
#
# default: <empty>
# overwritten by: VAULT_MOUNT
mount = ''

# Path within the mount point containing the secret
#
# default: <empty>
# overwritten by: VAULT_PATH
path = ''

# The key (name) of the secret
#
# default: <empty>
# overwritten by: VAULT_CONFIG_KEY
config_key = ''

# KV Version that the secret engine uses.
# Valid values are '1' or '2'.
#
# default: '2'
# overwritten by: VAULT_KV_VERSION
kv_version = ''

# You can provide a root certificate bundle, if you
# are running servers / clients Rauthy needs to connect
# to with self-signed certificates.
# The certificates need to be in PEM format.
#
# overwritten by: HTTP_CUST_ROOT_CA_BUNDLE
#root_ca_bundle = """
#-----BEGIN CERTIFICATE-----
#...
#-----END CERTIFICATE-----
#"""

# If you are testing locally with insecure connections,
# you can set the following var to make it work.
#
# Note: This must be an ENV var and cannot be
# given as a normal TOML value. It only exists here
# for completeness / documentation.
#
#DANGER_VAULT_INSECURE=true
```

[#1051](https://github.com/sebadob/rauthy/pull/1051)
[#1090](https://github.com/sebadob/rauthy/pull/1090)

#### Bluesky / AT Protocol

This version supports Bluesky's at-proto. This is probably not used by most people and it's opt-in:

```toml
[atproto]
# Set to `true` to enable the ATProto provider. If the public URL is
# 'localhost' it should be changed to '127.0.0.1', if `dev_mode = true`
# this also applies for the `provider_callback_url`.
#
# default: false
enable = false
```

[#644](https://github.com/sebadob/rauthy/pull/644)
[#1064](https://github.com/sebadob/rauthy/pull/1064)

#### Additional Event Types

Some new Rauthy Event types have been added. These are now configurable in their notification level
as all the other ones. The new Events are the following:

- An `Event::ForceLogout` will be created during `DELETE /sessions/{user_id}`. This will happen when
  and Admin clicks   "Force Logout" for a user in the Admin UI.
- An `Event::UserLoginRevoke` will be created after a user clicked the login revoke link in the (
  new) notification E-Mail that is sent out after a login from an unknown location.
- An `Event::SispiciousApiScan` will be created when Rauthy detects a suspicious, very much likely
  malicious API scan.

[#1085](https://github.com/sebadob/rauthy/pull/1085)

#### Anti-Lockout hooks in Admin UI

The Admin UI already had anti-lockout hooks for some important things like the `rauthy` client or
the `rauthy_admin` role. This release adds some of these hooks to user edit and delete pages, if the
currently looked at user matches the currently logged-in admin from the session. These hooks
prevent, disabling, expiring, and deleting this user, and also removing the `rauthy_admin` role from
itself. This means a Rauthy admin cannot degrade itself to a non-admin or delete itself by accident.
Only other admins can do this. This makes sure, that at least always at least one `rauthy_admin`
exists, and you can never fully lock yourself out of Rauthy.

[#1066](https://github.com/sebadob/rauthy/pull/1066)

#### IPv6 Support for IP Blacklisting

You can now also blacklist IPv6 address via the Admin UI -> Blacklist.

[#1087](https://github.com/sebadob/rauthy/pull/1087)

#### Default difficulty for PoWs reduced

Since PoWs have been added to the Login UI, and a user needs to calculate 2 PoWs if it's a
password-account, the default difficulty of `20` was a bit too high for not that powerful devices.
Therefore, the default value has been reduced from `20` to `19` to compensate for that.

[#1088](https://github.com/sebadob/rauthy/pull/1088)

### Bugfix

- The `cluster.backup_keep_days_local` setting was not always read properly and might not have been
  working like expected. This lead to local backup cleanup not working properly. Additionally, the
  default value in docs was wrong. It's not `3` days by default, but `30`. This was fixed in
  `hiqlite` directly and the version has been bumped.

## v0.30.2

### Changes

#### Hiqlite upgrade

Internally, `hiqlite` was updated to the latest stable version. This brings 2 advantages:

1. `cluster.wal_ignore_lock` has been removed completely. It is not necessary anymore, because
   `hiqlite` now can do proper cross-platform file locking and therefore can resolve all possible
   situations on its own. It can detect, if another `hiqlite` process is currently using an existing
   WAL directory and also do a proper cleanup / deep integrity check after a restart as well.
2. You have 2 additional config variables to configure the listen address for Hiqlites API and Raft
   server. This solves an issue in IPv6-only environments, because it used a hardcoded `0.0.0.0`
   before. You can now also restrict to a specific interface as well, which is beneficial for single
   instance deployments, or when you have lots of NICs.

```toml
[cluster]
# You can set the listen addresses for both the API and Raft servers.
# These need to somewaht match the definition for the `nodes` above,
# with the difference, that a `node` address can be resolved via DNS,
# while the listen addresses must be IP addresses.
#
# The default for both of these is "0.0.0.0" which makes them listen
# on all interfaces.
# overwritten by: HQL_LISTEN_ADDR_API
listen_addr_api = "0.0.0.0"
# overwritten by: HQL_LISTEN_ADDR_RAFT
listen_addr_raft = "0.0.0.0"
```

#### DB shutdown on unavailable SMTP

If the retries to connect to a configured SMTP server were exceeded, Rauthy panics, which is on
purpose. However, the behavior has been updated slightly, and it will now trigger a graceful DB
shutdown before it executes the panic, which is just cleaner overall.

[#1045](https://github.com/sebadob/rauthy/pull/1045)

### Bugfix

- A trigger for Backchannel Logout was missing for `DELETE /sessions/{user_id}`
  [#1031](https://github.com/sebadob/rauthy/pull/1031)
- `state` deserialization validation during `GET /authorize` was too strict in some cases.
  [#1032](https://github.com/sebadob/rauthy/pull/1032)
- The pre-shutdown delay should only be added in HA deployments, not for single instances.
  [#1038](https://github.com/sebadob/rauthy/pull/1038)
- The error messages in case of `webauthn` misconfiguration were not always very helpful.
  [#1040](https://github.com/sebadob/rauthy/pull/1040)

## v0.30.1

### Bugfix

- Fixed the encoding for `EdDSA` public keys. They have changed to b64 encoded DER format during the
  big JWt rework, when it should have been just raw bytes.
  [#1018](https://github.com/sebadob/rauthy/pull/1018)
- Added a short 3-second pre-shutdown delay to smooth out rolling releases inside K8s and also have
  a bit more headroom for bigger in-memory cache's replication
  [#1019](https://github.com/sebadob/rauthy/pull/1019)
- Small CSS fix to match the input width for group prefix login restriction for clients
  [#1020](https://github.com/sebadob/rauthy/pull/1020)
- In HA deployments, when the Leader was killed with an appended but not yet fully commited WAL log,
  there was a bug that made it possible that the log truncate would fail after restart and that the
  start-, instead of the end-offset would be adjusted. This has been fixed in `hiqlite-wal` in
  combination with a bump in patch version for `openraft`. If you run a **HA cluster, you should
  upgrade immediately**!.

## v0.30.0

### Breaking

#### Configuration Rework

You will need to migrate your whole configuration with this release.

Over the last months, Rauthy got so many new features and config options, that the old approach got
a bit messy and hard to follow. Variable names needed to become longer over time to avoid overlap,
and so on. Also, because of the way the application grew over the last years, configs, single static
variables and some snippets existed in many places.

This needed to change.

You can still have a config file and overwrite (most) values via ENV vars, if you like, but at least
the bare minimum must exist as a config file now. Even though ENV var are mostly now a security
issue, they are by default not protected depending on the situation. The whole config file has been
converted into a `TOML` file. This makes it possible to have different dedicated sections, shorter
variable names again, and it's easier to organize and comes with an already existing, basic
type-system and we can have arrays and typed primitives out of the box now.

The file has been renamed from `rauthy.cfg` to `config.toml`, and it must follow the `toml` syntax.
I created a lookup table so everyone can easily convert their existing config with not too much
work. The new config also is more verbose in terms of documentation and for each value (when it
exists), the overwriting ENV var is mentioned directly above.

In some cases, you cannot overwrite with an ENV var, and in others, you can only do something with
an ENV var. The situations when you can only do something via ENV are the ones, that will require
you do remove the var immediately afterward anyway, like e.g. triggering a `DB_MIGRATE_FROM`, or do
a new `HQL_WAL_IGNORE_LOCK`, and so on. But I guess it will become clear when you take a look at the
lookup table. I also tried to make it more clear, which values are absolutely mandatory to set.

On the long run, this change, even though it was a huge PR with almost 12k lines changed, will make
everything easier to maintain and the code a lot more approachable for newcomers. A nice side effect
is, that I also optimized it a bit so that it requires a little bit less memory.

> If anyone has issues with the config with this update, please open an issue and let me know. I did
> a lot of testing and I think, everything is fine, but that many changed lines make it hard to keep
> track of everything, even though this was necessary.

> This is a tiny one, but the value for `logging.log_level_access` has been changed to all lowercase
> to match the other log level values.

##### Lookup Table ENV var -> TOML value

Here you have the lookup table. When there is a `-` somewhere, it means that either no TOML or ENV
var exists in that situation. The `TOML path` colum contains the parent table and the value name
itself. For instance, when it says

```
access.userinfo_strict
access.sec_header_block    
```

It will look like this in the `config.toml`:

```
[access]
userinfo_strict = true
sec_header_block = true
```

For a complete documentation for each value, please take a look at the reference config from the
book.

| ENV VAR                                    | TOML path                                   | type       | required |
|--------------------------------------------|---------------------------------------------|------------|----------|
| USERINFO_STRICT                            | access.userinfo_strict                      | bool       |          |
| DANGER_DISABLE_INTROSPECT_AUTH             | access.danger_disable_introspect_auth       | bool       |          |
| DISABLE_REFRESH_TOKEN_NBF                  | access.disable_refresh_token_nbf            | bool       |          |
| SEC_HEADER_BLOCK                           | access.sec_header_block                     | bool       |          |
| SESSION_VALIDATE_IP                        | access.session_validate_ip                  | bool       |          |
| PASSWORD_RESET_COOKIE_BINDING              | access.password_reset_cookie_binding        | bool       |          |
| PEER_IP_HEADER_NAME                        | access.peer_ip_header_name                  | String     |          |
| COOKIE_MODE                                | access.cookie_mode                          | String     |          |
| COOKIE_SET_PATH                            | access.cookie_set_path                      | bool       |          |
| TOKEN_LEN_LIMIT                            | access.token_len_limit                      | u32        |          |
| AUTH_HEADERS_ENABLE                        | auth_headers.enable                         | bool       |          |
| AUTH_HEADER_USER                           | auth_headers.user                           | String     |          |
| AUTH_HEADER_ROLES                          | auth_headers.roles                          | String     |          |
| AUTH_HEADER_GROUPS                         | auth_headers.groups                         | String     |          |
| AUTH_HEADER_EMAIL                          | auth_headers.email                          | String     |          |
| AUTH_HEADER_EMAIL_VERIFIED                 | auth_headers.email_verified                 | String     |          |
| AUTH_HEADER_FAMILY_NAME                    | auth_headers.family_name                    | String     |          |
| AUTH_HEADER_GIVEN_NAME                     | auth_headers.given_name                     | String     |          |
| AUTH_HEADER_MFA                            | auth_headers.mfa                            | String     |          |
| BACKCHANNEL_LOGOUT_RETRY_COUNT             | backchannel_logout.retry_count              | u16        |          |
| BACKCHANNEL_DANGER_ALLOW_HTTP              | REMOVED -> global http_client used now      |            |          |
| BACKCHANNEL_DANGER_ALLOW_INSECURE          | REMOVED -> global http_client used now      |            |          |
| LOGOUT_TOKEN_LIFETIME                      | backchannel_logout.token_lifetime           | u32        |          |
| LOGOUT_TOKEN_ALLOW_CLOCK_SKEW              | backchannel_logout.allow_clock_skew         | u32        |          |
| LOGOUT_TOKEN_ALLOWED_LIFETIME              | backchannel_logout.allowed_token_lifetime   | u32        |          |
| BOOTSTRAP_ADMIN_EMAIL                      | bootstrap.admin_email                       | String     |          |
| BOOTSTRAP_ADMIN_PASSWORD_PLAIN             | bootstrap.password_plain                    | String     |          |
| BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID          | bootstrap.pasword_argon2id                  | String     |          |
| BOOTSTRAP_API_KEY                          | bootstrap.api_key                           | String     |          |
| BOOTSTRAP_API_KEY_SECRET                   | bootstrap.api_key_secret                    | String     |          |
| HQL_NODE_ID_FROM                           | cluster.node_id_from                        | "k8s"      | x *1     |
| HQL_NODE_ID                                | cluster.node_id                             | u64        | x *1     |
| HQL_NODES                                  | cluster.nodes                               | \[String\] | x        |
| HQL_DATA_DIR                               | cluster.data_dir                            | String     |          |
| HQL_FILENAME_DB                            | cluster.filename_db                         | String     |          |
| HQL_LOG_STATEMENTS                         | cluster.log_statements                      | bool       |          |
| -                                          | cluster.prepared_statement_cache_capacity   | u16        |          |
| HQL_READ_POOL_SIZE                         | cluster.read_pool_size                      | u16        |          |
| HQL_LOG_SYNC                               | cluster.log_sync                            | String     |          |
| HQL_WAL_SIZE                               | cluster.wal_size                            | u32        |          |
| HQL_CACHE_STORAGE_DISK                     | cluster.cache_storage_disk                  | bool       |          |
| HQL_LOGS_UNTIL_SNAPSHOT                    | cluster.logs_until_snapshot                 | u64        |          |
| HQL_SHUTDOWN_DELAY_MILLS                   | cluster.shutdown_delay_millis               | u32        |          |
| HQL_TLS_RAFT_KEY                           | cluster.tls_raft_key                        | String     |          |
| HQL_TLS_RAFT_CERT                          | cluster.tls_raft_cert                       | String     |          |
| -                                          | cluster.tls_raft_danger_tls_no_verify       | bool       |          |
| HQL_TLS_API_KEY                            | cluster.tls_api_key                         | String     |          |
| HQL_TLS_RAFT_KEY                           | cluster.tls_api_cert                        | String     |          |
| -                                          | cluster.tls_api_danger_tls_no_verify        | bool       |          |
| HQL_SECRET_RAFT                            | cluster.secret_raft                         | String     | x        |
| HQL_SECRET_API                             | cluster.secret_api                          | String     | x        |
| -                                          | cluster.health_check_delay_secs             | u32        |          |
| HQL_BACKUP_CRON                            | cluster.backup_cron                         | String     |          |
| HQL_BACKUP_KEEP_DAYS                       | cluster.backup_keep_days                    | u16        |          |
| HQL_BACKUP_KEEP_DAYS_LOCAL                 | s3_url.backup_keep_days_local               | u16        |          |
| HQL_BACKUP_RESTORE                         | -                                           | String     |          |
| HQL_BACKUP_SKIP_VALIDATION                 | -                                           | bool       |          |
| HQL_S3_URL                                 | cluster.s3_url                              | String     | *2       |
| HQL_S3_BUCKET                              | cluster.s3_bucket                           | String     | *2       |
| HQL_S3_REGION                              | cluster.s3_bucket                           | String     | *2       |
| HQL_S3_PATH_STYLE                          | cluster.s3_path_style                       | bool       |          |
| HQL_S3_KEY                                 | cluster.s3_key                              | String     | *2       |
| HQL_S3_SECRET                              | cluster.s3_secret                           | String     | *2       |
| HQL_PASSWORD_DASHBOARD                     | cluster.password_dashboard                  | String     |          |
| HQL_INSECURE_COOKIE                        | cluster.insecure_cookie                     | bool       |          |
| HQL_WAL_IGNORE_LOCK                        | cluster.wal_ignore_lock                     | bool       |          |
| HQL_DANGER_RAFT_STATE_RESET                | -                                           | bool       |          |
| HIQLITE                                    | database.hiqlite                            | bool       |          |
| HEALTH_CHECK_DELAY_SECS                    | database.health_check_delay_secs            | u32        |          |
| PG_HOST                                    | database.pg_host                            | String     | *3       |
| PG_PORT                                    | database.pg_port                            | u16        |          |
| PG_USER                                    | database.pg_user                            | String     | *3       |
| PG_PASSWORD                                | database.pg_password                        | String     | *3       |
| PG_DB_NAME                                 | database.pg_db_name                         | String     |          |
| PG_TLS_NO_VERIFY                           | database.pg_tls_no_verify                   | bool       |          |
| PG_MAX_CONN                                | database.pg_max_conn                        | u16        |          |
| MIGRATE_DB_FROM                            | -                                           | String     |          |
| MIGRATE_PG_HOST                            | database.migrate_pg_host                    | String     | *4       |
| MIGRATE_PG_PORT                            | database.migrate_pg_port                    | u16        |          |
| MIGRATE_PG_USER                            | database.migrate_pg_user                    | String     | *4       |
| MIGRATE_PG_PASSWORD                        | database.migrate_pg_password                | String     | *4       |
| MIGRATE_PG_DB_NAME                         | database.migrate_pg_db_name                 | String     |          |
| SCHED_USER_EXP_MINS                        | database.sched_user_exp_mins                | u32        |          |
| SCHED_USER_EXP_DELETE_MINS                 | database.sched_user_exp_delete_mins         | u32        |          |
| DEVICE_GRANT_CODE_LIFETIME                 | device_grant.code_lifetime                  | u32        |          |
| DEVICE_GRANT_USER_CODE_LENGTH              | device_grant.user_code_length               | u32        |          |
| DEVICE_GRANT_RATE_LIMIT                    | device_grant.rate_limit                     | u32        |          |
| DEVICE_GRANT_POLL_INTERVAL                 | device_grant.poll_interval                  | u32        |          |
| DEVICE_GRANT_REFRESH_TOKEN_LIFETIME        | device_grant.refresh_token_lifetime         | u32        |          |
| DPOP_FORCE_NONCE                           | dpop.force_nonce                            | bool       |          |
| DPOP_NONCE_EXP                             | dpop.nonce_exp                              | u32        |          |
| ENABLE_DYN_CLIENT_REG                      | dynamic_clients.enable                      | bool       |          |
| DYN_CLIENT_REG_TOKEN                       | dynamic_clients.reg_token                   | String     | *5       |
| DYN_CLIENT_DEFAULT_TOKEN_LIFETIME          | dynamic_clients.default_token_lifetime      | u32        |          |
| DYN_CLIENT_SECRET_AUTO_ROTATE              | dynamic_clients.secret_auto_rotate          | bool       |          |
| DYN_CLIENT_CLEANUP_INTERVAL                | dynamic_clients.cleanup_interval            | u32        |          |
| DYN_CLIENT_CLEANUP_MINUTES                 | dynamic_clients.cleanup_minutes             | u32        |          |
| DYN_CLIENT_RATE_LIMIT_SEC                  | dynamic_clients.rate_limit_sec              | u32        |          |
| RAUTHY_ADMIN_EMAIL                         | email.rauthy_admin_email                    | String     |          |
| EMAIL_SUB_PREFIX                           | email.sub_prefix                            | String     |          |
| SMTP_URL                                   | email.smtp_url                              | String     | *6       |
| SMTP_PORT                                  | email.smtp_port                             | u16        |          |
| SMTP_USERNAME                              | email.smtp_username                         | String     |          |
| SMTP_PASSWORD                              | email.smtp_password                         | String     |          |
| SMTP_FROM                                  | email.smtp_from                             | String     |          |
| SMTP_CONNECT_RETRIES                       | email.connect_retries                       | u16        |          |
| SMTP_DANGER_INSECURE                       | email.danger_insecure                       | bool       |          |
| ENC_KEYS                                   | encryption.keys                             | \[String\] | x        |
| ENC_KEY_ACTIVE                             | encryption.key_active                       | String     | x        |
| ENABLE_EPHEMERAL_CLIENTS                   | ephemeral_clients.enable                    | bool       |          |
| ENABLE_WEB_ID                              | ephemeral_clients.enable_web_id             | bool       |          |
| ENABLE_SOLID_AUD                           | ephemeral_clients.enable_solid_aud          | bool       |          |
| EPHEMERAL_CLIENTS_FORCE_MFA                | ephemeral_clients.force_mfa                 | bool       |          |
| EPHEMERAL_CLIENTS_ALLOWED_FLOWS            | ephemeral_clients.allowed_flows             | \[String\] |          |
| EPHEMERAL_CLIENTS_ALLOWED_SCOPES           | ephemeral_clients.allowed_scopes            | \[String\] |          |
| EPHEMERAL_CLIENTS_CACHE_LIFETIME           | ephemeral_clients.cache_lifetime            | u32        |          |
| EVENT_EMAIL                                | events.email                                | String     |          |
| EVENT_MATRIX_USER_ID                       | events.matrix_user_id                       | String     |          |
| EVENT_MATRIX_ROOM_ID                       | events.matrix_room_id                       | String     |          |
| EVENT_MATRIX_ACCESS_TOKEN                  | events.matrix_access_token                  | String     |          |
| EVENT_MATRIX_USER_PASSWORD                 | events.matrix_user_password                 | String     |          |
| EVENT_MATRIX_SERVER_URL                    | events.matrix_server_url                    | String     |          |
| EVENT_MATRIX_ROOT_CA_PATH                  | events.matrix_root_ca_path                  | String     |          |
| EVENT_MATRIX_DANGER_DISABLE_TLS_VALIDATION | events.matrix_danger_disable_tls_validation | bool       |          |
| EVENT_MATRIX_ERROR_NO_PANIC                | events.matrix_error_no_panic                | bool       |          |
| EVENT_SLACK_WEBHOOK                        | events.slack_webhook                        | String     |          |
| EVENT_NOTIFY_LEVEL_EMAIL                   | events.notify_level_email                   | Level      |          |
| EVENT_NOTIFY_LEVEL_MATRIX                  | events.notify_level_matrix                  | Level      |          |
| EVENT_NOTIFY_LEVEL_SLACK                   | events.notify_level_slack                   | Level      |          |
| EVENT_PERSIST_LEVEL                        | events.persist_level                        | Level      |          |
| EVENT_CLEANUP_DAYS                         | events.cleanup_days                         | u32        |          |
| EVENT_LEVEL_NEW_USER                       | events.level_new_user                       | Level      |          |
| EVENT_LEVEL_USER_EMAIL_CHANGE              | events.level_user_email_change              | Level      |          |
| EVENT_LEVEL_USER_PASSWORD_RESET            | events.level_user_password_reset            | Level      |          |
| EVENT_LEVEL_RAUTHY_ADMIN                   | events.level_rauthy_admin                   | Level      |          |
| EVENT_LEVEL_RAUTHY_VERSION                 | events.level_rauthy_version                 | Level      |          |
| EVENT_LEVEL_JWKS_ROTATE                    | events.level_jwks_rotate                    | Level      |          |
| EVENT_LEVEL_SECRETS_MIGRATED               | events.level_secrets_migrated               | Level      |          |
| EVENT_LEVEL_RAUTHY_START                   | events.level_rauthy_start                   | Level      |          |
| EVENT_LEVEL_RAUTHY_HEALTHY                 | events.level_rauthy_healthy                 | Level      |          |
| EVENT_LEVEL_RAUTHY_UNHEALTHY               | events.level_rauthy_unhealthy               | Level      |          |
| EVENT_LEVEL_IP_BLACKLISTED                 | events.level_ip_blacklisted                 | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_25               | events.level_failed_logins_25               | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_20               | events.level_failed_logins_20               | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_15               | events.level_failed_logins_15               | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_10               | events.level_failed_logins_10               | Level      |          |
| EVENT_LEVEL_FAILED_LOGINS_7                | events.level_failed_logins_7                | Level      |          |
| EVENT_LEVEL_FAILED_LOGIN                   | events.level_failed_login                   | Level      |          |
| DISABLE_APP_VERSION_CHECK                  | events.disable_app_version_check            | bool       |          |
| EXPERIMENTAL_FED_CM_ENABLE                 | fedcm.experimental_enable                   | bool       |          |
| SESSION_LIFETIME_FED_CM                    | fedcm.session_lifetime                      | u32        |          |
| SESSION_TIMEOUT_FED_CM                     | fedcm.session_timeout                       | u32        |          |
| ARGON2_M_COST                              | hashing.argon2_m_cost                       | u32        |          |
| ARGON2_T_COST                              | hashing.argon2_t_cost                       | u32        |          |
| ARGON2_P_COST                              | hashing.argon2_p_cost                       | u32        |          |
| MAX_HASH_THREADS                           | hashing.max_hash_threads                    | u32        |          |
| HASH_AWAIT_WARN_TIME                       | hashing.hash_await_warn_time                | u32        |          |
| JWK_AUTOROTATE_CRON                        | hashing.jwk_autorotate_cron                 | String     |          |
| HTTP_CONNECT_TIMEOUT                       | http_client.connect_timeout                 | u32        |          |
| HTTP_REQUEST_TIMEOUT                       | http_client.request_timeout                 | u32        |          |
| HTTP_MIN_TLS                               | http_client.min_tls                         | String     |          |
| HTTP_IDLE_TIMEOUT                          | http_client.idle_timeout                    | u32        |          |
| HTTP_DANGER_UNENCRYPTED                    | http_client.danger_unencrypted              | bool       |          |
| HTTP_DANGER_INSECURE                       | http_client.danger_insecure                 | bool       |          |
| HTTP_CUST_ROOT_CA_BUNDLE                   | http_client.root_ca_bundle                  | String     |          |
| FILTER_LANG_COMMON                         | i18n.filter_lang_common                     | \[String\] |          |
| FILTER_LANG_ADMIN                          | i18n.filter_lang_admin                      | \[String\] |          |
| REFRESH_TOKEN_GRACE_TIME                   | lifetimes.refresh_token_grace_time          | u16        |          |
| REFRESH_TOKEN_LIFETIME                     | lifetimes.refresh_token_lifetime            | u16        |          |
| SESSION_LIFETIME                           | lifetimes.session_lifetime                  | u32        |          |
| SESSION_RENEW_MFA                          | lifetimes.session_renew_mfa                 | bool       |          |
| SESSION_TIMEOUT                            | lifetimes.session_timeout                   | u32        |          |
| ML_LT_PWD_RESET                            | lifetimes.magic_link_pwd_reset              | u32        |          |
| ML_LT_PWD_FIRST                            | lifetimes.magic_link_pwd_first              | u32        |          |
| LOG_LEVEL                                  | logging.level                               | Level      |          |
| LOG_LEVEL_DATABASE                         | logging.level_database                      | Level      |          |
| LOG_LEVEL_ACCESS                           | logging.level_access                        | String     |          |
| LOG_FMT                                    | logging.log_fmt                             | "json"     |          |
| ADMIN_FORCE_MFA                            | mfa.admin_force_mfa                         | bool       |          |
| POW_DIFFICULTY                             | pow.difficulty                              | u16        |          |
| POW_EXP                                    | pow.exp                                     | u16        |          |
| SCIM_SYNC_DELETE_GROUPS                    | scim.sync_delete_groups                     | bool       |          |
| SCIM_SYNC_DELETE_USERS                     | scim.sync_delete_users                      | bool       |          |
| SCIM_RETRY_COUNT                           | scim.retry_count                            | u16        |          |
| LISTEN_ADDRESS                             | server.listen_address                       | String     |          |
| LISTEN_PORT_HTTP                           | server.port_http                            | u16        |          |
| LISTEN_PORT_HTTPS                          | server.port_https                           | u16        |          |
| LISTEN_SCHEME                              | server.scheme                               | String     |          |
| PUB_URL                                    | server.pub_url                              | String     | x        |
| HTTP_WORKERS                               | server.http_workers                         | u16        |          |
| PROXY_MODE                                 | server.proxy_mode                           | bool       | *7       |
| TRUSTED_PROXIES                            | server.trusted_proxies                      | \[String\] | *7       |
| ADDITIONAL_ALLOWED_ORIGIN_SCHEMES          | server.additional_allowed_origin_schemes    | \[String\] |          |
| METRICS_ENABLE                             | server.metrics_enable                       | bool       |          |
| METRICS_ADDR                               | server.metrics_addr                         | String     |          |
| METRICS_PORT                               | server.metrics_port                         | u16        |          |
| SWAGGER_UI_ENABLE                          | server.swagger_ui_enable                    | bool       |          |
| SWAGGER_UI_PUBLIC                          | server.swagger_ui_public                    | bool       |          |
| SSE_KEEP_ALIVE                             | server.see_keep_alive                       | u16        |          |
| SSP_THRESHOLD                              | server.ssp_threshold                        | u16        |          |
| SUSPICIOUS_REQUESTS_BLACKLIST              | suspicious_requests.blacklist               | u16        |          |
| SUSPICIOUS_REQUESTS_LOG                    | suspicious_requests.log                     | bool       |          |
| -                                          | \[templates\].lang                          | String     | *8       |
| -                                          | \[templates\].typ                           | String     | *8       |
| -                                          | \[templates\].subject                       | String     |          |
| -                                          | \[templates\].header                        | String     |          |
| -                                          | \[templates\].text                          | String     |          |
| -                                          | \[templates\].click_link                    | String     |          |
| -                                          | \[templates\].validity                      | String     |          |
| -                                          | \[templates\].expires                       | String     |          |
| -                                          | \[templates\].footer                        | String     |          |
| TLS_CERT                                   | tls.cert_path                               | String     |          |
| TLS_KEY                                    | tls.key_path                                | String     |          |
| PICTURE_STORAGE_TYPE                       | user_pictures.storage_type                  | String     |          |
| PICTURE_PATH                               | user_pictures.path                          | String     |          |
| PIC_S3_URL                                 | user_pictures.s3_url                        | String     | *2       |
| PIC_S3_BUCKET                              | user_pictures.bucket                        | String     | *2       |
| PIC_S3_REGION                              | user_pictures.region                        | String     | *2       |
| PIC_S3_KEY                                 | user_pictures.s3_key                        | String     | *2       |
| PIC_S3_SECRET                              | user_pictures.s3_secret                     | String     | *2       |
| PIC_S3_PATH_STYLE                          | user_pictures.s3_path_style                 | bool       |          |
| PICTURE_UPLOAD_LIMIT_MB                    | user_pictures.upload_limit_mb               | u16        |          |
| PICTURE_PUBLIC                             | user_pictures.public                        | bool       |          |
| OPEN_USER_REG                              | user_registration.enable                    | bool       |          |
| USER_REG_DOMAIN_RESTRICTION                | user_registration.domain_restriction        | String     |          |
| USER_REG_DOMAIN_BLACKLIST                  | user_registration.domain_blacklist          | \[String\] |          |
| USER_REG_OPEN_REDIRECT                     | user_registration.allow_open_redirect       | bool       |          |
| RP_ID                                      | webauthn.rp_id                              | String     | x        |
| RP_ORIGIN                                  | webauthn.rp_origin                          | String     | x        |
| RP_NAME                                    | webauthn.rp_name                            | String     |          |
| WEBAUTHN_REQ_EXP                           | webauthn.req_exp                            | u16        |          |
| WEBAUTHN_DATA_EXP                          | webauthn.data_exp                           | u16        |          |
| WEBAUTHN_RENEW_EXP                         | webauthn.renew_exp                          | u16        |          |
| WEBAUTHN_FORCE_UV                          | webauthn.force_uv                           | bool       |          |
| WEBAUTHN_NO_PASSWORD_EXPIRY                | webauthn.no_password_exp                    | bool       |          |

1. At least one of `cluster.node_id_from` / `cluster.node_id` is required
2. When `s3_url` is given, the other `s3_*` values are expected as well
3. Required when `database.hiqlite = false`
4. Required when `MIGRATE_DB_FROM=postgres`
5. Not strictly required but should probably almost be set when `dynamic_clients.enable = true` to
   not have an open dyn client registration.
6. When not set, E-Mail cannot be sent and things like user registration and self-service password
   requests will not work. You can operate Rauthy without this setting, but then an Admin needs to
   perform all these actions.
7. Required when running behind a reverse proxy
8. The `[templates]` block can be given multiple times for different languages / templates, but if
   so, `lang` + `typ`    are required inside.

> NOTE: All `\[String\]` types are Arrays inside the TOML, but a single String value for an ENV VAR,
> which separates the values by `\n`.

> All `Level` values can be one of: 'info', 'notice', 'warning', 'critical'

> Quite a few of these values, even when they are a `String` type, expect a certain format. Take a
> look at the reference config for more information on each one.

[#991](https://github.com/sebadob/rauthy/pull/991)

#### SwaggerUI Location Change

The way the SwaggerUI used to work was annoying since the very beginning. Even though it's an open
source project, you usually don't want to have your whole API documentation publicly available. The
issue was, that it was hard to work with the SwaggerUI because of some internals that made it
impossible to have much manual control. For this reason, up until `v0.29`, you had the option to
expose it internally only in addition to metrics on the internal HTTP server. This was very annoying
as well, just in another way.

After memory profiling and optimizations lately, I found out that the SwaggerUI internal config has
additional issues. It does a deep clone of the complete dataset instead of using pointers for each
additional HTTP worker, which led to very high memory consumption by default on machines with many
cores.

I did a complete rework of the way how it's being served and luckily, in the newer versions you can
get manual handles to the internal data, which finally made it possible to move it out of its
location. Instead of being available under `/docs/v1`, it's now under `/auth/v1/docs`, which makes a
lot more sense in the first place. Because it's served manually, a single instance will be kept in
static memory instead of cloning the data into each worker. This improves memory usage a lot
already. The other improvement this made possible, is that Rauthy can finally check the session and
(optionally) only serve it when a valid `rauthy_admin` session was found.

This change made it possible to always have it on the same port while still being able to serve all
the different use cases from before. You can enable or disable it, and you can decide whether it
should be public or not. No weird "internal only" stuff anymore. The config variables are still in
the server section and the non-public version is now fully independent of the metrics server.

```toml
[server]
# Can be set to `true` to enable the Swagger UI.
# This will consume ~13mb of additional memory.
#
# default: false
# overwritten by: SWAGGER_UI_ENABLE
swagger_ui_enable = false

# Can be set to `true` to make the Swagger UI publicly
# available. By default, you can only access it with a
# valid `rauthy_admin` session.
#
# default: false
# overwritten by: SWAGGER_UI_PUBLIC
swagger_ui_public = false
```

[#981](https://github.com/sebadob/rauthy/pull/981)

#### Type change for `zip` / `postcal_code`

The type for `zip` / `postcal_code` has been changed from an Integer to a String in all locations.
This means not only the Admin + Account Dashboards, but of course also inside the `address` claim
for `id_token`s. This change brings compatibility for countries that use non-numeric postal codes.

[#1002](https://github.com/sebadob/rauthy/pull/1002)

### Changes

#### Hiqlite Optimizations

Hiqlite has received lots of optimizations and its version has been integrated in this Rauthy
version. The updates were mainly about stability, especially during cluster rolling-releases in
environments like Kubernetes, where the versions before had some issues with the ephemeral,
in-memory Cache Raft state, which could get into a deadlock during a race condition. This has been
fixed completely.

Another thing is that it's now possible to optionally have the Cache Raft state like WAL + Snapshots
on disk instead of in-memory. The on-disk solution is of course quite a bit slower, because it's
limited by your disk speed, but the memory usage will be lower as well. If all of that data is kept
in-memory, you basically need 3 times the size of a single cache value (+1 in WAL logs and +1 in
Snapshot, kind of, depending on TTL of course). Writing the state to disk has other advantages as
well. The cache can be rebuilt and you never lose cached data even if the whole cluster went down.
On restart, the whole in-memory cache layer can be rebuilt from WAL + Snapshots.

The next improvement is that `hiqlite` received a fully custom WAL implementation and we can drop
`rocksdb` as the WAL store. `rocksdb` is really good and very fast, but a huge overkill for the job,
a very heavyweight dependency and brings quite a few issues in regard to compilation targets. This
feature version still contains code and dependencies to migrate existing `rocksdb` Log Stores to the
new `hiqlite-wal` store, but from the next feature version, we can fully remove the dependency. Just
removing `rocksdb` will reduce the release binary size by ~7MB. On top of that, `hiqlite-wal` is
quite a bit more efficient with your memory at runtime without any sacrifices in throughput.

The new `hiqlite-wal` now also has some mechanisms to try to repair a WAL file and recover records
that might have been corrupted because of a bad crash for instance. `rocksdb` could get into a state
where it would be almost impossible to recover. If you run your Rauthy instance in a container
environment, or anywhere where you can guarantee, that no 2nd Rauthy process might try to access the
same data on disk, you probably want to set `HQL_IGNORE_WAL_LOCK=true` now, which will start and try
the repair routine, even if it detects a non-graceful shutdown. It is crucial though that it can
never happen, that another instance is still running accessing the same data. That's what this
warning is for in such a scenario. If you rather have full control and double check in case of an
error, leave this value unset. The default is `false`.
The other new value is `HQL_CACHE_STORAGE_DISK`, which is `true` by default. This will store the
WAL + Snapshots for the in-memory cache on disk and therefore free up that memory. If you would
rather have everything in-memory though, set this value to `false`.

```toml
[cluster]
# Hiqlite WAL files (when not using the `rocksdb` feature) will
# always have a fixed size, even when they are still "empty", to
# reduce disk operations while writing. You can set the WAL size
# in bytes. The default value is 2 MB, while the minimum size is
# 8 kB.
#
# default: 2097152
# overwritten by: HQL_WAL_SIZE
#wal_size = 2097152

# Set to `false` to store Cache WAL files + Snapshots in-memory only.
# If you run a Cluster, a Node can re-sync cache data after a restart.
# However, if you restart too quickly or shut down the whole cluster,
# all your cached data will be gone.
# In-memory only hugegly increases the throughput though, so it
# depends on your needs, what you should prefer.
#
# default: true
# overwritten by: HQL_CACHE_STORAGE_DISK
cache_storage_disk = true

# Can be set to true to start the WAL handler even if the
# `lock.hql` file exists. This may be necessary after a
# crash, when the lock file could not be removed during a
# graceful shutdown.
#
# IMPORTANT: Even though the Database can "heal" itself by
# simply rebuilding from the existing Raft log files without
# even needing to think about it, you may want to only
# set `HQL_WAL_IGNORE_LOCK` when necessary to have more
# control. Depending on the type of crash (whole OS, maybe
# immediate power loss, force killed, ...), it may be the
# case that the WAL files + metadata could not be synced
# to disk properly and that quite a bit of data is lost.
#
# In such a case, it is usually a better idea to delete
# the whole volume and let the broken node rebuild from
# other healthy cluster members, just to be sure.
#
# However, you can decide to ignore the lock file and start
# anyway. But you must be 100% sure, that no orphaned
# process is still running and accessing the WAL files!
#
# default: false
# overwritten by: HQL_WAL_IGNORE_LOCK
wal_ignore_lock = false
```

And if all of this was not enough yet, I was able to improve the overall throughput of `hiqlite`
by ~30%. For more information, take at the [Hiqlite Repo](https://github.com/sebadob/hiqlite)
directly.

> If you are currently running a HA Cluster with Hiqlite as your database, you should make sure you
> have a backup before
> starting this version, just in case, because of the Log Store Migration. It would be even more
> safe if you can afford
> to shut down the whole cluster cleanly first and then restart from scratch.

#### Default for `HTTP_WORKERS` changed

The default value for `HTTP_WORKERS` has been changed. Even though you probably almost always want
to set this value manually in production, especially when running your instance on a huge underlying
host, the default has been tuned to fit smaller hosts a bit better. Here is the new description in
the config:

```toml
[server]
# Limits the amount of HTTP worker threads. This value
# heavily impacts memory usage, even in idle. The default
# values are:
# - less than 4 CPU cores -> 1
# - 4+ cores -> max(2, cores - MAX_HASH_THREADS - reserve)
#   where `reserve` is 2 when `HIQLITE=true` and 1 otherwise.
#
# CAUTION: If you run your instance on a big underlying host,
# you almost always want to manually set an appropriate
# value. Rauthy can only see all available cores and not any
# possibly set container limits. This means if it runs inside
# a container on something like a 96 core host, Rauthy will
# by default spawn very many threads.
#
# overwritten by: HTTP_WORKERS
http_workers = 1
```

[#975](https://github.com/sebadob/rauthy/pull/975)

#### Early `panic` in case of misconfiguration

Most lazy static config variables are now being triggered at the very start of the application. This
brings two benefits:

1. If you have any issues in your config, it will panic and error early, instead of "some time
   later" when this value is used for the first time.
2. It will make the start of the Heap way more compact, because all values will align perfectly, and
   therefore reduce memory fragmentation a little bit.

[#969](https://github.com/sebadob/rauthy/pull/969)

#### Restrict client login by group prefix

In addition to the already existing "Force MFA" switch for each client, which will fore users to
have MFA enabled, you can now also restrict the login to each client by an additional group prefix,
just like it has been added for SCIM syncs recently.

For instance, if you have groups like `app:admin` and `app:user`, you can restrict the client with
the prefix `app:`, which will only allow users assigned to one of these groups to do the login.

Usually, such decisions are done on the client side, depending on the claims in the tokens, because
it's a lot more powerful and roles can be assigned properly, and so on. However, not all client apps
support claim restrictions. In these cases, you can now do it on Rauthys side, or maybe just in
addition to provide a better UX, because the user will get the error message during Rauthys login
already and not after the token was exchanged with the client.

[#952](https://github.com/sebadob/rauthy/pull/952)

#### Less strict group names

Group names can now contain uppercase letters and spaces.

[#1012](https://github.com/sebadob/rauthy/pull/1012)

#### User-Editable custom attributes

User attributes can now be set to be user-editable. A new section has been added to the account
dashboard which shows these user-editable attributes (if any exist), and the user can modify them
without the need of an admin.

CAUTION: Setting an attribute to be user-editable is a one-way operation and cannot be reverted! The
reason is, that user inputs are always unverified and cannot be trusted, and these attributes MUST
NEVER be used in any context of authn / authz on clients. To avoid the possibility of leaking,
unvalidated inputs, once an attribute was user-editable, it can never be changed back, because you
cannot trust the existing values at that point. This should act as a safety-net for admins.

[#964](https://github.com/sebadob/rauthy/pull/964)

#### Default values for custom attributes

You can now define a default value for each custom attribute. If the value is not set for a user,
the default will be added to token claims. This makes it possible to set things like default storage
quotas for all users via a custom attribute, and still allows to overwrite them. It is probably the
most powerful, when you can so modify many users at once, because it resolves dynamically during
token creation and does not need an update for each single user.

[#958](https://github.com/sebadob/rauthy/pull/958)

#### User account type in Admin UI

The user account type is now also being shown in the Admin UI, just like it is in the account
dashboard. With this update, Admins can see which accounts are federated and to which upstream auth
provider they are linked.

[#960](https://github.com/sebadob/rauthy/pull/960)

#### Custom JWT implementation

Do be more flexible and more efficient, the external dependency for generating JWT tokens has been
dropped in favor of a new, fully custom implementation. This is a lot more light weight, more
efficient, and makes Rauthy independent for things like PQC algorithms / FIPS 204 in the future.
This change alone dropped 21 external dependencies in exchange for only a few hundred lines of code
in comparison to the old setup.

To avoid possible resource exhaustion attacks on the token introspection and userinfo endpoints, the
token size (in characters) is now limited to 4096. This is more than double the amount of an RS512
signed `id_token` with more than only the default values, so you should never have any problems
reaching the limit. Theoretically, it is of course possible, so you get a new config variable to
tune this:

```toml
[access]
# Sets the limit in characters for the maximum JWT token length that
# will be accepted when validating it. The default of 4096 is high
# enough that you should never worry about this value. A typical
# `id_token` with quite a few additional custom attributes and scopes,
# signed with RS512, will usually be below 2000 characters.
#
# Only if you create very big tokens and you get errors on the
# `/userinfo` for instance, you might want to increase this value.
# Otherwise, don't worry about it.
#
# default: 4096
# overwritten by: TOKEN_LEN_LIMIT
token_len_limit = 4096
```

[#941](https://github.com/sebadob/rauthy/pull/941)

#### IP Blacklisting Rework

The IP blacklisting has been reworked. This was done via listen / notify and an instance-local
thread, that manages all blacklisted IPs purely in memory. We usually do not want to trigger any DB
writes in such a case to keep the work as low a possible in case of a DDoS for instance.

However, this old design was from a time when Hiqlite did not even exist yet. The latest version
brings distributed counters, which made it possible to easily push blacklisted IPs into the Hiqlite
caching layer. In combination with another new feature in the latest Hiqlite, which is an optionally
persistent WAL + Snapshot on disk for the Cache as well (enabled by default), you can now have
blacklisted IPs in memory for very fast access, while still being able to persist them in a way,
that the full in-memory cache can be rebuilt after a restart.

Pushing this logic into Hiqlite removes a maintenance burden from the Rauthy code. It exists inside
Hiqlite anyway already.

[#985](https://github.com/sebadob/rauthy/pull/985)

#### `PATCH` is possible on `/users/{id}`

The user modification endpoint now provides a `PATCH` operation, which the Admin UI user config uses
by default. It will only update the values, that actually have been changed in the UI. This reduces
the possibility of overwriting values that have been modified by the user via the account dashboard
in the exact same moment.

[#951](https://github.com/sebadob/rauthy/pull/951)

#### `jemalloc` feature flag

Rauthy can optionally be compiled with the `jemalloc` feature flag, which will exchange the glibc
`malloc` for `jemalloc`. It avoids memory fragmentation over time and is a lot more performant. You
can also adjust it to match your workloads and the default tuning will probably be aimed at being
efficient. However, if you run a Rauthy instance with thousands or even millions of users, you can
custom-compile a version with optimized tuning, which will use more memory, but handle this many
concurrent allocations better. Take a look at the book for tuning advise.

The `jemalloc` feature flag will be enabled by default from this version on.

[#949](https://github.com/sebadob/rauthy/pull/949)

#### Rauthy theme as fallback

The custom theme you apply for the `rauthy` client will now be used as a fallback for all other
clients, when they don't have their own custom values.

[#982](https://github.com/sebadob/rauthy/pull/982)

#### Dedicated Password Reset Request Page

A link to a dedicated password reset request page has been added to password reset E-Mails. This
helps users request a new magic link quickly when they have an expired one for a password reset in
their inbox.

[#1011](https://github.com/sebadob/rauthy/pull/1011)

#### Production Init Hardening

To harden the production initialization of the database for a fresh instance a bit more, the user ID
for the very first initial admin account is being re-generated randomly. In combination with
`bootstrap.admin_email`, this makes it now 100% impossible to guess any value for a possibly
existing default admin for any Rauthy instance.

[#983](https://github.com/sebadob/rauthy/pull/983)

#### Memory optimizations

All `derive` impl's on all API types have been checked and quite a lot of unnecessary `derive`s have
been removed (were e.g. only necessary during development / testing). This is a small optimization
regarding release compile-time and binary size. Additionally, lots of other small improvements have
been made all over the code to reduce the number of overall memory allocations in general.

[#956](https://github.com/sebadob/rauthy/pull/956)
[#968](https://github.com/sebadob/rauthy/pull/968)

### Bugfix

- The legacy Rauthy placeholder logo was not removed from the `hiqlite` DB migrations for fresh
  instances
  [#942](https://github.com/sebadob/rauthy/pull/942)
- Some mobile browsers started failing recently because of some issue with the PKCE challenge
  generation. The external dependency for this has been dropped in favor of a custom implementation.
  [#953](https://github.com/sebadob/rauthy/pull/953)
- `user.federation_uid` was saved as an escaped JSON string inside the DB instead of as a raw String
  directly. This was not an issue on its own, because during login, both the escaped values were
  compared, but it didn't look good.
  [#962](https://github.com/sebadob/rauthy/pull/962)
- The default value for the `SMTP_FROM` was changed from `rauthy@localhost.de` to just
  `rauthy@localhost`.
  [#963](https://github.com/sebadob/rauthy/pull/963)
- The `state` encoding during `/authorize` was broken in some situations, because the urlencoding
  was removed in some situations, when this param should be sent back to the client unchanged.
  [#980](https://github.com/sebadob/rauthy/pull/980)

## v0.29.4

### Changes

#### Custom Attributes via SCIM

Rauthy's SCIM functionality now also adds custom user attributes. These will be added, just like
inside the `id_token` claims, under `custom: *` as JSON values. These will be added only if the
client should receive these attributes defined via the allowed `scope`s.

This is a non-RFC value, but it should be ignored by any client, that does not understand the
`custom` field. While the `rauthy-client` has been released with a new version in the meantime, its
implementation gives you access to custom attributes, if some exist.

[#936](https://github.com/sebadob/rauthy/pull/936)

### Bugfix

- It was possible, that Rauthy tried to sync groups via SCIM even if group syncing has been
  disabled.
  [#936](https://github.com/sebadob/rauthy/pull/936)
- The legacy Rauthy logo showed up at the login page in versions `0.29.0` - `0.29.3` for fresh
  Hiqlite deployments. This was due to a missing, permanent database migration that removes the old
  logo. This was shadowed by the manual version migration queries that existed during the `0.28`
  release. This patch version has a programmatic query and will add a permanent migration with the
  `0.30` release to not introduce breaking changes with a patch
  level.
  [#943](https://github.com/sebadob/rauthy/pull/943)

## v0.29.3

### Bugfix

- `v0.29.2`s fix for the HTTP 302 instead of 200 on `/logout` broke the logout from the UI without
  additional
  `id_token_hint`.
  [#930](https://github.com/sebadob/rauthy/pull/930)
- Form validation for Upstream Auth Provider config did not set some fields to required under
  certain conditions and would therefore return the backend validation error instead of a nicer way
  of handling it.
  [#931](https://github.com/sebadob/rauthy/pull/931)
- Fixed another DB migration query between Hiqlite and Postgres.
  [#932](https://github.com/sebadob/rauthy/pull/932)

## v0.29.2

### Bugfix

- DB Migrations between Hiqlite and Postgres have not been working as expected since database driver
  and query reworks with `v0.29`.
  [#919](https://github.com/sebadob/rauthy/pull/919)
- Front-Channel Logout returned HTTP 200 instead of 302 for auto-redirect under certain conditions.
  [#920](https://github.com/sebadob/rauthy/pull/920)

## v0.29.1

### Bugfix

- Because of the default admin renaming with `0.29.0`, the 2 test admin accounts have not been
  deleted when starting up a new production instance.
  [#909](https://github.com/sebadob/rauthy/pull/909)
- Error logs with an invalid `rauthy.cfg` have been emitted as tracing logs instead of `eprintln`,
  when logging was not initialized yet.
  [#910](https://github.com/sebadob/rauthy/pull/910)
- Because of the new session validation rework and additional safety-nets, a login was impossible
  when
  `SESSION_VALIDATE_IP=false`
  [#911](https://github.com/sebadob/rauthy/pull/911)

## v0.29.0

### Breaking

#### Postgres: `sqlx` dropped in favor of `tokio-postgres`

Internally, quite a big has happened. `sqlx` has been completely dropped in favor of
`tokio-postgres` in combination with some other dependencies. This solved a few issues.

`sqlx` has been blocking updates quite a few times in the past. It pulls in a huge amount of
dependencies, even if they are not used, which often block other crates. It's also pretty slow with
updates, which means you might be forced to stick with older versions for several months, and it has
some other limitations I don't like.

Even though the compile-time checked queries are the best feature of `sqlx`, they also produced a
lot of issues and confusion when other people were trying to build from source, and so on.

The bonus for everyone not working with the code directly is, that `tokio-postgres` has about half
the latency of `sqlx` in my testing.

The breaking changes that comes with this internal change is actually a good one:

You don't specify the `DATABASE_URL` like before. `DATABASE_URL` and `DATABASE_MAX_CONN` have been
completely removed. Instead, you now have separate config vars for each part of the URL. This
removes the limitation that you could only have alphanumeric characters inside your password (
otherwise the URL would be invalid).

One additional thing is, that `sqlx` silently ignored TLS certificate validation if it failed, which
is actually a pretty bad behavior. If you are running a Postgres with self-signed certificates, you
need to explicitly allow this.

You now need to provide the following values, if you are using Postgres:

```
# If you set `HIQLITE=false` and want to use Postgres as your database,
# you need to set the following variables:
PG_HOST=
# default: 5432
#PG_PORT=5432
PG_USER=
PG_PASSWORD=
# default: rauthy
#PG_DB_NAME=rauthy

# If your database uses a self-signed certificate, which cannot
# be verified, you might want to set this to `true`.
# default: false
#PG_TLS_NO_VERIFY=false

# Max DB connections for the Postgres pool.
# default: 20
#PG_MAX_CONN=20
```

**CAUTION:** This is a pretty big change, because existing `sqlx` migrations will be converted
manually. Just to be safe, you should probably do a database backup before starting `v0.29`!

[#822](https://github.com/sebadob/rauthy/pull/822)

#### Default Admin change

The default admin has been changed from `admin@localhost.de` to just `admin@localhost`. This is due
to the fact that `localhost.de` is a real domain with a valid TLD and it could introduce issues, if
these users exist on a Rauthy instance.

If you still had an `admin@localhost.de` around in your database, this will also be changed even for
an existing deployment! You will see a `warn!` logging during the whole `v0.29` release to make you
aware of this. After the new version has been started up, your default admin will be
`admin@localhost`.

[#866](https://github.com/sebadob/rauthy/pull/866)

#### Additional PoW requirements

If you have been providing your own, custom login UI, be aware of the fact that the `LoginRequest` +
`ProviderLoginRequest` now require a solved PoW inside the request body. For further information
about this, see the notes down below -> *Defense in depth*;

[#883](https://github.com/sebadob/rauthy/pull/883)

#### Internal Metrics now opt-in

The internal `/metrics` HTTP server is not used by most people, but it has been enabled by default,
while using ~10mb of memory in idle and adding the overhead to each API request that metrics are
actually collected internally. For this reason, the internal metrics (and therefore also internal
Swagger UI) are not opt-in instead of opt-out. If you are using prometheus metrics right now, make
sure that you explicitly set

```
METRICS_ENABLE=true
```

[#876](https://github.com/sebadob/rauthy/pull/876)

#### Removed Custom CA from Auth Providers

The custom root CA option has been removed from Upstream Auth Providers. This was due to performance
and efficiency optimizations. There is a new global HTTP client (mentioned below) which will handle
custom root CAs and will be used for auth providers as well.

If you are currently using an upstream auth provider with a custom root CA, make sure to add this CA
to `HTTP_CUST_ROOT_CA_BUNDLE` as mentioned further down below`

### Changes

#### OpenID Connect Backchannel Logout

Rauthy now
supports [OpenID Connect Backchannel Logout](https://openid.net/specs/openid-connect-backchannel-1_0.html).

If downstream clients support it, you can now provide a `backchannel_logout_uri` in the config view
via the Admin UI. Rauthy itself can act as a downstream client for an upstream as well. The default
`/auth/v1/oidc/logout` endpoint now optionally accepts the `logout_token` and will propagate
Backchannel Logouts, if the token is actually coming from a valid, configured upstream auth
provider.  
Backchannel Logouts will of course also be triggered in other situations like forced session
invalidation by an admin, user expiration, or deletion, and so on.

The feature can be seen as in beta state now. Everything has been implemented following the RFC and
basic integration tests exist as well. However, to catch some weird edge cases, it needs testing in
the real world as well. If you have any problems using it, please open an issue about it.

There are some new config variables as well:

```
#####################################
######## BACKCHANNEL LOGOUT #########
#####################################

# The maximum amount of retries made for a failed backchannel logout.
# Failed backchannel logouts will be retried every 60 - 90 seconds
# from all cluster nodes. The timeout between retries is randomized
# to avoid overloading clients. It will be executed on each cluster
# member to increase the chance of a successful logout in case of
# network segmentations.
# default: 100
#BACKCHANNEL_LOGOUT_RETRY_COUNT=100

# Can be set to `true` to allow plain HTTP backchannel
# logout requests.
# default: false
#BACKCHANNEL_DANGER_ALLOW_HTTP=false

# Can be set to `true` to allow insecure HTTPS logout requests.
# This will disable TLS certificate validation.
# default: false
#BACKCHANNEL_DANGER_ALLOW_INSECURE=false

# The lifetime / validity for Logout Tokens in seconds.
# These Logout Tokens are being generated during OIDC
# Backchannel Logout requests to configured clients.
# The token lifetime should be as short as possible and
# at most 120 seconds.
# default: 30
#LOGOUT_TOKEN_LIFETIME=30

# You can allow a clock skew during the validation of
# Logout Tokens, when Rauthy is being used as a client
# for an upstream auth provider that uses backchannel
# logout.
#
# The allowed skew will be in seconds and a value of
# e.g. 5 would mean, that 5 seconds are added to the
# `iat` and `exp` claim validations and expand the range.
#
# default: 5
#LOGOUT_TOKEN_ALLOW_CLOCK_SKEW=5

# The maximum allowed lifetime for Logout Tokens.
# This value is a security check for upstream auth
# providers. If Rauthy receives a Logout Token, it will
# check and validate, that the difference between `iat`
# and `exp` is not greater than LOGOUT_TOKEN_ALLOWED_LIFETIME.
# This means Rauthy will reject Logout Tokens from clients
# with a way too long validity and therefore poor
# implementations. The RFC states that tokens should
# be valid for at most 120 seconds.
# default: 120
#LOGOUT_TOKEN_ALLOWED_LIFETIME=120
```

[#794](https://github.com/sebadob/rauthy/pull/794)
[#819](https://github.com/sebadob/rauthy/pull/819)

#### RP Initiated Logout

While implementing OpenID Connect Backchannel Logout, support
for [RP Initiated Logout](https://openid.net/specs/openid-connect-rpinitiated-1_0.html) has been
added as well to provide a complete package.

[#794](https://github.com/sebadob/rauthy/pull/794)
[#806](https://github.com/sebadob/rauthy/pull/806)

#### SCIM

This version brings SCIM support to Rauthy.

You can define a custom SCIM base uri per client. The client must support SCIM v2 and the `filter`
parameter. Groups syncing is optional.

You can also define a group prefix which will act as a filter for the to-be-synced data.
For instance, if you define a prefix of `myapp:`, only the groups starting with it will be synced,
like e.g. `myapp:admin`, `myapp:user`. Additionally, only users that are assigned to at least one of
these filtered groups will by synced, while others are skipped.   
The advantage of this is, let's say you have an application with limited seats, or where each
existing user costs a license (Gitlab Enterprise would be an example), you need to pay for each
registered user. If you now split your users into groups and only your developers need to exist on
that instance, you can assign a group like `dev:*` and only sync the `dev:` groups via SCIM.

The SCIM impl is not integration tested, as this usually requires an external deployment. For the
future, the idea is to implement the client side into the `rauthy-client` and use this for
integration testing against itself. However, I followed the RFC closely and tested all operations
manually against https://scim.dev .

Just as for the OIDC Backchannel Logout, this feature can be seen as in beta state now. We need some
real world testing next, but I think it should be just fine.

```
#####################################
############### SCIM ################
#####################################

# If set to `true`, already possibly synced groups / users on a
# SCIM server may be deleted if either sync if disabled further
# down the road.
# Depending on your clients specifically, deleting a group / user
# can have a major impact, like e.g. maybe the user had important
# data inside that application that would be deleted automatically
# as well. Depending on your use case, you may want to handle
# deletions manually.
#
# If set to false, only the `externalId` field would be removed
# for the targeted resources, but they would not be fully deleted.
# This will basically unlink the resource from Rauthy, leaving it
# behind independent.
#
# default: false
#SCIM_SYNC_DELETE_GROUPS=false
#SCIM_SYNC_DELETE_USERS=false

# The maximum amount of retries made for a failed SCIM task.
# Failed tasks will be retried every 60 - 90 seconds from all
# cluster nodes. The timeout between retries is randomized to
# avoid overloading clients. It will be executed on each cluster
# member to increase the chance of a update in case of network
# segmentations.
# default: 100
#SCIM_RETRY_COUNT=100
```

[#666](https://github.com/sebadob/rauthy/issues/666)

#### Defense in depth

Small improvements have been done in various location inside the code to have additional defense in
depth or safety-nets during maintenance and development.

The `LoginRequest` during `POST /authorize`, as well as the `ProviderLoginRequest` require a
solved [Proof of Work](https://github.com/sebadob/spow) from the frontend now. This is an additional
DoS protection, and it mitigates any automated login checks / brute-force by bots or even tools like
burp suite, in addition to the already existing login rate-limiting.   
As an additional UX optimization, all PoW calculations have been removed from the JS main thread and
pushed into a Web Worker. This keeps the UI responsive, even if you set a higher difficulty which
takes a bit longer to compute.

Even though not necessary from a security point of view, the `redirect_uri` during an
`authorization_code` flow is checked multiple times between requests. Rauthy validates the URI at
the very first possibility and makes it impossible to change it during the flow, but additional
verification is a nice addition.

To make future maintenance tasks a bit more safe, the session state is being checked in 2 different
locations now. This is a tiny overhead of 1 additional `if` clause, but you get a safety net on all
API endpoints.

[#834](https://github.com/sebadob/rauthy/pull/834)
[#881](https://github.com/sebadob/rauthy/pull/881)

#### Graceful Client Secret Rotation

To be able to follow security best practices, you can now rotate client secrets gracefully.

Rauthy does not allow multiple client secrets, and up until now, if you generated a new one, the old
one would be gone immediately. Now, when you generate a new secret, you have the option to cache the
hash of the current one in-memory. You can also enter a time between 1 and 24 hours. This makes it
possible to rotate secrets as a best practice without any interruption in service.   
If you however need to generate a new secret because of a leak somewhere, you should of course not
enable this option.

[#867](https://github.com/sebadob/rauthy/pull/867)

#### UI Accessibility

Some minor improvements for the accessibility have been made. For instance, some missing `aria-*`
labels have been added and the colors from the scheme have been switched around slightly in some
situations to provide a better contrast out of the box without giving up the look.

[#861](https://github.com/sebadob/rauthy/pull/861)
[#862](https://github.com/sebadob/rauthy/pull/862)

#### Manual Admin-Initiated user init

When an admin created a user manually, it is now possible to initialize this user manually and set a
password, even if the user never used or received the initial password reset E-Mail.

[#815](https://github.com/sebadob/rauthy/issues/815)

#### Additional CORS headers

Additional CORS headers and validation have been added to a few different endpoints, where it made
sense, like e.g. the `/userinfo` or `/introspect` endpoints. Internally, the Origin header
extraction and validation has been simplified as well to make it a bit easier to maintain.

[#855](https://github.com/sebadob/rauthy/pull/855)
[#856](https://github.com/sebadob/rauthy/pull/856)

#### Global HTTP Client

To optimize memory usage a little bit, Rauthy now only uses a global, lazily initialized HTTP client
for for outgoing requests (except for Matrix notifications). This static client makes use of
connection pooling and keepalives under the hood to reduce the overall amount of TLS handshakes and
latency during daily operation for a few tasks.

Apart from Matrix event notifications, if you need any custom root CA, you need to add it to this
client. You can add as many as you like, just statically upfront. The dynamic approach from before
like for instance for upstream auth providers had a huge drawback, which was that the client had to
be created dynamically for each request depending on the database state. This was very inefficient
and a custom root CA usually changes like every 10 years, so it's not any issue at all to define
them via the configuration.

With these changes, you also have access to way more configuration options for the HTTP client and a
whole new section in the config:

```
#####################################
########### HTTP CLIENT #############
#####################################

## In this section, you can configure the HTTP Client
## that Rauthy uses for all different kind's of tasks,
## like e.g. fetching ephemeral client information,
## remote JWKS, connecting to Upstream Auth Providers,
## or for Slack notifications.
##
## NOTE: The only exception is the Matrix Event
## Notification client, if you have this configured.
## Because of the internal structure of Matrix dependencies,
## this cannot easily re-use the global client!
##
## Rauthy creates a single Lazily Initialized instance
## with connection pooling to reduce the amount of TLS
## handshakes necessary, especially during high traffic.

# The connect timeout in seconds for new connections.
# default: 10
#HTTP_CONNECT_TIMEOUT=10

# The total request timeout in seconds for all outgoing
# requests.
# default: 10
#HTTP_REQUEST_TIMEOUT=10

# Set the min TLS version for all outgoing connections.
# Allowed values: '1.3', '1.2', '1.1', '1.0'
# default: 1.3
#HTTP_MIN_TLS=1.3

# The duration in seconds for idle connections in the pool.
# To reduce memory consumption slightly, you may reduce
# this value at the cost of needing more TLS handshakes
# and re-connecting more often.
# default: 900
#HTTP_IDLE_TIMEOUT=900

# By default, the HTTP Client will enforce HTTPS and
# simply fail if an unencrypted HTTP URL is given
# anywhere.
# However, you can allow this by setting `true`.
# default: false
#HTTP_DANGER_UNENCRYPTED=false

# By default, the HTTP Client will enforce valid TLS
# certificates and simply fail if an invalid certificate
# is used anywhere.
# However, you can allow this by setting `true`.
# default: false
#HTTP_DANGER_INSECURE=false

# You can provide a root certificate bundle, if you
# are running servers / clients Rauthy needs to connect
# to with self-signed certificates.
# The certificates need to be in PEM format.
#HTTP_CUST_ROOT_CA_BUNDLE="
#-----BEGIN CERTIFICATE-----
#...
#-----END CERTIFICATE-----
#-----BEGIN CERTIFICATE-----
#...
#-----END CERTIFICATE-----
#"
```

[#874](https://github.com/sebadob/rauthy/pull/874)

#### Additional safeguards for `MIGRATE_DB_FROM`

Rauthy can migrate between Hiqlite and Postgres on the fly. This feature is only supported as long
as you do not bump the major or minor version at the same time. Additional safeguards and checks
have been added that prevent you from doing trying this by accident.

[#811](https://github.com/sebadob/rauthy/pull/811)

#### Hiqlite optimizations

After `sqlx` was dropped, I could finally upgrade `hiqlite` to the latest version with quite a few
improvements in terms of shutdown speed and stability.

Before, it was possible that you could get into a state with a HA deployment, where the cache leader
election got stuck after a rolling release e.g. inside Kubernetes. This was already fixed in a new
version, but `sqlx` blocked the upgrade for a few months. During that time, I have been able to
improve the shutdown behavior even more, and the shutdown time in HA deployments has been cut in
half. For a single instance, it is gone completely now.

Because of many improvements of `hiqlite`s codebase, I was also able to improve the throughput by
another ~28%. The database would never be the bottleneck for a Rauthy deployment (this will always
be password hashing, which has to be slow and expensive on purpose, to make it secure), but it's a
nice to have.

#### `SMTP_PORT`

You can now specify a custom `SMTP_PORT` in production. `SMTP_DANGER_INSECURE_PORT` has been dropped
in favor of just `SMTP_PORT`, which will now also be applied on secure connections in production,
not just during local development.

[#840](https://github.com/sebadob/rauthy/pull/840)

#### Latin Extended-B

The `user.given_name`, `user.familiy_name` and `client.name` now also allow characters from the
Latin Extended-B.

[#873](https://github.com/sebadob/rauthy/pull/873)

### Bugfix

- If wrong credentials for the SMTP server have been provided, the optional inner error has been
  masked.
  [#803](https://github.com/sebadob/rauthy/pull/803)
- The page title for the user registration page has been broken.
  [#812](https://github.com/sebadob/rauthy/pull/812)
- A bug in the Admin UI could get you into situations, where you would not see a scrollbar for the
  user navigation.
  [#869](https://github.com/sebadob/rauthy/pull/869)

## v0.28.3

### Bugfix

- The new Admin UI has always added an `expires` value during the creation of a new user, even if
  the checkbox was unchecked.
  [#797](https://github.com/sebadob/rauthy/pull/797)
- It has been possible to construct a situation in which a housekeeping scheduler for expired magic
  links could have been deleting newly registered users.
  [#798](https://github.com/sebadob/rauthy/pull/798)
  [#799](https://github.com/sebadob/rauthy/pull/799)

## v0.28.2

### Important

This is a pretty important bugfix update. The 0.28 versions before had an error where the `groups`
config component in the Admin UI has been changed to contain `roles` after the first render. This
means, if you have saved a user in the Admin UI (even without changing groups), the groups might
have been changed depending on your setup.

The backend sanitizes any submitted `roles` / `groups` on user updates, so only actually existing
values can exist, but if you had a `roles` combination `groups` with the exact same naming, like
e.g. an `admin` role and an `admin` groups, it could have been possible that a user ends up with a
group assignment he should not have. At the same time, a user might have had some groups removed he
had before.

This situation can only occur on users you have updated via the Admin UI with either `0.28.0` or
`0.28.1`. You should **update to `0.28.2` asap and double check group assignments** for your users
you may have updated with an earlier `0.28` version via the Admin UI.

### Bugfix

- Fixes a bug in the Admin UI / Users view where the content of the `groups` config component
  contained `roles` instead of groups.
  [#790](https://github.com/sebadob/rauthy/pull/790)

## v0.28.1

### Security

#### CVE-2025-29787

Rauthy itself has not been vulnerable, but he `zip` dependency pulled in by the Swagger UI as a 3rd
party dependency has been vulnerable when reading files. `zip` inside the Swagger UI has only been
used at build time, never at runtime. However, the version has been bumped to a non-vulnerable one.

[CVE-2025-29787](https://nvd.nist.gov/vuln/detail/CVE-2025-29787)

[#785](https://github.com/sebadob/rauthy/pull/785)

### Changes

#### Updated Translations

`zh` and `ko` translations have received updates. `ko` is now also available for the Admin UI and
has been added to the `FILTER_LANG_ADMIN` value from the `0.28.0` release.

[#769](https://github.com/sebadob/rauthy/pull/769)
[#775](https://github.com/sebadob/rauthy/pull/775)

### Bugfix

- UI: Fixed and updated some translations to make them more clear
  [#779](https://github.com/sebadob/rauthy/pull/779)
  [#783](https://github.com/sebadob/rauthy/pull/783)
- UI: The `Allow Insecure TLS` checkbox for upstream auth providers has been shown twice in some
  situations.
  [#780](https://github.com/sebadob/rauthy/pull/780)
- UI: The `Invalid Input` message has not been reset in some views and situations.
  [#781](https://github.com/sebadob/rauthy/pull/781)

## v0.28.0

### Breaking

#### Environment Variable Only Config

If you configured Rauthy via environment variables only, you might have breaking changes with this
update.

If a configuration is being made purely via env vars, and a proper `rauthy.cfg` (at least an empty
file) is not being created and mounted inside the container, the application would actually use demo
values as long as they are not overwritten by env vars manually.

To improve the security out of the box, the container setup has been changed and the demo config has
a separate filename, which will only be parsed when `LOCAL_TEST=true` is passed in as an env var
before app startup. Setting this value inside the usual `rauthy.cfg` has no effect.

The insecure local testing values that have been set before (again, with an env vars only setup),
can be found here https://github.com/sebadob/rauthy/blob/v0.27.3/rauthy.deploy.cfg for reference, so
you can check, if you would have breaking changes.

If no `rauthy.cfg` is ever being created, default values will be used, and you can configure the
application safely with env vars only. If you decide to use both, env vars will keep on having the
higher priority over values set inside the config file, just like it has been before.

[#763](https://github.com/sebadob/rauthy/pull/763)

#### Changed header names for session CSRF and password reset tokens

This may concern you, if you have built custom UI parts in front of Rauthy.

The Headers names for the session and password reset CSRF tokens have been changed and now contain a
leading `x-`. This make the API more clean, since custom headers should be marked with a leading
`x-`.

- `csrf-token` -> `x-csrf-token`
- `pwd-csrf-token` -> `x-pwd-csrf-token`

[#749](https://github.com/sebadob/rauthy/pull/749)

#### Custom Client Branding

With the migration to Svelte 5 (mentioned below), the way theming is done has been changed from the
ground up in such a way, that it is not possible to migrate possibly existing custom client
branding. This means that you will lose and need to re-create a possibly existing custom branding
with this version.

#### Paginated Users / Sessions

This may only concern you if you are doing direct API calls to `GET` users or sessions on a very big
Rauthy instance in combination with server side pagination. When you added `backwards=true` before,
the offset of a single page has been added automatically in the backend. This is not the case
anymore to provide more flexibility with this API. You need to add the `offset` yourself now while
going `backwards`.

[#732](https://github.com/sebadob/rauthy/pull/732)

### Security

#### CVE-2025-24898

Even though the vulnerable code blocks have not been used directly, the `openssl` and `openssl-sys`
dependencies have been bumped to
fix [CVE-2025-24898](https://nvd.nist.gov/vuln/detail/CVE-2025-24898).

[#717](https://github.com/sebadob/rauthy/pull/717)

#### GHSA-67mh-4wv8-2f99

This could have only been affecting dev environments, not any production build,
but [GHSA-67mh-4wv8-2f99](https://github.com/advisories/GHSA-67mh-4wv8-2f99) has been fixed by
bumping frontend dependencies.

[#735](https://github.com/sebadob/rauthy/pull/735)

### Changes

#### Svelte 5 Migration

The whole UI has been migrated to Svelte 5 + Typescript. Many parts and components have been
re-written from the ground up to provide a better DX and maintainability in the future.

This whole migration comes with a lot of changes, most of them under the hood regarding performance
and efficiency. There are so many changes, that it does not make much sense to list them all here,
but the TL;DR is:

- The whole UI is now based on Svelte 5 + TS with improved performance.
- The DX and UX has been improved a lot.
- Accessibility has been improved by a huge margin.
- Rauthy now comes with a light and dark mode, even for the custom client branding login site.
- We have a new logo, which makes it a lot easier to identify Rauthy in a tab overview and so on.
- The whole UI is now fully responsive and usable even down to mobile devices.
- The whole design of the UI has been changed in a way that most components and payloads can now be
  cache infinitely.
- The engine for server side rendering of the static HTML content has been migrated
  from [askama](https://github.com/rinja-rs/askama) to [rinja](https://github.com/rinja-rs/rinja) (
  based on askama with lots of improvements).
- The backend now comes with caching and dynamic pre-compression of all dynamic SSR HTML content.
- The way i18n is done has been changed a lot and moved from the backend into a type-checked
  frontend file to make it a bit easier to get into and provide caching again.
- The admin UI can now be translated as well. The i18n for common user sites and the admin UI are
  split for reduced payloads for most users. Currently, only `en` and `de` exist for the Admin UI,
  but these can be extended easily in the future as soon as someone provides a PR. They are also
  independent with the only requirement that a common i18n must exist before an admin i18n. (
  Translations for E-Mails are still in the backend of course)
- Part of the state for the Admin UI has been moved into the URL, which makes it possible to copy &
  paste most links and actually end up where you were before.

NOTICE: Since the whole UI has basically been re-written, or at least almost every single line has
been touched, the new UI can be seen as in beta state. If you have any problems with it, please open
an issue.

[#642](https://github.com/sebadob/rauthy/issues/642)

#### User Pictures / Avatars

It is now possible to upload an avatar / picture for each user. This can be done via the account
dashboard.

Rauthy uses the term `picture` to match the OIDC RFC spec. If the `scope` during login includes
`profile` and the user has a picture, the `picture` claim will be included in the `id_token` and
will contain the URL where the user picture can be found. User Picture URLs are "safe" to be used
publicly, and they contain 2 cryptographically random secure IDs. This makes it possible to even
make them available without authentication for ease of use. By default, a session / API Key / token
is required to fetch them, but you can opt-out of that.

For storage options, the default is database. This is not ideal and should only be done for small
instances with maybe a few hundred users. They can fill up the database pretty quickly, even though
images are optimized after upload, they will end up somewhere in the range of ~25 - 40kB each.
For single instance deployments, you can use local `file` storage, while for HA deployments, you
should probably use an S3 bucket to do so.

Uploading user pictures can be disabled completely by setting `PICTURE_STORAGE_TYPE=disabled`

The following new config variables are available:

```
#####################################
######### User Pictures #############
#####################################

# The storage type for user pictures.
# By default, they are saved inside the Database, which is not ideal.
# If you only have a couple hundred users, this will be fine, but
# anything larger should use an S3 bucket if available. For single
# instance deployments, file storage to local disk is available
# as well, but this must not be used with multi replica / HA
# deployments.
# Images will ba reduced in size to max 192px on the longest side.
# They most often end up between 25 - 40kB in size.
#
# Available options: db file s3 disabled
# Default: db
#PICTURE_STORAGE_TYPE=db

# If `PICTURE_STORAGE_TYPE=file`, the path where pictures will be
# saved can be changed with this value.
# default: ./pictures
#PICTURE_PATH="./pictures"

# Access values for the S3 bucket if `PICTURE_STORAGE_TYPE=s3`.
# Not needed otherwise.
#PIC_S3_URL=https://s3.example.com
#PIC_S3_BUCKET=my_bucket
#PIC_S3_REGION=example
#PIC_S3_KEY=s3_key
#PIC_S3_SECRET=s3_secret
# default: true
#PIC_S3_PATH_STYLE=true

# Set the upload limit for user picture uploads in MB.
# default: 10
#PICTURE_UPLOAD_LIMIT_MB=10

# By default, user pictures can only be fetched with a valid
# session, an API Key with access to Users + Read, or with a
# valid token for this user. However, depending on where and
# how you are using Rauthy for your user management, you may
# want to make user pictures available publicly without any
# authentication.
#
# User Picture URLs are "safe" in a way that you cannot guess
# a valid URL. You will need to know the User ID + the Picture
# ID. Both values are generated cryptographically secure in the
# backend during creation. The Picture ID will also change
# with every new upload.
#
# default: false
#PICTURE_PUBLIC=false
```

[#743](https://github.com/sebadob/rauthy/pull/743)

#### Static HTML + prepared queries added to version control

To make it possible to build Rauthy from source in environments like e.g. FreeBSD, all pre-built
static HTML files have been added to version control, even though they are built dynamically each
time in release pipelines. Additionally, all DB queries used by `sqlx` are added to version control
as well.

The reason is that the UI cannot be built in certain environments. With these files checked-in, you
can build from source with just `cargo build --release` by having Rust available. You don't need to
build the UI or have a Postgres running anymore, if you only care about building from source.

#### I18n - Korean

Korean has been added to the translations for all user-facing UI parts.
[#670](https://github.com/sebadob/rauthy/pull/670)

#### Filter I18n UI Languages

Since it is likely that the available translations will expand in the future and you may not need or
want to show all options to the users, because you maybe only have a local / regional deployment,
you can now apply a filter to the Languages that are shown in the UI selector.

```
#####################################
############### I18n ################
#####################################

# Can be set to filter the languages to show in the UI.
# If not set, all available i18n translations will be
# show in the language selector.
# To show only specific ones, provide them here as a
# space-separated value.
#
# Languages for all user-facing pages.
# Available Options: en de zhhans ko
#FILTER_LANG_COMMON="en de zhhans ko"
# Languages for the Admin UI.
# Available Options: en de
#FILTER_LANG_ADMIN="en de"
```

[#748](https://github.com/sebadob/rauthy/pull/748)

#### Manual `Debug` impls

For types in the background, that hold sensitive data like passwords or client secrets, manual impls
for `Debug` have been added to make sure that no sensitive information can leak into logs with
`LOG_LEVEL=debug`.

CAUTION: If you enable `HQL_LOG_STATEMENTS=true` for DB statement logging, it is guaranteed that you
will see sensitive information in logs sooner or later, so be very careful when using this in
production.

[#708](https://github.com/sebadob/rauthy/pull/708)

### Bugfix

- When using the `MIGRATE_DB_FROM` in combination with a Sqlite DB as base that has been created
  with Rauthy < `v0.20.0`, the migration might fail for you in `v0.27.0` because of an old value in
  the `config` table that might not have been cleaned up properly in some cases.
  [#669](https://github.com/sebadob/rauthy/pull/685)
- Fixed a regression from v0.27.0 which made it impossible to use `zh-Hans` as a users' language.
  The deserialization of the database value would fail when using Hiqlite.
  [#693](https://github.com/sebadob/rauthy/pull/693)
- Fixed a bug in the UI - Custom User Attributed: When only a single existing attributed has been
  deleted, the list would not properly update and remove it.
  [#695](https://github.com/sebadob/rauthy/pull/695)
- Type information has been missing for a few endpoints in the OpenAPI definition, which have been
  fixed in various PRs.
- A bug has been fixed where the `/logout` endpoint did not correctly forward the optional `state`,
  if it has been initiated by a downstream client.
  [#690](https://github.com/sebadob/rauthy/pull/690)

## v0.27.3

### Changes

#### Upstream Identity Providers

To provide additional compatibility for some upstream providers like Active Directory Federation
Severices, some changes have been applied to Rauthy's behavior.

The first thing is that the HTTP client used for upstream Logins does not force TLS v1.3 anymore,
but also allows TLS v1.2. Both v1.2 and v1.3 are considered being secure by current standards. This
is necessary, because some OSes like Windows Server 2019 do not support TLS 1.3.

The second change is for the way upstream providers are configured. The behavior until now was, that
Rauthy added the client credentials as both Basic Authentication in headers, and in the body for
maximum compatibility. However, some IdP'S (like ADFS for instance) complain about this and only
expect it in one place. To make this happen, there are 2 new fields for the upstream IdP
configuration:

- `client_secret_basic: bool`
- `client_secret_post: bool`

These are available as switches in the Admin UI for each upstream provider. To not introduce
breaking changes, all possibly existing configurations will have both options enabled like it has
been up until now.

[#659](https://github.com/sebadob/rauthy/pull/659)

#### Note

Even though this changes the request and response objects on the API, this change is **NOT** being
handled as a breaking change. API clients are forbidden to modify upstream IdPs for security
reasons, which means this change should only affect the Rauthy Admin UI.

#### Gitlab as Upstream IdP

Gitlab is special and does its own, annoying thing to make it usable as an upstream IdP. An issue
has been found when someone tries to log in with no publicly shown email address. In this worst case
scenario, a successful login to Github while retrieving all necessary information (email is
mandatory for Rauthy), you need to do 3 different API requests.

This version also makes it possible to log in via Github IdP with an account with only private email
addresses. A different `scope` for the login is necessary to make this possible. The template in the
UI has been updated, but this will not affect existing Github IdP Providers. If you are currently
using Github as upstream IdP, please change the `scope` manually from `read:user` to `user:email`.

[#665](https://github.com/sebadob/rauthy/pull/665)

### Bugfix

- During the deletion of a custom scope, that has been mapped to only a clients default scopes, but
  not the free ones, the mapping would be skipped during the whole client cleanup and end up being
  left-over after the deletion, which needed a manual cleanup afterward.
  [#663](https://github.com/sebadob/rauthy/pull/663)

## v0.27.2

### Changes

Even though not recommended at all, it is now possible to opt-out of the `refresh_token` nbf claim,
and disable it.

By default, A `refresh_token` will not be valid before `access_token_lifetime - 60 seconds`, but
some (bad) client implementations try to refresh `access_tokens` while they are still valid for a
long time. To opt-out, you get a new config variable:

```
# By default, `refresh_token`s will have an `nbf` claim, making them valid
# at `access_token_lifetime - 60 seconds`. Any usage before this time will
# result in invalidation of not only the token itself, but also all other
# linked sessions and tokens for this user to prevent damage in case a client
# leaked the token by accident.
# However, there are bad / lazy client implementations that do not respect
# either `nbf` in the `refresh_token`, or the `exp` claim in `access_token`
# and will refresh early while the current access_token is still valid.
# This does not only waste resources and time, but also makes it possible
# to have multiple valid `access_token`s at the same time for the same
# session. You should only disable the `nbf` claim if you have a good
# reasons to do so.
# If disabled, the `nbf` claim will still exist, but always set to *now*.
# default: false
DISABLE_REFRESH_TOKEN_NBF=false
```

[#651](https://github.com/sebadob/rauthy/pull/653)

### Bugfix

The Rauthy deployment could get stuck in Kubernetes when you were running a HA-Cluster with Postgres
as your database of choice. The cache raft re-join had an issue sometimes because of a race
condition, which needed a full restart of the cluster. This has been fixed
in [hiqlite-0.3.2](https://github.com/sebadob/hiqlite/releases/tag/v0.3.2) and the dependency has
been bumped.

## v0.27.1

### Bugfix

With the big migration to [Hiqlite](https://github.com/sebadob/hiqlite) under the hood, a bug has
been introduced with `v0.27.0` that made it possible to end up with a `NULL` value for the password
policy after an update. Which would result in errors further down the road after a restart, because
the policy could not be read again.

This version fixes the issue itself and checks at startup if the database needs a fix for this issue
because of an already existing `NULL` value. In this case, the default password policy will be
inserted correctly at startup.

[#646](https://github.com/sebadob/rauthy/pull/646)

## v0.27.0

### Breaking

#### Single Container Image

The different versions have been combined into a single container image. The image with the `-lite`
extension simply does not exist anymore and all deployments can be done with just the base image.
Since Postgres was the default before, you need to change your image name when you do not use
Postgres as your database, just remove the `-lite`.

#### Dropped `sqlx` SQLite in favor of [Hiqlite](https://github.com/sebadob/hiqlite)

From this version on, Rauthy will not support a default SQLite anymore. Instead, it will use
[Hiqlite](https://github.com/sebadob/hiqlite), which under the hood uses SQLite again and is another
project of mine.

[Hiqlite](https://github.com/sebadob/hiqlite) will bring lots of advantages. It will use a few more
resources than a direct, plain SQLite, but only ~10-15 MB of memory for small instances. In return,
you will get higher consistency and never blocking writes to the database during high traffic. It
also reduces the latency for all read statements by a huge margin compared to the solution before.
Rauthy always enables the `dashboard` feature for [Hiqlite](https://github.com/sebadob/hiqlite),
which will be available over the Hiqlite API port / server.

The biggest feature it brings though is the ability to **run a HA cluster without any external
dependencies**. You can use [Hiqlite](https://github.com/sebadob/hiqlite) on a single instance and
it would "feel" the same as just a SQLite, but you can also spin up 3 or 5 nodes to get High
Availability without the need for an external database. It uses the Raft algorithm to sync data
while still using just a simple SQLite under the hood. The internal design of Hiqlite has been
optimized a lot to provide way higher throughput as you would normally get when you just use a
direct connection to a SQLite file. If you are interested more about the internals, take a look at
the [hiqlite/README.md](https://github.com/sebadob/hiqlite/blob/main/README.md)
or [hiqlite/ARCHITECTURE.md](https://github.com/sebadob/hiqlite/blob/main/ARCHITECTURE.md).

With these features, Hiqlite will always be the preferred database solution for Rauthy. You should
really not spin up a dedicated Postgres instance just for Rauthy, because it would just use too many
resources, which is not necessary. If you have a Postgres up and running anyway, you can still
opt-in to use it.

This was a very big migration and tens of thousands of lines of code has been changed. All tests are
passing and a lot of additional checks have been included. I could not find any leftover issues or
errors, but please let me know if you find something.

If you are using Rauthy with Postgres as database, you don't need to do that much. If however you
use SQLite, no worries, Rauthy can handle the migration for you after adopting a few config
variables. Even if you do the auto-migration from an existing SQLite to Hiqlite, Rauthy will keep
the original SQLite file in place for additional safety, so you don't need to worry about a backup (
as long as you set the config correctly of course). The next bigger release will maybe do cleanup
work when everything worked fine for sure, or you can do it manually.

##### New / Changed Config Variables

There are quite a few new config variables and some old ones are gone. What you need to set to
migration will be explained below.

```
#####################################
############# BACKUPS ###############
#####################################

# When the auto-backup task should run.
# Accepts cron syntax:
# "sec min hour day_of_month month day_of_week year"
# default: "0 30 2 * * * *"
HQL_BACKUP_CRON="0 30 2 * * * *"

# Local backups older than the configured days will be cleaned up after
# the backup cron job.
# default: 30
#HQL_BACKUP_KEEP_DAYS=30

# Backups older than the configured days will be cleaned up locally
# after each `Client::backup()` and the cron job `HQL_BACKUP_CRON`.
# default: 3
#HQL_BACKUP_KEEP_DAYS_LOCAL=3

# If you ever need to restore from a backup, the process is simple.
# 1. Have the cluster shut down. This is probably the case anyway, if
#    you need to restore from a backup.
# 2. Provide the backup file name on S3 storage with the
#    HQL_BACKUP_RESTORE value.
# 3. Start up the cluster again.
# 4. After the restart, make sure to remove the HQL_BACKUP_RESTORE
#    env value.
#HQL_BACKUP_RESTORE=

# Access values for the S3 bucket where backups will be pushed to.
#HQL_S3_URL=https://s3.example.com
#HQL_S3_BUCKET=my_bucket
#HQL_S3_REGION=example
#HQL_S3_PATH_STYLE=true
#HQL_S3_KEY=s3_key
#HQL_S3_SECRET=s3_secret

#####################################
############# CLUSTER ###############
#####################################

# Can be set to 'k8s' to try to split off the node id from the hostname
# when Hiqlite is running as a StatefulSet inside Kubernetes.
#HQL_NODE_ID_FROM=k8s

# The node id must exist in the nodes and there must always be
# at least a node with ID 1
# Will be ignored if `HQL_NODE_ID_FROM=k8s`
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
HQL_NODES="
1 localhost:8100 localhost:8200
"

# Sets the limit when the Raft will trigger the creation of a new
# state machine snapshot and purge all logs that are included in
# the snapshot.
# Higher values can achieve more throughput in very write heavy
# situations but will end up in more disk usage and longer
# snapshot creations / log purges.
# default: 10000
#HQL_LOGS_UNTIL_SNAPSHOT=10000

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

#####################################
############ DATABASE ###############
#####################################

# Max DB connections for the Postgres pool.
# Irrelevant for Hiqlite.
# default: 20
#DATABASE_MAX_CONN=20

# If specified, the currently configured Database will be DELETED and 
# OVERWRITTEN with a migration from the given database with this variable. 
# Can be used to migrate between different databases.
#
# !!! USE WITH CARE !!!
#
#MIGRATE_DB_FROM=sqlite:data/rauthy.db
#MIGRATE_DB_FROM=postgresql://postgres:123SuperSafe@localhost:5432/rauthy

# Hiqlite is the default database for Rauthy.
# You can opt-out and use Postgres instead by setting the proper
# `DATABASE_URL=postgresql://...` by setting `HIQLITE=false`
# default: true
#HIQLITE=true

# The data dir hiqlite will store raft logs and state machine data in.
# default: data
#HQL_DATA_DIR=data

# The file name of the SQLite database in the state machine folder.
# default: hiqlite.db
#HQL_FILENAME_DB=hiqlite.db

# If set to `true`, all SQL statements will be logged for debugging
# purposes.
# default: false
#HQL_LOG_STATEMENTS=false

# The size of the pooled connections for local database reads.
#
# Do not confuse this with a pool size for network databases, as it
# is much more efficient. You can't really translate between them,
# because it depends on many things, but assuming a factor of 10 is 
# a good start. This means, if you needed a (read) pool size of 40 
# connections for something like a postgres before, you should start 
# at a `read_pool_size` of 4.
#
# Keep in mind that this pool is only used for reads and writes will
# travel through the Raft and have their own dedicated connection.
#
# default: 4
#HQL_READ_POOL_SIZE=4

# Enables immediate flush + sync to disk after each Log Store Batch.
# The situations where you would need this are very rare, and you
# should use it with care.
#
# The default is `false`, and a flush + sync will be done in 200ms
# intervals. Even if the application should crash, the OS will take
# care of flushing left-over buffers to disk and no data will get
# lost. If something worse happens, you might lose the last 200ms 
# of commits (on that node, not the whole cluster). This is only
# important to know for single instance deployments. HA nodes will
# sync data from other cluster members after a restart anyway.
#
# The only situation where you might want to enable this option is
# when you are on a host that might lose power out of nowhere, and
# it has no backup battery, or when your OS / disk itself is unstable.
#
# `sync_immediate` will greatly reduce the write throughput and put
# a lot more pressure on the disk. If you have lots of writes, it
# can pretty quickly kill your SSD for instance.
#HQL_SYNC_IMMEDIATE=false

# The password for the Hiqlite dashboard as Argon2ID hash.
# '123SuperMegaSafe' in this example
#
# You only need to provide this value if you need to access the
# Hiqlite debugging dashboard for whatever reason. If no password
# hash is given, the dashboard will not be reachable.
#HQL_PASSWORD_DASHBOARD=JGFyZ29uMmlkJHY9MTkkbT0xOTQ1Nix0PTIscD0xJGQ2RlJDYTBtaS9OUnkvL1RubmZNa0EkVzJMeTQrc1dxZ0FGd0RyQjBZKy9iWjBQUlZlOTdUMURwQkk5QUoxeW1wRQ==
```

##### Migration (Postgres)

If you use Rauthy with Postgres and want to keep doing that, the only thing you need to do is to
opt-out of Hiqlite.

```
HIQLITE=false
```

##### Migration (SQLite)

If you use Rauthy with SQLite and want to migrate to Hiqlite, you can utilize all the
above-mentioned new config variables, but mandatory are the following ones.

###### Backups

Backups for the internal database work in the same way as before, but because I moved the backup
functionality directly into [Hiqlite](https://github.com/sebadob/hiqlite), the variable names have
been changed so they make sense if it is used in another context.

- `BACKUP_TASK` -> `HQL_BACKUP_CRON`
- `BACKUP_NAME` does not exist anymore, will be chosen automatically depending on the cluster leader
  name
- `BACKUP_RETENTION_LOCAL` -> `HQL_BACKUP_KEEP_DAYS_LOCAL`
- `RESTORE_BACKUP` -> `HQL_BACKUP_RESTORE`
- `S3_URL` -> `HQL_S3_URL`
- `S3_REGION` -> `HQL_S3_REGION`
- `S3_PATH_STYLE` -> `HQL_S3_PATH_STYLE`
- `S3_BUCKET` -> `HQL_S3_BUCKET`
- `S3_ACCESS_KEY` -> `HQL_S3_KEY`
- `S3_ACCESS_SECRET` -> `HQL_S3_SECRET`

`S3_DANGER_ALLOW_INSECURE` stayed as it is.

`BACKUP_RETENTION_LOCAL` is new, and it will actually handle the backup cleanup on the S3 storage
for you, without defining retention rules for the whole bucket.

###### Hiqlite Dashboard

Rauthy comes with the `dashboard` feature from Hiqlite enabled. If you want to make use of it, you
need to set the password for logging in. This is a static config variable, and it will only be a
single password, no users / accounts. The main idea behind the dashboard is to have debugging
capabilities in production, which is usually hard to do with a SQLite running inside a container.  
You need to generate a random password (at least 16 characters), hash it with Argon2ID, and then
base64 encode it. You can do all this manually, or use the `hiqlite` cli to generate a complete
hiqlite config where you copy it from.

**Manual:**

Use an online tool like for instance https://argon2.online to generate an Argon2**ID** hash.
Set the following options:

- Salt: Random
- Parallelism Factor: 2
- Memory Cost: 32
- Iterations: 2
- Hash Length: 32
- Argon2id

Then copy the **Output in Encoded Form** and base64 encode it, for instance
using https://www.base64encode.org.

**`hiqlite` cli:**

Currently, you can only install the `hiqlite` cli via `cargo`:

- `cargo install hiqlite --features server`
- `hiqlite generate-config -p YouRandomSecurePasswordAtLeast16Chars`
- grab the password from the config, location written in the output

###### Migration

Unless you specified a custom target path on disk for SQLite(`HQL_DATA_DIR`)) before, you should be
good with the configuration now. If you start up Rauthy now, it will be like a fresh install, which
you most probably don't want. To migrate your current SQLite to Hiqlite at startup, you need to set
the `MIGRATE_DB_FROM` once at startup. If you used the default path before, you need to set:

```
MIGRATE_DB_FROM=sqlite:data/rauthy.db
```

For a custom path, just adopt the value accordingly. This works as well by the way, if you want to
migrate from Postgres to Hiqlite.

**!!! CAUTION !!!**  
You must remove this variable after Rauthy has been started successfully! Otherwise, it would do the
migration again and again with each following restart and therefore remove everything that has
happened in between!

[#592](https://github.com/sebadob/rauthy/pull/592)
[#601](https://github.com/sebadob/rauthy/pull/601)
[#603](https://github.com/sebadob/rauthy/pull/603)

#### User Registration - Redirect Hint

As an additional hardening, the open redirect hint for user registrations has been locked down a bit
by default. If you used this feature before, you should update `Client URI`s via the Admin UI, so
all possible `redirect_uri`s you are using will still be considered valid, or opt-out of the
additional hardening.

```
# If set to `true`, any validation of the `redirect_uri` provided during
# a user registration will be disabled.
# Clients can use this feature to redirect the user back to their application
# after a successful registration, so instead of ending up in the user
# dashboard, they come back to the client app that initiated the registration.
#
# The given `redirect_uri` will be compared against all registered
# `client_uri`s and will throw an error, if there is no match. However,
# this check will prevent ephemeral clients from using this feature. Only
# if you need it in combination with ephemeral clients, you should
# set this option to `true`. Otherwise it is advised to set the correct
# Client URI in the admin UI. The `redirect_uri` will be allowed if it starts
# with any registered `client_uri`.
#
# default: false
#USER_REG_OPEN_REDIRECT=true
```

#### Optional User Family Name

Since I received quite a few questions and requests regarding the mandatory `family_name` for users,
I decided to change it and make it optional. This change should not affect you in any way if you
only consumed `id_token`s, because the `family_name` inside them has been optional before already.
Its existence in the `id_token` depends on allowed and requested claims.

However, if you used to communicate with the Rauthy API directly, you should be aware of this
change. The `User.family_name` is now optional in all situations.

[#631](https://github.com/sebadob/rauthy/pull/631)

### Changes

#### Efficiency and speed improvements

During the migration to Hiqlite, I stumbled about a few DB queries in different places that were low
haging fruits for efficiency and speed improvements. I did these while migrating the code base.
There were a few ones, for instance in situations like session invalidation, for password reminder
cron jobs, roles / groups / scopes name updates, and so on. These do not affect the behavior, just
the handling under the hood has been improved.

#### SVG Sanitization

Rauthy allows you to upload SVGs as either client or upstream IdP logos. This is an action, that
only an authorized `rauthy_admin` role can do. However, as an additional defense in-depth and
protection against an evil admin, Rauthy now sanitizes all uploaded SVGs, no matter what.

[#615](https://github.com/sebadob/rauthy/pull/615)
[#616](https://github.com/sebadob/rauthy/pull/616)

#### Latin-1 Extended A added

The allowed characters for input validation for:

- user `given_name`
- user `family_name`
- client name

have been expanded and will also allow characters from `Latin-1 Extended A`.

[#617](https://github.com/sebadob/rauthy/pull/617)

### Bugfix

- When Rauthy sent E-Mails, the Name of the name of the recipient has not been set correctly
  [#602](https://github.com/sebadob/rauthy/pull/602)
- The banner should not be logged as plain text when `LOG_FMT=json` is set
  [#605](https://github.com/sebadob/rauthy/pull/605)
- When requesting a single user by its ID using an API key, you would get an invalid session error
  response. This was due to an earlier migration a few versions back. The session check should be
  done only when no API key is present to make this request work.
  [#609](https://github.com/sebadob/rauthy/pull/609)

## 0.26.2

### Bugfix

This patch reverts an unintended change to the `user:group` inside the container images.
This will fix issues with migrations from existing deployments using SQLite with manually managed
volume access rights.

v0.26.0 changed from `scratch` to `gcr.io/distroless/cc-debian12:nonroot` as the base image for the
final deployment. The distroless image however sets a user of `65532` by default, while it always
has been `10001:10001` before. The affected versions are

- `0.26.0`
- `0.26.1`

Starting from this release (`0.26.2`), the user inside the container will be the same one as before:

`10001:10001`

[839724001710cb095f39ff7df6be00708a01801a](https://github.com/sebadob/rauthy/pull/596/commits/839724001710cb095f39ff7df6be00708a01801a)

## 0.26.1

### Changes

#### Upstream Auth Provider Query Params

Some upstream auth providers need custom query params appended to their authorization endpoint URL.
Rauthy will now accept URLs in the auth provider config with pre-defined query params, as long as
they don't interfere with OIDC default params.

[7dee26a](https://github.com/sebadob/rauthy/commit/7dee26af0ab757cd80395652fe03f82ffbc2c8bc)

#### Option Log Fmt as JSON

To make automatic parsing of logs possible (to some extent), you now have the ability to change the
logging output from text to json with the following new config variable:

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

- With relaxing requirements for password resets for new users, a bug has been introduced that would
  prevent a user from registering an only-passkey account when doing the very first "password
  reset".
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

If you did not care about the response body, there is nothing to do for you. The body itself returns
different values now:

```rust
struct HealthResponse {
  db_healthy: bool,
  cache_healthy: bool,
}
```

[0919767](https://github.com/sebadob/rauthy/commit/09197670e6491f83a8b739c0f195d4b842abe771)

### Changes

#### ZH-Hans Translations

Translations for `ZH-Hans` have been added to Rauthy. These exist in all places other than the Admin
UI, just like the existing ones already.

[ec6c2c3](https://github.com/sebadob/rauthy/commit/ec6c2c3bb4e8b41fa0cd2a60ccc4043d051c17a5)  
[fcba3c7](https://github.com/sebadob/rauthy/commit/fcba3c7cd7bce7e15d911c0f9d7f55f852e7c424)

#### Support for deep-linking client apps like Tauri

Up until v0.25, it was not possible to set the `Allowed Origin` for a client in a way that Rauthy
would allow access for instance from inside a Tauri app. The reason is that Tauri (and most probably
others) do not set an HTTP / HTTPS scheme in the `Origin` header, but something like `tauri://`.

Rauthy has now support for such situations with adjusted validation for the Origin values and a new
config variable to allow specific, additional `Origin` schemes:

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
The quorum is also checked, which will detect network segmentations. To achieve this and still make
it possible to use the health check in situations like Kubernetes rollouts, a delay has been added,
which will simply always return `true` after a fresh app start. This initial delay make it possible
to use the endpoint inside Kubernetes and will not prevent from scheduling the other nodes. This
solves a chicken-and-egg problem.

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

To send out Matrix notifications, Rauthy was using the `matrix-sdk` up until now. This crate however
comes with a huge list of dependencies and at the same time pushes too few updates. I had quite a
few issues with it in the past because it was blocking me from updating other dependencies.

To solve this issue, I decided to drop `matrix-sdk` in favor of `ruma`, which it is using under the
hood anyway. With `ruma`, I needed to do a bit more work myself since it's more low level, but at
the same time I was able to reduce the list of total dependencies Rauthy has by ~90 crates.

This made it possible to finally bump other dependencies and to start the internal switch
from [redhac](https://github.com/sebadob/redhac) to [Hiqlite](https://github.com/sebadob/hiqlite)
for caching.

**IMPORTANT:**  
If you are using a self-hosted homeserver or anything else than the official `matrix.org` servers
for Matrix event notifications, you must set a newly introduced config variable:

```
# URL of your Matrix server.
# default: https://matrix.org
#EVENT_MATRIX_SERVER_URL=https://matrix.org
```

[0b50376](https://github.com/sebadob/rauthy/commit/0b5037610d475e7ad6fc0a8bf3b851330088cab1)

#### Internal Migration from `redhac` to `hiqlite`

The internal cache layer has been migrated from [redhac](https://github.com/sebadob/redhac)
to [Hiqlite](https://github.com/sebadob/hiqlite).

A few weeks ago, I started rewriting the whole persistence layer from scratch in a separate project.
`redhac` is working fine, but it has some issues I wanted to get rid of.

- its network layer is way too complicated which makes it very hard to maintain
- there is no "sync from other nodes" functionality, which is not a problem on its own, but leads to
  the following
- for security reasons, the whole cache is invalidated when a node has a temporary network issue
- it is very sensitive to even short term network issues and leader changes happen too often for my
  taste

I started the [Hiqlite](https://github.com/sebadob/hiqlite) project some time ago to get rid of
these things and have additional features. It is outsourced to make it generally usable in other
contexts as well.

This first step will also make it possible to only have a single container image in the future
without the need to decide between Postgres and SQLite via the tag.

[0919767](https://github.com/sebadob/rauthy/commit/09197670e6491f83a8b739c0f195d4b842abe771)

#### Local Development

The way the container images are built, the builder for the images is built and also the whole
`justfile` have been changed quite a bit. This will not concern you if you are not working with the
code.

The way of wrapping and executing everything inside a container, even during local dev, became
tedious to maintain, especially for different architectures and I wanted to get rid of the burden of
maintenance, because it did not provide that many benefits. Postgres and Mailcrab will of course
still run in containers, but the code itself for backend and frontend will be built and executed
locally.

The reason I started doing all of this inside containers beforehand was to not need a few additional
tool installed locally to make everything work, but the high maintenance was not worth it in the
end. This change now reduced the size of the Rauthy builder image from 2x ~4.5GB down to 1x ~1.9GB,
which already is a big improvement. Additionally, you don't even need to download the builder image
at all when you are not creating a production build, while beforehand you always needed the builder
image in any case.

To encounter the necessary dev tools installation and first time setup, I instead added a new `just`
recipe called `setup` which will do everything necessary, as long as you have the prerequisites
available (which you needed before as well anyway, apart from `npm`). This has been updated in the
[CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md).

### Bugfix

- The `refresh_token` grant type on the `/token` endpoint did not set the original `auth_time` for
  the `id_token`, but instead calculated it from `now()` each time.
  [aa6e07d](https://github.com/sebadob/rauthy/commit/aa6e07db8822e72e28329b0ecea52e6113851d4a)

## v0.25.0

### Changes

#### Token Introspection

The introspection endpoint has been fixed in case of the encoding like mentioned in bugfixes.  
Additionally, authorization has been added to this endpoint. It will now make sure that the request
also includes an `AUTHORIZATION` header with either a valid `Bearer JwtToken` or
`Basic ClientId:ClientSecret` to prevent token scanning.

The way of authorization on this endpoint is not really standardized, so you may run into issues
with your client application. If so, you can disable the authentication on this endpoint with

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

In preparation for a clean v1.0.0, some older API routes have been fixed regarding their casing and
naming. The "current" or old routes and names will be available for exactly one release and will be
phased out afterward to have a smooth migration, just in case someone uses these renamed routes.

- `/oidc/tokenInfo` -> `/oidc/introspect`
- `/oidc/rotateJwk` -> `/oidc/rotate_jwk`

Since I don't like `kebab-case`, most API routes are written in `snake_case`, with 2 exceptions that
follow RFC namings:

- `openid-configuration`
- `web-identity`

All the `*info` routes like `userinfo` or `sessioninfo` are not `kebab_case` on purpose, just to
match other IdPs and RFCs a bit more.

There is not a single `camelCase` anymore in the API routes to avoid confusion and issues in
situations where you could for instance mistake an uppercase `I` as a lowercase `l`. The current
`camelCase` endpoints only exist for a smoother migration and will be phased out with the next
bigger release.

[107f148](https://github.com/sebadob/rauthy/commit/107f14807760e56a1671e587fa9b08284589f932)

#### Config Read

The current behavior of reading in config variables was not working as intended.

Rauthy reads the `rauthy.cfg` as a file first and the environment variables afterward. This makes it
possible to configure it in any way you like and even mix and match.   
However, the idea was that any existing variables in the environment should overwrite config
variables and therefore have the higher priority. This was exactly the other way around up until
`v0.24.1` and has been fixed now.

How Rauthy parses config variables now correctly:

1. read `rauthy.cfg`
2. read env var
3. all existing env vars will overwrite existing vars from `rauthy.cfg` and therefore have the
   higher priority

[28b2457](https://github.com/sebadob/rauthy/commit/28b2457a53bf31163e94a363f2009b811e1b0b76)

### Bugfixes

- The token introspection endpoint was only accepting requests with `Json` data, when it should have
  instead been with `Form` data.

## 0.24.1

The last weeks were mostly for updating the documentation and including all the new features that
came to Rauthy in the last months. Some small things are still missing, but it's almost there.

Apart from that, this is an important update because it fixes some security issues in external
dependencies.

### Security

Security issues in external crates have been fixed:

- moderate [matrix-sdk-crypto](https://github.com/sebadob/rauthy/security/dependabot/54)
- moderate [openssl](https://github.com/sebadob/rauthy/security/dependabot/55)
- low [vodozemac](https://github.com/sebadob/rauthy/security/dependabot/53)

### Changes

#### `S3_DANGER_ACCEPT_INVALID_CERTS` renamed

The config var `S3_DANGER_ACCEPT_INVALID_CERTS` has been renamed to `S3_DANGER_ALLOW_INSECURE`. This
is not a breaking change right now, because for now Rauthy will accept both versions to not
introduce a breaking change, but the deprecated values will be removed after v0.24.

#### S3 Compatibility

Quite a few internal dependencies have been updated to the latest versions (where it made sense).

One of them was my own [cryptr](https://github.com/sebadob/cryptr). This was using the `rusty-s3`
crate beforehand, which is a nice one when working with S3 storages, but had 2 issues. One of them
is that it is using pre-signed URLs. That is not a flaw in the first place, just a design decision
to become network agnostic. The other one was that it signed the URL in a way that would make the
request not compatible with [Garage](https://garagehq.deuxfleurs.fr/).   
I migrated `cryptr` to my own [s3-simple](https://github.com/sebadob/s3-simple) which solves these
issues.

This update brings compatibility with the `garage` s3 storage for Rauthy's S3 backup feature.

[f1eab35](https://github.com/sebadob/rauthy/commit/f1eab35dcbf195ed38d11756e1df69f42e05e7e0)

### Bugfixes

- Fetching the favicon (and possibly other images) was forbidden because of the new CSRF middleware
  from some weeks ago.
  [76cd728](https://github.com/sebadob/rauthy/commit/76cd7281fcd1493c9f0cbb208c3fa7ef93814422)
- The UI and the backend had a difference in input validation for `given_name` and `family_name`
  which could make some buttons in the UI get stuck. This has been fixed and the validation for
  these 2 is the same everywhere and at least 1 single character is required now.
  [19d512a](https://github.com/sebadob/rauthy/commit/19d512ad6ea930467f51d7b704252d3edee7ef1c)

## v0.24.0

Many thousands of lines have been refactored internally to provide better maintainability in the
future. These are not mentioned separately, since they did not introduce anything new. Apart from
this, there are only small changes, but one of them is an important breaking change.

### Breaking

#### `TRUSTED_PROXIES` Config Variable

The new config variable `TRUSTED_PROXIES` introduces a breaking change in some cases.  
If you are running Rauthy with either `PROXY_MODE=true` or with a set `PEER_IP_HEADER_NAME` value,
you must add the `TRUSTED_PROXIES` to your existing config before updating.

This value specifies trusted proxies in the above situation. The reason is that Rauthy extracts the
client IP from the HTTP headers, which could be spoofed if they are used without validating the
source. This was not a security issue, but gave an attacker the ability to blacklist or rate-limit
IPs that do not belong to him.

When `PROXY_MODE=true` or set `PEER_IP_HEADER_NAME`, Rauthy will now only accept direct connections
from IPs specified with `TRUSTED_PROXIES` and block all other requests. You can provide a list of
CIDRs to have full flexibility for your deployment.

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
Keep in mind, that you must include IPs for direct health checks like for instance inside Kubernetes
here, if they are not being sent via a trusted proxy.

[e1ae491](https://github.com/sebadob/rauthy/commit/e1ae49164f9753e3ec57cb4b6e2aed8614227bce)

### Features

#### User Registration Domain Blacklisting

If you are using an open user registration without domain restriction, you now have the possibility
to blacklist certain E-Mail provider domains. Even if your registration endpoint allows
registrations, this blacklist will be checked and deny requests with these domains.   
This is mainly useful if you want to prevent malicious E-Mail providers from registering and
spamming your database.

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

Even though it was not needed so far, the OIDC userinfo endpoint now has a proper `POST` handler in
addition to the existing `GET` to comply with the RFC.  
[05a8793](https://github.com/sebadob/rauthy/commit/05a8793f013f864db1855ae0ee1c848ec36b9254)

### Bugfixes

- The upstream crate `curve25519-dalek` had a moderate timing variability security issue
  [8bb4069](https://github.com/sebadob/rauthy/commit/8bb4069e5f22439b08f9f5a51059561007b90af3)

## v0.23.5

### Upstream IdP Locale Fix

This patch fixes a regression from fixing the special characters encoding in upstream IdP JWT
tokens. A panic was possible when the upstream IdP did not include a `locale` in the `id_token`.  
[ea24e7e](https://github.com/sebadob/rauthy/commit/ea24e7e73f883446a30d7c4f8e38837148da9732)  
[481c9b3](https://github.com/sebadob/rauthy/commit/481c9b36d3304a2100c2c4906013224c8a5a934f)

## v0.23.4

This is a tiny update, but brings an important bugfix for upstream IdPs.

### Bugfix

A bug has been fixed in case an upstream IdP included special characters inside Strings in the
returned JWT token after a successful user login flow.  
Since JWT tokens should use UNICODE encoding in these cases, it is not possible to do zero-copy
deserialization into Rust UTF8 string slices in that case. This has been fixed in a way, that only
when there are existing special characters, Rauthy will now do the additional, necessary String
allocations for the deserialization process.   
This should fix current issues when logging in via an upstream IdP with special characters inside
the E-Mail address for instance.  
[aa97cb8](https://github.com/sebadob/rauthy/commit/aa97cb8ba2100f540d48e98aa597c11963c84be3)

Apart from that, there were some minor UX improvements for the Admin UI providers setup page like
earlier client side checking of variables and preventing form submission when some required ones
were missing.  
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

More strict checking and validation for `allowed_origins` has been implemented, when you configure
clients. Before, the regex only checked for the input to be a valid URI, which is not strict enough
for validation an origin.  
This should improve the UX and prevents hard to debug bugs, when someone enters an invalid origin.

At the same time, a better visual separation has been added to the Origins / URI section in the UI
when configuring clients.

[55704f3](https://github.com/sebadob/rauthy/commit/55704f3cd3a5dcf04e5796a66ab4aba48b8c70dd)  
[d993d42](https://github.com/sebadob/rauthy/commit/d993d420fae628b069ac3857dfc1e69d812b16f7)  
[8d4e455](https://github.com/sebadob/rauthy/commit/8d4e455aa9455418aa8fc90612b707da1d72ce57)

#### Small performance optimizations

Small improvements have been made in a lot of places, which resulted in less memory allocations.  
[9144f2a](https://github.com/sebadob/rauthy/commit/9144f2afad3d1052bc4a927d0cbeaf24c6ea5589)

#### POST `/authorize` simplification

The logic on POST `/authorize` has been simplified internally. The code grew to an over-complicated
state with new features coming in all the time until a point where it was hard to follow. This has
been simplified. This makes the software better maintainable in the future.  
[af0db9d](https://github.com/sebadob/rauthy/commit/af0db9d0cbe560de327db60e33b61ccf32e33776)

### Bugfix

- add all `/fed_cm/` routes as exceptions to the new CSRF protection middleware
  [360ce46](https://github.com/sebadob/rauthy/commit/360ce46c19bad81ee60de817f3b3f74f0dd3c408)
- upstream auth provider templates could get stuck in the UI when switching between them
  [d2b928a](https://github.com/sebadob/rauthy/commit/d2b928ad9d34302f0ef2af7830d19cadaf784d5a)
- when a problem with an upstream provider occurs on `/callback`, you will now see the detailed
  error in the UI
  [8041c95](https://github.com/sebadob/rauthy/commit/8041c95b57292ac83330c7613698398857864e30)

## v0.23.2

This release brings some very minor features and bugfixes.

### Changes

#### New CSRF protection middleware

CSRF protection was there already without any issues.  
However, a new middleware has been added to the whole routing stack in addition to the existing
checks. This provides another defense in depth. The advantage of the new middleware is, that this
can be enforced all the way in the future after enough testing in parallel.  
If this works fine without any issues, we might get rid of the current way of doing it and only use
the new middleware, which is easier to maintain and to work with.

To not break any existing deployments and make sure I did not forget route exceptions for the new
middleware, you can set it to warn-only mode for this minor release. This option will be removed in
future releases though and should only be a temporary solution:

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

This is not really considered a new feature, but Rauthy now has experimental support for FedCM in
its current state. This is opt-in and disabled by default. You should not attempt to use it in
production because the FedCM implementation itself still has a few bumps and sharp edges.  
The only reason the experimental support is there is to help smooth out these things and hopefully
have FedCM as a really nice addition. It does not really bring any new possibilities to the table,
but it would improve the UX quite a bit, if it hopefully turns out great.

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

The input validation for ephemeral `client_id`s has been relaxed. This now makes it possible to test
them with OIDC playgrounds, which typically generate pretty long testing URLs, which were being
rejected for their length beforehand.
Rauthy now accepts URLs of up to 256 characters as `client_id`s.  
[62405bb](https://github.com/sebadob/rauthy/commit/62405bbad7230f2e3af864b004081a90ab505f6f)

#### Bumped Argon2ID defaults

The default values for the Argon2ID hashing algorithm have been bumped up quite a bit. Rauthy's goal
is to be as secure as possible by default. The old values were quite a bit above the OWASP
recommendation, but still way too low imho. The values will of course still need tuning and
adjustment to the target architecture / deployment, but they provide a way better starting point and
can be considered really secure even if not adjusted.

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

- Ephemeral client's now work properly with the `/userinfo` endpoint in strict-validation mode.
  Their validation is simply being skipped at that point, because it does not make much sense to do
  an `enabled` check at that point.
  [90b0367](https://github.com/sebadob/rauthy/commit/90b03677ffd3b99372ac9496540449b302af66d5)
- A small bug appeared in the UI after you have added new custom user attributes. Instead of
  resetting the input values to empty strings after the registration, they were set to undefined.
  [ab77595](https://github.com/sebadob/rauthy/commit/ab775958dec70eee3b2915fe2faa8b4a9816ec2e)
- Because of a bug in the account overview UI, it was not possible to link an already existing
  account to an upstream IdP after the registration.
  [22751ee](https://github.com/sebadob/rauthy/commit/22751ee6e9b31361d2ee5047a8c8795518d30745)

## v0.23.1

### Features

#### Global Cookie Encryption

All Rauthy cookies (except for the locale) are now encrypted globally inside the whole app by
default.  
This is just another defense in depth. The AEAD algorithm makes sure, that you can't tamper with the
cookie values, even if you would try to do it manually.

[4fdb3f2](https://github.com/sebadob/rauthy/commit/4fdb3f262f128fe803d1542418c06eff08bf210c)

#### Easier extraction of CSRF tokens with external Frontend

If you are in the situation where you run Rauthy behind a reverse proxy on the exact same origin
with another app, and you want to build custom user facing UI parts, you had to retrieve the
original HTML for `/authorize` or the password reset to extract the CSRF token from the HTML
content.  
Doing this in tests is fine, but very tedious and wasteful for a production deployment.

For this reason, there are now 2 new possibilities:

- POST `/oidc/session` endpoint to create a session in `Init` state, which will return the cookie
  and the correct CSRF token in a json body
- the password reset link returns a json with a CSRF token instead of an HTML document, if you
  request it with a `Accept: application/json` header

[c37e1f5](https://github.com/sebadob/rauthy/commit/c37e1f5bc27ebd679c4424ef568a8bae82c523bc)

### Bugfix

- the password expiry reminder E-Mail had a wrong a link to the account page, a left over from older
  versions with `.html` appended
  [d728317](https://github.com/sebadob/rauthy/commit/d728317ef31e20a117a0ca2a903e767e34c556d4)

## v0.23.0

This release does the first preparations to prepare a future v1.0.0 release.  
Quite a few values have been cleaned up or improved.

### Breaking

#### `rauthy-client` compatibility

If you are using the [rauthy-client](https://crates.io/crates/rauthy-client), you should upgrade to
`v0.4.0` before upgrade Rauthy to `v0.23.0`. Any older client version will not understand the new
grant type for theOAuth2 Device Authorization grant.

#### Removal of `UNSAFE_NO_RESET_BINDING` in favor of `PASSWORD_RESET_COOKIE_BINDING`

The config variable `UNSAFE_NO_RESET_BINDING` has been removed in favor of
`PASSWORD_RESET_COOKIE_BINDING`. The logic for this security feature has been reversed. The default
behavior until now was to block subsequent requests to the password reset form if they provided an
invalid binding cookie. This created issues for people that were using evil E-Mail providers. These
would scan their users E-Mails and use links inside them. This link usage however made it impossible
for "the real user" to use the link properly, because it has been used already by its provider.  
In some cases, this hurts the UX more than it is a benefit to the security, so this feature is now
an opt-in hardening instead of opt-out evil provider error fixing.  
Additionally, to improve the UX even further, the additional E-Mail input form has been removed from
the password reset page as well. The security benefits of this were rather small compared to the UX
degradation. #365
[1af7b92](https://github.com/sebadob/rauthy/commit/1af7b92204a99de4883154055bb3081dc196d759)

#### Removal of `OFFLINE_TOKEN_LIFETIME` config var

`OFFLINE_TOKEN_LIFETIME` has been removed from the config. This variable has been deprecated since a
lof of versions now. The `offline_access` scope was not even allowed via the UI for a long time now,
so these offline tokens were never issued anyway.  
The "new" mechanism Rauthy uses with the switch in the Admin UI to issue / allow refresh tokens for
a client is much more clear, since the `offline_access` scope produces a lot of confusion for people
new to OIDC. From the name, it simply makes no sense that you need to activate `offline_access` to
get a refresh token. Having an option named "allow refresh tokens" is just so much better.
[71db7fe](https://github.com/sebadob/rauthy/commit/71db7fef18568a599f30cae6e494bba40cb33e7d)

#### Change in `GET /clients/{id}/secret`

If you used the endpoint for retrieving a client secret with an API key before, you need to change
the method. The endpoint works exactly the same, but the method has been changed from a `GET` to a
`POST` to request and validate the additional CSRF token from the Admin UI.  
[72f077f](https://github.com/sebadob/rauthy/commit/72f077f462e4b28624d101510cfb50b64700e425)

#### Removal of the `Refresh Token` switch in Admin UI

The `Refresh Token` switch for a client config in the Admin UI has been removed.  
The old behavior was misleading and unintuitive, I just got rid of that switch.

If you want to use the refresh flow with a client, the only thing you need to do is to allow the
`refresh_token` flow. You needed to do this before anyway, but in addition enable the switch further
down below. So this is not really a breaking change, but could lead to confusion, if this switch is
just gone.

[2ece6ed](https://github.com/sebadob/rauthy/commit/2ece6ed6da214b353cc7c9adfbe7904c0f2f6bce)

### Features

#### OAuth 2.0 Device Authorization Grant

This release brings support for the OAuth 2.0 Device Authorization Grant.  
On top of the default RFC spec, we have some additional features like optional rate limiting and
being able to do the flow with confidential clients as well.  
The [rauthy-client](https://crates.io/crates/rauthy-client) has the basics implemented as well for
fetching tokens via the `device_code` flow. An automatic refresh token handler is
on the TODO list though. A
small [example](https://github.com/sebadob/rauthy/blob/main/rauthy-client/examples/device-code/src/main.rs)
exists as well.  
You will find new sections in the account and admin -> user view, where you can see all linked
devices, can give them a friendly name and revoke refresh tokens, if they exist.

[544bebe](https://github.com/sebadob/rauthy/commit/544bebe162797870401ae60ad98dfb8cb6ecae92)
[8d028bf](https://github.com/sebadob/rauthy/commit/8d028bf6273819395d946bc13f2215ec6289a8b6)
[e8077ce](https://github.com/sebadob/rauthy/commit/e8077ce2f6c6c21d0e83ba531efe2bf5ef1c6d84)
[62d41bc](https://github.com/sebadob/rauthy/commit/62d41bc2c1eb0e9b6b85f1c4528b333f8d6fb97e)
[51a50ac](https://github.com/sebadob/rauthy/commit/51a50ac20118a87dd2914006e858bf159bcf4675)
[9352b3c](https://github.com/sebadob/rauthy/commit/9352b3c73885f4467e209c1d797116216562c256)

#### Dynamic Server Side Search + Pagination

Until now, the Admin UI used client side searching and pagination. This is fine for most endpoints,
but the users can grow quite large depending on the instance while all other endpoints will return
rather small "GET all" data.   
To keep big Rauthy instances with many thousands of users fast and responsive, you can set a
threshold for the total users count at which Rauthy will dynamically switch from client side to
server side pagination and searching for the Admin UI's Users and Sessions page.

```
# Dynamic server side pagination threshold
# If the total users count exceeds this value, Rauthy will dynamically
# change search and pagination for users in the Admin UI from client
# side to server side to not have a degradation in performance.
# default: 1000
SSP_THRESHOLD=1000
```

For smaller instances, keeping it client side will make the UI a bit more responsive and snappy.
For higher user counts, you should switch to do this on the server though to keep the UI fast and
not send huge payloads each time.

[b4dead3](https://github.com/sebadob/rauthy/commit/b4dead36169cc284c97af5a982cc33fb8a0be02b)
[9f87af3](https://github.com/sebadob/rauthy/commit/9f87af3dfb49b48300b885bf406f852579470193)
[e6d39d1](https://github.com/sebadob/rauthy/commit/e6d39d1e1118e18aeb020fbbb477a944fcd1467a)

#### UX Improvement on Login

The login form now contains a "Home" icon which will appear, if a `client_uri` is registered for the
current client. A user may click this and be redirected to the client, if a login is not desired for
whatever reason. Additionally, if the user registration is configured to be open, a link to the user
registration will be shown at the bottom as well.

[b03349c](https://github.com/sebadob/rauthy/commit/b03349c9d3f998aaecd3e4177c7b62bda067bf8b)
[b03349c](https://github.com/sebadob/rauthy/commit/b03349c9d3f998aaecd3e4177c7b62bda067bf8b)

#### Unlink Account from Provider

A new button has been introduced to the account view of federated accounts.  
You can now "Unlink" an account from an upstream provider, if you have set it up with at least
a password or passkey before.

[8b1d9a8](https://github.com/sebadob/rauthy/commit/8b1d9a882b0d4b059f3ed884deaacfcdeb109856)

#### Link Existing Account to Provider

This is the counterpart to the unlink feature from above.
This makes it possible to link an already existing, unlinked user account to an upstream auth
provider. The only condition is a matching `email` claim after successful login. Apart from that,
there are quite a few things going on behind the scenes and you must trigger this provider link from
an authorized, valid session from inside your user account view. This is necessary to prevent
account takeovers if an upstream provider has been hacked in some way.

[fdc683c](https://github.com/sebadob/rauthy/commit/fdc683cec0181e03bb86da1e42fff213715718f0)

#### Bootstrap default Admin in production

You can set environment variables either via `rauthy.cfg`, `.env` or as just an env var during
initial setup in production. This makes it possible to create an admin account with the very first
database setup with a custom E-Mail + Password, instead of the default `admin@localhost.de` with
a random password, which you need to pull from the logs. A single API Key may be bootstrapped as
well.

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

You can now set a new config variable called `USERINFO_STRICT`. If set so true, Rauthy will do
additional validations on the `/userinfo` endpoint and actually revoke (even otherwise still valid)
access tokens, when any user / client / device it has been issued for has been deleted, expired or
disabled. The non-strict mode will simply make sure the token is valid and that the user still
exists. The additional validations will consume more resources because they need 1-2 additional
database lookups but will provide more strict validation and possible earlier token revocation. If
you don't need it that strict, and you are resource constrained, set it to `false`

[198e7f9](https://github.com/sebadob/rauthy/commit/198e7f957c32fef5f0f786b145408f7d625f20ce)

#### `at_hash` in `id_token`

The Rauthy `id_token` now contains the access token hash `at_hash` claim. This is needed for
additional downstream validation, if a client provides both tokens and they are not coming from
Rauthy directly. With the additional validation of the `at_hash` claim, clients can be 100% sure,
that a given `id_token` belongs to a specific `access_token` and has not been swapped out.

[d506865](https://github.com/sebadob/rauthy/commit/d506865898e61fce45e5cf4c754ad4300bd37161)

#### Better roles, groups and scopes names

The allowed names for roles, groups and scopes have been adjusted. Rauthy allows names of up to 64
characters now and containing `:` or `*`. This will make it possible to define custom scopes with
names like `urn:matrix:client:api:guest` or `urn:matrix:client:api:*`.

[a5982d9](https://github.com/sebadob/rauthy/commit/a5982d91f37a2f2917ed4215dc6ded216dc0fd69)
[50d0214](https://github.com/sebadob/rauthy/commit/50d021440eb50473977ec851a46c0bc979bbd12b)

#### Configurable Cookie Security

Depending on your final deployment, you may want to change the way Rauthy's set's its cookies, for
instance if you want to create your own UI endpoints but still want to be able to communicate with
the API.

The default cookie setting has been changed in a way that all cookies will have the `__Host-` prefix
now, which provides the highest level of security. There might be cases where you don't want this
and rather have the path restriction to `/auth` from before, for instance when you host an
additional app on the same origin behind a reverse proxy, that should not be able to read Rauthy's
cookies.

And finally, for all Safari users, since Safari does not consider `localhost` to be secure when
testing, you can even set insecure cookies for testing purposes.

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
By default, IPs from such requests will be blacklisted for 24 hours, but you can of course configure
this.

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

The whoami endpoint has been changed. It does not return all headers anymore, because this could
possibly leak sensitive headers in some environments, especially with the new auth headers feature
in some situations.  
Instead, it only returns the peer IP that Rauthy extracted for this request. This can be very
helpful if you need to configure the extraction, for instance when you are behind a reverse proxy or
CDN.

[758b31c](https://github.com/sebadob/rauthy/commit/758b31cb5dc2277a0cc3ec31f15b5de90ff00ea7)

#### New sorting options for users

Users in the Admin UI can now be sorted by their `created_at` or `last_login` timestamp.  
Users that never have logged in will always be at the end of the list, since this value might be
`undefined`.  
[4c41d64](https://github.com/sebadob/rauthy/commit/4c41d64f570dbbe94dcb8b681ac31608ed492652)

### Bugfixes

- The button for requesting a password reset from inside a federated account view has been
  disabled when it should not be, and therefore did not send out requests.
  [39e585d](https://github.com/sebadob/rauthy/commit/39e585d1d53a2490b273ba5c33b864ec0d7835d5)
- A really hard to reproduce bug where the backend complained about a not-possible mapping
  from postgres `INT4` to Rust `i64` as been fixed. This came with the advantage of having
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
- `/authorize` for logins had a bit too strict validation for the user password, which had a chance
  that a new password a user just set, would be rejected because of some invalid special chars not
  being allowed
  [9bb0a72](https://github.com/sebadob/rauthy/commit/9bb0a72fe2e3cc87e000b6db36c84fcf2d255bf5)
- when resources in the Admin UI have been re-fetched, for instance because of a user deletion, the
  search input has not been emptied
  [033db25](https://github.com/sebadob/rauthy/commit/033db25db695d2565bc4adcdfe07a77125c2a9a5)
- the deprecated `x-xss-protection` header has been removed
  [5008438](https://github.com/sebadob/rauthy/commit/50084385df887f4782bbe9224e63bc60719600fd)

## 0.22.1

### Security

This version fixes
a [potential DoS in rustls](https://rustsec.org/advisories/RUSTSEC-2024-0336.html) which has
been found yesterday.  
[f4d65a6](https://github.com/sebadob/rauthy/commit/f4d65a6b056183f914075d6047384e2a7a4f0329)

### Features

#### Dedicated `/forward_auth` + Trusted Authn/Authz Headers

In addition to the `/userinfo` endpoint specified in the OIDC spec, Rauthy implements an additional
endpoint specifically for ForwardAuth situations. You can find it at `/auth/v1/oidc/forward_auth`
and it can be configured to append optional Trusted Header with User Information for downstream
applications, that do not support OIDC on their own.

The HeaderNames can be configured to match your environment. Please keep in mind, that you should
only use these, if you legacy application does not support OIDC natively, because Auth Headers come
with a lot of pitfalls, when your environment is not configured properly.

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

- allow CORS requests for the GET PoW and the user sign up endpoint's to make it possible to build a
  custom UI without having a server side. At the same time, the method for requesting a PoW **has
  been changed from `GET` to `POST`**. This change has been done because even though only in-memory,
  a request would create data in the backend, which should never be done by a `GET`.
  Technically, this is a breaking change, but since it has only been available from the Rauthy UI
  itself because of the CORS header setting, I decided to only bump the patch, not the minor
  version.
  [e4d935f](https://github.com/sebadob/rauthy/commit/e4d935f7b51459031a37fb2ec2eb9952bc278f2e)

## v0.22.0

### Breaking

There is one breaking change, which could not have been avoided.  
Because of a complete rewrite of the logic how custom client logos (uploaded via
`Admin UI -> Clients -> Client Config -> Branding`), you will loose custom logos uploaded in the
past for a client. The reason is pretty simple. Just take a look at `Auto Image Optimization` below.

Apart from this, quite a few small internal improvements have been made to make life easier for
developers and new contributors. These changes are not listed in the release notes.

### Changes

#### Upstream Auth Providers

Rauthy v0.22.0 brings (beta) support for upstream authentication providers.  
This is a huge thing. It will basically allow you to set up things like *Sign In with Github* into
Rauthy. You could use your Github account for signup and login, and manage custom groups, scopes,
and so on for the users on Rauthy. This simplifies the whole onboarding and login for normal users a
lot.

You can add as many auth providers as you like. They are not statically configured, but actually
configurable via the Admin UI. A user account can only be bound to one auth provider though for
security reasons. Additionally, when a user already exists inside Rauthy's DB, was not linked to an
upstream provider and then tries a login but produces an email conflict, the login will be rejected.
It must be handled this way, because Rauthy can not know for sure, if the upstream email was
actually been verified. If this is not the case, simply accepting this login could lead to
account takeover, which is why this will not allow the user to login in that case.  
The only absolutely mandatory information, that Rauthy needs from an upstream provider, is an
`email` claim in either the `id_token` or as response from the userinfo endpoint. If it cannot find
any `name` /`given_name` / `family_name`, it will simply insert `N/A` as values there. The user will
get a warning on his next values update to provide that information.

The supported features (so far) are:

- auto OpenID Connect metadata discovery
- accept invalid TLS certs for upstream clients, for instance inside self-hosted environments
- provide a root certificate for an upstream client for the same reason as above
- choose a template for the config (currently Google and Github exist)
- fully customized endpoint configuration if the provider does not support auto-lookup
- optional mfa claim mapping by providing a json parse regex:  
  If the upstream provider returns information about if the user has actually done at least a 2FA
  sign in, Rauthy can extract this information dynamically from the returned JSON. For instance,
  Rauthy itself will add an `amr` claim to the `id_token` and you can find a value with `mfa` inside
  it, if the user has done an MFA login.  
  Github returns this information as well (which has been added to the template).
- optional `rauthy_admin` claim mapping:
  If you want to allow full rauthy admin access for a user depending on some value returned by the
  upstream provider, you can do a mapping just like for the mfa claim above.
- upload a logo for upstream providers Rauthy does not (and never will do) an automatic logo
  download from a provider, because this logo will be shown on the login page and must be trusted.
  However, if Rauthy would download any arbitrary logo from a provider, this could lead to code
  injection into the login page. This is why you need to manually upload a logo after configuration.

**Note:**  
If you are testing this feature, please provide some feedback
in [#166](https://github.com/sebadob/rauthy/issues/166) in any case - if you have errors or not. It
would be nice to know about providers that do work already and those, that might need some
adoptions. All OIDC providers should work already, because for these we can rely on standards and
RFCs, but all others might produce some edge cases and I simply cannot test all of them myself.  
If we have new providers we know of, that need special values, these values would be helpful as
well, because Rauthy could provide a template in the UI for these in the future, so please let me
know.

#### Auto Image Optimization

The whole logic how images are handled has been rewritten.
Up until v0.21.1, custom client logos have been taken as a Javascript `data:` url because of easier
handling. This means however, that we needed to allow `data:` sources in the CSP for `img-src`,
which can be a security issue and should be avoided if possible.

This whole handling and logic has been rewritten. The CSP hardening has been finalized by removing
the `data:` allowance for `img-src`. You can still upload SVG / JPG / PNG images under the client
branding (and for the new auth providers). In the backend, Rauthy will actually parse the image
data, convert the images to the optimized `webp` format, scale the original down and save 2
different versions of it. The first version will be saved internally to fit into 128x128px
for possible later use, the second one even smaller. The smaller version will be the one actually
being displayed on the login page for Clients and Auth Providers.   
This optimization reduces the payload sent to clients during the login by a lot, if the image has
not been manually optimized beforehand. Client Logos will typically be in the range of ~5kB now
while the Auth Providers ones will usually be less than 1kB.

[6ccc541](https://github.com/sebadob/rauthy/commit/6ccc541b0e6d155024aa9ba4d832dcb01f135528)

#### Custom Header Name for extracting Client IPs

With the name config variable `PEER_IP_HEADER_NAME`, you can specify a custom header name which will
be used for extracting the clients IP address. For instance, if you are running Rauthy behind a
Cloudflare proxy, you will usually only see the IP of the proxy itself in the `X-FORWARDED-FOR`
header. However, cloudflare adds a custom header called `CF-Connecting-IP` to the request, which
then shows the IP you are looking for.    
Since it is very important for rate limiting and blacklisting that Rauthy knows the clients IP, this
can now be customized.

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

For each client, you can now specify contacts and a URI, where the application is hosted. These
values might be shown to users during login in the future. For Rauthy itself, the values will be set
with each restart in the internal anti lockout rule. You can specify the contact via a new config
variable:

```
# This contact information will be added to the `rauthy`client
# within the anti lockout rule with each new restart.
RAUTHY_ADMIN_EMAIL="admin@localhost.de"
```

[6ccc541](https://github.com/sebadob/rauthy/commit/6ccc541b0e6d155024aa9ba4d832dcb01f135528)

#### Custom `redirect_uri` During User Registration

If you want to initiate a user registration from a downstream app, you might not want your users to
be redirected to their Rauthy Account page after they have initially set the password. To encounter
this, you can redirect them to the registration page and append a
`?redirect_uri=https%3A%2F%2Frauthy.example.com` query param. This will be saved in the backend
state and the user will be redirected to this URL instead of their account after they have set
their password.

#### Password E-Mail Tempalte Overwrites

You can not overwrite the template i18n translations for the NewPassword and ResetPassword E-Mail
templates.  
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
- The default path to TLS certificates inside the container image has been fixed in the deploy cfg
  template. This makes it possible now to start the container for testing with TLS without
  explicitly specifying the path manually.
  [3a04dc0](https://github.com/sebadob/rauthy/commit/3a04dc02a878263cec2d841553747c78a41b7c4a)
- The early Passkey implementations of the Bitwarden browser extension seem to have not provided all
  correct values, which made Rauthy complain because of not RFC-compliant requests during Passkey
  sign in. This error cannot really be reproduced. However, Rauthy tries to show more error
  information to the user in such a case.
  [b7f94ff](https://github.com/sebadob/rauthy/commit/b7f94ff8ad3cd9aad61baf3710e4f9788c498ca6)
- Don't use the reset password template text for "new-password emails"
  [45b4160](https://github.com/sebadob/rauthy/commit/45b41604b1e0c0c9af423be65021953116b88150)

## v0.21.1

### Changes

- host `.well-known additionally` on root `/`
  [3c594f4](https://github.com/sebadob/rauthy/commit/3c594f4628c8d301ed1556d5dd3ea5a004afdcac)

### Bugfix

- Correctly show the `registration_endpoint` for dynamic client registration in the
  `openid-configuration`
  if it is enabled.
  [424fdd1](https://github.com/sebadob/rauthy/commit/424fdd10e57639c1d60dde61f551462e67ff4934)

## v0.21.0

### Breaking

The access token's `sub` claim had the email as value beforehand. This was actually a bug.  
The `sub` of access token and id token must be the exact same value. `sub` now correctly contains
the user ID, which is 100% stable, even if the user changes his email address.  
This means, if you used the `sub` from the access token before to get the users email, you need to
pay attention now. The `uid` from the access token has been dropped, because this value is now in
`sub`. Additionally, since many applications need the email anyway, it makes sense to have it inside
the access token. For this purpose, if `email` is in the requested `scope`, it will be mapped to the
`email` claim in the access token.

### Features

#### OpenID Core Compatibility

Rauthy should now be compliant with the mandatory part of the OIDC spec.
A lot of additional things were already implemented many versions ago.
The missing thing was respecting some additional params during GET `/authorize`.

#### OpenID Connect Dynamic Client Registration

Rauthy now supports Dynamic Client registration as
defined [here](https://openid.net/specs/openid-connect-registration-1_0.html).

Dynamic clients will always get a random ID, starting with `dyn$`, followed by a random alphanumeric
string, so you can distinguish easily between them in the Admin UI.  
Whenever a dynamic client does a `PUT` on its own modification endpoint with the
`registration_token` it received from the registration, the `client_secret` and the
`registration_token` will be rotated and the response will contain new ones, even if no other value
has been modified. This is the only "safe" way to rotate secrets for dynamic clients in a fully
automated manner. The secret expiration is not set on purpose, because this could easily cause
troubles, if not implemented properly on the client side.   
If you have a badly implemented client that does not catch the secret rotation and only if you
cannot fix this on the client side, maybe because it's not under your control, you may deactivate
the auto-rotation with `DYN_CLIENT_SECRET_AUTO_ROTATE=false`. Keep in mind, that this reduces the
security level of all dynamic clients.

Bot and spam protection is built-in as well in the best way I could think of. This is disabled, if
you set the registration endpoint to need a `DYN_CLIENT_REG_TOKEN`. Even though this option exists
for completeness, it does not really make sense to me though. If you need to communicate a token
beforehand, you could just register the client directly. Dynamic clients are a tiny bit less
performant than static ones, because we need one extra database round trip on successful token
creation to make the spam protection work.   
However, if you do not set a `DYN_CLIENT_REG_TOKEN`, the registration endpoint would be just open to
anyone. To me, this is the only configuration for dynamic client registration, that makes sense,
because only that is truly dynamic. The problem then are of course bots and spammers, because they
can easily fill your database with junk clients. To counter this, Rauthy includes two mechanisms:

- hard rate limiting - After a dynamic client has been registered, another one can only be
  registered
  after 60 seconds (default, can be set with `DYN_CLIENT_RATE_LIMIT_SEC`) from the same public IP.
- auto-cleanup of unused clients - All clients, that have been registered but never used, will be
  deleted
  automatically 60 minutes after the registration (default, can be set with
  `DYN_CLIENT_CLEANUP_MINUTES`).

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

This is a small UX improvement in some situations. If a downstream client needs a user to log in,
and it knows the users E-Mail address somehow, maybe because of an external initial registration, It
may append the correct value with appending the `login_hint` to the login redirect. If this is
present, the login UI will pre-fill the E-Mail input field with the given value, which make it one
less step for the user to log in.

### Changes

- The `/userinfo` endpoint now correctly respects the `scope` claim from withing the given `Bearer`
  token and provides more information. Depending on the `scope`, it will show the additional user
  values that were introduced with v0.20
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
[cryptr](https://github.com/sebadob/cryptr) were causing a crash at startup for a fresh
installation. This is the only thing that has been fixed with this version. They are now simply
ignored and a warning is logged into the console at the very first startup.

## v0.20.0

### Breaking

This update is not backwards-compatible with any previous version. It will modify the database under
the hood which makes it incompatible with any previous version. If you need to downgrade for
whatever reason, you will only be able to do this by applying a database backup from an older
version. Testing has been done and everything was fine in tests. However, if you are using Rauthy in
production, I recommend taking a database backup, since any version <= v0.19 will not be working
with a v0.20+ database.

### IMPORTANT Upgrade Notes

If you are upgrading from any earlier version, there is a manual action you need to perform, before
you can start v0.20.0. If this has not been done, it will simply panic early and not start up.
Nothing will get damaged.

The internal encryption of certain values has been changed. Rauthy now
uses [cryptr](https://github.com/sebadob/cryptr) to handle these things, like mentioned below as
well.

However, to make working with encryption keys easier and provide higher entropy, the format has
changed. You need to convert your currently used `ENC_KEYS` to the new format:

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

**4. cryptr will output the correct format for either usage in config or as kubernetes secret again
**

**5. Paste the new format into your Rauthy config / secret and restart.**

#### Option 2: Manual

Rauthy expects the `ENC_KEYS` now base64 encoded, and instead of separated by whitespace it expects
them to be separated by `\n` instead.   
If you don't want to use `cryptr` you need to convert your current keys manually.

For instance, if you have

```
ENC_KEYS="bVCyTsGaggVy5yqQ/S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4 q6u26onRvXVG4427/3CEC8RJWBcMkrBMkRXgx65AmJsNTghSA"
```

in your config, you need to convert the enc key itself, the value after the `/`, to base64, and then
separate
them with `\n`.

For instance, to convert `bVCyTsGaggVy5yqQ/S9n7oCen53xSJLzcsmfdnBDvNrqQ63r4`, split off the enc key
part
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
Make sure to not add any newline characters or spaces when copying values around when doing the
bas64 encoding!

### Encrypted SQLite backups to S3 storage

Rauthy can now push encrypted SQLite backups to a configured S3 bucket.
The local backups to `data/backups/` do still exist. If configured, Rauthy will now push backups
from SQLite
to an S3 storage and encrypt them on the fly. All this happens with the help
of [cryptr](https://github.com/sebadob/cryptr)
which is a new crate of mine. Resource usage is minimal, even if the SQLite file would be multiple
GB's big.
The whole operation is done with streaming.

### Auto-Restore SQLite backups

Rauthy can now automatically restore SQLite backups either from a backup inside `data/backups/`
locally, or fetch an encrypted backup from an S3 bucket. You only need to set the new
`RESTORE_BACKUP` environment variable at startup and Rauthy will do the rest. No manually copying
files around.  
For instance, a local backup can be restored with setting
`RESTORE_BACKUP=file:rauthy-backup-1703243039` and an
S3 backup with `RESTORE_BACKUP=s3:rauthy-0.20.0-1703243039.cryptr`.

### Test S3 config at startup

To not show unexpected behavior at runtime, Rauthy will initialize and test a configured S3
connection at startup. If anything is not configured correctly, it will panic early. This way, when
Rauthy starts and the tests are successful, you know it will be working during the backup process at
night as well, and it will not crash and throw errors all night long, if you just had a typo
somewhere.

### Migration to `spow`

The old (very naive) Proof-of-Work (PoW) mechanism for bot and spam protection has been migrated to
make use of the [spow](https://github.com/sebadob/spow) crate, which is another new project of mine.
With this implementation, the difficulty for PoW's a client must solve can be scaled up almost
infinitely, while the time is takes to verify a PoW on the server side will always be `O(1)`, no
matter how high the difficulty was. `spow` uses a modified version of the popular Hashcat PoW
algorithm, which is also being used in the Bitcoin blockchain.

### Separate users cache

A typical Rauthy deployment will have a finite amount of clients, roles, groups, scopes, and so on.
The only thing that might scale endlessly are the users. Because of this, the users are now being
cached inside their own separate cache, which can be configured and customized to fit the
deployment's needs. You can now set the upper limit and the lifespan for cached user's. This is one
of the first upcoming optimizations, since Rauthy gets closer to the first v1.0.0 release:

### E-Mails as lowercase only

Up until now, it was possible to register the same E-Mail address multiple times with using
uppercase characters.
E-Mail is case-insensitive by definition though. This version does a migration of all currently
existing E-Mail addresses in the database to lowercase only characters. From that point on, it will
always convert any address to lowercase only characters to avoid confusion and conflicts.  
This means, if you currently have the same address in your database with different casing, you need
to resolve this issue manually. The migration function will throw an error in the console at
startup, if it finds such a conflict.

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

The scope `profile` now additionally adds the following claims to the ID token (if they exist for
the user):

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
  This makes the whole value encryption way easier, more stable and future-proof, because values
  have their own
  tiny header data with the minimal amount of information needed. It not only simplifies encryption
  key rotations,
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
- Pre-Compute CSP's for all HTML content at build-time and get rid of the per-request nonce
  computation
  [8fd2c99](https://github.com/sebadob/rauthy/commit/8fd2c99d25aea2f307e0197f6f91a585b4408dce)
- `noindex, nofollow` globally via headers and meta tag -> Rauthy as an Auth provider should never
  be indexed
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
- Make the `ItemTiles` UI componend used for roles, groups, and so on, wrap nicely on smaller
  screens
  [6f83e4a](https://github.com/sebadob/rauthy/commit/6f83e4a917ffd4eaec9b543b66170dc5ea76ed6e)
- Show the corresponding E-Mail address for `UserPasswordReset` and `UserEmailChange` events in the
  UI
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
are using Microsoft (the only service provider where this problems can be replicated 100% of the
time) and / or Outlook. These users were unable to use password reset links at all.
The reason is a "Feature" from Microsoft. They fully scan the user's E-Mails and even follow all
links inside it. The problem is, that the binding cookie from Rauthy will go to the Microsoft
servers instead of the user, making is unusable and basically invalidating everything before the
user has any chance to use the link properly.

The usage of this config variable is **highly discouraged,** and you should **avoid it, if you can
**. However, big enterprises are moving slowly (and often not at all). This new config variable can
be
used
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
not use them at all. There is a whole new section in
the [Config](https://sebadob.github.io/rauthy/config/config.html)
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
- A much nicer way of generating both DEV and PROD TLS certificates by
  using [Nioca](https://github.com/sebadob/nioca)
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
- For developers, a
  new [CONTRIBUTING.md](https://github.com/sebadob/rauthy/blob/main/CONTRIBUTING.md)
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
The Apache is way more permissive and make the integration with other open source projects and
software a lot easier.

### DPoP Token Support (Experimental)

The first steps towards DPoP Token support have been made.
It is marked as experimental though, because the other authentication methods have been tested and
verified with various real world applications already. This is not the case for DPoP yet.
Additionally, the only supported alg for DPoP proofs is EdDSA for now. The main reason being that I
am using Jetbrains IDE's and the Rust plugin for both IDEA and RustRover are currently broken in
conjunction with the `rsa` crate (and some others) which makes writing code with them a nightmare.
RSA support is prepared as much as possible though and I hope they will fix this bug soon, so it can
be included.

If you have or use a DPoP application, I would really appreciate testing with Rauthy and to get some
feedback, so I can make the whole DPoP flow more resilient as well.

Please note that Authorization Code binding to a DPoP key is also not yet supported, only the
`/token` endpoint accepts and validates the `DPoP` header for now.

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

Both a `linux/amd64` and a `linux/arm64` are supported. This means you can "just use it" now on
Raspberry Pi and others, or on Ampere architecture from Cloud providers without the need to compile
it yourself.

#### Events and Auditing

Rauthy now produces events in all different kinds of situations. These can be used for auditing,
monitoring, and so on. You can configure quite a lot for them in the new `EVENTS / AUDIT` section in
the [Rauthy Config](https://sebadob.github.io/rauthy/config/config.html).

These events are persisted in the database, and they can be fetched in real time via a new Server
Sent Events(SSE) endpoint `/auth/v1/events/stream`. There is a new UI component in the Admin UI that
uses the same events stream. In case of a HA deployment, Rauthy will use one additional DB
connection (all the time) from the connection pool to distribute these events via pg listen / notify
to the other members. This makes a much simpler deployment and there is no real need to deploy
additional resources like Nats or something like that. This keeps the setup easier and therefore
more fault-tolerant.

> You should at least set `EVENT_EMAIL` now, if you update from an older version.

#### Slack and Matrix Integrations

The new Events can be sent to a Slack Webhook or Matrix Server.

The Slack integration uses the simple (legacy) Slack Webhooks and can be configured with
`EVENT_SLACK_WEBHOOK`:

```
# The Webhook for Slack Notifications.
# If left empty, no messages will be sent to Slack.
#EVENT_SLACK_WEBHOOK=
```

The Matrix integration can connect to a Matrix server and room. This setup requires you to provide a
few more
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
This makes sure, that an attacker cannot just lock a users account by doing invalid logins and
therefore
kind of DoS the user. Additionally, Rauthy can detect Brute-Force or DoS attempts independently of
a users account.

There are certain thresholds at 7, 10, 15, 20, 25 invalid logins, when a clients IP will get fully
blacklisted (explained below) for a certain amount of time. This is a good DoS and even DDoS
prevention.

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
- Send out E-Mail change confirmations E-Mails to both old and new address when an admin changes the
  address
  [8e97e31](https://github.com/sebadob/rauthy/commit/8e97e310a6369bd5150d087cd5cd402d8edc221e)
  [97197db](https://github.com/sebadob/rauthy/commit/97197dbcdf3268647a3269c5cf5648176f0000d7)
- Allow CORS requests to the `.well-known` endpoints to make the oidc config lookup from an external
  UI possible
  [b57656f](https://github.com/sebadob/rauthy/commit/b57656ffaf7e822527d2d8c9d74b7c948193c220)
- include the `alg` in the JWKS response for the `openid-configuration`
  [c9073cb](https://github.com/sebadob/rauthy/commit/c9073cb473be04092e326c95ca7a1b3502379f40)
- The E-Mail HTML templates have been optically adjusted a bit to make them "prettier"
  [926de6e](https://github.com/sebadob/rauthy/commit/926de6e7348a8294fe284500cb41381a56a5cd2b)

### Bugfixes

- A User may have not been updated correctly in the cache when the E-Mail was changed.
  [8d9cdce](https://github.com/sebadob/rauthy/commit/8d9cdce61992ee59e381876b71b18168e7e3ce31)
- With v0.16, it was possible to not be able to switch back to a password account type from passkey,
  when it was a password account before already which did update its password in the past and
  therefore
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
If you had an active and valid session from a v0.15.0, did an update to v0.16.0 and your session was
still valid, it did not have valid information about the peer IP. This is mandatory for a new
feature introduced with v0.16.0 though and the warning logging in that case had an unwrap for the
remote IP (which can never be null from v0.16.0 on), which then would panic.

This is a tiny bugfix release that just gets rid of the possible panic and prints `UNKNOWN` into the
logs instead.

### Changes

- print `UNKNOWN` into the logs for non-readable / -existing peer IPs
  [6dfd0f4](https://github.com/sebadob/rauthy/commit/6dfd0f4299b90c03d9b5a2fb4106d72f153146af)

## v0.16.0

### Breaking

This version does modify the database and is therefore not backwards compatible with any previous
version.
If you need to downgrade vom v0.15 and above, you will only be able to do this via by applying a DB
Backup.

### New Features

#### User Expiry

It is now possible to limit the lifetime of a user.  
You can set an optional expiry date and time for each user. This enables temporary access, for
instance in a support case where an external person needs access for a limited time.

Once a user has expired, a few things will happen:

- The user will not be able to log in anymore.
- As soon as the scheduler notices the expiry, it will automatically invalidate all possibly
  existing sessions and refresh tokens for this user. How often the scheduler will run can be
  configured with the `SCHED_USER_EXP_MINS` variable. The default is 'every 60 minutes' to have a
  good balance between security and resource usage. However, if you want this to be very strict, you
  can adjust this down to something like '5 minutes' for instance.
- If configured, expired users can be cleaned up automatically after the configured time.
  By default, expired users will not be cleaned up automatically. You can enable this feature with
  the ´SCHED_USER_EXP_DELETE_MINS` variable.

#### `WEBAUTHN_NO_PASSWORD_EXPIRY`

With this new config variable, you can define, if users with at least one valid registered passkey
will have expiring passwords (depending on the current password policy), or not.   
By default, these users do not need to renew their passwords like it is defined in the password
policy.

#### Peer IP's for sessions -> `SESSION_VALIDATE_IP`

When a new session is being created, the peer / remote IP will be extracted and saved with the
session information. This peer IP can be checked with each access and the session can be rejected,
if this IP has changed, which will force the user to do a new login.

This will of course happen if a user is "on the road" and uses different wireless networks on the
way, but it prevents a session hijack and usage from another machine, if an attacker has full access
to the victims machine and even can steal the encrypted session cookie and(!) the csrf token saved
inside the local storage. This is very unlikely, since the attacker would need to have full access
to the machine anyway already, but it is just another security mechanism.

If this IP should be validated each time can be configured with the new `SESSION_VALIDATE_IP`
variable. By default, peer IP's will be validated and a different IP for an existing session will be
rejected.

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

For all registered passkeys, the User Verification (UV) state is now being saved and optionally
checked. You can see the status for each device with the new fingerprint icon behind its name in the
UI.

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

This version does modify the database and is therefore not backwards compatible with any previous
version.
If you need to downgrade vom v0.15 and above, you will only be able to do this via by applying a DB
Backup.

### Changes

This release is all about new Passkey Features.

- A user is not limited to just 2 keys anymore
- During registration, you can (and must) provide a name for the passkey, which helps you identify
  and distinguish your keys, when you register multiple ones.
- The `exclude_credentials` feature is now properly used and working. This makes sure, that you
  cannot register the same Passkey multiple times.
- The Passkeys / MFA section in the Admin UI has been split from the User Password section to be
  more convenient to use

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
The next v0.15 will be an "in-between-release" which will do some migration preparations for
Webauthn / FIDO 2 updates and features coming in the near future.

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
- UI: All navigation routes can be reached via their own link now. This means a refresh of the page
  does not return to the default anymore
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

- Improved container security: Rauthy is based off a Scratch container image by default now. This
  improved the security quite a lot, since you cannot even get a shell into the container anymore,
  and it reduced the image size by another ~4MB.  
  This makes it difficult however if you need to debug something, for instance when you use a SQLite
  deployment. For this reason, you can append `-debug` to a tag, and you will get an Alpine based
  version just like before.
  [1a7e79d](https://github.com/sebadob/rauthy/commit/1a7e79dc96d27d8d180d1e4394644c8851cbdf70)
- More stable HA deployment: In some specific K8s HA deployments, the default HTTP2 keep-alive's
  from [redhac](https://github.com/sebadob/redhac) were not good enough and we got broken pipes in
  some environments which caused the leader to change often. This has been fixed
  in [redhac-0.6.0](https://github.com/sebadob/redhac/releases/tag/v0.6.0) too, which at the same
  time makes Rauthy HA really stable now.
- The client branding section in the UI has better responsiveness for smaller screens
  [dfaa23a](https://github.com/sebadob/rauthy/commit/dfaa23a30ccf77da2b29654c7dd3b41a4ca78168)
- For a HA deployment, cache modifications are now using proper HA cache functions. These default
  back to the single instance functions in non-HA mode
  since [redhac-0.6.0](https://github.com/sebadob/redhac/releases/tag/v0.6.0)
  [7dae043](https://github.com/sebadob/rauthy/commit/7dae043d7b42724adad85b5ed54f1dcd9d143d27)
- All static UI files are now precompressed with gzip and brotli to use even fewer resources
  [10ad51a](https://github.com/sebadob/rauthy/commit/10ad51a296c5a7596b34f9c726fe87480b6ec42c)
- CSP script-src unsafe-inline was removed in favor of custom nonce's
  [7de918d](https://github.com/sebadob/rauthy/commit/7de918d601007d2807701a096d6403bf2b3274c9)
- UI migrated to Svelte 4
  [21f73ab](https://github.com/sebadob/rauthy/commit/21f73abfb0332be3fc391b9d108655a0cd5a3cec)

## v0.12.0

Rauthy goes open source
