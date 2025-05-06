# Logging and Auditing

Rauthy logs most things into the console, depending on the configuration of different log levels. In addition, more
important events can be sent to Matrix, Slack or via E-Mail. All of this is highly configurable and you should be able
to achieve whatever you need. All events are logged into the console as well with their configured level. This means,
if Rauthy fires an event of type `NewUser` with the level `info` and you have configured a log level of at least the
same, you will see the event in the console as well. So you could only use log aggregation and use existing tools
without configuring other targets and still catch everything.

## Logging

You can configure not only different levels for logging, but also different targets / situations.

### `LOG_LEVEL`

The `LOG_LEVEL` variable configures the default logging in most situations. This defines the logging for instance
for logging information from different function runs or things that have been triggered.

```
# This is the log level for stdout logs
# Accepts: error, info, debug, trace (default: info)
LOG_LEVEL=info
```

### `LOG_LEVEL_DATABASE`

The Hiqlite database logging is at the time of writing pretty verbose on purpose. The whole persistence layer with the
Raft cluster setup has been written from the ground up. The amount of logging will be reduced in later versions, when
the whole layer has been proven to be really solid, but for now you get more information just in case you need to debug
something.

You can reduce the default logging and for instance set it to `warn` or `error` only.

```
# The log level for the `Hiqlite` persistence layer.
# default: info
LOG_LEVEL_DATABASE=info
```

### `LOG_LEVEL_ACCESS`

For changing the logging behavior for access logs to the API endpoints, you will need to set the `LOG_LEVEL_ACCESS`.
If you have access logging configured at your firewall or reverse proxy, you can disable the `LOG_LEVEL_ACCESS` fully
to reduce duplicated log outputs.

```
# This is a special config which allows the configuration of
# customized access logs. These logs will be logged with each
# request in addition to the normal LOG_LEVEL logs.
# The following values are valid:
# - Debug
#   CAUTION: The Debug setting logs every information available
#   to the middleware which includes SENSITIVE HEADERS
#   DO NOT use the Debug level in a working production environment!
# - Verbose
#   Verbose logging without headers - generates huge outputs
# - Basic
#   Logs access to all endpoints apart from the Frontend ones
#   which all js, css, ...
# - Modifying
#   Logs only requests to modifying endpoints and skips all GET
# - Off
# default: Modifying
LOG_LEVEL_ACCESS=Basic
```

### `LOG_FMT`

Rauthy can output logs as JSON data with the following variable:

```
# You can change the log output format to JSON, if you set:
# `LOG_FMT=json`.
# Keep in mind, that some logs will include escaped values,
# for instance when `Text` already logs a JSON in debug level.
# Some other logs like an Event for instance will be formatted 
# as Text anyway. If you need to auto-parse events, please consider 
# using an API token and listen ot them actively.
# default: text
#LOG_FMT=text
```

## Events

Events are used for auditing and never miss anything. If something important happens, you usually need to inspect logs
to catch it, but why should you, if you did not notice any problems? This is where Rauthy Events are helping you out.
You need to set up basic configuration for Event targets and then you could customize the different levels.

### Basic Setup

Rauthy has the following Event targets built-in:

- E-Mail
- Matrix
- Slack

You can see the full set of config option in the `EVENTS / AUDIT` in the [Reference Config](config.md).

#### E-Mail

To be able to receive Events via E-Mail, you need to have set up an SMTP server and have a working connection. With
a working SMTP, you only need to set `EVENT_EMAIL`, that's it.

```
# The E-Mail address event notifications should be sent to.
EVENT_EMAIL=admin@localhost
```

#### Matrix

Matrix is often deployed with home servers that may not even have real TLS certificates or if just running behind closed
doors, may use self-signed certificates. To make it work in all of these situations, you can configure quite a lot for
the connection to Matrix. In the end, you will only need to have some credentials and a room ID, so Rauthy knows where
it should post the events.

```
# Matrix variables for event notifications.
# `EVENT_MATRIX_USER_ID` and `EVENT_MATRIX_ROOM_ID` are mandatory.
# Depending on your Matrix setup, additionally one of
# `EVENT_MATRIX_ACCESS_TOKEN` or `EVENT_MATRIX_USER_PASSWORD` is needed.
# If you log in to Matrix with User + Password, you may use `EVENT_MATRIX_USER_PASSWORD`.
# If you log in via OIDC SSO (or just want to use a session token you can revoke),
# you should provide `EVENT_MATRIX_ACCESS_TOKEN`.
# If both are given, the `EVENT_MATRIX_ACCESS_TOKEN` will be preferred.
#
# If left empty, no messages will not be sent to Matrix.
# Format: `@<user_id>:<server address>`
#EVENT_MATRIX_USER_ID=
# Format: `!<random string>:<server address>`
#EVENT_MATRIX_ROOM_ID=
#EVENT_MATRIX_ACCESS_TOKEN=
#EVENT_MATRIX_USER_PASSWORD=
# URL of your Matrix server.
# default: https://matrix.org
#EVENT_MATRIX_SERVER_URL=https://matrix.org
# Optional path to a PEM Root CA certificate file for the Matrix client.
#EVENT_MATRIX_ROOT_CA_PATH=path/to/my/root_ca_cert.pem
# May be set to disable the TLS validation for the Matrix client.
# default: false
#EVENT_MATRIX_DANGER_DISABLE_TLS_VALIDATION=false
# The default behavior is, that Rauthy will panic at startup if it cannot connect
# to a configured Matrix server. The reason is that event notifications cannot be
# dropped silently.
# However, if you use a self-hosted Matrix server which uses Rauthy as its OIDC
# provider and both instances went offline, you will have a chicken and egg problem:
# - Rauthy cannot connect to Matrix and will panic
# - Your Matrix server cannot connect to Rauthy and will panic
# To solve this issue, you can temporarily set this value to 'true' and revert
# back, after the system is online again.
# default: false
#EVENT_MATRIX_ERROR_NO_PANIC=false
```

```admonish hint
I suggest that you create a separate room for these events. If you experience issues with Matrix encryption, you should
maybe disbale the encryption for the Events room. I came across some weird errors from Matrix in the past when I was
using a session token which has been created on another machine.
```

#### Slack

To receive messages via Slack, you need to create a legacy webhook inside your Slack account. This is then the only
config variable you need to set:

```
# The Webhook for Slack Notifications.
# If left empty, no messages will be sent to Slack.
#EVENT_SLACK_WEBHOOK=
```

#### Custom Target

If you need your events to be sent somewhere custom, you can always create an API key with `read` access for Events.
Then write a small app that simply listens to the events stream, which can process or just forwards the events to where
ever you need them.

1. Log in to the Admin UI and create an API key with `read` access for events.

![api_key_events](img/api_key_1.png)

2. Save the key, expand the new entry, navigate to `Secret` and `Generate New`

![api_key_events](img/api_key_2.png)

3. Copy the new Secret from the hidden input that will appear.
   You will also see 2 prepared `curl` statements to test your new API key.
   From the test, you should get an output similar to this:

```json
{
  "name": "events",
  "created": 1720428190,
  "expires": null,
  "access": [
    {
      "group": "Events",
      "access_rights": [
        "read"
      ]
    }
  ]
}
```

4. Listen to the events stream.
   You need to add the API key in the `Authorization` header with the prefix `API-Key`, for instance like

```
Authorization: API-Key events$SUcpBxcPmfwH9z1Kb4ExOUYDSXpxOj9mFLadjuQ1049XaWzdWB328aa97k2nj21E
```

You can either periodically fetch via the `/auth/v1/events` endpoint (see Swagger documentation in the Admin UI), or
by listening to the `/auth/v1/events/stream`, which will be a Server Sent Events stream.

The events will be sent in JSON format and have the following content:

```rust
struct Event {
    pub id: String,
    pub timestamp: i64,
    pub level: EventLevel,
    pub typ: EventType,
    pub ip: Option<String>,
    pub data: Option<i64>,
    pub text: Option<String>,
}

// the `EventLevel`s will be converted to lower case
enum EventLevel {
    Info,
    Notice,
    Warning,
    Critical,
}

enum EventType {
    InvalidLogins,
    IpBlacklisted,
    IpBlacklistRemoved,
    JwksRotated,
    NewUserRegistered,
    NewRauthyAdmin,
    NewRauthyVersion,
    PossibleBruteForce,
    RauthyStarted,
    RauthyHealthy,
    RauthyUnhealthy,
    SecretsMigrated,
    UserEmailChange,
    UserPasswordReset,
    Test,
}
```

```admonish note
Keep in mind, that depending on proxies you might have in between, they might kill your connection to the events stream
after some timeout without messages. Your app should work around this and either re-connect on connection loss, or you
could fix it with a different proxy config or keep alive messages.
```

### Testing

You can test your Event handler / pipeline setup when you log in to the Admin UI and simply press the `Test` button
in the upper right corner in the Events sidebar. This will send a Test event to all configured targets and listeners
independent of any configured event level restrictions. Test events will always be sent as long as everything is working
properly.

### `EVENT_NOTIFY_LEVEL`

You can set different levels for each target. By default, Only events with `warning` or higher are sent via E-Mail
while Matrix / Slack would receive Events with `notice` or higher. If you want a different behavior, you can get this:

```
# The notification level for events. Works the same way as a logging level. 
# For instance: 'notice' means send out a notifications for all events with 
# the notice level or higher.
# Possible values:
# - info
# - notice
# - warning
# - critical
#
# default: 'warning'
EVENT_NOTIFY_LEVEL_EMAIL=warning
# default: 'notice'
EVENT_NOTIFY_LEVEL_MATRIX=notice
# default: 'notice'
EVENT_NOTIFY_LEVEL_SLACK=notice
```

### Event Persistence

Rauthy's Admin UI has a component for inspecting Events from the past for analytical purposes. By default, events with
the level `info` or higher are persisted for `31` days. After this period, they will be removed from the database to
keep it clean.

You can configure both the level which should be persisted, for instance set "only persist Events with level warning
or higher" and the days how long they should be kept.

```
# Define the level from which on events should be persisted inside the 
# database. All events with a lower level will be lost, if there is no 
# active event subscriber.
# Possible values:
# - info
# - notice
# - warning
# - critical
# default: 'info'
#EVENT_PERSIST_LEVEL=info

# Define the number of days when events should be cleaned up from the database.
# default: 31
#EVENT_CLEANUP_DAYS=31
```

### `EVENT_LEVEL_` Values

There are a lot of values starting with `EVENT_LEVEL_`. These can be used to configure the level for different kinds
of event being fired by Rauthy.

For instance, let's say you only want to receive events with a level of `warning` or higher, but you also want to
receive a notification when there are more than 7 failed logins from an IP. By default, 7 failed logins would trigger an
event with the level of `notice`. You can then set

```
EVENT_LEVEL_FAILED_LOGINS_7=warning
```

to also receive these while still only receiving `warning` events.

The full list of these configurable levels is the following:

```
# The level for the generated Event after a new user has been registered.
# default: info
EVENT_LEVEL_NEW_USER=info
# The level for the generated Event after a user has changed his E-Mail
# default: notice
EVENT_LEVEL_USER_EMAIL_CHANGE=notice
# The level for the generated Event after a user has reset its password
# default: notice
EVENT_LEVEL_USER_PASSWORD_RESET=notice
# The level for the generated Event after a user has been given the 
# 'rauthy_admin' role
# default: notice
EVENT_LEVEL_RAUTHY_ADMIN=notice
# The level for the generated Event after a new App version has been found
# default: notice
EVENT_LEVEL_RAUTHY_VERSION=notice
# The level for the generated Event after the JWKS has been rotated
# default: notice
EVENT_LEVEL_JWKS_ROTATE=notice
# The level for the generated Event after DB secrets have been migrated 
# to a new key
# default: notice
EVENT_LEVEL_SECRETS_MIGRATED=notice
# The level for the generated Event after a Rauthy instance has been 
# started
# default: info
EVENT_LEVEL_RAUTHY_START=info
# The level for the generated Event after a Rauthy entered a healthy 
# state (again)
# default: notice
EVENT_LEVEL_RAUTHY_HEALTHY=notice
# The level for the generated Event after a Rauthy entered an unhealthy 
#state
# default: critical
EVENT_LEVEL_RAUTHY_UNHEALTHY=critical
# The level for the generated Event after an IP has been blacklisted
# default: warning
EVENT_LEVEL_IP_BLACKLISTED=warning
# The level for the generated Event after certain amounts of false 
# logins from an IP
# default: critical
EVENT_LEVEL_FAILED_LOGINS_25=critical
# default: critical
EVENT_LEVEL_FAILED_LOGINS_20=critical
# default: warning
EVENT_LEVEL_FAILED_LOGINS_15=warning
# default: warning
EVENT_LEVEL_FAILED_LOGINS_10=warning
# default: notice
EVENT_LEVEL_FAILED_LOGINS_7=notice
# default: info
EVENT_LEVEL_FAILED_LOGIN=info
```