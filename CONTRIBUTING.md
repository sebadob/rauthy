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
and `delete` and have their own `impl` block for the structs to be "easily" interchangeable. Basically, you should have
a general idea what a function does if you just read its name.
- Simple and readable code is way better than making everything generic without a good reason just because you can.
- Any function that returns a `Result<_>` should return Rauthy's custom `ErrorResponse` from `rauthy-common` as the
error type. This decision was made instead of just simple `anyhow` error Strings for instance to actually make
errors understandable by external code by using the `error` field and by humans by using the `message` field.

## Getting Started

The most important thing to compile the code is obviously `rust` and `cargo`. The frontend part is based on
[svelte](https://svelte.dev/), which means you need `npm` as well. Working with this project is fully set up with
[just](https://github.com/casey/just), which is a modern alternative to `make`, written in Rust.

### Prerequisites 

- Rust installed and `cargo` available
- Node / `npm` available
- `docker` available, if you want to build images
- [just](https://github.com/casey/just) installed
- BASH (no cmd.exe or powershell) - when you are on windows, you can use something like Git BASH

### Get it up and running

Once prerequisites are met, the rest should be really simple:

1. Install node-modules
    - `cd frontend`
    - `npm install`
2. If this is a fresh pull, build the UI once to populate the `templates/html`
    - `just build-ui`
3. Create and migrate SQLite to make the compile-time checked queries happy
    - `just migrate-sqlite`

This should be it with the setup part. If you just want to work on the backend, you only need:

`just run-sqlite`

If you want to work on the frontend too, in another terminal:

`just run-ui`

There is only one thing about the UI in local dev. The way the static file adapter from svelte is set up right now,
you will not be able to access http://localhost:5173/auth/v1, you actually would need to append `/index` here once.
The rest works just the same and as expected: http://localhost:5173/auth/v1/index

As long as you don't use passkeys, it should work right away. If you however want to test passkeys with the local
dev ui, you need to adjust the port for `RP_ORIGIN` in the `rauthy.cfg`.

### Local SMTP Relay

If you want to use a local SMTP relay for testing E-Mails, there are just recipes for [maildev](https://github.com/maildev/maildev):

`just maildev-start` will start the docker container for `maildev`. The UI will be available via http://localhost:1080
You need to set in the config:
- `SMTP_URL=localhost`
- `SMTP_DANGER_INSECURE=true`

You can stop the container with `just maildev-stop`.

## Before Submitting a PR

This project does not have any actions and automatic pipelines set up yet.  
This means, that you should make sure, that `clippy` does not give any warnings for your code and that all the test 
cases are fine.

As long as you do not have done anything like feature dependant queries and use different `query!` macro depending
on the uses features, you are fine with just executing `clippy` and tests for sqlite only:

- `just clippy-sqlite`
- `just test-sqlite`

> One thing about `just test-sqlite`: This does build the backend, start it in a background task and then executes
> all tests incl integration tests. However, is uses `set -euxo pipefail` inside the script, which make it fail if
> any test case fails. The way the script works, this does not kill the background task currently, which you have to do
> manually. The reasons this option is set is that it would otherwise be possible to execute the whole build pipeline
> and oversee failing test cases. If you are working on tests locally, you can out-comment this line temporarily.
