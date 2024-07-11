# Bootstrapping

Rauthy supports some basic bootstrapping functionality to make it easier to use in some kind of automated environment.

## Admin User

By default, the admin user will always be `admin@localhost.de` and the password will be auto-generated and shown
only once in the logs. This is the easiest way if you want to set up a new instance manually.

However, you can change the default admin username and also provide a bootstrap password for this user, if Rauthy
starts with an empty database for the very first time. This makes it possible to either generate a secure password with
external tooling and have more control over it, or just use the same one all the time, because you may spin up an
instance in test pipelines.

You can change the default admin user E-Mail with:

```
# If set, the email of the default admin will be changed
# during the initialization of an empty production database.
BOOTSTRAP_ADMIN_EMAIL="alfred@batcave.io"
```

The password can be bootstrapped in 2 ways:

- Provide a plain test password
- Provide the password hashed with argon2id

```
# If set, this plain text password will be used for the
# initial admin password instead of generating a random
# password.
BOOTSTRAP_ADMIN_PASSWORD_PLAIN="123SuperSafe"

# If set, this will take the argon2id hashed password
# during the initialization of an empty production database.
# If both BOOTSTRAP_ADMIN_PASSWORD_PLAIN and
# BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID are set, the hashed version
# will always be prioritized.
BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID='$argon2id$v=19$m=32768,t=3,p=2$mK+3taI5mnA+Gx8OjjKn5Q$XsOmyvt9fr0V7Dghhv3D0aTe/FjF36BfNS5QlxOPep0'
```

```admonish caution
When you provide the password hashed with argon2id, you need to properly quote the string like in this example with
single `''`. This will probably change in a future version, so you would need to encode it as base64, but this is not 
the case right now (v0.24.0).
```

## API Key

In automated environments, you would usually not use a normal user to access the Rauthy API, since it is a bit more
cumbersome to use from outside the browser because of additional CSRF tokens and security features. If you want to
automatically set up Rauthy with external tooling after the first startup, you would want to do this with an API key
most probably.

If Rauthy starts up with an empty database, you can bootstrap a single API key with providing a base64 encoded json.

An example json, which would create a key named `bootstrap` with access to `clients, roles, groups` with all
`read, write, update, delete` could look like this:

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

```
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
# an example. Afterwards, just `base64 api_key_example.json | tr -d '\n'`
BOOTSTRAP_API_KEY="ewogICJuYW1lIjogImJvb3RzdHJhcCIsCiAgImV4cCI6IDE3MzU1OTk2MDAsCiAgImFjY2VzcyI6IFsKICAgIHsKICAgICAgImdyb3VwIjogIkNsaWVudHMiLAogICAgICAiYWNjZXNzX3JpZ2h0cyI6IFsKICAgICAgICAicmVhZCIsCiAgICAgICAgImNyZWF0ZSIsCiAgICAgICAgInVwZGF0ZSIsCiAgICAgICAgImRlbGV0ZSIKICAgICAgXQogICAgfSwKICAgIHsKICAgICAgImdyb3VwIjogIlJvbGVzIiwKICAgICAgImFjY2Vzc19yaWdodHMiOiBbCiAgICAgICAgInJlYWQiLAogICAgICAgICJjcmVhdGUiLAogICAgICAgICJ1cGRhdGUiLAogICAgICAgICJkZWxldGUiCiAgICAgIF0KICAgIH0sCiAgICB7CiAgICAgICJncm91cCI6ICJHcm91cHMiLAogICAgICAiYWNjZXNzX3JpZ2h0cyI6IFsKICAgICAgICAicmVhZCIsCiAgICAgICAgImNyZWF0ZSIsCiAgICAgICAgInVwZGF0ZSIsCiAgICAgICAgImRlbGV0ZSIKICAgICAgXQogICAgfQogIF0KfQ=="
```

The secret needs to be set with a second variable. Just make sure it contains at least 64 alphanumeric characters.

```
# The secret for the above defined bootstrap API Key.
# This must be at least 64 alphanumeric characters long.
# You will be able to use that key afterwards with setting
# the `Authorization` header:
#
# `Authorization: API-Key <your_key_name_from_above>$<this_secret>`
BOOTSTRAP_API_KEY_SECRET=twUA2M7RZ8H3FyJHbti2AcMADPDCxDqUKbvi8FDnm3nYidwQx57Wfv6iaVTQynMh
```

With the values from this example, you will then be able to use the API key with providing the `Authorization` header
with each request in the following format:

```
Authorization: API-Key <API Key name>$<API Key secret>
```

```
Authorization: API-Key bootstrap$twUA2M7RZ8H3FyJHbti2AcMADPDCxDqUKbvi8FDnm3nYidwQx57Wfv6iaVTQynMh
```

```admonish hint
API Keys are not nicely documented in this book yet, but should be self-explanatory when you take a look at the section
in the Admin UI.
```