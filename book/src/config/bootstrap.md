# Bootstrapping

Rauthy can bootstrap most values that are nore statically configured. There are only very few
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

If Rauthy starts up with an empty database, you can bootstrap a single API key with providing a
base64 encoded json.

An example json, which would create a key named `bootstrap` with access to `clients, roles, groups`
with all `read, write, update, delete` could look like this:

```json
{
  "name": "bootstrap",
  "exp": 1735599600,
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
```

The config documentation for the bootstrap value should explain all further questions:

```toml
[bootstrap]
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
At the time of writing, it is not possible (yet) to provide API key secrets as encrypted values.
This will probably be added in a future version. If you bootstrap API keys for production setup,
make sure they always auto-expire.
```

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
