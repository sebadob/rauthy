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
- Any function that returns a `Result<_>` should return Rauthys custom `ErrorResponse` from `rauthy-common` as the
  error type. This decision was made instead of just simple `anyhow` error Strings for instance to actually make
  errors understandable by external code by using the `error` field and by humans by using the `message` field.

### Code Style

For a uniform code style, this project makes use of `cargo fmt` for all Rust code, and `prettier` for all other frontend
related files. There is also a `.editorconfig`, if your editor can make use of it. If your IDE auto-formats code, make
sure it uses `cargo fmt` for Rust code, the `frontend/.prettierrc` for the UI, and optionally the `.editorconfig`,
even though the other 2 should have you covered.

You can also use `just fmt` manually to make sure all code is formatted properly. Before submitting a PR ready for
review, all code must be formatted.

## Getting Started

### Prerequisites

To work with this project, you need to have the following tools available on your system:

- a BASH shell
- [just](https://github.com/casey/just)
- at least Rust v1.88
- `npm`
- `docker` / `podman`

> `npm` is not strictly necessary, but recommended to have. If you just want to build the static UI files, you can also
> do it inside a container. However, if you want to do any form of development on the UI, you need `npm` to run it in
> dev mode locally.
> Another solution, if you cannot build the UI, is to `just extract-ui-archive`. A pre-built version of the static UI
> files is checked into version control with each new release, and this recipe will extract the files into the correct
> locations.

### Building from source

If you only care about building from source and don't need any testing or development, you need to do 3 things:

- `just setup`
- `just build-ui`
- `cargo build --release`

This will build a release binary for your current platform in the simplest form. You can also build the multi arch
container image. However, you cannot just use `just build` (like explained further down). You then need to do something
like

```
just build my/registry/rauthy
```

This will build the container AND will try to `push` the image as well. If you only want a local image without pushing
it, you can do

```
just build my/registry/rauthy nopush
```

which will then build the multi arch image and load it into your local container registry afterward.

#### CAUTION

If you are building from source for production, you MUST always build from a stable release tag and never from `main`.

You MUST NEVER use a build from `main` or any nightly image (tag typically with `*-some_date` appended) in production.
The `main` branch might contain unstable database migrations. If these are applied to your production database and
needed changes before the next release, it will produce a conflict that is impossible to be solved automatically. You
either need to roll back and apply an older DB backup, or undo the unstable migrations manually. Just don't ever use
`main` or any nightly images you might find in production.

### Initial Development Setup

If you start inside a freshly cloned project, you first need to execute

```
just setup
```

which will run `npm install` for the frontend, to make all the `node_modules` available for either building the UI or
running it in development mode. You only need to run this once or after a version bump, which will typically bump
all dependencies and therefore needs another `npm install`.

After the `setup`, you need:

- `just dev-env-start`
- `just build-ui`

`just dev-env-start` will start the Mailcrab and Postgres containers. You might see errors when you run this multiple
times, if the containers are already running, but these can be ignored. You don't strictly need the Postgres if you
are only developing with `hiqlite`. If you want to save the resources, instead of `just dev-env-start`, you could only
do `just mailcrab-start` to start the local email test server.

Rauthy is using compile-time checked templating with [askama](https://crates.io/crates/askama). If you ever see any
templating related errors from `askama`, you most like just need to rebuild the UI with another `just build-ui`.

> If you want to use a Postgres other than the one created by `just postgres-start`, you need to provide the proper
> PG_* config values.

### Config

The default config file is the `config.toml`. This has reasonable defaults for local dev and should work out of the box
in most scenarios. If you want to modify any values to adjust to your setup, you should create a `./.env` file and
only configure here. You can even safely put secrets into it. It has been added to the `.gitignore` and cannot be
(automatically) checked into version control by accident.

Any values specified inside `.env` or via inline env vars will always have the highest priority and overwrite defaults
or values that are specified in `config.toml`.

Just as an example, let's say you want to develop against a remote Postgres database. To do so, create your `.env` file,
paste and adjust all the `PG_*` values from the `[database]` section in the `config.toml`. Most values can be
overwritten by env vars, and the corresponding var name is always mentioned.

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

will start Rauthy with a Hiqlite backend serving the statically built HTML UI, all on port `8080` by default. The
default config expects you to work with the development UI though, so in another terminal, execute

```
just run ui
```

which will make the Svelte UI in dev mode available via port `5173` on your machine.

If you want to test against Postgres, instead of the above `just run`, simply execute

```
just run postgres
```

> If you don't want to keep running your Postgres and Mailcrab containers up and running, you can instead of `just run`
> also use `just watch` (or `just watch posgres`), which will start dev containers as needed and watch for changes in
> the Rust code to automatically recompile and restart rauthy. This command requires
> the [watchexec](https://watchexec.github.io/) tool.

### Default Credentials

When `DEV_MODE=true` (which is the default for local dev), Rauthy will do some programmatic DB migrations. For instance
the JWKS will always be the same for faster startup, as well as the credentials. It will also allow some insecure
settings in different places to make local development easier.

The default admin and password with `DEV_MODE=true` are always:

```
admin@localhost
123SuperSafe
```

### Dev Env Containers

If you want to stop the dev containers, `just dev-env-stop` will take care of this, to remove the containers use
`just dev-env-rm`. If you are working on DB migrations and are still changing your new migration, you probably need to
clean upt he DB while tuning, because of hash mismatches. For Postgres, `just postgres-rm` and `just postgres-start`.
For Hiqlite, `just delete-hiqlite`.

Your local email test server [mailcrab](https://github.com/tweedegolf/mailcrab) will be available on `localhost:1080`.
It will intercept and grab any outgoing emails in the default configuration. Even addresses that don't exist will be
grabbed successfully.

If you want to connect to the Postgres during development, the default user and password are `rauthy : 123SuperSafe`.
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

## Architecture and Internals

### DEV vs PROD mode

Rauthys architecture in production is really simple, but in local development, it's a bit more involved.

During **local development**, you have the Rust backend, which basically is "Rauthy". It serves the whole API. In
addition, you typically want to run the UI in dev mode as well, which is done via `just run ui`, like explained above.
This will run NodeJS and serve the Svelte UI on port `5173`.

In **production**, the whole UI will be compiled into static HTML and served by the Rust backend. This means you only
have this single server running doing all the work.

For the UI, `<template>` blocks are used to do a little bit of SSR. Some values will be injected into the HTML (in prod)
before it's sent to the user. To access these `<template>` blocks, there is a dedicated Svelte component in
`frontend/src/lib/Template.svelte` which will take care of this. In prod, it will simply get the information from the
DOM. During local dev, it will fetch the data async from the Rust backend via the `/auth/v1/template/${id}` endpoint.
This endpoint is only available when Rauthy is compiled with `debug_assertions` and not every scenario works with it
like it prod. These cases are mentioned above already as well, just to keep that in mind. It's mostly about password
resets.

Some actions and flows only work like normal, when the backend is served as static HTML from the Rust API. To work
around these edge cases, Rauthy exposes dev-only endpoints in `src/api/src/dev_only/dev_handler.rs`. These are used
in some situations, where you might have a chicken-and-egg problem because of non-static HTML or things like that.

### Database Migrations

DB migrations live in `migrations`. They work very similar for both DB versions with slight differences.

The files in both cases MUST follow a strictly ascending order with their ID to be considered valid. For `hiqlite`, the
files therefore must always start with `<id>_`, while for Postgres (because of different crates under the hood), it must
be `V<id>__`.

> The **most important part** is, that you NEVER modify already existing files that have been published in any stable
> version before. These migrations will be hashed at startup and compared to the already applied hashes, and they will
> `panic` if there is any difference, to never operate on a database in an unknown state.

### I18n

#### UI

Internationalization has been reworked a few times until it reached the current, hopefully easy to understand state:

For the UI, everything you need to do is to take a look at `frontend/src/i18n`. This folder will container `admin`
and `common`. Translations are split into these categories, so that a normal user would never have to fetch any
translations that are only necessary for the Admin UI part. Therefore, if you add anything, make sure it is done in the
correct section. It is done in this way to have all the type-checked instead of the usual "fetch some JSON and interpret
it"-way of doing it.

You can access them inside the code via already existing hooks and I always use `t` for the `common` translations, and
`ta` for all `admin` translations:

```typescript
let t = useI18n();
let ta = useI18nAdmin();
```

#### E-Mail

E-Mails are rendered in the backend. Translations are being handled in `src/data/src/email/i18n/`. Most of these have
fixed presets. But for the "New Password" and "Password Reset" E-Mail, some custom `TPL_*` env vars can be set to
overwrite the defaults and customize these mails a bit more.

### CSS

CSS is being done very straight forward. To make it short: I really hate any CSS frameworks.

There is `frontend/src/css/global.css` which contains a few global CSS values, that are just used in many places.

`frontend/src/css/theme.css` only exists to make the IDE happy and have a better DX. In reality, the themes are always
(even during local dev) fetched from the backend to make them dynamically updatable and also cacheable. In
`frontend/src/app.html`, you can see the `<link rel="stylesheet" href="/auth/v1/theme/{{client_id}}/{{theme_ts}}"/>` to
fetch themes dynamically, depending on the `client_id`.

Apart from this, all components are styled locally in the `<style>` blocks with just normal CSS.

### UI Compilation

To make the static HTML compilation work and not fail in some edge cases, there is also the
`frontend/src/hooks.server.js` which you might need to do something in, if you added something new that's failing now.
This file is only used when the UI is served during local dev and will never be taken into account for the static HTML
build or in production.

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

If it's a unit test, it's pretty straight forward, `just test your_unit_test`. If it's an integration test however, you
may not always want to execute the whole range of tests with `just test-hiqlite` or `just test-posgtres`, because it
takes quite some time.

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
> new Rust version needs to be used. Just use the existing builder image from GitHub, which is the default anyway.

## Updating the Book

You are welcome to update the documentation in the Rauthy book. You will need `mdbook` and `mdbook-admonish`.
If you don't have them:

- `cargo install mdbook`
- `cargo install mdbook-admonish`

Once installed:

- `cd book`
- `mdbook serve --open`

You can then update the `*.md` files in `book/src`. It should be pretty self-explanatory.
`just build-docs` will rebuild them and make them available on GitHub after merging.

## Before Submitting a PR

This project does not have any actions and automatic pipelines set up yet, but there is a `just` recipe to make sure
everything is fine.

Before submitting any PR ready for merge, please execute

```
just fmt
just pre-pr-checks
```

## FreeBSD

If you want to compile from source on FreeBSD, you may have a few limitations. Compiling the UI on FreeBSD seems to not
work. You can use `just build-ui container` to use a `node` container to build the static UI files in that case, or run
the docker command from the recipe manually, if you don't have `just` available.  
You could also `just extract-ui-archive`, which will extract pre-built archives from the last release build.

Since FreeBSD uses some cargo mechanism at build time differently, you may also run into linking issues like e.g. for
`openssl`, `sqlite` or `rocksdb`. In these situations, you should be able to fix it by installing the dependency on your
build host.

If you stumble about other limitations, please do not hesitate to submit a PR and update this section.

## Not working `just` recipes

There are a few recipes that will not work for you, since you need to have access to the Rauthy repo. These are for
instance `just build` (without the above-mentioned options), `just publish`, `just publish-latest` and `just release`.