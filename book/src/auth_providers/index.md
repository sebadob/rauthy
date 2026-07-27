# Upstream Authentication Providers

This section is about upstream authentication providers, for instance things like **Sign in with GitHub**.

This feature can be considered "in beta state" and the only setup guide is for GitHub for now.  
Additional ones may come, when people are using other providers and give feedback or actually contribute to these docs.

## Direct Login with `idp_hint`

Rauthy supports the `idp_hint` query parameter in the OIDC authorization request.
When this parameter is provided with the ID of an upstream authentication provider, Rauthy will automatically redirect the user to that provider's login page, bypassing the Rauthy login selection.

For example, if you have a GitHub provider with the ID `github` configured:
`GET /auth/v1/oidc/authorize?client_id=...&idp_hint=github`

This is useful if you want to provide a "Login with GitHub" button directly in your application that skips the intermediate Rauthy login selection page.
The `idp_hint` value must match the `ID` of the provider as configured in the Admin UI.