# Token Exchange (RFC 8693)

[RFC 8693](https://www.rfc-editor.org/rfc/rfc8693) lets a client trade an existing token for a new
one. The typical case is a service that receives an access token from a caller and needs to call a
second service downstream: instead of forwarding the original token, which was minted for itself, it
exchanges it for a token whose audience is the downstream service.

Rauthy supports three shapes of this:

- **Impersonation**: the new token looks like a token for the original subject. A downstream service
  cannot tell that somebody exchanged it.
- **Delegation**: the new token additionally carries an `act` (actor) claim naming the acting party,
  so a downstream service can distinguish "A acting on behalf of B" from "B".
- **ID-JAG**: an ID token is exchanged for an Identity Assertion JWT Authorization Grant, which the
  client redeems at another authorization server. See [ID-JAG](#id-jag) below.

## The token exchange request

The exchange happens on the normal token endpoint (`POST /oidc/token`) with

```
grant_type=urn:ietf:params:oauth:grant-type:token-exchange
```

The client authenticates as itself, exactly like for a `client_credentials` request, and the
exchange is only allowed for confidential clients.

| Parameter | | Description |
| --- | --- | --- |
| `subject_token` | required | The token representing the identity the new token should act for. |
| `subject_token_type` | required | Must be `urn:ietf:params:oauth:token-type:access_token`, or `urn:ietf:params:oauth:token-type:id_token` for an ID-JAG. |
| `actor_token` | optional | Presence turns the exchange into a delegation and adds the `act` claim. |
| `actor_token_type` | required with `actor_token` | Must be `urn:ietf:params:oauth:token-type:access_token`. |
| `audience` | optional | The target the new token is for. Validated against `allowed_resources`. Required for an ID-JAG. |
| `resource` | optional | Same as `audience`. Only one of the two may be given, except for an ID-JAG. |
| `scope` | optional | Narrows the scopes. It can never widen the `subject_token`'s scopes. See below for an ID-JAG. |
| `requested_token_type` | optional | `urn:ietf:params:oauth:token-type:access_token`, which is also the default, or `urn:ietf:params:oauth:token-type:id-jag`. |

```admonish note
Apart from the ID-JAG, Rauthy only accepts **access tokens** as `subject_token` / `actor_token`,
and the exchanged token is always an access token. The exchanged token never comes with a refresh
token: a refresh must come from the client that owns the original token, with a fresh exchange on
top. The `may_act` claim is
not supported yet.
```

## Per-client configuration

The exchange is **deny by default** and needs two things configured on the exchanging client, in the
Admin UI under the client config or via the clients API:

1. **`token_exchange` in the client's Authentication Flows.** Without it, any exchange request from
   this client is rejected. It behaves like every other flow.
2. **`allowed_resources`** must contain the target, if an `audience` / `resource` is requested. This
   is the same allow-list used for [Resource Indicators](resource_indicators.md), because the
   question is the same one: may this client mint a token for that target? A requested target that
   is not on the list is rejected with the RFC error code `invalid_target`.

A `subject_token` that is not a valid access token, or has been revoked, is rejected with
`invalid_grant`.

## The `act` claim

With an `actor_token`, the exchanged token carries the acting party per
[RFC 8693 section 4.1](https://www.rfc-editor.org/rfc/rfc8693#section-4.1):

```json
{
  "sub": "the-original-subject",
  "act": {
    "sub": "the-acting-party"
  }
}
```

If the `actor_token` was itself the result of a delegation, its `act` claim is nested inside the new
one, so the whole chain stays visible to a downstream service.

## ID-JAG

An [Identity Assertion JWT Authorization Grant][id-jag] lets a client get an access token from
another authorization server for the logged-in user, without a second login there. The client
exchanges the user's ID token at Rauthy for an ID-JAG, and then redeems the ID-JAG at the other
authorization server with the `urn:ietf:params:oauth:grant-type:jwt-bearer` grant. This is used
by the MCP [Enterprise-Managed Authorization][mcp-ema] and Okta's Cross App Access.

```text
grant_type=urn:ietf:params:oauth:grant-type:token-exchange
requested_token_type=urn:ietf:params:oauth:token-type:id-jag
subject_token=<id token>
subject_token_type=urn:ietf:params:oauth:token-type:id_token
audience=https://as.example.com/
resource=https://mcp.example.com/mcp
scope=chat.read
```

Nothing is issued until the other authorization server is configured as a `[[id_jag.audiences]]`
entry in the config. Each entry lists the clients that may request an ID-JAG for it, and the
`resource` and `scope` values they may request. The client's `allowed_resources` are not used here.

- The client needs `token_exchange` in its Authentication Flows, just like for any other exchange.
- The `subject_token` must be an ID token Rauthy issued to the exchanging client, with a session
  that is still valid. ID tokens without a session, like the ones from the `password` grant, are
  rejected, and so is an ID token after a logout.
- An ID token bound to a DPoP key needs a DPoP proof with the same key.
- `audience` is required and is the issuer of the other authorization server. An unknown
  `audience` or `resource` is rejected with `invalid_target`, a client or `scope` that is not
  allowed with `access_denied`.
- `scope` has the meaning of the other authorization server.
- An `actor_token` is not supported.

The `client_id` claim is the Rauthy client id, so the client must be registered with the same id at
the other authorization server.

The ID-JAG has the header `typ: oauth-id-jag+jwt`, is always signed with RS256 and contains `iss`,
`sub`, `aud`, `client_id`, `jti`, `iat`, `exp`, `auth_time`, `amr`, plus `resource` and `scope` if
requested. `email` is added if the ID token contained it and it is verified. A DPoP proof on the
request adds a `cnf` claim with the key thumbprint. The ID-JAG never outlives the ID token or its
session and is never accepted by Rauthy itself.

The response has `issued_token_type: urn:ietf:params:oauth:token-type:id-jag` and
`token_type: N_A`, and the ID-JAG is in `access_token`.

[id-jag]: https://datatracker.ietf.org/doc/draft-ietf-oauth-identity-assertion-authz-grant/
[mcp-ema]: https://modelcontextprotocol.io/extensions/auth/enterprise-managed-authorization
