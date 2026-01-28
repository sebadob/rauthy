# Ephemeral Clients

In addition to static, upfront registered clients, which should always be preferred if they work out for your, and
dynamically registered clients via OIDC DCR, Rauthy supports an additional feature, which is *Ephemeral Clients*.
Originally implemented when the support for SolidOIDC came, this feature is fully working on its own. These are very
easy to use and provide a completely new functionality, but keep in mind that these are the least efficient of the 3,
as they require an additional network round trip to where ever their document is hosted (apart from caching).

Just to make clear what I am talking about, a high level comparison of these different clients:

**Static clients** are the ones that you register and configure via the Admin UI. These are the most efficient and
secure ones. They require less work in the backend and exist inside the Rauthy database.

Then there is OIDC **Dynamic Client Registration** (DCR), which is an OIDC extension that Rauthy supports as well. If a
downstream application has support for this feature, it can self-register a client, which then can be used for the
login afterward. This sound very nice in the beginning, but brings quite a few problems:

- If the endpoint is open, anyone can register a client, also bots and spammers
- If the endpoint is secured an additional token for the registration, there is not much benefit from it, because the
  token must be communicated upfront anyway. If so, you could simply register a faster and more efficient static client.
- The downstream application must manage its own client. If they don't do it properly, they can spam Rauthys database
  again.
- As mentioned already, dynamic clients are a bit less efficient than static ones and require at least one additional
  database round trip during each login.

```admonish info
In case of an open, unauthenticated endpoint for DCR, Rauthy has a simple internal cleanup logic. For instance, if a 
dynamic client is registered by bots or spammers and is not used within the first hour after the registration, it will 
be deleted automatically. Usually, when an app registers via DCR, the client will be used immeditately for at least one
login.
```

As the third option, Rauthy gives you the ability to use **Ephemeral Clients**. These are disabled by default, because
they are not a default OIDC feature yet (There is an open Draft to include them though). Ephemeral Clients provide
way fewer configuration options, but make it possible that user can self-manage their own clients without any additional
upfront tasks on Rauthy's side. This means as long as you have an account on a Rauthy instance with this feature
enabled, you would not even need an admin to be able to connect any new application to this Rauthy instance.

The client definition exists in some JSON document that is hosted on some domain the user controls, which must be
reachable by Rauthy. The format must be parsable by Rauthy of course, which I will mention later. Whatever information
you provide in this JSON document, Rauthy will use it to authenticate your login request with that very client. The
most important part about this is, that the `client_id` **must match** the exact URI the document is hosted on.  
For instance, let's say you have a JSON on a GitHub repo at
[https://github.com/sebadob/rauthy/blob/main/ephemeral_client_example.json](https://github.com/sebadob/rauthy/blob/main/ephemeral_client_example.json)
, then the `client_id` **must** be

```
"client_id": "https://github.com/sebadob/rauthy/blob/main/ephemeral_client_example.json"
```

```admonish danger
It is very important that the domain for an ephemeral client JSON is under your control and no one else you don't fully
trust has access. Otherwise, someone else could modify this JSON which affects your login flow. For instance, add an
additional allowed `redirect_uri` and trick you into a scam website.
```

To keep the whole setup as simple as possible, apart from the `client_id`, there is only one other mandatory field
in this document, which is `redirect_uris`. This must be an array with at least one allowed redirect URI which is
allowed during the login flow. Rauthy allows a wildcard `*` at the end of given redirect uri, but be careful with these.
Wildcards should only be used during initial setup and testing, when you don't know the final URI yet.

```admonish danger
Technically, it is possible to just use a single <code>*</code> as a <code>redirect_uri</code>. This would allow any
redirect URI without further validation. You must never use this apart from maybe some internal DEV or testing setups!
```

The JSON document for the ephemeral clients follows the same rules and works in the same ways any other OIDC client.

## Configuration

The support for ephemeral clients is opt-in. You need to set at least

```toml
[ephemeral_clients]
# Can be set to 'true' to allow the dynamic client lookup via
# URLs as 'client_id's during authorization_code flow initiation.
#
# default: false
# overwritten by: ENABLE_EPHEMERAL_CLIENTS
enable = false
```

Apart from this, there are more options you can modify. Ephemeral clients are the least flexible, and they share some
common restrictions. These are valid for all ephemeral clients used with this instance, no matter if they define
something else in their JSON document. This is important so a Rauthy admin can configure a security standard that
can't be broken or ignored.

```toml
[ephemeral_clients]
# If set to 'true', MFA / Passkeys will be forced for ephemeral
# clients.
#
# default: false
# overwritten by: EPHEMERAL_CLIENTS_FORCE_MFA
force_mfa = false

# The allowed flows for ephemeral clients.
#
# default: ['authorization_code', 'refresh_token']
# overwritten by: EPHEMERAL_CLIENTS_ALLOWED_FLOWS - single String, \n separated values
allowed_flows = ['authorization_code', 'refresh_token']

# The allowed scopes separated by ' ' for ephemeral clients.
#
# default: ['openid', 'profile', 'email', 'webid']
# overwritten by: EPHEMERAL_CLIENTS_ALLOWED_SCOPES - single String, \n separated values
allowed_scopes = ['openid', 'profile', 'email', 'webid']
```

If you need support for Solid OIDC, you need to at least enable web IDs and the solid `aud`:

```toml
[ephemeral_clients]
# Can be set to 'true' to enable WebID functionality like needed
# for things like Solid OIDC.
#
# default: false
# overwritten by: ENABLE_WEB_ID
enable_web_id = false

# If set to 'true', 'solid' will be added to the 'aud' claim from
# the ID token for ephemeral clients.
#
# default: false
# overwritten by: ENABLE_SOLID_AUD
enable_solid_aud = false
```

The last option is the caching timeout. The cache for these documents is very important for performance. Without any
cache, Rauthy would need to do an additional network round trip to the client JSON **with each login**. This is very
inefficient, because usually these documents rarely change after the initial setup.

```toml
[ephemeral_clients]
# The lifetime in seconds ephemeral clients will be kept inside
# the cache.
#
# default: 3600
# overwritten by: EPHEMERAL_CLIENTS_CACHE_LIFETIME
cache_lifetime = 3600
```

```admonish hint
If you are developing or testing, set the <code>cache_lifetime</code> to a very low value. Otherwise, Rauthy would not 
see any changes you do to this document as long as the internal cache has not expired.
```

## Minimal Client JSON Document

The minimal requirements for the client JSON were described above already. They are `client_id` and `redirect_uris`.
This means a minimal example JSON could look like this

```json
{
  "client_id": "https://example.com/my_ephemeral_client.json",
  "redirect_uris": [
    "https://this.is.where/my/website/is"
  ]
}
```

If this is all you care about, you're done.  
If you want to configure some more things, keep reading.

## Advanced Client JSON Document

All values in the client document are validated of course. Below you see a simplified version of the current Rust
`struct` Rauthy tries to deserialize the document into:

```rust
struct EphemeralClientRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$`
    client_id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    client_name: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    client_uri: Option<String>,
    /// Validation: `Vec<^[a-zA-Z0-9\+.@/]{0,48}$>`
    contacts: Option<Vec<String>>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    redirect_uris: Vec<String>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    post_logout_redirect_uris: Option<Vec<String>>,
    /// Validation: `Vec<^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$>`
    grant_types: Option<Vec<String>>,
    /// Validation: `60 <= access_token_lifetime <= 86400`
    default_max_age: Option<i32>,
    /// Validation: `[a-z0-9-_/:\s*]{0,512}`
    scope: Option<String>,
    require_auth_time: Option<bool>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    access_token_signed_response_alg: Option<JwkKeyPairAlg>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    id_token_signed_response_alg: Option<JwkKeyPairAlg>,
}
```

```admonish into
The `grant_types` exist for completeness only. They are actually ignored and overridden by the 
`EPHEMERAL_CLIENTS_ALLOWED_FLOWS` config variable.
```

With this information, we can now build a complete example JSON with all the possible values you can set:

```json
{
  "client_id": "https://example.com/my_ephemeral_client.json",
  "client_name": "My Ephemeral Client",
  "client_uri": "https://this.is.where",
  "contacts": [
    "mail@ephemeral.info"
  ],
  "redirect_uris": [
    "https://this.is.where/my/website/is"
  ],
  "post_logout_redirect_uris": [
    "https://this.is.where"
  ],
  "default_max_age": 300,
  "scope": "openid email custom",
  "require_auth_time": true,
  "access_token_signed_response_alg": "RS256",
  "id_token_signed_response_alg": "RS256"
}
```

```admonish hint
Rauthy fetches the document with an <code>Accept: application/json</code> header. This makes it possible to host your
file on GitHub for instance and receive a valid JSON.
```