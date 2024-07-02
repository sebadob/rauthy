# Sessions

When you log in to anything, be it your account dashboard, the admin UI, or a downstream application, you will get a
Rauthy session. This is independent of any client / application login. These sessions are used to authorize against
Rauthy only for things like account management. Depending on your configuration, users may be logged in to a downstream
client immediately, if they already have a still valid session.

You can configure quite a lot like session timeouts and so on, but the default are safe. However, there is one
**really important thing**:

You need to make sure, that Rauthy can extract the connecting clients IP properly. This is
very important for failed login counters, login delays, blacklisting, and so on.

If your instance is exposed directly, in most situations the IP extractions works just fine. This may change though
when running behind a reverse proxy or a CDN.

```admonish hint
To check which IP Rauthy will extract for your requests, you don't to search through logs. You can use the whoami 
endpoint. This is unauthenticated and will just return your current IP from the request headers. If the returned
IP is correct, your setup is fine.

The endpoint is reachable via: <code>/auth/v1/whoami</code>
```

## Running behind a reverse proxy

If you are running behind a reverse proxy, you need to set at least 2 config variable properly.  
First, you need to set

```
PROXY_MODE=true
```

Secondly, you need to tell Rauthy which proxy source IP's it can trust. This is important, because when behind a reverse
proxy, Rauthy will only see the IP of the proxy itself by default, which would be the same for each client connecting
though it. However, a reverse proxy adds headers which contain the clients real IP, like e.g. the `X-FORWARED-FOR`
header and maybe others (depending on the proxy).

These headers can be spoofed from an attacker, if the source IP is not validated. This is what Rauthy needs you to set
the trusted proxies config for:

```
# A `\n` separated list of trusted proxy CIDRs.
# When `PROXY_MODE=true` or `PEER_IP_HEADER_NAME` is set,
# these are mandatory to be able to extract the real client
# IP properly and safely to prevent IP header spoofing.
# All requests with a different source will be blocked.
TRUSTED_PROXIES="
192.168.0.1/32
192.168.100.0/24
"
```

The more you can narrow down the CIDR for your reverse proxy, the better. For instance if you know your proxy is your
firewall at the same time, which always will have the IP `192.168.0.1`, then add the `/32` subnet to it. If you are
running in a more dynamic environment like Docker or Kubernetes, where your proxy could get an IP dynamically from an
internal pool, you need to add all the possible IPs as trustworthy.

```admonish caution
When Rauthy is running in proxy mode, it will block every incoming request that does not match the 
<code>TRUSTED_PROXIES</code> IP pool. This means if you have internal tooling set up like health checks, monitoring or
metrics, which do not connect via the proxy, you need to add these source IPs to the <code>TRUSTED_PROXIES</code> list.
```

### Running behind a CDN

If you are running behind a CDN which proxies your requests like for instance cloudflare, you have a reverse proxy
setup again, just so that cloudflare is (another) reverse proxy for you. This means you need to set up the above
configuration at least.

In addition, you would maybe end up seeing the CDN proxy IP when you do a `GET /auth/v1/whoami`. If this is the case,
There is an additional variable you can set:

```
# Can be set to extract the remote client peer IP from a custom header name
# instead of the default mechanisms. This is needed when you are running
# behind a proxy which does not set the `X-REAL-IP` or `X-FORWARDED-FOR` headers
# correctly, or for instance when you proxy your requests through a CDN like
# Cloudflare, which adds custom headers in this case.
# For instance, if your requests are proxied through cloudflare, your would
# set `CF-Connecting-IP`.
PEER_IP_HEADER_NAME="CF-Connecting-IP"
```

The CDN usually adds some other headers than the default `X-FORWARED-FOR` headers, like in this example
`CF-Connecting-IP` to the request. If this is the case, you can tell Rauthy to always check for this header first and
only use the other methods as fallback, if this does not exist.

### Session peer IP binding

You most probably do not need to care about this configuration, but depending on your application you may want to
disable it.

Whenever you get a session from Rauthy and you authenticate succesfully, your current IP will be extracted and
persisted. By default, Rauthy will check your origin IP and compare it to the one you had when creating the session
with each single request. If your IP does not match the original one, the session will be ignored and the request will
be treated as being unauthenticated.

This prevents scenarios where an attacker would be able to steal session data from your machine, copy the information
and use it on their own. This means even if you would send your session cookie and CSRF token to someone, they would not
be able to use it, as long as the requests are not coming from the exact same source IP.

```admonish note
Rauthy has lots of mechanisms in place to prevent things like cookie stealing, session takeover, and so on , but it 
can't do anything about it, when the client's OS itself is infected. All these mechanisms add up to the defense in 
depth, but at the end of the day, when the clients machine itself is infected, there is not much any application can do 
about it. There just is no silver bullet.
```

This defense is a really nice thing, but it may annoy your users, depending on where your deployed Rauthy, because
this also means that each time when a client's IP changes, like for instance when you are in a mobile network or in
a WIFI and often reconnect, your session will not be accepted. With as passkey added to your account, the login will
take only seconds and another touch on the device, but you may still want to disable it. In this case, here is the
configuration:

```
# If set to 'true', this will validate the remote peer IP address with
# each request and compare it with the IP which was used during the initial
# session creation / login. If the IP is different, the session will be
# rejected. This is a security hardening and prevents stolen access credentials,
# for instance if an attacker might have copied the encrypted session cookie
# and the XSRF token from the local storage from a user. However, this event
# is really unlikely, since it may only happen if an attacker has direct access
# to the machine itself.
#
# If your users are using mobile networks and get new IP addresses all the time,
# this means they have to do a new login each time. This is no big deal at all with 
# Webauthn / FIDO keys anyway and should not be a reason to deactivate this feature.
#
# Caution: If you are running behind a reverse proxy which does not provide the 
# X-FORWARDED-FOR header correctly, or you have the PROXY_MODE in this config
# disabled, this feature will not work. You can validate the IPs for each session
# in the Admin UI. If these are correct, your setup is okay.
#
# (default: true)
#SESSION_VALIDATE_IP=true
```

### Lifetimes and Timeouts

The default session lifetimes and timeouts are pretty secure, but you may find them to be too strict. You can adjust
them with the following config variables:

```
# Session lifetime in seconds - the session can not be
# extended beyond this time and a new login will be forced.
# This is the session for the authorization code flow. 
# (default: 14400)
SESSION_LIFETIME=14400

# If 'true', a 2FA / MFA check will be done with each automatic
# token generation, even with an active session, which kind of
# makes the session useless with Webauthn enabled, but provides
# maximum amount of security.
# If 'false', the user will not get an MFA prompt with an active
# session at the authorization endpoint.
# (default: false)
SESSION_RENEW_MFA=false

# Session timeout in seconds
# When a new token / login is requested before this timeout hits
# the limit, the user will be authenticated without prompting for
# the credentials again.
# This is the value which can extend the session, until it hits
# its maximum lifetime set with SESSION_LIFETIME.
SESSION_TIMEOUT=5400
```