# Forward Authentication

If you want to secure applications running behind a reverse proxy that do not have authn / authz on their own in terms
of being able to use OIDC flows or similar with them, you can use something like the [Traefik](https://traefik.io/)
middleware [ForwardAuth](https://doc.traefik.io/traefik/middlewares/http/forwardauth/). Other proxies support this as
well.

Incoming requests can be intercepted and forwarded to Rauthy first. It will check for a valid `Bearer` token in the
`Authorization` header. If it is valid, Rauthy will return an `HTTP 200 Ok` and will append additional headers with
information about the user to the request. These headers could easily be read by a downstream application.

The forward auth headers are disabled by default, because they can leak information, if you do not set up your internal
environment carefully. You can enable the feature with

```toml
[auth_headers]
# You can enable authn/authz headers which would be added to the
# response of the `/auth/v1/oidc/forward_auth` endpoint. When set to
# `true`, the headers below will be added to authenticated requests.
# These could be used on legacy downstream applications, that don't
# support OIDC on their own.
#
# However, be careful when using this, since this kind of authn/authz
# has a lot of pitfalls out of the scope of Rauthy.
#
# default: false
# overwritten by: AUTH_HEADERS_ENABLE
enable = false
```

You can also change the header names containing the information, if you need to support some older application that
needs special naming for them:

```toml
[auth_headers]
# Configure the header names being used for the different values. You
# can change them to your needs, if you cannot easily change your
# downstream apps.
#
# default: x-forwarded-user
# overwritten by: AUTH_HEADER_USER
user = 'x-forwarded-user'
# default: x-forwarded-user-roles
# overwritten by: AUTH_HEADER_ROLES
roles = 'x-forwarded-user-roles'
# default: x-forwarded-user-groups
# overwritten by: AUTH_HEADER_GROUPS
groups = 'x-forwarded-user-groups'
# default: x-forwarded-user-email
# overwritten by: AUTH_HEADER_EMAIL
email = 'x-forwarded-user-email'
# default: x-forwarded-user-email-verified
# overwritten by: AUTH_HEADER_EMAIL_VERIFIED
email_verified = 'x-forwarded-user-email-verified'
# default: x-forwarded-user-family-name
# overwritten by: AUTH_HEADER_FAMILY_NAME
family_name = 'x-forwarded-user-family-name'
# default: x-forwarded-user-given-name
# overwritten by: AUTH_HEADER_GIVEN_NAME
given_name = 'x-forwarded-user-given-name'
# default: x-forwarded-user-mfa
# overwritten by: AUTH_HEADER_MFA
mfa = 'x-forwarded-user-mfa'
```

```admonish caution
This feature makes it really easy for any application behind your reverse proxy to serve protected resources, but you
really only use it, if you cannot use a proper OIDC client or something like that. 

Auth forward has many pitfalls and you need to be careful with your whole setup when using it. A direct token validation
inside your downstream application should always be preferred.
```
