# NetBird

[NetBird](https://netbird.io) is a WireGuard-based overlay network. A self-hosted NetBird uses its own embedded IdP,
which can log users in with any external OIDC provider. You can add Rauthy there as a **Generic OIDC** provider.

> [!NOTE]
> You need Rauthy v0.37.1 or newer. The Generic OIDC connector in NetBird reads the `name` claim
> from the `id_token` only,
> and older versions of Rauthy do not include it there.

## Rauthy Client

Create a new client in the Admin UI:

- **Client ID**: `netbird`
- **Client Name**: `NetBird`
- **Confidential**: on
- **Redirect URIs**: `https://netbird.example.com/oauth2/callback`
- **Post Logout Redirect URIs** (optional): `https://netbird.example.com/oauth2/logout/callback`

Replace `netbird.example.com` with the domain of your NetBird dashboard. The defaults for a new client work as they are:
the `authorization_code` flow with `S256` PKCE, and the `openid`, `profile` and `email` scopes that NetBird requests.

Afterward, open the client and copy its secret.

## NetBird Identity Provider

In the NetBird dashboard, go to **Settings** -> **Identity Providers** -> **Add Identity Provider** and set:

| Field         | Value                                |
|---------------|--------------------------------------|
| Type          | Generic OIDC                         |
| Name          | Rauthy                               |
| Client ID     | `netbird`                            |
| Client Secret | the secret from the Rauthy client    |
| Issuer        | `https://rauthy.example.com/auth/v1` |

The issuer must contain the `/auth/v1` path. After saving, NetBird shows its redirect URL. If it differs from the one
you configured on the client, add it to the client's Redirect URIs.

## Groups

NetBird does not request the `groups` scope, even if the client allows it. To get the `groups` claim into the
`id_token`, add `groups` to the **Default Scopes** of the client. Then, in NetBird, go to **Settings** -> **Groups**,
enable **JWT group sync** and set the **JWT claim** to `groups`.

If only some of your users should be able to log in to NetBird, set a **Login Group Prefix** on the client.
