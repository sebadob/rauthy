# Swagger UI

Rauthy comes with its own OpenAPI documentation and Swagger UI. There are some things you should know about it.

It is publicly exposed when you start Rauthy for testing locally, but by default it is not. The reason is that it is
not easily possible with the built-in options to only show it to authenticated Rauthy admins. Rauthy is open source and
anyone can simply start it and take a look at the API docs, but exposing the API docs in production to anyone is not a
good practice, especially because it might increase the attack surface.

For this reason, you have multiple ways of accessing the Swagger UI.

```admonish info 
The UI is not meant to be used for testing out endpoints. Rauthy has way to many options and details regarding to the
authentication and it's not that simple, that the default Swagger UI can handle it, at least to my knowledge.

If you want to test certain endpoints, I suggest you <b>create an API Key</b> for that case, which you can use with any tool
you like.
```

## External Access

If you open the Rauthy Admin UI and navigate to `Docs`, you will find a link to the Swagger UI that each Rauthy instance
hosts. By default, in production this will not be exposed and the link in the UI will not be working as mentioned in the
description in the same place.

If you don't care about exposing it publicly, you can do so with the following and make the link work:

```
# If the Swagger UI should be served externally as well. This makes the link in the Admin UI work.
#
# CAUTION: The Swagger UI is open and does not require any login to be seen!
# Rauthy is open source, which means anyone could just download it and see on their own,
# but it may be a security concern to just expose less information.
# (default: false)
SWAGGER_UI_EXTERNAL=true
```

## Internal Access

In addition to the default HTTP server, Rauthy will start a second one on a different port for metrics (if not disabled
on purpose). The Swagger UI will be exposed on the second port by default. You can expose the default port while making
the second one available only in an internal network for instance.

If you are in a network where you have access to the second internal port, you can reach the Swagger UI via
**/docs/v1/swagger-ui/** on port `9090`.

You can change the internal exposing or port as well. The metrics and internal Swagger UI share the same config:

```
# To enable or disable the additional HTTP server to expose the /metrics endpoint
# default: true
#METRICS_ENABLE=true

# The IP address to listen on for the /metrics endpoint.
# You do not want to expose your metrics on a publicly reachable endpoint!
# default: 0.0.0.0
#METRICS_ADDR=0.0.0.0

# The post to listen on for the /metrics endpoint.
# You do not want to expose your metrics on a publicly reachable endpoint!
# default: 9090
#METRICS_PORT=9090

# If the Swagger UI should be served together with the /metrics route on the internal server.
# It it then reachable via:
# http://METRICS_ADDR:METRICS_PORT/docs/v1/swagger-ui/
# (default: true)
#SWAGGER_UI_INTERNAL=true
```

