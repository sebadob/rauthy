# User Logout

User logout can mean quite a few different things and there are multiple ways for logging out users.

## User Initiated Logout

This is the most straight forward logging out. If a user (or an admin in the Admin UI) clicks the logout button and
confirms the logout, the associated Rauthy session will be logged out. This means, to access any Rauthy UI's again, the
user needs to do a fresh login.

If you have OIDC Backchannel Logout (mentioned below) configured for any clients, this will trigger it for the Session
ID `sid` only. This means only clients where the user logged in in the past that are associated with this `sid` will
receive a Logout Token. All others will not. This is NOT a global user logout, only for this very session.

```admonish caution
The User Initiated Logout does NOT affect already signed and issue stateless(!) tokens. A stateless token, once singned,
cannot be revoked by design, even though you might read things like this in many places on the internet.

It is impossible to revoke a stateless JWT token.
```

## Admin-Forced User Logout

An admin can of course force-logout users via the Admin UI as well. This can be done in 2 places:

- Admin UI -> Sessions -> click delete on a specific session or `Invalidate All Sessions`
- Admin UI -> Users -> select user -> Logout Tab -> Logout

The first session-targeted logout / invalidation will only trigger Backchannel Logouts for this specific session only.
If you log out a user via the Logout Tab for a selected user, it will be a global User Logout everywhere. This will
delete all existing sessions on all devices, delete all refresh tokens and trigger a Backchannel Logout for the whole
user for all configured clients.

```admonish note
The global, forced user logout does NOT affect logged in devices via the device flow, which are usually either IoT 
devices or maybe a CLI tool on a headless server. If you also want to remove access for all of these, do this 
additionally via the Admin UI -> Users -> select user -> Devices Tab.
```

## RP Initiated Logout

An RP Initiated Logout can be done by any client, that has a valid `id_token` for a given user. This logout provides
the best UX and makes it possible to redirect the user back to the downstream client afterward instead of Rauthy itself.

To make this possible, you first need to make sure, that you set a proper `Redirect URI` for your client in the Admin
UI -> Client Config. Only URLs added here upfront will be allowed for the automated redirect. This logout procedure also
does not ask for Logout validation and the whole process works without any user interaction.

If you added a redirect URI, you can do this logout in 2 ways:

- Use the users browser, for instance after clicking a *Logout* button in the downstream client app
- Do it in the background without the user knowing

Via the browser is pretty simple:

- Redirect the user to Rauthy's logout endpoint, which is `/auth/v1/oidc/logout` and provide a valid `id_token` via the
  `id_token_hint` URL parameter. If this contains a valid `id_token`, Rauthy will not ask for Logout confirmation and
  it will accept an optionally given, valid redirect URI.
- Add the `post_logout_redirect_uri` param and provide a URI, that has been allowed for this client upfront to avoid
  open redirects.
- You may also add a `state` parameter if you need to provide any information back to your client. This `state` will be
  appended to the `post_logout_redirect_uri`, if it exists.

Via the backend without user interaction, an RP can initiate a logout in the same way. This a lot simpler though. You
only would do a `POST /auth/v1/oidc/logout?id_token_hint=pasteAValidIdTokenHere`. It will work in the exact same way
with the exception, that you of course don't need any redirect or state, because your backend would do this request
directly.

In both situations, you at least need a valid `id_token` which has been returned in the past via the `/token` endpoint
after a successful login or token refresh.

The *RP Initiated Logout* will always trigger a Backchannel Logout on configured clients that are associated to the
`sid` from the `id_token`.

## Backchannel Logout - Client Side

A Backchannel Logout is sent from Rauthy to your downstream client application, if you configured it to do so. You can
configure it in the `Admin UI -> Clients -> select your client -> Backchannel Logout`.

Either your client application already
supports [OIDC Backchannel Logout](https://openid.net/specs/openid-connect-backchannel-1_0.html), or you might need to
implement it on your own. Accepting these logout requests is pretty straight forward, if you just take a look at the
RFC.

The `rauthy-client` will provide all the necessary tools for this in a future version as well.

## Backchannel Logout - Rauthy's Side

An [OIDC Backchannel Logout](https://openid.net/specs/openid-connect-backchannel-1_0.html) can be done as well against
Rauthy. You usually never implement it yourself though. Rauthy only accepts these backchannel logouts from configured
Upstream Auth Providers. These will provide a `logout_token` on the logout endpoint, which Rauthy will validate and on
success, forward the backchannel logout to all configured and associated clients.

You have quite a few config options regarding Backchannel Logout. All these values have reasonable defaults, but you
might take a look at them if you have issues like e.g. an upstream provider with self-signed certificates or inside
test environments.

```
#####################################
######## BACKCHANNEL LOGOUT #########
#####################################

# The maximum amount of retries made for a failed backchannel logout.
# Failed backchannel logouts will be retried every 60 - 90 seconds
# from all cluster nodes. The timeout between retries is randomized
# to avoid overloading clients. It will be executed on each cluster
# member to increase the chance of a successful logout in case of
# network segmentations.
# default: 100
#BACKCHANNEL_LOGOUT_RETRY_COUNT=100

# The lifetime / validity for Logout Tokens in seconds.
# These Logout Tokens are being generated during OIDC
# Backchannel Logout requests to configured clients.
# The token lifetime should be as short as possible and
# at most 120 seconds.
# default: 30
#LOGOUT_TOKEN_LIFETIME=30

# You can allow a clock skew during the validation of
# Logout Tokens, when Rauthy is being used as a client
# for an upstream auth provider that uses backchannel
# logout.
#
# The allowed skew will be in seconds and a value of
# e.g. 5 would mean, that 5 seconds are added to the
# `iat` and `exp` claim validations and expand the range.
#
# default: 5
#LOGOUT_TOKEN_ALLOW_CLOCK_SKEW=5

# The maximum allowed lifetime for Logout Tokens.
# This value is a security check for upstream auth
# providers. If Rauthy receives a Logout Token, it will
# check and validate, that the difference between `iat`
# and `exp` is not greater than LOGOUT_TOKEN_ALLOWED_LIFETIME.
# This means Rauthy will reject Logout Tokens from clients
# with a way too long validity and therefore poor
# implementations. The RFC states that tokens should
# be valid for at most 120 seconds.
# default: 120
#LOGOUT_TOKEN_ALLOWED_LIFETIME=120
```