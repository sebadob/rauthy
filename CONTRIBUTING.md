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

To work with this project, you need to have a few things available on your system. Most tools can be installed
automatically. As a prerequisite though, you need at least to have

- [just](https://github.com/casey/just)
- a BASH shell
- at least Rust v1.81
- `npm`

### Initial Setup

The internal setup of this project is a bit more complex because of support for different databases and compile-time
checked queries for each of these. This means you will not be able to easily do something like `cargo run`.
Instead, you will use [just](https://github.com/casey/just) for everything, which will handle all the additional
stuff behind the scenes for you.

Because the initial setup requires a few tools and install steps, there is a `setup` just recipe which does all the
work for you. After a fresh clone, you can type

```
just
```

to see all available public recipes. Internal ones do exist as well, but you most likely don't need to care about these.

Rauthy is using compile-time checked templating with [askama](https://crates.io/crates/askama) and the UI will be
compiled into static HTML files during the build process. To make the initial database migrations (and therefore the
compiler) happy, the first thing we need to do is to build the HTML files. This is kind of a chicken-and-egg problem.
You also need a local dev mail server to receive E-Mails / Events and a Postgres instance.

All of this setup is done automatically by simply calling

```
just setup
```

The `setup` recipe will install all necessary tools and dependencies:

- `just`
- `sd`
- `mdbook` + `mdbook-admonish`
- `sqlx-cli`

It will then compile the frontend to static HTML to make the templating engine happy.
Afterward, it will create a docker network named `rauthy-dev` and start `mailcrab` and `postgres` containers.

As the last step, it will do a `cargo build` to make sure there are not compile errors.

### Config

The default config file is the `rauthy.cfg`. This has reasonable defaults for local dev.
Additionally, Rauthy will read from a `.env` file, which has been added to the `.gitignore`.
This will prevent you from accidentally pushing sensitive information into git.

You need to add at least one entry in a `.env` file, which points to the public IP of your build host.

To make getting into the project easier for new developers, all tooling needed is installed in a `rauthy-builder`
container image, which is used internally by `just` to invoke all kinds of commands. This uses your hosts network
layer for the most amount of compatibility and the least painful setup on different machines and environments.
These commands and containers use the `PUB_URL` in lots of places for inter-container communication.

```
PUB_URL=192.168.0.1:8443
```

You can set additional variables here and even insert sensitive information. These will never be pushed into git.

### Get it up and running

Once prerequisites are met, the rest should be really simple:

```
just run
```

will start Rauthy with a SQLite backend serving the statically built HTML UI, all on port `8443` by default.
If you want to work on the frontend too, execute in another terminal:

```
just run ui
```

which will make the Svelte UI in dev mode available via port `5173` on your machine.
There is only one thing about the UI in dev mode: The way the static file adapter from svelte is set up right now,
you will not be able to access http://localhost:5173/auth/v1, you actually would need to append `/index` here once.
The rest works just the same and as expected: http://localhost:5173/auth/v1/index

If you want to test against Postgres, instead of the above `just run`, simply execute

```
just run postgres
```

If you don't use passkeys, it should work right away. If you however want to test passkeys with the local
dev ui, you need to adjust the port for `RP_ORIGIN`.

If you want to work on Upstream Auth Providers in local dev, you need to change
`DEV_MODE_PROVIDER_CALLBACK_URL="localhost:5173"`. This will adopt the `redirect_uri` for an upstream login call
to point back to the UI in dev mode.

#### Note

The whole dev setup and pipelines have gotten a full rework just recently.
It should be working right away (hopefully), but I am pretty sure that there are still some rough edges in some cases.
Please let me know if you have problems somewhere.

### Default Credentials

When `DEV_MODE=true` (which is the default for local dev), Rauthy will do some programmatic DB migrations. For instance
the JWKS will always be the same for faster startup, as well as the credentials.

The default admin and password with `DEV_MODE=true` are always:

```
admin@localhost.de
123SuperSafe
```

### Dev Backend

Everything necessary for local dev is handled by the `just setup` step above. However, o start the test / dev backend
after you have stopped it maybe, you can

```
just backend-start
```

which will start a [mailcrab](https://github.com/tweedegolf/mailcrab) container, so you can receive E-Mails from Rauthy
without the need to set up any additional SMTP server. You can access the UI via `http://<your_host_ip>:1080`.
It will also start a Postgres database in another container. You can access it with the user `rauthy` and default
password `123SuperSafe`, but the access is already pre-configured.

This command will also apply all existing database migrations to both a local SQLite file and the newly started
Postgres container. This is the reason we needed to build the UI files beforehand.

### Rebuilding the UI

If you are working with the UI, you need to

```
just run
```

to start the backend, and do another

```
just run ui
```

in another terminal to start the `npm` dev server. The UI will then be available on port `5173` during local
development. To test the UI served via the backend after you are finished, you can

```
just build-ui
```

which will build the UI into static HTML files and populate the `templates/html` folder to make the `askama`
templating engine happy.

## Before Submitting a PR

This project does not have any actions and automatic pipelines set up yet, but there is a `just` recipe to make sure
everything is fine.

Before submitting any PR ready for merge, please execute

```
just pre-pr-checks
```
