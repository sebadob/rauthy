# LDAP

Some apps can only check a username and password against a directory with an LDAP bind, for
instance the native Jellyfin apps through the Jellyfin LDAP plugin. Rauthy can serve these apps
directly with a minimal, **read-only** LDAPS server, so you don't need a second user directory next
to it.

It is disabled by default. Enable it in the `[ldap]` section:

```toml
[ldap]
enable = true
port = 6360
base_dn = 'dc=example,dc=com'
# optional, defaults to the `[tls]` certificate
#cert_path = 'tls/ldap.crt'
#key_path = 'tls/ldap.key'
```

Only implicit TLS (LDAPS) is supported, StartTLS is not. If Rauthy itself runs plain HTTP behind a
reverse proxy, set `cert_path` and `key_path`, because LDAP clients connect to Rauthy directly.

## Supported Operations

Simple bind, search, unbind, the RootDSE and Who Am I (RFC 4532). Every other operation returns
`unwillingToPerform`. Search filters support `and`, `or`, `not`, equality, presence and substring.

## Clients

Each LDAP app is a Rauthy client with the `password` flow enabled. Binds and searches are scoped to
this client, and every client gets its own subtree:

```text
cn=<client_id>,ou=clients,<base_dn>                        the app's bind DN and base DN
uid=<uid>,ou=people,cn=<client_id>,ou=clients,<base_dn>    users
cn=<group>,ou=groups,cn=<client_id>,ou=clients,<base_dn>   groups
```

- **Service bind:** the app binds as `cn=<client_id>,ou=clients,<base_dn>` with the client secret.
  This requires a confidential client.
- **User bind:** a user binds as `uid=<uid>,ou=people,cn=<client_id>,ou=clients,<base_dn>` with
  their password. This runs the same checks as the `password` grant: brute-force prevention and
  login delays, credential stuffing detection, enabled / expired users and the client's group
  restriction (`restrict_group_prefix`). A user bind can only search its own entry, the whole
  directory is only visible to the service bind.

All users of an app bind from the app's IP. By default, a few failed user binds with different
usernames therefore trigger the credential stuffing detection and block the app's IP for everyone.
If the LDAPS port is only reachable for your apps, you can set
`ldap.disable_ip_brute_force_protection = true`. User binds then skip all per-IP checks and only
the per-user failed login counter slows down repeated failures.

Like the `password` grant, LDAP binds cannot do MFA. A user with MFA or passkeys who also has a
local password can log in with the password alone. Only users with a local password can log in,
passkey-only or upstream-provider-only accounts are not part of the directory.

Connections are closed after 5 minutes idle and after 1 hour at most, so a rotated client secret
applies to open connections within that time.

## Directory Layout

The layout follows LLDAP, so existing app presets only need a different base DN.

Users (`ou=people`) are all users allowed for the client that have a local password and are enabled
and not expired. A PAM username or `preferred_username` that is not unique case-insensitively, or
that equals another user's email, falls back to the email as `uid`:

| attribute                                   | value                                                    |
|---------------------------------------------|----------------------------------------------------------|
| `uid`                                       | the PAM username, else the `preferred_username`, else the email |
| `entryUUID`                                 | the Rauthy user ID                                       |
| `mail`                                      | email                                                    |
| `cn`, `displayName`                         | given name + family name                                 |
| `givenName`, `sn`                           | given name, family name                                  |
| `memberOf`                                  | the DNs of the user's groups                             |
| `uidNumber`, `gidNumber`, `homeDirectory`, `loginShell` | only for users with a [PAM](pam.md) user     |

Groups (`ou=groups`) are Rauthy groups, filtered by the client's `restrict_group_prefix` if set,
with `cn` and `member` / `uniqueMember`.

## Example: Jellyfin LDAP Plugin

Create a confidential client `jellyfin` with the `password` flow enabled, optionally with a group
prefix like `jellyfin`, and configure the plugin with:

| setting                    | value                                                        |
|----------------------------|--------------------------------------------------------------|
| LDAP Server / Port         | your Rauthy host / `6360`                                    |
| Secure LDAP                | enabled                                                      |
| LDAP Bind User             | `cn=jellyfin,ou=clients,dc=example,dc=com`                   |
| LDAP Bind User Password    | the client secret                                            |
| LDAP Base DN for searches  | `cn=jellyfin,ou=clients,dc=example,dc=com`                   |
| LDAP Search Filter         | `(objectClass=inetOrgPerson)`                                |
| LDAP Search Attributes     | `uid, mail`                                                  |
| LDAP Uid Attribute         | `entryUUID`                                                  |
| LDAP Username Attribute    | `uid`                                                        |
| LDAP Admin Filter          | `(memberOf=cn=jellyfin_admins,ou=groups,cn=jellyfin,ou=clients,dc=example,dc=com)` |

You can test a setup with the OpenLDAP tools:

```bash
ldapwhoami -H ldaps://rauthy.example.com:6360 \
  -D 'cn=jellyfin,ou=clients,dc=example,dc=com' -w '<client secret>'
ldapsearch -H ldaps://rauthy.example.com:6360 \
  -D 'cn=jellyfin,ou=clients,dc=example,dc=com' -w '<client secret>' \
  -b 'cn=jellyfin,ou=clients,dc=example,dc=com' '(uid=alice)'
```
