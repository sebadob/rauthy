# Token Exchange (RFC 8693)

[RFC 8693](https://www.rfc-editor.org/rfc/rfc8693) lets a client trade an existing token for a new
one. The typical case is a service that receives an access token from a caller and needs to call a
second service downstream: instead of forwarding the original token, which was minted for itself, it
exchanges it for a token whose audience is the downstream service.

Rauthy supports two shapes of this:

- **Impersonation**: the new token looks like a token for the original subject. A downstream service
  cannot tell that somebody exchanged it.
- **Delegation**: the new token additionally carries an `act` (actor) claim naming the acting party,
  so a downstream service can distinguish "A acting on behalf of B" from "B".

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
| `subject_token_type` | required | Must be `urn:ietf:params:oauth:token-type:access_token`. |
| `actor_token` | optional | Presence turns the exchange into a delegation and adds the `act` claim. |
| `actor_token_type` | required with `actor_token` | Must be `urn:ietf:params:oauth:token-type:access_token`. |
| `audience` | optional | The target the new token is for. Validated against `allowed_resources`. |
| `resource` | optional | Same as `audience`. Only one of the two may be given. |
| `scope` | optional | Narrows the scopes. It can never widen the `subject_token`'s scopes. |
| `requested_token_type` | optional | Only `urn:ietf:params:oauth:token-type:access_token` is supported, which is also the default. |

```admonish note
Rauthy only accepts **access tokens** as `subject_token` / `actor_token`, and the exchanged token is
always an access token. The exchanged token never comes with a refresh token: a refresh must come
from the client that owns the original token, with a fresh exchange on top. The `may_act` claim is
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
