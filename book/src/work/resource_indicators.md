# Resource Indicators (RFC 8707)

[RFC 8707](https://www.rfc-editor.org/rfc/rfc8707) lets a client tell Rauthy which protected
resource (resource server) an access token is intended for, using a `resource` request parameter.
Rauthy validates the request and binds the issued access token to that resource via its `aud`
(audience) claim. A resource server can then reject a token that was not minted for it, which stops
a token issued for one service being replayed against another.

This is especially useful when Rauthy acts as the authorization server for multiple resource
servers, for instance a fleet of MCP servers.

## The `resource` request parameter

The `resource` parameter is accepted on:

- the **authorization request** (`GET /oidc/authorize`), carried through the issued auth code,
- the **device authorization request** (`POST /oidc/device`), carried through the device code, and
- the **token request** (`POST /oidc/token`) for the `authorization_code`, `device_code`,
  `client_credentials`, and `refresh_token` grants, as well as the
  [Token Exchange](token_exchange.md).

The requested value is matched verbatim against the client's allow-list (see below). The RFC
recommends an absolute URI such as `https://api.example.com/mcp`, but Rauthy treats the value as
opaque, so an operator decides what a valid value looks like. A missing, unknown, or disallowed
value is rejected with the RFC error code `invalid_target`.

Rauthy currently accepts a single `resource` value per request. A `resource` on the token request
of the `authorization_code` or `device_code` grant must match the one granted with the code, or it
is rejected with `invalid_target`. Without it, the granted resource is used. On a `refresh_token`
grant the granted resource is carried inside the refresh token and reused, so a refreshed access
token keeps its audience binding. A `resource` sent with the refresh must match the granted one; a
refresh can never widen or switch the resource set.

```admonish note
The `password` grant and logins brokered through an upstream identity provider do not process the
`resource` parameter yet.
```

## Per-client configuration

Two optional, comma-separated fields can be configured per client, in the Admin UI under the client
config or via the clients API:

- **`allowed_resources`** is the allow-list against which a requested `resource` is validated. If it
  is empty, any `resource` request is rejected with `invalid_target` (deny by default). To let a
  client request a resource, add the exact value here. The same list is used for the target of a
  [Token Exchange](token_exchange.md), because the question is the same one: may this client mint a
  token for that target?
- **`default_aud`** is a list of audiences that are **always** added to this client's access tokens,
  independent of any `resource` parameter. This is handy for less capable clients or IoT devices
  that cannot send a `resource` parameter but still need a fixed audience.

The final access-token `aud` is the client id, plus every `default_aud` entry, plus the granted
`resource` (de-duplicated).

## The `aud` claim shape

The `aud` claim is a single string when there is exactly one audience, and a JSON array when there
are two or more. Both forms are valid per RFC 7519. Consumers that already read `aud` as a string
keep working for the single-audience case.

```admonish note
Solid-OIDC ephemeral clients (`enable_solid_aud`) previously received an `aud` value that was a
string containing array-like text. With this feature it is emitted as a proper JSON array.
```

## Ephemeral clients

An ephemeral client document may declare its own `allowed_resources`, which is validated the same
way as for a normal client. If it declares none, a requested `resource` is rejected unless the
operator opts in with:

```toml
[ephemeral_clients]
danger_allow_unvalidated_resource = true
```

This defaults to `false` (deny) so that a client without a declared allow-list cannot silently
obtain a token for an arbitrary audience.

```admonish danger
Only enable `danger_allow_unvalidated_resource` if you know exactly what you are doing and have a
good reason. It can lead to an easy privilege escalation, because an ephemeral client could then
mint tokens for an arbitrary audience.
```

### Consent sources

With `consent.enable`, a consent source is resolved per client and `resource` for the consent the
login page asks for. A `[[consent.sources]]` entry for the requested `resource` wins, then the
client's own consent source from the Admin UI, then the built-in `local` one:

```toml
[consent]
#enable = true

[[consent.sources]]
resource = 'https://api.example.com'
source = 'https://consent.example.com/api'
name = 'Example API'
```

Ephemeral clients have no stored configuration, so their consent comes from this registry or the
built-in source. While the registry has an entry, an ephemeral client must request a `resource`:
an authorization request without one is answered with `error=consent_required`.

The registry `name` is shown on the consent screen and wins over the `name` of a remote consent
document. The remote `name` is only used for a client's own consent source from the Admin UI.

A remote source is fetched via `GET <url>?lang=<BCP47 language>` with the same redirect and proxy
rules as ephemeral client documents, but with its own connection limit and a 3 second timeout, so a
slow consent source cannot hold up ephemeral client lookups. Unlike an ephemeral client document, a
consent source itself may resolve to a non-public address, because only an admin configures it,
and such a source may redirect to another one. A redirect from a source on a public address into
the internal network, to a loopback, private or otherwise non-public address, is refused. The
content is cached for `consent.content_cache_ttl`.

The `lang=en` response is required and the default: the content is taken from the response for
the user's language, or the `lang=en` one if that is unavailable. The version of the consent content
is the hash of the content as shown: the name, `intro`, `terms_url` and the requested scopes with
their title, description and warning (the warnings of the `lang=en` response are shown in every
language). Any edit to a shown field asks for consent again, an edit to a scope that was not
requested does not. A user who switches the language may be asked again. If the `lang=en`
response is unavailable or returns an invalid document, the built-in content is used instead. These
fallbacks change the version as well and may ask again: the built-in content while `lang=en` is
unavailable or invalid, and the `lang=en` content while the translation is unavailable. A
failed fetch is cached for `min(content_cache_ttl, 60s)` per language, so an unavailable
source is not requested on every lookup.

### Consent screen

With `consent.enable`, the login page shows a consent block below the login form whenever the
user's consent grant for the client and `resource` does not cover the request: for every login
without a session, for a requested scope that was not accepted before, after any change of the
consent content of the accepted scopes, and with `prompt=consent`. With a still valid session,
the page shows only the consent block and signs in without asking for credentials again. A request
with `prompt=none` that needs consent is answered with `error=consent_required`.

The client's `default_scopes` are always accepted and shown locked, any other requested scope can
be declined. The block must be confirmed before the login. The login then writes the grant with
the accepted scopes, the time, IP and user agent, and the issued tokens carry only the accepted
scopes. With a Passkey or an OTP, the grant is written once the ceremony succeeded.

For a `local` source, the standard OIDC scopes come with built-in descriptions. For a `remote`
one, the `description` of a scope is shown as plain text, HTML in it is not rendered. A user
can list and revoke the grants in the Privacy tab of the account dashboard, one by one or all at
once, and a revoke also deletes the refresh tokens of the client and all sessions of the user, who
then signs in again, with a backchannel logout. A revoke by an admin does the same. FedCM shows no consent block and
needs a grant from an earlier login.

## Introspection

The token introspection endpoint (`POST /oidc/introspect`) reflects the restricted `aud`
automatically, using the same single-string-or-array shape as the token.
