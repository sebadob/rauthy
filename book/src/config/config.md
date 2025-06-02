# Reference Config

This shows a full example config with (hopefully) every value nicely described.

You can configure a lot here, but the most important variables you most-likely want to change when going into production
are the following. Lines beginning with `!!!` are absolutely critical. The order matches their location in the
reference config below.

- `OPEN_USER_REG`, `USER_REG_DOMAIN_RESTRICTION`
- `PEER_IP_HEADER_NAME` - when behind a CDN
- `HQL_BACKUP_CRON`, `HQL_BACKUP_KEEP_DAYS`, `HQL_S3_URL`, `HQL_S3_BUCKET`, `HQL_S3_REGION`, `HQL_S3_PATH_STYLE`,
  `HQL_S3_KEY`, `HQL_S3_SECRET` - for Hiqlite backups, does not matter when using Postgres
- `BOOTSTRAP_ADMIN_EMAIL`
- `HQL_NODE_ID_FROM` or `HQL_NODE_ID` + `HQL_NODES` - HA only
- !!! `HQL_SECRET_RAFT` + `HQL_SECRET_API` - set even when not using HA
- `DATABASE_URL` + `HIQLITE` - if you want to use Postgres
- if you use `HIQLITE`:
    - `HQL_CACHE_STORAGE_DISK`
    - `HQL_IGNORE_WAL_LOCK`
    - `HQL_SECRET_RAFT` + `HQL_SECRET_API`
- `RAUTHY_ADMIN_EMAIL`
- `EMAIL_SUB_PREFIX`, `SMTP_URL`, `SMTP_USERNAME`, `SMTP_PASSWORD`, `SMTP_FROM`
- !!! `ENC_KEY_ACTIVE` + `ENC_KEYS`
- `MAX_HASH_THREADS` in combination with `HTTP_WORKERS`
- any target in the `EVENTS / AUDIT` section
- !!! `PUB_URL`
- `PROXY_MODE` + `TRUSTED_PROXIES`
- `TLS_CERT` + `TLS_KEY` - if you don't terminate TLS on your reverse proxy
- `HQL_TLS_RAFT_KEY` + `HQL_TLS_RAFT_CERT`+  `HQL_TLS_API_KEY` + `HQL_TLS_API_CERT` - if you want to internally encrypt
  database / cache traffic
- !!! `RP_ID` + `RP_ORIGIN` + `RP_NAME`

```admonish caution
When you go into production, make sure that you provide the included secrets / sensistive information in this
file in an appropriate way. With docker, you can leave them inside this file (with proper access rights!), but when 
deploying with Kubernetes, extract these values into Kubernetes Secrets.
```

```
#####################################
############## ACCESS ###############
#####################################

# If the User Registration endpoint should be accessible by anyone.
# If not, an admin must create each new user.
# default: false
#OPEN_USER_REG=false

# If set to true, the `/userinfo` endpoint will do additional validations.
# The non-strict mode will fetch the user by id from the `sub` claim and make
# sure it still exists and is enabled. The strict validation will do additional
# database fetches and validates every possible value.
# Additionally, it will look up a possibly linked user device from the `did` claim
# and make sure it still exists. It will also extract the `client_id` the token
# has been originally issued for from the `azp` claim, fetch it and make sure it
# still exists and is enabled.
#
# If you need CORS headers on the /userinfo endpoint, you must enable strict mode,
# because otherwise the additional data would be missing.
#
# If you don't need the extra validations, you can set this to `false` to
# save some resources, if your clients to a lot of `/userinfo` lookups.
# default: true
#USERINFO_STRICT=true

# Can be set to `true` to disable authorization on `/oidc/introspect`.
# This should usually never be done, but since the auth on that endpoint is not
# really standardized, you may run into issues with your client app.
# If so, please open an issue about it.
# default: false
#DANGER_DISABLE_INTROSPECT_AUTH=false

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
#DISABLE_REFRESH_TOKEN_NBF=false

# Can be used when 'OPEN_USER_REG=true' to restrict the domains
# for a registration. For instance, set it to
# 'USER_REG_DOMAIN_RESTRICTION=gmail.com' to allow only
# registrations with 'user@gmail.com' (default: '')
#USER_REG_DOMAIN_RESTRICTION=some-domain.com

# If `OPEN_USER_REG=true`, you can blacklist certain domains
# on the open registration endpoint.
# Provide the domains as a `\n` separated list.
#USER_REG_DOMAIN_BLACKLIST="
#example.com
#evil.net
#"

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

# If set to true, a violation inside the CSRF protection middleware based
# on Sec-* headers will block invalid requests. Usually you always want this
# enabled. You may only set it to false during the first testing phase if you
# experience any issues with an already existing Rauthy deployment.
# In future releases, it will not be possible the disable these blocks.
# default: true
#SEC_HEADER_BLOCK=true

# If set to 'true', this will validate the remote peer IP address with
# each request and compare it with the IP which was used during the initial
# session creation / login. If the IP is different, the session will be
# rejected. This is a security hardening and prevents stolen access credentials,
# for instance if an attacker might have copied the encrypted session cookie
# and the XSRF token from the local storage from a user. However, this event
# is really unlikely, since it may only happen if an attacker has direct access
# to the machine itself.
#
# If your users are using mobile networks and get new IP addresses all the time,
# this means they have to do a new login each time. This is no big deal at all with 
# Webauthn / FIDO keys anyway and should not be a reason to deactivate this feature.
#
# Caution: If you are running behind a reverse proxy which does not provide the 
# X-FORWARDED-FOR header correctly, or you have the PROXY_MODE in this config
# disabled, this feature will not work. You can validate the IPs for each session
# in the Admin UI. If these are correct, your setup is okay.
#
# default: true
#SESSION_VALIDATE_IP=true

# By default, Rauthy will log a warning into the logs, if an active password
# reset form is being access multiple times from different hosts. You can set
# this to `true` to actually block any following request after the initial one.
# This hardens the security of the password reset form a bit more, but will
# create problems with E-Mail providers like Microsoft, which cans the customers
# E-Mails and even uses links inside, which make them unusable with this set to
# true.
# This feature works by setting an encrypted cookie to the host whichever opens
# the password reset form for the very first time. All subsequent requests either
# need to provide that cookie or would otherwise be rejected.
#PASSWORD_RESET_COOKIE_BINDING=true

# Can be set to extract the remote client peer IP from a custom header name
# instead of the default mechanisms. This is needed when you are running 
# behind a proxy which does not set the `X-REAL-IP` or `X-FORWARDED-FOR` headers
# correctly, or for instance when you proxy your requests through a CDN like
# Cloudflare, which adds custom headers in this case.
# For instance, if your requests are proxied through cloudflare, your would 
# set `CF-Connecting-IP`.
#PEER_IP_HEADER_NAME="CF-Connecting-IP"

# You can enable authn/authz headers which would be added to the response
# of the `/auth/v1/oidc/forward_auth` endpoint. With  `AUTH_HEADERS_ENABLE=true`,
# the headers below will be added to authenticated requests. These could
# be used on legacy downstream applications, that don't support OIDC on
# their own.
# However, be careful when using this, since this kind of authn/authz has
# a lot of pitfalls out of the scope of Rauthy.
# default: false
#AUTH_HEADERS_ENABLE=true

# Configure the header names being used for the different values.
# You can change them to your needs, if you cannot easily change your
# downstream apps.
# default: x-forwarded-user
#AUTH_HEADER_USER=x-forwarded-user
# default: x-forwarded-user-roles
#AUTH_HEADER_ROLES=x-forwarded-user-roles
# default: x-forwarded-user-groups
#AUTH_HEADER_GROUPS=x-forwarded-user-groups
# default: x-forwarded-user-email
#AUTH_HEADER_EMAIL=x-forwarded-user-email
# default: x-forwarded-user-email-verified
#AUTH_HEADER_EMAIL_VERIFIED=x-forwarded-user-email-verified
# default: x-forwarded-user-family-name
#AUTH_HEADER_FAMILY_NAME=x-forwarded-user-family-name
# default: x-forwarded-user-given-name
#AUTH_HEADER_GIVEN_NAME=x-forwarded-user-given-name
# default: x-forwarded-user-mfa
#AUTH_HEADER_MFA=x-forwarded-user-mfa

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

# The "catch all" route handler on `/` will compare the request path
# against a hardcoded list of common scan targets from bots and attackers.
# If the path matches any of these targets, the IP will be blacklisted
# preemptively for the set time in minutes.
# You can disable it with setting it to `0`.
# default: 1440
#SUSPICIOUS_REQUESTS_BLACKLIST=1440

# This will emit a log with level of warning if a request to `/` has
# been made that has not been caught by any of the usual routes and
# and handlers. Apart from a request to just `/` which will end in
# a redirect to `/auth/v1`, all additional path's will be logged.
# This can help to improve the internal suspicious blocklist in the
# future.
# default: false
#SUSPICIOUS_REQUESTS_LOG=false

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

#####################################
############# BACKUPS ###############
#####################################

# When the auto-backup task should run.
# Accepts cron syntax:
# "sec min hour day_of_month month day_of_week year"
# default: "0 30 2 * * * *"
#HQL_BACKUP_CRON="0 30 2 * * * *"

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
HQL_S3_URL=https://s3.example.com
HQL_S3_BUCKET=my_bucket
HQL_S3_REGION=example
HQL_S3_PATH_STYLE=true
HQL_S3_KEY=s3_key
HQL_S3_SECRET=s3_secret

#####################################
############ BOOTSTRAP ##############
#####################################

# If set, the email of the default admin will be changed
# during the initialization of an empty production database.
BOOTSTRAP_ADMIN_EMAIL=admin@localhost

# If set, this plain text password will be used for the
# initial admin password instead of generating a random
# password.
#BOOTSTRAP_ADMIN_PASSWORD_PLAIN="123SuperSafe"

# If set, this will take the argon2id hashed password
# during the initialization of an empty production database.
# If both BOOTSTRAP_ADMIN_PASSWORD_PLAIN and
# BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID are set, the hashed version
# will always be prioritized.
#BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID='$argon2id$v=19$m=32768,t=3,p=2$xr23OhOHw+pNyy3dYKZUcA$CBO4NpGvyi6gvrb5uNrnsS/z/Ew+IuS0/gVqFmLKncA'

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
#BOOTSTRAP_API_KEY_SECRET=twUA2M7RZ8H3FyJHbti2AcMADPDCxDqUKbvi8FDnm3nYidwQx57Wfv6iaVTQynMh

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

# Set to `false` to store Cache WAL files + Snapshots in-memory only.
# Depending on your environment and setup, this can lead to cluster
# issues if a node crashes and cannot do a graceful shutdown. It is not
# recommended to keep the Raft state in-memory only, apart from for
# testing and in special cases.
# default: true
#HQL_CACHE_STORAGE_DISK=true

# Can be set to true to start the WAL handler even if the
# `lock.hql` file exists. This may be necessary after a
# crash, when the lock file could not be removed during a
# graceful shutdown.
#
# IMPORTANT: Even though the Database can "heal" itself by
# simply rebuilding from the existing Raft log files without
# even needing to think about it, you may want to only
# set `HQL_IGNORE_WAL_LOCK` when necessary to have more
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
#HQL_IGNORE_WAL_LOCK=false

#####################################
############ DATABASE ###############
#####################################

# If specified, the currently configured Database will be
# DELETED and OVERWRITTEN with a migration from the given
# database with this variable. Can be used to migrate between
# different databases.
# To migrate from Hiqlite, use the `sqlite:path/to/database.sqlite` format.
# To migrate from postgres, just set `postgres` and then all the
# `MIGRATE_PG_*` values below.
#
# !!! USE WITH CARE !!!
#
#MIGRATE_DB_FROM=sqlite:data/state_machine/db/hiqlite.db
#MIGRATE_DB_FROM=postgres

# If `MIGRATE_DB_FROM=postgres`, these values are mandatory to open the
# database connection to the Postgres database you want to migrate away from.
MIGRATE_PG_HOST=
# default: 5432
#MIGRATE_PG_PORT=5432
MIGRATE_PG_USER=
MIGRATE_PG_PASSWORD=
# default: rauthy
#MIGRATE_PG_DB_NAME=rauthy

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
# lost. Only if something worse happens, you might lose the last
# 200ms of commits.
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
#HQL_PASSWORD_DASHBOARD=JGFyZ29uMmlkJHY9MTkkbT0zMix0PTIscD0xJE9FbFZURnAwU0V0bFJ6ZFBlSEZDT0EkTklCN0txTy8vanB4WFE5bUdCaVM2SlhraEpwaWVYOFRUNW5qdG9wcXkzQQ==

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

# If you set `HIQLITE=false` and want to use Postgres as your database,
# you need to set the following variables.
# These will be ignored as long as `HIQLITE=true`.
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
PG_MAX_CONN=20

# Disables the housekeeping schedulers (default: false)
#SCHED_DISABLE=true

# The interval in minutes in which the scheduler for expired 
# users should run. If this finds expired users, it invalidates 
# all existing sessions and refresh tokens for this user.
# default: 60
#SCHED_USER_EXP_MINS=60

# The threshold in minutes after which time the user expiry 
# scheduler should automatically clean up expired users.
# If not set at all, expired users will never be cleaned up 
# automatically.
# default: disabled / not set
#SCHED_USER_EXP_DELETE_MINS=7200

#####################################
######### DEVICE GRANT ##############
#####################################

# The lifetime in secods  of auth codes for the Device Authorization
# Grant flow. You may increase the default of 300 seconds, if you have
# "slow users" and they are simply not fast enough with the verification.
# default: 300
#DEVICE_GRANT_CODE_LIFETIME=300

# The length of the `user_code` the user has to enter manually for
# auth request validation. This must be < 64 characters.
# default: 8
#DEVICE_GRANT_USER_CODE_LENGTH=8

# Specifies the rate-limit in seconds per IP for starting new Device
# Authorization Grant flows. This is especially important for public
# clients, because a code request for this flow will actually create
# cached data. If this would happen on an unrestricted, open endpoint,
# the application could easily be DoS'ed.
# If you use the `device_code` grant with confidential clients only,
# you can leave this unset, which will not rate-limit the endpoint.
# default: not set
#DEVICE_GRANT_RATE_LIMIT=1

# The interval in seconds which devices are told to use when they
# poll the token endpoint during Device Authorization Grant flow.
# default: 5
#DEVICE_GRANT_POLL_INTERVAL=5

# You can define a global lifetime in hours for refresh tokens issued
# from a Device Authorization Grant flow. You might want to have a
# higher lifetime than normal refresh tokens, because they might be
# used in IoT devices which may be offline for longer periods of time.
# default: 72
#DEVICE_GRANT_REFRESH_TOKEN_LIFETIME=72

#####################################
############## DPOP #################
#####################################

# May be set to 'false' to disable forcing the usage of 
# DPoP nonce's.
# default: true
#DPOP_FORCE_NONCE=true

# Lifetime in seconds for DPoP nonces. These are used to 
# limit the lifetime of a client's DPoP proof. Do not set
# lower than 30 seconds to avoid too many failed client 
# token requests.
# default: 900
#DPOP_NONCE_EXP=900

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

#####################################
############# E-MAIL ################
#####################################

# This contact information will be added to the `rauthy`client
# within the anti lockout rule with each new restart.
RAUTHY_ADMIN_EMAIL="admin@localhost"

# Will be used as the prefix for the E-Mail subject for each E-Mail 
# that will be sent out to a client.
# This can be used to further customize your deployment.
# default: "Rauthy IAM"
EMAIL_SUB_PREFIX="Rauthy IAM"

# Rauthy will force TLS and try a downgrade to STARTTLS, if
# TLS fails. It will never allow an unencrypted connection.
# You might want to set `SMTP_DANGER_INSECURE=true` if you
# need this for local dev.
#SMTP_URL=
# optional, default will be used depending on TLS / STARTTLS
#SMTP_PORT=
#SMTP_USERNAME=
#SMTP_PASSWORD=
# Format: "Rauthy <rauthy@localhost>"
# default: "Rauthy <rauthy@localhost>"
#SMTP_FROM=

# The number of retries that should be done for connecting to
# the given SMTP_URL.
# When these retries are exceeded without success, Rauthy will
# panic and exit, so no E-Mail can get lost silently because of
# a missing SMTP connection.
# default: 3
#SMTP_CONNECT_RETRIES=3

# You can set this to `true` to allow an unencrypted and
# unauthenticated SMTP connection to an SMTP relay on your localhost
# or for development purposes.
# When set to `true`, `SMTP_USERNAME` and `SMTP_PASSWORD` will be
# ignored and you can modify the target port with
# `SMTP_DANGER_INSECURE_PORT`.
# default: false
#SMTP_DANGER_INSECURE=false

#####################################
###### ENCRYPTION / HASHING #########
#####################################

# You need to define at least one valid encryption key.
# These keys are used in various places, like for instance
# encrypting confidential client secrets in the database, or
# encryption cookies, and so on.
#
# The format must match:
# ENC_KEYS="
# q6u26onRvXVG4427/M0NFQzhSSldCY01rckJNa1JYZ3g2NUFtSnNOVGdoU0E=
# bVCyTsGaggVy5yqQ/UzluN29DZW41M3hTSkx6Y3NtZmRuQkR2TnJxUTYzcjQ=
# "
#
# The first part until the first `/` is the key ID.
# The ID must match '[a-zA-Z0-9]{2,20}'
#
# The key itself begins after the first `/` has been found.
# The key must be exactly 32 bytes long, encoded as base64.
# 
# You can find a more detailed explanation on how to generate
# keys in the documentation:
# 1. https://sebadob.github.io/rauthy/getting_started/k8s.html#create-and-apply-secrets
# 2. https://sebadob.github.io/rauthy/config/encryption.html
#
# You can provide multiple keys to make things like key 
# rotation work. Be careful with removing old keys. Make sure
# that all secrets have been migrated beforehand.
# You can find a utility in the Admin UI to do this for you.
#
#ENC_KEYS="
#q6u26onRvXVG4427/M0NFQzhSSldCY01rckJNa1JYZ3g2NUFtSnNOVGdoU0E=
#bVCyTsGaggVy5yqQ/UzluN29DZW41M3hTSkx6Y3NtZmRuQkR2TnJxUTYzcjQ=
#"

# This identifies the key ID from the `ENC_KEYS` list, that
# should actively be used for new encryptions.
ENC_KEY_ACTIVE=bVCyTsGaggVy5yqQ

# Argon2ID hashing parameters. Take a look at the documentation
# for more information:
# https://sebadob.github.io/rauthy/config/argon2.html
# M_COST should never be below 32768 in production
ARGON2_M_COST=131072
# T_COST must be greater than 0
ARGON2_T_COST=4
# P_COST should never be below 2 in production
ARGON2_P_COST=8

# Limits the maximum amount of parallel password hashes at
# the exact same time to never exceed system memory while
# still allowing a good amount of memory for the argon2id
# algorithm (default: 2)
#
# CAUTION: You must make sure, that you have at least
# (MAX_HASH_THREADS * ARGON2_M_COST / 1024) + idle memory
# of your deployment available
MAX_HASH_THREADS=1

# The time in ms when to log a warning, if a request waited
# longer than this time. This is an indicator, that you have
# more concurrent logins than allowed and may need config 
# adjustments,
# if this happens more often. (default: 500)
#HASH_AWAIT_WARN_TIME=500

# JWKS auto rotate cronjob. This will (by default) rotate 
# all JWKs every 1. day of the month. If you need smaller 
# intervals, you may adjust this value. For security reasons,
# you cannot fully disable it.
# In a HA deployment, this job will only be executed on the 
# current cache leader at that time.
# Format: "sec min hour day_of_month month day_of_week year"
# default: "0 30 3 1 * * *"
#JWK_AUTOROTATE_CRON="0 30 3 1 * * *"

#####################################
####### EPHEMERAL CLIENTS ###########
#####################################

# Can be set to 'true' to allow the dynamic client lookup via URLs as
# 'client_id's during authorization_code flow initiation.
# default: false
#ENABLE_EPHEMERAL_CLIENTS=false

# Can be set to 'true' to enable WebID functionality like needed
# for things like Solid OIDC.
# default: false
#ENABLE_WEB_ID=false

# If set to 'true', 'solid' will be added to the 'aud' claim from the ID token
# for ephemeral clients.
# default: false
#ENABLE_SOLID_AUD=false

# If set to 'true', MFA / Passkeys will be forced for ephemeral clients.
# default: false
#EPHEMERAL_CLIENTS_FORCE_MFA=false

# The allowed flows separated by ' ' for ephemeral clients.
# default: "authorization_code"
#EPHEMERAL_CLIENTS_ALLOWED_FLOWS="authorization_code refresh_token"

# The allowed scopes separated by ' ' for ephemeral clients.
# default: "openid profile email webid"
#EPHEMERAL_CLIENTS_ALLOWED_SCOPES="openid profile email webid"

# The lifetime in seconds ephemeral clients will be kept inside the cache.
# default: 3600
#EPHEMERAL_CLIENTS_CACHE_LIFETIME=3600

#####################################
######### EVENTS / AUDIT ############
#####################################

# The E-Mail address event notifications should be sent to.
#EVENT_EMAIL=admin@localhost

# Matrix variables for event notifications.
# `EVENT_MATRIX_USER_ID` and `EVENT_MATRIX_ROOM_ID` are mandatory.
# Depending on your Matrix setup, additionally one of
# `EVENT_MATRIX_ACCESS_TOKEN` or `EVENT_MATRIX_USER_PASSWORD` is needed.
# If you log in to Matrix with User + Password, you may use `EVENT_MATRIX_USER_PASSWORD`.
# If you log in via OIDC SSO (or just want to use a session token you can revoke),
# you should provide `EVENT_MATRIX_ACCESS_TOKEN`.
# If both are given, the `EVENT_MATRIX_ACCESS_TOKEN` will be preferred.
#
# If left empty, no messages will not be sent to Matrix.
# Format: `@<user_id>:<server address>`
#EVENT_MATRIX_USER_ID=
# Format: `!<random string>:<server address>`
#EVENT_MATRIX_ROOM_ID=
#EVENT_MATRIX_ACCESS_TOKEN=
#EVENT_MATRIX_USER_PASSWORD=
# URL of your Matrix server.
# default: https://matrix.org
#EVENT_MATRIX_SERVER_URL=https://matrix.org
# Optional path to a PEM Root CA certificate file for the Matrix client.
#EVENT_MATRIX_ROOT_CA_PATH=path/to/my/root_ca_cert.pem
# May be set to disable the TLS validation for the Matrix client.
# default: false
#EVENT_MATRIX_DANGER_DISABLE_TLS_VALIDATION=false
# The default behavior is, that Rauthy will panic at startup if it cannot connect
# to a configured Matrix server. The reason is that event notifications cannot be
# dropped silently.
# However, if you use a self-hosted Matrix server which uses Rauthy as its OIDC
# provider and both instances went offline, you will have a chicken and egg problem:
# - Rauthy cannot connect to Matrix and will panic
# - Your Matrix server cannot connect to Rauthy and will panic
# To solve this issue, you can temporarily set this value to 'true' and revert
# back, after the system is online again.
# default: false
#EVENT_MATRIX_ERROR_NO_PANIC=false

# The Webhook for Slack Notifications.
# If left empty, no messages will be sent to Slack.
#EVENT_SLACK_WEBHOOK=

# The notification level for events. Works the same way as a logging level. 
# For instance: 'notice' means send out a notifications for all events with 
# the notice level or higher.
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

# Define the level from which on events should be persisted inside the 
# database. All events with a lower level will be lost, if there is no 
# active event subscriber.
# Possible values:
# - info
# - notice
# - warning
# - critical
# default: 'info'
#EVENT_PERSIST_LEVEL=info

# Define the number of days when events should be cleaned up from the database.
# default: 31
#EVENT_CLEANUP_DAYS=31

# The level for the generated Event after a new user has been registered.
# default: info
EVENT_LEVEL_NEW_USER=info
# The level for the generated Event after a user has changed his E-Mail
# default: notice
EVENT_LEVEL_USER_EMAIL_CHANGE=notice
# The level for the generated Event after a user has reset its password
# default: notice
EVENT_LEVEL_USER_PASSWORD_RESET=notice
# The level for the generated Event after a user has been given the 
# 'rauthy_admin' role
# default: notice
EVENT_LEVEL_RAUTHY_ADMIN=notice
# The level for the generated Event after a new App version has been found
# default: notice
EVENT_LEVEL_RAUTHY_VERSION=notice
# The level for the generated Event after the JWKS has been rotated
# default: notice
EVENT_LEVEL_JWKS_ROTATE=notice
# The level for the generated Event after DB secrets have been migrated 
# to a new key
# default: notice
EVENT_LEVEL_SECRETS_MIGRATED=notice
# The level for the generated Event after a Rauthy instance has been 
# started
# default: info
EVENT_LEVEL_RAUTHY_START=info
# The level for the generated Event after a Rauthy entered a healthy 
# state (again)
# default: notice
EVENT_LEVEL_RAUTHY_HEALTHY=notice
# The level for the generated Event after a Rauthy entered an unhealthy 
#state
# default: critical
EVENT_LEVEL_RAUTHY_UNHEALTHY=critical
# The level for the generated Event after an IP has been blacklisted
# default: warning
EVENT_LEVEL_IP_BLACKLISTED=warning
# The level for the generated Event after certain amounts of false 
# logins from an IP
# default: critical
EVENT_LEVEL_FAILED_LOGINS_25=critical
# default: critical
EVENT_LEVEL_FAILED_LOGINS_20=critical
# default: warning
EVENT_LEVEL_FAILED_LOGINS_15=warning
# default: warning
EVENT_LEVEL_FAILED_LOGINS_10=warning
# default: notice
EVENT_LEVEL_FAILED_LOGINS_7=notice
# default: info
EVENT_LEVEL_FAILED_LOGIN=info

# If set to 'true', it will disable the app version checker.
# This is a scheduled task that looks up the latest version periodically
# by doing a request to the Github API to check the latest release.
# This ignores any type of prerelease and will only notify for a new stable.
# default: false
#DISABLE_APP_VERSION_CHECK=false

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
# Available Options: en de ko
#FILTER_LANG_ADMIN="en de ko"

#####################################
####### LIFETIMES / TIMEOUTS ########
#####################################

# Set the grace time in seconds for how long in seconds the
# refresh token should still be valid after usage. Keep this
# value small, but do not set it to 0 with an HA deployment
# to not get issues with small HA cache latencies.
#
# If you have an external client, which does concurrent
# requests, from which the request interceptor wants to refresh
# the token, you may have multiple hits on the endpoint and all
# of them should be valid.
#
# Caching is done on the endpoint itself, but grace time of 0
# will only be good for a single instance of rauthy.
# default: 5
#REFRESH_TOKEN_GRACE_TIME=5

# Session lifetime in seconds - the session can not be
# extended beyond this time and a new login will be forced.
# This is the session for the authorization code flow. 
# default: 14400
#SESSION_LIFETIME=14400

# If 'true', a 2FA / MFA check will be done with each automatic
# token generation, even with an active session, which kind of
# makes the session useless with Webauthn enabled, but provides
# maximum amount of security.
# If 'false', the user will not get an MFA prompt with an active
# session at the authorization endpoint.
# default: false
#SESSION_RENEW_MFA=false

# Session timeout in seconds
# When a new token / login is requested before this timeout hits
# the limit, the user will be authenticated without prompting for
# the credentials again.
# This is the value which can extend the session, until it hits
# its maximum lifetime set with SESSION_LIFETIME.
# default: 5400
#SESSION_TIMEOUT=5400

# ML: magic link
# LT: lifetime
# Lifetime in minutes for reset password magic links (default: 30)
#ML_LT_PWD_RESET=30

# Lifetime in minutes for the first password magic link, for
# setting the initial password. (default: 4320)
#ML_LT_PWD_FIRST=4320

#####################################
############# LOGGING ###############
#####################################

# This is the log level for stdout logs
# Accepts: error, info, debug, trace (default: info)
#LOG_LEVEL=info

# The log level for the `Hiqlite` persistence layer.
# default: info
LOG_LEVEL_DATABASE=info

# This is a special config which allows the configuration of
# customized access logs. These logs will be logged with each
# request in addition to the normal LOG_LEVEL logs.
# The following values are valid:
# - Debug
#   CAUTION: The Debug setting logs every information available
#   to the middleware which includes SENSITIVE HEADERS
#   DO NOT use the Debug level in a working production environment!
# - Verbose
#   Verbose logging without headers - generates huge outputs
# - Basic
#   Logs access to all endpoints apart from the Frontend ones
#   which all js, css, ...
# - Modifying
#   Logs only requests to modifying endpoints and skips all GET
# - Off
# default: Modifying
#LOG_LEVEL_ACCESS=Basic

# You can change the log output format to JSON, if you set:
# `LOG_FMT=json`.
# Keep in mind, that some logs will include escaped values,
# for instance when `Text` already logs a JSON in debug level.
# Some other logs like an Event for instance will be formatted 
# as Text anyway. If you need to auto-parse events, please consider 
# using an API token and listen ot them actively.
# default: text
#LOG_FMT=text

#####################################
################ MFA ################
#####################################

# If 'true', MFA for an account must be enabled to access the
# rauthy admin UI (default: true)
#ADMIN_FORCE_MFA=true

#####################################
############## POW  #################
#####################################

# The difficulty for a Proof-of-Work (PoW).
# The default is 20, which is reasonable for modern processors.
# This is currently only used for the user registration via UI.
# The value must be between 10 and 99.
# default: 20
POW_DIFFICULTY=20

# The expiration duration in seconds for a PoW
# default: 30
POW_EXP=30

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

#####################################
############# SERVER ################
#####################################

# The server address to listen on. Can bind to a specific IP.
# default: 0.0.0.0
#LISTEN_ADDRESS=0.0.0.0

# The listen ports for HTTP / HTTPS, depending on the
# activated 'LISTEN_SCHEME'
# default: 8080
#LISTEN_PORT_HTTP=8080
# default: 8443
#LISTEN_PORT_HTTPS=8443

# The scheme to use locally, valid values:
# http | https | http_https | unix_http | unix_https (default: http_https)
# For more details about the UNIX domain socket, check out its documentation page.
LISTEN_SCHEME=http

# The Public URL of the whole deployment
# The LISTEN_SCHEME + PUB_URL must match the HTTP ORIGIN
# HEADER later on, which is especially important when running
# rauthy behind a reverse proxy. In case of a non-standard
# port (80/443), you need to add the port to the PUB_URL
PUB_URL=localhost:8080

# Limits the amount of HTTP worker threads. This value
# heavily impacts memory usage, even in idle. The default
# values are:
# - less than 4 CPU cores -> 1
# - 4+ cores -> max(2, cores - MAX_HASH_THREADS - reserve)
#   where `reserve` is 2 if `HIQLITE=true` and 1 otherwise.
#
# CAUTION: If you run your instance on a big underlying host,
# you almost always want to manually set an appropriate
# value. Rauthy can only see all available cores and not any
# possibly set container limits. This means if it runs inside
# a container on something like a a 96 core host, Rauthy will 
# by default spawn very many threads.
#HTTP_WORKERS=1

# When rauthy is running behind a reverse proxy, set to true
# default: false
PROXY_MODE=false

# A `\n` separated list of trusted proxy CIDRs.
# When `PROXY_MODE=true` or `PEER_IP_HEADER_NAME` is set,
# these are mandatory to be able to extract the real client
# IP properly and safely to prevent IP header spoofing.
# All requests with a different source will be blocked.
#TRUSTED_PROXIES="
#192.168.14.0/24
#"

# To bring support for applications using deep-linking, you can set custom URL 
# schemes to be accepted when present in the `Origin` header. For instance, a
# Tauri app would set `tauri://` instead of `https://`.
#
# Provide the value as a space separated list of Strings, like for instance:
# "tauri myapp"
#ADDITIONAL_ALLOWED_ORIGIN_SCHEMES="tauri myapp"

# To enable or disable the additional HTTP server to expose the /metrics endpoint
# default: false
#METRICS_ENABLE=false

# The IP address to listen on for the /metrics endpoint.
# You do not want to expose your metrics on a publicly reachable endpoint!
# default: 0.0.0.0
#METRICS_ADDR=0.0.0.0

# The port to listen on for the /metrics endpoint.
# You do not want to expose your metrics on a publicly reachable endpoint!
# default: 9090
#METRICS_PORT=9090

# Can be set to `true` to enable the Swagger UI.
# This will consume ~13mb of additional memory.
# default: false
SWAGGER_UI_ENABLE=true

# Can be set to `true` to make the Swagger UI publicly 
# available. By default, you can only access it with a 
# valid `rauthy_admin` session.
# default: false
SWAGGER_UI_PUBLIC=false

# The interval in seconds in which keep-alives should be sent to SSE clients.
# Depending on your network setup, proxy timeouts, ...
# you may adjust this value to fit your needs.
# default: 30
#SSE_KEEP_ALIVE=30

#####################################
############ TEMPLATES ##############
#####################################

# You can overwrite some default email templating values here.
# If you want to modify the basic templates themselves, this is
# currently only possible with a custom build from source.
# The content however can mostly be set here.
# If the below values are not set, the default will be taken.
#
# NOTE: All the `TPL_<language_tag>_*` values below exist for 
# multiple languages. Their name is always the same, just with 
# the other language tag as UPPERCASE.
# Available language tags: EN, DE, KO, ZH_HANS

# New Password E-Mail
#TPL_EN_PASSWORD_NEW_SUBJECT="New Password"
#TPL_EN_PASSWORD_NEW_HEADER="New password for"
#TPL_EN_PASSWORD_NEW_TEXT=""
#TPL_EN_PASSWORD_NEW_CLICK_LINK="Click the link below to get forwarded to the password form."
#TPL_EN_PASSWORD_NEW_VALIDITY="This link is only valid for a short period of time for security reasons."
#TPL_EN_PASSWORD_NEW_EXPIRES="Link expires:"
#TPL_EN_PASSWORD_NEW_BUTTON="Set Password"
#TPL_EN_PASSWORD_NEW_FOOTER=""

# Password Reset E-Mail
#TPL_EN_RESET_SUBJECT="Password Reset Request"
#TPL_EN_RESET_HEADER="Password reset request for"
#TPL_EN_RESET_TEXT=""
#TPL_EN_RESET_CLICK_LINK="Click the link below to get forwarded to the password request form."
#TPL_EN_RESET_VALIDITY="This link is only valid for a short period of time for security reasons."
#TPL_EN_RESET_EXPIRES="Link expires:"
#TPL_EN_RESET_BUTTON="Reset Password"
#TPL_EN_RESET_FOOTER=""

#####################################
############### TLS #################
#####################################

## UI + API TLS

# Overwrite the path to the TLS certificate file in PEM
# format for rauthy (default: tls/tls.crt)
#TLS_CERT=tls/tls.crt
# Overwrite the path to the TLS private key file in PEM
# format for rauthy. If the path / filename ends with
# '.der', rauthy will parse it as DER, otherwise as PEM.
# (default: tls/tls.key)
#TLS_KEY=tls/tls.key

## Database / Cache internal TLS

# If given, these keys / certificates will be used to establish
# TLS connections between nodes.
HQL_TLS_RAFT_KEY=tls/key.pem
HQL_TLS_RAFT_CERT=tls/cert-chain.pem
HQL_TLS_RAFT_DANGER_TLS_NO_VERIFY=true

HQL_TLS_API_KEY=tls/key.pem
HQL_TLS_API_CERT=tls/cert-chain.pem
HQL_TLS_API_DANGER_TLS_NO_VERIFY=true

#####################################
############# WEBAUTHN ##############
#####################################

# The 'Relaying Party (RP) ID' - effective domain name
# (default: localhost)
# CAUTION: When this changes, already registered devices will stop
# working and users cannot log in anymore!
RP_ID=localhost

# Url containing the effective domain name
# (default: http://localhost:8080)
# CAUTION: MUST include the port number!
RP_ORIGIN=http://localhost:8080

# Non critical RP Name
# Has no security properties and may be changed without issues
# (default: Rauthy Webauthn)
RP_NAME='Rauthy Webauthn'

# The Cache lifetime in seconds for Webauthn requests. Within
# this time, a webauthn request must have been validated.
# (default: 60)
#WEBAUTHN_REQ_EXP=60

# The Cache lifetime for additional Webauthn Data like auth
# codes and so on. Should not be lower than WEBAUTHN_REQ_EXP.
# The value is in seconds (default: 90)
#WEBAUTHN_DATA_EXP=90

# With webauthn enabled for a user, he needs to enter
# username / password on a new system. If these credentials are
# verified, rauthy will set an additional cookie, which will
# determine how long the user can then use only (safe) MFA
# passwordless webauthn login with yubikeys, apple touch id,
# Windows hello, ... until he needs to verify his credentials
# again.
# Passwordless login is generally much safer than logging in
# with a password. But sometimes it is possible, that the
# Webauthn devices do not force the user to include a second
# factor, which in that case would be a single factor login
# again. That is why we should ask for the original password
# in addition once in a while to set the cookie.
# The value is in hours (default: 2160)
#WEBAUTHN_RENEW_EXP=2160

# This feature can be set to 'true' to force User verification
# during the Webauthn ceremony. UV will be true, if the user
# does not only need to verify its presence by touching the key,
# but by also providing proof that he knows (or is) some secret
# via a PIN or biometric key for instance. With UV, we have a
# true MFA scenario where UV == false (user presence only)
# would be a 2FA scenario (with password). Be careful with this
# option, since Android and some special combinations of OS +
# browser to not support UV yet.
# (default: false)
#WEBAUTHN_FORCE_UV=false

# Can be set to 'true' to disable password expiry for users
# that have at least one active passkey. When set to 'false',
# the same password expiry from the set policy will apply to
# these users as well.
# With this option active, rauthy will ignore any password
# expiry set by the password policy for Webauthn users.
# default: true
#WEBAUTHN_NO_PASSWORD_EXPIRY=true

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
