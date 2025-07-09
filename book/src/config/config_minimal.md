# Minimal Production Config

This section gives you a template for probably the most minimal config you could do when going into production.

You can either copy & paste this (with adjustments of course) and expand it when necessary, or take a look at the
complete [Reference Config](config.md) and check each existing value.

```toml
[bootstrap]
# If set, the email of the default admin will be changed during
# the initialization of an empty production database.
#
# default: 'admin@localhost'
# overwritten by: BOOTSTRAP_ADMIN_EMAIL
admin_email = "admin@localhost"

[cluster]
# Can be set to 'k8s' to try to split off the node id from the hostname
# when Hiqlite is running as a StatefulSet inside Kubernetes.
#
# default: unset
# overwritten by: HQL_NODE_ID_FROM
#node_id_from = "k8s"

# The node id must exist in the nodes and there must always be
# at least a node with ID 1
# Will be ignored if `node_id_from = k8s`
#
# At least `node_id_from` or `node_id` are required.
#
# default: 0 (invalid)
# overwritten by: HQL_NODE_ID
node_id = 1

# All cluster member nodes. For a single instance deployment, 
# `"1 localhost:8100 localhost:8200"` will work just fine.
# Each array value must have the following format:
# `id addr_raft addr_api`
#
# default: ["1 localhost:8100 localhost:8200"]
# overwritten by: HQL_NODES
nodes = [
    "1 localhost:8100 localhost:8200",
    #    "2 localhost:8101 localhost:8201",
    #    "3 localhost:8102 localhost:8202",
]

# Secrets for Raft internal authentication as well as for the API.
# These must be at least 16 characters long and you should provide
# different ones for both variables.
#
# default: not set - required
# overwritten by: HQL_SECRET_RAFT
secret_raft = "SuperSecureSecret1337"
# default: not set - required
# overwritten by: HQL_SECRET_API
secret_api = "SuperSecureSecret1337"

[email]
# Rauthy will force TLS and try a downgrade to STARTTLS, if
# TLS fails. It will never allow an unencrypted connection.
# You might want to set `SMTP_DANGER_INSECURE=true` if you
# need this for local dev.
#
# overwritten by: SMTP_URL
smtp_url = "localhost"
# optional, default will be used depending on TLS / STARTTLS
# overwritten by: SMTP_PORT
#smtp_port = 
# overwritten by: SMTP_USERNAME
smtp_username = "username"
# overwritten by: SMTP_PASSWORD
smtp_password = "password"
# Format: "Rauthy <rauthy@localhost>"
# default: "Rauthy <rauthy@localhost>"
# overwritten by: SMTP_FROM
smtp_from = "Rauthy <rauthy@localhost>"

[encryption]
# You need to define at least one valid encryption key.
# These keys are used in various places, like for instance
# encrypting confidential client secrets in the database, or
# encryption cookies, and so on.
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
# overwritten by: ENC_KEYS - single String, \n separated values
keys = ["q6u26/M0NFQzhSSldCY01rckJNa1JYZ3g2NUFtSnNOVGdoU0E="]

# This identifies the key ID from the `ENC_KEYS` list, that
# should actively be used for new encryptions.
#
# overwritten by: ENC_KEY_ACTIVE
key_active = "q6u26"

[events]
# The E-Mail address event notifications should be sent to.
#
# overwritten by: EVENT_EMAIL
email = "admin@localhost"

[server]
# The scheme to use locally, valid values:
# http | https | http_https | unix_http | unix_https
# For more details about the UNIX domain socket, check out its
# documentation page.
#
# default: http_https
# overwritten by: LISTEN_SCHEME
scheme = "http"

# The Public URL of the whole deployment
# The LISTEN_SCHEME + PUB_URL must match the HTTP ORIGIN HEADER
# later on, which is especially important when running Rauthy
# behind a reverse proxy. In case of a non-standard port (80/443),
# you need to add the port to the PUB_URL
#
# default: not set - mandatory
# overwritten by: PUB_URL
pub_url = "localhost:8080"

# When rauthy is running behind a reverse proxy, set to true
#
# default: false
# overwritten by: PROXY_MODE
proxy_mode = false

# A list of trusted proxy CIDRs. When `proxy_mode = true`
# or `peer_ip_header_name` is set, these are mandatory to
# be able to extract the real client IP properly and safely
# to prevent IP header spoofing. All requests witha
# different source will be blocked.
#
# default: []
# overwritten by: TRUSTED_PROXIES - single String, \n separated values
#trusted_proxies = ['192.168.14.0/24']

[webauthn]
# The 'Relaying Party (RP) ID' - effective domain name.
#
# CAUTION: When this changes, already registered devices will
# stop working and users cannot log in anymore!
#
# default: 'localhost'
# overwritten by: RP_ID
rp_id = "localhost"

# Url containing the effective domain name.
#
# DEV: If you want to test Webauthn via the Svelte DEV UI,
# change the port number to :5173.
#
# !!! CAUTION: Must ALWAYS include the port number !!!
#
# default: 'http://localhost:8080'
# overwritten by: RP_ORIGIN
rp_origin = "http://localhost:5173"
```

```admonish hint
TL;DR for generating a `encryption.keys`:

<pre><code>echo "$(openssl rand -hex 4)/$(openssl rand -base64 32)"</code></pre>
```

```admonish hint
If you can guarantee that there can only ever be a single process / container that uses your volume mount, you probably
always want to set `cluster.wal_ignore_lock = true`. This will start an auto-recovery process after a container crashed
or was force-killed. If `hiqlite-wal` finds an existing lockfile, it will ignore it and try to recover orphaned logs,
and it does a deep integrity check of the latest WAL file to make sure all open in-memory buffers from before the crash
were written, or it will throw some away if their CRC does not match the header value.

Such a guarantee is given for instance when you only ever run Rauthy inside a container with a fixed name, that uses
some directory on disk. If that is true, your container runtime will only allow a single one, because there would be a
name conflict otherwise. Or if you run inside K8s with `ReadWriteOnce` volumes, a volume can only be mounted to a single 
Pod at a time. In these situations, you always want `cluster.wal_ignore_lock = true`.
```