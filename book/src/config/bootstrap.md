# Bootstrapping

Rauthy can bootstrap most values that are not statically configured. There are only very few
exceptions, so you should be able to achieve anything you need. The most important things live in
the config to be able to run everything from a single file. However, when you want to do more
advanced / custom stuff, you will need to provide additional JSON files.

## Admin User

By default, the admin user will always be `admin@localhost` and the password will be auto-generated
and shown only once in the logs during the very first startup on the first node.

You can change the default admin email and also provide a bootstrap password for it. This makes it
possible to either generate a secure password with external tooling and have more control over it,
or just use the same one all the time, because you may spin up an instance in test pipelines.

You can change the default admin user E-Mail with:

```toml
[bootstrap]
# If set, the email of the default admin will be changed during
# the initialization of an empty production database.
#
# default: 'admin@localhost'
# overwritten by: BOOTSTRAP_ADMIN_EMAIL
admin_email = 'admin@localhost'
```

The password can be bootstrapped in 2 ways:

- Provide a plain test password (only use for testing)
- Provide the password hashed with argon2id

```toml
[bootstrap]
# If set, this plain text password will be used for the initial
# admin password instead of generating a random password.
#
# default: random -> see logs on first start
# overwritten by: BOOTSTRAP_ADMIN_PASSWORD_PLAIN
password_plain = '123SuperSafe'

# If set, this will take the Argon2ID hashed password during the
# initialization of an empty production database. If both
# `password_plain` and `pasword_argon2id` are set, the hashed
# version will always be prioritized.
#
# default: random -> see logs on first start
# overwritten by: BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID
pasword_argon2id = '$argon2id$v=19$m=32768,t=3,p=2$mK+3taI5mnA+Gx8OjjKn5Q$XsOmyvt9fr0V7Dghhv3D0aTe/FjF36BfNS5QlxOPep0'
```

You can use any tool for hashing as long as the output format is a proper argon2id string.
The [Rauthy CLI](cli.md) can do it for you as well.

## API Key

In automated environments, you would usually not use a normal user to access the Rauthy API, since
it is a bit more cumbersome to use from outside the browser because of additional CSRF tokens and
security features. If you want to automatically set up Rauthy with external tooling after the first
startup, you would want to do this with an API key most probably.

If Rauthy starts up with an empty database, you can bootstrap API keys via
`api_keys.json` in your configured `bootstrap_dir`. The older `bootstrap.api_key`
config value shown below is still supported for a single key.

An example `api_keys.json`, which would create a key named `bootstrap` with access to
`clients`, `roles`, and `groups` with all `read`, `create`, `update`, `delete`
rights could look like this:

```json
[
  {
    "name": "bootstrap",
    "exp": 1735599600,
    "secret": {
      "Plain": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    },
    "access": [
      {
        "group": "Clients",
        "access_rights": [
          "read",
          "create",
          "update",
          "delete"
        ]
      },
      {
        "group": "Roles",
        "access_rights": [
          "read",
          "create",
          "update",
          "delete"
        ]
      },
      {
        "group": "Groups",
        "access_rights": [
          "read",
          "create",
          "update",
          "delete"
        ]
      }
    ]
  }
]
```

The `secret` can be `{"Plain": "..."}`, `{"Encrypted": "..."}`, or `"generate"`. Plain secrets must be at
least 64 characters long. Encrypted secrets are encrypted with
[`cryptr`](https://github.com/sebadob/cryptr) and base64-encoded, just like encrypted client
secrets. Generated API-key secrets store the full usable `name$secret` token in the generated
bootstrap secret container under `api-key/<name>/token`.

```json
[
  {
    "name": "provision",
    "exp": 1735599600,
    "secret": "generate",
    "access": [
      {
        "group": "Clients",
        "access_rights": ["read", "create", "update", "delete"]
      },
      {
        "group": "Secrets",
        "access_rights": ["read", "update"]
      }
    ]
  }
]
```

```admonish caution
Only bootstrap short-lived API keys for production automation. Set `exp` to a timestamp
that expires after your first-start provisioning is complete, for example after about 10
minutes. Long-lived bootstrap API keys increase the blast radius if the bootstrap files or
deployment secrets are leaked. The example timestamp above is intentionally finite; replace it
with a short-lived value for your deployment.
```

The legacy config documentation for bootstrapping a single API key should explain all further
questions:

```toml
[bootstrap]
# You can provide a single API Key during the initial prod database
# bootstrap. This legacy config value must match the format and pass validation.
# Prefer `api_keys.json` in `bootstrap_dir` for new deployments.
# You need to provide it as a base64 encoded JSON in the format:
#
# ```
# struct ApiKeyRequest {
#     /// Validation: `^[a-zA-Z0-9_-/]{2,24}$`
#     name: String,
#     /// Unix timestamp in seconds
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
#     Pam,
# }
#
# #[serde(rename_all="lowercase")]
# enum AccessRights {
#     Read,
#     Create,
#     Update,
#     Delete,
# }
# ```
#
# You can use the `api_key_example.json` from `/` as
# an example. Afterward, just
# `base64 api_key_example.json | tr -d '\n'`
#
# overwritten by: BOOTSTRAP_API_KEY
api_key = 'ewogICJuYW1lIjogImJvb3RzdHJhcCIsCiAgImV4cCI6IDE3MzU1OTk2MDAsCiAgImFjY2VzcyI6IFsKICAgIHsKICAgICAgImdyb3VwIjogIkNsaWVudHMiLAogICAgICAiYWNjZXNzX3JpZ2h0cyI6IFsKICAgICAgICAicmVhZCIsCiAgICAgICAgImNyZWF0ZSIsCiAgICAgICAgInVwZGF0ZSIsCiAgICAgICAgImRlbGV0ZSIKICAgICAgXQogICAgfSwKICAgIHsKICAgICAgImdyb3VwIjogIlJvbGVzIiwKICAgICAgImFjY2Vzc19yaWdodHMiOiBbCiAgICAgICAgInJlYWQiLAogICAgICAgICJjcmVhdGUiLAogICAgICAgICJ1cGRhdGUiLAogICAgICAgICJkZWxldGUiCiAgICAgIF0KICAgIH0sCiAgICB7CiAgICAgICJncm91cCI6ICJHcm91cHMiLAogICAgICAiYWNjZXNzX3JpZ2h0cyI6IFsKICAgICAgICAicmVhZCIsCiAgICAgICAgImNyZWF0ZSIsCiAgICAgICAgInVwZGF0ZSIsCiAgICAgICAgImRlbGV0ZSIKICAgICAgXQogICAgfQogIF0KfQ=='

```

The secret needs to be set with a second variable. Make sure it contains at least 64 alphanumeric
characters.

```toml
[bootstrap]
# The secret for the above defined bootstrap API Key.
# This must be at least 64 alphanumeric characters long.
# You will be able to use that key afterward with setting
# the `Authorization` header:
#
# `Authorization: API-Key <your_key_name_from_above>$<this_secret>`
#
# overwritten by: BOOTSTRAP_API_KEY_SECRET
api_key_secret = 'twUA2M7RZ8H3FyJHbti2AcMADPDCxDqUKbvi8FDnm3nYidwQx57Wfv6iaVTQynMh'
```

With the values from this example, you will then be able to use the API key with providing the
`Authorization` header with each request in the following format:

```
Authorization: API-Key <API Key name>$<API Key secret>
```

```
Authorization: API-Key bootstrap$twUA2M7RZ8H3FyJHbti2AcMADPDCxDqUKbvi8FDnm3nYidwQx57Wfv6iaVTQynMh
```

```admonish caution
If you bootstrap API keys for production setup, make sure they always auto-expire. Bootstrap
JSON is only read while initializing an empty production database; changing `api_keys.json`
later does not reconcile live API keys.
```

## Generated Bootstrap Secrets

Generated bootstrap credentials are stored in an encrypted local container before their matching
database rows are inserted. This keeps later phases from creating live client, user, or API-key
rows whose generated plaintext cannot be recovered.

The container is a single AEAD-encrypted JSON payload. The expiry deadline lives inside that
encrypted payload, so startup and runtime expiry checks decrypt the container first. A positive TTL
also schedules a runtime purge after each write. Set the TTL to `0` only if you intentionally want
to keep the encrypted container for manual extraction and purge it yourself.

Generated values are a first-boot mechanism only. The JSON bootstrap files are read while Rauthy
initializes an empty production database. If you add `"generate"` to `clients.json`, `users.json`,
or `api_keys.json` after the database has already been initialized, that day-2 change is ignored by
the first-boot bootstrap gate and no new generated secret is written.

```toml
[bootstrap]
# Path to the encrypted generated-secret container. If unset, Rauthy stores it
# below Hiqlite's configured data directory as:
#
# `${cluster.data_dir}/bootstrap.secrets.enc`
#
# overwritten by: BOOTSTRAP_GENERATED_SECRETS_FILE
#generated_secrets_file = 'data/bootstrap.secrets.enc'

# Time in seconds before generated bootstrap secrets expire. A value of `0`
# disables expiry and runtime auto-purge.
#
# default: 600
# overwritten by: BOOTSTRAP_GENERATED_SECRETS_TTL
generated_secrets_ttl = 600
```

Use the local CLI to retrieve or purge the encrypted container after first start. The command does
not talk to the Rauthy server.

```bash
rauthy bootstrap get \
  -c config.toml \
  --kind client \
  --id my-service \
  --field secret \
  --format raw

rauthy bootstrap get -c config.toml --format env
rauthy bootstrap purge -c config.toml
```

`get --format env` emits names that include the entity kind, sanitized id, and field, for example
`RAUTHY_BOOTSTRAP_CLIENT_MY_SERVICE_SECRET=...` and
`RAUTHY_BOOTSTRAP_USER_ADMIN_EXAMPLE_COM_PASSWORD=...`.

The CLI loads the existing Rauthy config and initializes the configured encryption keys before it
decrypts the container. It does not accept encryption keys or the container path as command-line
arguments.

API keys can request a generated token with `"secret": "generate"`. `get --format env` emits it as
`RAUTHY_BOOTSTRAP_API_KEY_<SANITIZED_NAME>_TOKEN=...`.

Clients can request a generated confidential-client secret with `"secret": "generate"`:

```json
[
  {
    "id": "my-service",
    "name": "My Service",
    "secret": "generate",
    "redirect_uris": ["https://my-service.example.com/callback"],
    "enabled": true,
    "flows_enabled": ["authorization_code", "refresh_token"],
    "access_token_alg": "EdDSA",
    "id_token_alg": "EdDSA",
    "auth_code_lifetime": 60,
    "access_token_lifetime": 3600,
    "scopes": ["openid", "profile", "email"],
    "default_scopes": ["openid", "profile", "email"],
    "force_mfa": false
  }
]
```

If the `secret` field is absent or `null`, the client remains public and Rauthy forces `S256` PKCE.
Use `"secret": "generate"` only when you want a confidential client and plan to extract the
generated secret from the encrypted container.

Users can request a generated password with `"password": "generate"`:

```json
[
  {
    "email": "admin@example.com",
    "password": "generate",
    "roles": ["admin"],
    "enabled": true,
    "email_verified": true
  }
]
```

Rauthy writes the plaintext password to the encrypted container before hashing it and inserting the
user row. The database stores only the password hash.

API keys in `api_keys.json` can request a generated token with `"secret": "generate"`. The full
usable `name$secret` token is written to the encrypted container before the DB row is inserted.
Extract it with `--kind api-key --id <key-name> --field token` and use it directly as the
`Authorization` header value: `Authorization: API-Key <token>`.

## Advanced / Custom Configuration

If you want to bootstrap other things like users or clients, you need to do it a bit differently.
First, make sure to check the `bootstrap_dir` and configure it if necessary:

```toml
[bootstrap]
# This is the directory where the bootstrap logic will look for
# additional bootstrapping data. It will expect JSON files with
# fixed names inside this folder. If it finds any of them, it will
# try to parse and apply them during the bootstrapping process.
#
# The following files will be parsed in the given directory:
# - api_keys.json
# - roles.json
# - groups.json
# - scopes.json
# - user_attributes.json
# - users.json
# - clients.json
#
# For information about the structure and the applied validations,
# check `src/data/src/migration/bootstrap/types.rs`, and the
# `bootstrap/*.json` files for examples.
#
# default: 'bootstrap'
# overwritten by: BOOTSTRAP_DIR
bootstrap_dir = 'bootstrap'
```

By default, it expects JSON files in `./bootstrap/`. The reason why these are read as JSON instead
of from the config, is that you can then rely on your IDEs auto-completion and syntax checking
features, which do not work in a TOML string. You can check example
files [here](https://github.com/sebadob/rauthy/tree/main/bootstrap), or take a look at the
[Rust definitions](https://github.com/sebadob/rauthy/blob/main/src/data/src/migration/bootstrap/types.rs).
