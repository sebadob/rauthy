# Contributing to Rauthy

This document should help you to contribute to Rauthy and setting up your local dev environment.

## Guidelines for writing code

- **Performance and footprint matters**. If you write code, think about performance and especially memory footprint.
  For instance, avoid cloning values around when borrowing works fine too, or avoid allocating new memory in general,
  when you do not need to. The final performance optimizations have not been done yet, but everything we can do already
  is appreciated.
- **Use safe Rust only**. All crates forbid the usage of unsafe code globally. We do not care about the very last tiny
  bit of performance in special places when we would give up memory safety for that.
- **Stick to idiomatic Rust**. Stick to idiomatic Rust semantics and always use rust the `cargo fmt` for your code.
- **Use meaningful names**. For instance, a function that returns a `bool` should have a name like `is_admin` or
  `has_access`. All functions that do any database lookup currently use keywords like `find`, `create`, `update`, `save`
  and `delete` and have their own `impl` block for the structs to be "easily" interchangeable. Basically, you should
  have
  a general idea what a function does if you just read its name.
- Simple and readable code is way better than making everything generic without a good reason just because you can.
- Any function that returns a `Result<_>` should return Rauthy's custom `ErrorResponse` from `rauthy-common` as the
  error type. This decision was made instead of just simple `anyhow` error Strings for instance to actually make
  errors understandable by external code by using the `error` field and by humans by using the `message` field.

## Getting Started

### Prerequisites

To work with this project, you need to have the following tools available on your system:

- a BASH shell
- [just](https://github.com/casey/just)
- at least Rust v1.85
- `npm`
- `docker` / `podman`

### Building from source

If you just want to build from source yourself without doing any development, all necessary files have been checked into
version control to make this possible. In some environments, `just build-ui` does not work (like FreeBSD e.g.). To still
be able to build from source on those systems, the pre-built static HTML has been checked into version control.

```
cargo build --release
```

#### CAUTION

If you are building from source, either for production or without rebuilding the UI, you MUST always build
from a stable release tag and never from main. To not have messy PRs, the UI will only be rebuilt during releases or
in special occasions.  
You also MUST NEVER use a build from `main` in production. The `main` branch might contain unstable database migrations.
If these are applied to your production database and needed changes before the next release, it will produce a conflict
that is impossible to be solved automatically. You either need to roll back and apply an older DB backup, or undo the
unstable migrations manually. Just don't ever use `main` or any nightly images you might find in production.

### Initial Development Setup

A lot of work has been put into simplifying the development setup lately.

If you start inside a freshly cloned project, you only need to execute

```
just setup
```

which will run `npm install` for the frontend, build the UI static files for the templating engine, and then start
the Postgres and Mailcrab containers for development.

However, if you are resuming work, you don't need the additional `npm install` each time. You can also do these steps
separately:

- `just backend-start`
- `just build-ui`

`just backend-start` will start the Mailcrab and Postgres containers. You might see errors when you run this multiple
times, if the containers are already running, but these can be ignored. You don't strictly need the Postgres if you
are only developing with `hiqlite`. If you want to save the resources, instead of `just backend-start`, you could only
do `just mailcrab-start` to start the local email test server.

Even though pre-built static HTML files are checked into version control to make building from source possible on some
systems, where the UI cannot be built, you MUST always rebuild it, when you are developing locally. This is done with
`just build-ui`. The pre-built files checked into version control are usually only updated during releases to not have
messy PRs all the time.

Rauthy is using compile-time checked templating with [askama](https://crates.io/crates/askama). If you ever see any
templating related errors from `askama`, you most like just need to rebuild the UI with another `just build-ui`.

> If you want to use a Postgres other than the one created by `just postgres-start`, you need to provide the proper
> PG_* config values.

### Config

The default config file is the `rauthy.cfg`. This has reasonable defaults for local dev and should work out of the box
in most scenarios. If you want to modify any values to adjust to your setup, you should create a `./.env` file and
only configure here. You can even safely put secrets into it. It has been added to the `.gitignore` and cannot be
(automatically) checked into version control by accident.

Any values specified inside `.env` or via inline env vars will always have the highest priority and overwrite defaults
or values that are specified in `rauthy.cfg`.

Just as an example, let's say you want to develop against a remote Postgres database. To do so, create your `.env` file,
paste and adjust all the `PG_*` values from the `DATABASE` section in the `rauthy.cfg`.

> The default config is set up in a way that local development with a UI served on port `5173` will work properly. If
> you want to work with pre-built HTML from the backend only, you probably need to adjust:
> - `PUB_URL`
> - `RP_ORIGIN`
> - `DEV_MODE_PROVIDER_CALLBACK_URL`
> - `COOKIE_MODE`

#### CAUTION

If you use a remote host, a dev container, or basically anything else than `localhost` for development, you need to
enable TLS to get access to the Web Crypto API inside the browser.

##### TLS for the backend

Set in your `.env` file:

```
LISTEN_SCHEME=https
PUB_URL=example.com:8443
```

If you use a container for development, you need to expose port `8443` instead of the default `8080`.

##### TLS for the UI

If you need to work on the UI, you will later on start it via `npm`. On anything else than `localhost`, you need to
enable TLS for the Web Crypto API. To do this, out-comment the `https` section in `frontend/vite.config.js` and you
may need to update the `const backend` variable in the same file and adjust it to your setup.

### Get it up and running

Once prerequisites are met, the rest should be really simple:

```
just run
```

will start Rauthy with a Hiqlite backend serving the statically built HTML UI, all on port `8080` by default.
The default config expects you to work with the development UI though, so in another terminal, execute

```
just run ui
```

which will make the Svelte UI in dev mode available via port `5173` on your machine.

If you want to test against Postgres, instead of the above `just run`, simply execute

```
just run postgres
```

### Default Credentials

When `DEV_MODE=true` (which is the default for local dev), Rauthy will do some programmatic DB migrations. For instance
the JWKS will always be the same for faster startup, as well as the credentials. It will also allow some insecure
settings in different places to make local development easier.

The default admin and password with `DEV_MODE=true` are always:

```
admin@localhost
123SuperSafe
```

### Dev Backend

If you want to stop the dev containers, `just backend-stop` will take care of this. If you are working on DB migrations
and are still changing your new migration, you probably need to clean upt he DB while tuning, because of hash
mismatches. For Postgres, `just postgres-stop` and `just postgres-start`. For Hiqlite, `just delete-hiqlite`.

Your local email test server [mailcrab](https://github.com/tweedegolf/mailcrab) will be available on `localhost:1080`.
It will intercept and grab any outgoing emails in the default configuration. Even addresses that don't exist will be
grabbed successfully.

If you want to connect to the Postgres during development, the default user and password are `rauthy / 123SuperSafe`.
To connect to Hiqlite, you can ignore the Raft log and use any SQLite client to open `data/state_machine/db/hiqlite.db`.

### Rebuilding the UI

The UI during local dev is available on 2 different ports, which may seem confusing at first.

On port `5173`, after `just run-ui`, you will have the dev UI served by NodeJS dynamically. You basically always want
to use this during development. However, on Rauthys "real" port `8080`, you will have it available too. Here you will
get the pre-built static HTML and NOT anything that you might see or do in `frontend/`!

If you `just build-ui`, the current UI from `frontend/` will be built and made available as static HTML. After you
rebuilt the UI, you will have the static HTML files updates, and you see the latest state on port `8080` as well.

> Like mentioned above, if you, for whatever reason, ever end up in a state where `askama` in the backend complains
> about template errors and issues, you need to rebuild the UI. This can happen if you for instance `just build-ui`, but
> it fails because of some issue. At that point, the pre-built HTML will be cleaned up already, but the new files are
> not added of course because of the UI error.
> You need to fix the UI error first, then rebuild it, and then the `askama` template errors will go away.

## Known Limitations

Rauthys setup is a bit complex during local development. It builds the UI into static HTML which will be used by a
templating engine (`askama`) to inject `<template>` blocks for a better UX in production. However, since the DEV UI is
being served by NodeJS, it comes with a few workarounds and limitations during local development.

You mostly don't need to worry about anything of this, as the Svelte `<Template>` component that already exists handles
everything for you. However, if you for instance want to use the password reset form during local development, it will
not work as expected. It will require an already existing session (most probably for the default admin) and reset the
password for the current session instead of for the user from the magic link. This is due to not being able to properly
extract and inject some necessary values in that form during local dev. Usually, you would need to click a magic link to
get this information.
This is a compromise for improved DX, because the backend auto-creates a new magic link when working on that form during
local development and uses information from the session to do this. Because of this, you don't need to send out a new
one each time just because you want to test something. If you really want to use the password reset form properly, you
need to disable `DEV_MODE` and use the static HTML served by Rauthy instead of the NodeJS one, with all the additional
requirements mentioned above.

## Test Debugging

If you are adding a new test and want to debug it, it depends on the type of test how to do that best.

If it's a unit test, it's pretty straight forward, `just test your_unit_test`.  
If it's an integration test however, you may not always want to execute the whole range of tests with
`just test-hiqlite` or `just test-posgtres`, because it takes quite some time.

In such a case, you can `just test-backend`, which will start the test backend with `hiqlite`. If you then
`just test your_unit_test`, it will only run the single integration test without starting the backend automatically.

## Building a Container Image

If you want to build a container image for testing, you cannot `just build`, because you won't have access to the Rauthy
registry. If you have your own one, `just build your/registry/rauthy` should be working fine. The container image will
be built twice. One version for `x86_64` and one for `arm64`. If you don't have a registry available, you can also
disable auto-push with for instance `just build rauthy load` and load the image after building into
your local registry.

> The optimizations for a release build are pretty heavy and depending on your machine, it can take a very long time to
> do a release build, especially twice. Just as a reference, on my Ryzen 9950X it currently takes ~10 minutes.

> There is a `just build-builder` recipe, which you don't need to worry about. A new builder is only built when e.g. a
> new Rust version needs to be used. Just use the existing builder image from Github, which is the default anyway.

## Updating the Book

You are welcome to update the documentation in the Rauthy book. You will need `mdbook` and `mdbook-admonish`.
If you don't have them:

- `cargo install mdbook`
- `cargo install mdbook-admonish`

Once installed:

- `cd book`
- `mdbook serve --open`

You can then update the `*.md` files in `book/src`. It should be pretty self-explanatory.
`just build-docs` will rebuild them and make them available on Github after merging.

## Before Submitting a PR

This project does not have any actions and automatic pipelines set up yet, but there is a `just` recipe to make sure
everything is fine.

Before submitting any PR ready for merge, please execute

```
just pre-pr-checks
```

## FreeBSD

If you want to compile from source on FreeBSD, you may have a few limitations. Compiling the UI on FreeBSD seems to not
work. That is why the `static/` html is checked into version control so it does not prevent you from building from
source.

Since FreeBSD uses some cargo mechanism at build time differently, you may also run into linking issues like e.g. for
`openssl`, `sqlite` or `rocksdb`. In these situations, you should be able to fix it by installing the dependency on your
build host.

If you stumble about other limitations, please do not hesitate to submit a PR and update this section.

## Not working `just` recipes

There are a few recipes that will not work for you, since you need to have access to the Rauthy repo. These are for
instance `just build` (without the above-mentioned options), `just publish`, `just publish-latest` and `just release`.