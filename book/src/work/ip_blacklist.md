# IP Blacklisting

Rauthy can blacklist certain IP that should be prevented from even trying to access it.  
Each blacklisting will always have an expiry. This is important because most client IPs will be
ephemeral.

## Automatic Blacklisting

Blacklisting with different timeouts will happen automatically, for instance, when thresholds for
invalid logins have been reached or it is obvious that someone is scanning Rauthys API. The scan
detection is very basic, but will catch suspicious requests and bots automatically.

With failed logins, not only the timeout will be increased to send an answer back to the client to
prevent brute-force attacks. Rauthy also has the following default thresholds for blacklisting IPs:

| failed logins | blacklist  |
|---------------|------------|
| 7             | 1 minute   |
| 10            | 10 minutes |
| 15            | 15 minutes |
| 20            | 1 hour     |
| 25+           | 1 day      |

In addition to blacklisting, the timeouts for failed logins in between these steps will be longer
the higher the failed attempts counter is.

## Suspicious Request Blacklisting

As mentioned already, Rauthy has basic capabilities to detect API scanners and bots. These are
called *suspicious requests* internally.

By default, Rauthy will immediately block such IPs if it detects them. However, you may not want
this for whatever reason. You can modify this behavior with the following config setting:

```toml
[suspicious_requests]
# The "catch all" route handler on `/` will compare the request
# path against a hardcoded list of common scan targets from bots
# and attackers. If the path matches any of these targets, the
# IP will be blacklisted preemptively for the set time in minutes.
# You can disable it by setting it to `0`.
#
# default: 1440
# overwritten by: SUSPICIOUS_REQUESTS_BLACKLIST
blacklist = 1440

# Will emit a log with level of warning if a request to `/` has
# been made that has not been caught by any of the usual routes
# and handlers. Apart from a request to just `/` which will end
# in a redirect to `/auth/v1`, all additional path's will be
# logged. This can help to improve the internal suspicious
# blocklist in the future.
#
# default: false
# overwritten by: SUSPICIOUS_REQUESTS_LOG
log = false
```

At the time of writing, events for suspicious requests do not exist yet. This might change in the
future.

## Manual Blacklisting

You can also manually blacklist an IP, either via the Admin UI or with an [API Key](api_keys.md)
with the correct access rights. Just navigate to `Blacklist` in the Admin UI and click
`Blacklist IP`.

## Persistence

The blacklist currently is in-memory only. This means you loose all blacklisted IPs when Rauthy
restarts, if you opt-in to `cluster.cache_storage_disk = false`.

The reason behind this is that blacklisting usually happens in scenarios under attack, when you want
to do as little work as possible, for instance to not end up with a DoS. The blacklisting middleware
is also the very first one in the API stack, even before access logging, to make sure Rauthy has the
least amount of work blocking blacklisted IP's.

## Expiry

After a blacklisting expires, the entry will be fully removed from Rauthy and you will not see it
anymore.

## Blacklist Events

You may or may not be notified about different blacklisting events. All auto-blacklisting will
trigger an event. The levels change depending on the failed login counter. You can adopt the levels
to your likings by setting:

```toml
[events]
# The level for the generated Event after certain
# amounts of false logins from an IP
#
# default: critical
# overwritten by: EVENT_LEVEL_FAILED_LOGINS_25
level_failed_logins_25 = 'critical'
# default: critical
# overwritten by: EVENT_LEVEL_FAILED_LOGINS_20
level_failed_logins_20 = 'critical'
# default: warning
# overwritten by: EVENT_LEVEL_FAILED_LOGINS_15
level_failed_logins_15 = 'warning'
# default: warning
# overwritten by: EVENT_LEVEL_FAILED_LOGINS_10
level_failed_logins_10 = 'warning'
# default: notice
# overwritten by: EVENT_LEVEL_FAILED_LOGINS_7
level_failed_logins_7 = 'notice'
# default: info
# overwritten by: EVENT_LEVEL_FAILED_LOGIN
level_failed_login = 'info'
```

The event level for all other blacklisting can be set with this value:

```toml
[events]
# The level for the generated Event after an IP has
# been blacklisted
#
# default: warning+
# overwritten by: EVENT_LEVEL_IP_BLACKLISTED
level_ip_blacklisted = 'warning'
```
