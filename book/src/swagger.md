# Swagger UI

Rauthy comes with its own OpenAPI documentation and Swagger UI. There are some things you should know about it.

It is publicly exposed when you start Rauthy for testing locally, but by default it is not. Rauthy is open source and
anyone can simply start it and take a look at the API docs, but exposing the API docs in production to anyone is not a
good practice, especially because it might increase the attack surface.

By default, the SwaggerUI is disabled, because it requires ~13MB of memory at runtime. You can enable it via
configuration:

```toml
[server]
# Can be set to `true` to enable the Swagger UI.
# This will consume ~13mb of additional memory.
#
# default: false
# overwritten by: SWAGGER_UI_ENABLE
swagger_ui_enable = true
```

```admonish info 
The UI is not meant to be used for testing out endpoints. Rauthy has way to many options and details regarding to the
authentication and it's not that simple, that the default Swagger UI can handle it, at least to my knowledge.

If you want to test certain endpoints, I suggest you <b>create an API Key</b> for that case, which you can use with any tool
you like.
```

```admonish hint
If you just want to take a look at the SwaggerUI when you have no running Rauthy instance yet, I have a test / dev 
instance running which exposes the API docs: 
<a href="https://iam.sebadob.dev/auth/v1/docs/" target="_blank">Swagger UI</a>

Please note that this is a testing instance and it might not be online all the time or be running the latest nightly
build, but everything should be fine most of the time.
```

## External Access

If you open the Rauthy Admin UI and navigate to `Docs`, you will find a link to the Swagger UI that each Rauthy instance
hosts. By default, in production this will not be exposed and the link in the UI will not be working as mentioned in the
description in the same place.

If you don't care about exposing it publicly, or you need non-`rauthy_admin`s to have access, you can do so with the
following and make the link work:

```toml
[server]
# Can be set to `true` to make the Swagger UI publicly
# available. By default, you can only access it with a
# valid `rauthy_admin` session.
#
# default: false
# overwritten by: SWAGGER_UI_PUBLIC
swagger_ui_public = false
```

## Internal Access

By defautl, the SwaggerUI is only visible to `rauthy_admin`s with a valid session. You can access it via the Admin UI,
or via `/auth/v1/docs/` on your Rauthy instance.

