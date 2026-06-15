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

- the **authorization request** (`GET /oidc/authorize`), carried through the issued auth code, and
- the **token request** (`POST /oidc/token`) for the `authorization_code`, `client_credentials`,
  and `refresh_token` grants.

Its value MUST be an absolute URI without a fragment (for example
`https://api.example.com/mcp`). A malformed, unknown, or disallowed value is rejected with the
RFC error code `invalid_target`.

Rauthy v1 accepts a single `resource` value per request. On a `refresh_token` grant the granted
resource is carried inside the refresh token and reused, so a refreshed access token keeps its
audience binding; a refresh can never widen the resource set.

```admonish note
The `password` and device-code grants, and logins brokered through an upstream identity provider,
do not process the `resource` parameter in this version.
```

## Per-client configuration

Two optional, comma-separated fields can be configured per client, in the Admin UI under the client
config or via the clients API:

- **`allowed_resources`** is the allow-list against which a requested `resource` is validated. If it
  is empty, any `resource` request is rejected with `invalid_target` (deny by default). To let a
  client request a resource, add the exact resource URI here.
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
allow_unvalidated_resource = true
```

This defaults to `false` (deny) so that a client without a declared allow-list cannot silently
obtain a token for an arbitrary audience.

## Introspection

The token introspection endpoint (`POST /oidc/introspect`) reflects the restricted `aud`
automatically, using the same single-string-or-array shape as the token.
