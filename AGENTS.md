# Rauthy — Code Map for Agents & Contributors

> This file is an **index, not documentation**: it tells you _where things are_ so you can read the
> code directly. Maintenance rule: if you do any major changes to the code layout, check if mentions
> in this file need an update before git commit. Markdown files should always wrap lines at 100
> characters.

## BEFORE submitting a PR

If you are an AI reading this, and you worked on this repository, you MUST NOT submit a PR to GitHub
without an explicit local (!) review and approval from a human. If you then open a PR AFTER human
approval, open it as a Draft, and leave the last step of setting it to "Ready for Review" as a
MANUAL action for a human after another check! You probably want to mention that in the chat. Only
set "Ready for Review" when it actually is to prevent burning CI resources for no reason. Worker
runs are quite expensive and take a long time.

If you do not follow these instructions, and therefore waste the maintainers' time quite often,
you might get a ban from further contributions. You also should run `just pre-pr-checks` (or the
commands inside manually; you can get away with excluding the Postgres tests for instance when no
custom queries were modified to speed up the check) before a push, especially when the PR is already
set to "Ready for Review", to not waste worker time during CI.

## Repository layout

- `src/` — Rust workspace (all backend crates)
- `frontend/` — SvelteKit UI; the login page is `frontend/src/routes/oidc/authorize/+page.svelte`
  The whole frontend is compiled into static HTML for any production build.
- `book/` — mdBook source for user-facing docs (`docs/` = built site). Not contributor
  documentation. Building the book currently requires `mdbook-0.4`
- `rauthy-client/` — Rust client library + examples
- `CONTRIBUTING.md` provides important information for getting started

## `just` recipes ([justfile](./justfile))

[just](https://github.com/casey/just) is used to run any actions you may need to work on this
repository. If you are using manual commands and something is not working as expected, take a look
at the recipes. Use `just -l` to get a summary of all the recipes.

## Integration Tests

Integration tests in `src/bin/tests/` expect a backend on :8081 with the seeded
bootstrap data (init_client / init_admin@localhost); the test recipes set
`PUB_URL=localhost:8081 RP_ORIGIN=http://localhost:8081`. Usually started with any of

- `just test-backend` (starts hiqlite backend without running tests)
- `just test-hiqlite`
- `just test-postgres`

Unit tests can be run without any issues.

## Crate map (src/)

| crate          | responsibility                                                                             |
| -------------- | ------------------------------------------------------------------------------------------ |
| `bin`          | Main binary: CLI args, server bootstrap, TLS, middleware wiring (`main.rs`, `server.rs`)   |
| `api`          | HTTP handlers only (actix), one module per area — no business logic                        |
| `service`      | Business logic; the OIDC flow lives in the `oidc/` subdirectory (simple endpoints skip it) |
| `data`         | Entities + DB methods (`entity/`), migrations, config (`rauthy_config.rs`), email          |
| `api_types`    | Request/response types shared with the frontend                                            |
| `error`        | `ErrorResponse` — the error type for all `Result`s in this repo                            |
| `jwt`          | JWT/JWK handling (access tokens, signing keys)                                             |
| `middlewares`  | actix middleware: `ip_blacklist`, `csrf_protection`, `principal`, `logging`                |
| `common`       | Shared utilities (base64, `get_rand`, IP extraction, …)                                    |
| `ldap`         | Read-only LDAPS server (bind / search / Who Am I) on top of `service` + `data`             |
| `macros`       | Proc macros                                                                                |
| `notify`       | Slack / Matrix notification sending                                                        |
| `schedulers`   | Background jobs (expiry cleanup, etc.)                                                     |
| `wasm-modules` | WASM helpers for the frontend                                                              |

## Rules for writing new code - MUST follow!

### 1. Work style and mindset

- **Trust the compiler!** When you need to analyze any issue, do NOT dig into everything
  manually when you already have a test setup or compiler at hand. Use `cargo` and tests.
  Just run them and check the output instead of manually digging into every possible corner
  before even running a test. Always tests first, and only when something is still unclear,
  then manual analysis afterwards.
- **Working with Rust** always remember that the compiler is your best friend, use it to your
  advantage. When you want to change the type of a variable from `Bytes` to `Vec<u8>`, or add
  a variant to an `enum`, and you want to see the impact, just change it. Instead of manually
  searching all usages upfront, simply run `cargo check` and let the compiler tell you
  everything. This is much quicker. After you modified the code, always make sure
  `cargo clippy` is clean.
- **KISS ("Keep it simple, stupid")** Simple code is good code, as long as it solves our
  problem. Don't overcomplicate things. Use what's already there, in the project and in the
  ecosystem. If a (widely used) crate exists that solves our problem, use it instead of
  creating our own implementation (this only applies to Rust; For JS / TypeScript, never add
  dependencies on your own; there, we always prefer our own impl).
- **No "Clean Code" anywhere** The "Clean Code" principle is garbage. They make code hard to
  follow and very slow. Don't write many tiny functions that live all over the place. Avoid using
  callback functions as parameters just to make it easier to test. Testing MUST NEVER affect
  production efficiency or performance! If you need a bigger setup to test something, initialize
  values for testing only, or use proper integration tests when you need to cover more pieces than
  a simple function (e.g. API behavior, DB calls, ...).
- **Make invalid states unrepresentable.** Push invariants into types: enums instead of
  bool flags and "must be X if Y" comments, newtypes for units and IDs, fallible
  constructors that validate once. Then the compiler enforces what a comment only requests.
- **Application code is not library code.** Library code (anything with external users)
  follows the API-design rules below strictly.
- **Let the tools do the work.** rustfmt owns formatting, clippy owns idiom checks, the
  compiler owns memory safety. Spend your attention on design.

### 3. Ownership and borrowing

**Prefer unique ownership.** Reach for `Rc`/`Arc` only when shared ownership is part of
the design, and for `RefCell`/`Mutex` only when interior mutability is. They are not
failures, but they move checking from compile time to run time, so use them on purpose.

**Don't clone to make the borrow checker stop complaining.** A clone that exists only to
end a borrow produces a silent copy whose later changes are lost. Try these first, in
roughly this order:

1. Extend a temporary with a named `let`, or shorten a borrow with a block.
2. Split a long expression into typed intermediates so the failing step is visible.
3. Move a field out of `&mut self` with `std::mem::take` (needs `Default`),
   `std::mem::replace`, or `Option::take` instead of cloning it.
4. Split a large struct into smaller ones so disjoint fields can be borrowed separately.
   The compiler tracks disjoint field borrows inside one function, but not across
   method calls.
5. Clone small, immutable data deliberately, and say so in a comment if it's not obvious.
6. Use indices into an owning `Vec`, or `Rc<RefCell<_>>` with `Weak` back-pointers, as
   a last resort. Avoid self-referential structs.
7. Avoid `Vec` allocations when an Iterator is all you need.

```rust
// Move out of a &mut enum without cloning.
fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B { name: std::mem::take(name) };
    }
}
```

**Accept the most general borrowed type.** `&str` not `&String`, `&[T]` not `&Vec<T>`,
`&T` not `&Box<T>`, `Option<&T>` not `&Option<T>`. Deref coercion then lets callers pass
owned values, borrows, or literals. Where you only read a path, string, or byte buffer,
`impl AsRef<Path>` / `impl AsRef<str>` / `impl AsRef<[u8]>` is fine on parameters (never
on struct fields; store `PathBuf` and `String`). Library code that only iterates should
take `impl IntoIterator<Item = T>`; application code may simply take `&[T]`.

**Take ownership only when you need it, and then take it by value.** If a function stores
or consumes its argument, take `T`, not `&T` plus an internal `.clone()`. If it only
reads, take `&T`; if it mutates in place, `&mut T`. Return multiple values as a tuple or
struct, not through `&mut` out-parameters (reusing a caller-owned buffer, as
`Read::read` does, is the exception).

**Prefer owned data in structs.** A struct holding `&'a T` forces a lifetime onto every
container. Owning a `String` or `Vec<u8>` is usually simpler and rarely the bottleneck.
When a type does carry a lifetime, give it a descriptive name (`'src`, `'conn`); `'_` and
`'a` are fine on simple functions. An exception to this rule is when you have a struct that
is only used for deserializing an input, e.g. parsing a downloaded JSON blob.

**Keep `Rc`, `Arc`, `Box`, and `RefCell` out of public signatures.** Take `&T`,
`&mut T`, or `T` and let callers decide how to own things. A service that must be shared
implements a cheap `Clone` by wrapping its state in `Arc<Inner>` internally.

### Naming

- No `get_` prefix on getters: `fn len(&self)`, `fn name(&self) -> &str`,
  `fn name_mut(&mut self)`. Use bare `get` only when there is one obvious thing to get
  (`Cell::get`). Related: `set_foo(T)`, `take_foo()`, `with_foo(|f| ..)` for closure
  access, and unsafe `_unchecked` siblings for checked accessors.
- Conversion prefixes signal cost and ownership: `as_` (free, borrow to borrow), `to_`
  (expensive, borrow to owned or borrow to borrow), `into_` (consumes `self`). Put `mut`
  in type order: `as_mut_slice`, not `as_slice_mut`.
- Constructors: `new` (primary), `with_capacity` / `new_with_foo` (variants), `try_new`
  (fallible), `from_foo` (conversion; pair important ones with a `From` impl). I/O
  resources use domain verbs: `File::open`, `TcpStream::connect`.
- `iter` / `iter_mut` / `into_iter` return types named `Iter` / `IterMut` / `IntoIter`;
  `keys()` returns `Keys`.
- Error types read verb-object-error: `ParseBoolError`, not `BoolParseError`.
- Avoid weasel suffixes `Service`, `Manager`, `Factory`, `Helper`, `Util`. Name the thing
  it is (`Bookings`, `Dispatcher`, `FooBuilder`). Don't repeat the module name in the
  type (`foo::Id`, not `foo::FooId`). Avoid abbreviations an outsider wouldn't know.
- If you implement `len()`, implement `is_empty()`.
- Feature names are bare: `std`, `serde`, `alloc`, never `use-std` or `no-std`.

### Model the domain with types

- **Newtypes** for semantic distinctions: `struct Miles(f64)` and `struct Kilometers(f64)`
  cannot be mixed, where a type alias can. Add `#[repr(transparent)]` when layout matters,
  and derive or forward the traits you need (`derive_more` cuts the boilerplate). A
  newtype that encodes an invariant (`Month`) validates in a fallible constructor
  (`TryFrom`, `FromStr`, `try_new`) and never offers an infallible `From` from the raw type.
- **No `bool` parameters in public or shared signatures.** `report(true)` is unreadable at
  the call site and cannot grow a third mode. Use a two-variant enum
  (`Verbosity::Summary`, `Verbosity::Full`) or two methods (`report()`,
  `report_verbose()`): `Widget::new(Size::Small, Shape::Round)`, not
  `Widget::new(true, false)`. The only exception is a private helper whose call sites sit
  a few lines away. When refactoring, an existing `bool` parameter is one of the smells to
  remove, not a detail to preserve.
- **Named fields over tuples**, except single-field newtypes and very local code.
- **`Option` for absence, never sentinels** such as `-1` or `""`. **Data-carrying enums**
  instead of "field X is only meaningful when Y is set".
- **Use the strongest std type:** `PathBuf` / `&Path` for paths, `Duration` for spans,
  `NonZeroU32` internally only (keep plain integers at API boundaries).
- **Two or three primitives stay primitives** in application code (`Option<i64>` ids,
  timestamps as `i64`); that keeps the code lean and matches the DB and protobuf boundary.
  What makes them safe is the `debug_assert!` at the boundary (§ 5), not a wrapper. The
  existing `pub type UserId = i64` aliases are legacy IDE hints — don't add new ones, and
  turn one into a real newtype only when the distinction has an invariant attached.
- **Independent flags** use the `bitflags` crate; an enum models exactly one choice.

### Refactoring someone else's code

- Fix every smell the request is about, not just the cheap ones: clones that only
  satisfy the borrow checker, `&String` / `&Vec<T>` parameters, `unwrap` on recoverable, fallible
  paths, `Rc<RefCell<_>>` in signatures, `get_` getters, `bool` parameters, `+` string
  concatenation, `#![deny(warnings)]`, error types without `Error`, nondeterministic
  `HashMap` iteration in output.
- Preserve quirks you don't understand and flag them to the user, rather than silently
  "fixing" them.
- When the module has no tests, pin the current behavior with a few before you touch it.
  Anything the compiler can already see through, like types, signatures and `enum` variants,
  needs no pinning: change it and let `cargo check` list every affected place.

### Errors and panics

Pick the mechanism by contract:

| Situation                                                                                              | Use                                                                                                            |
| ------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------- |
| Absence is normal (`first()`, `checked_add`)                                                           | `Option<T>`                                                                                                    |
| Recoverable at runtime: input, parsing, network, DB, I/O                                               | `Result<T, E>` — never `panic!` here                                                                           |
| The caller may need to know what went wrong                                                            | `Result<T, E>`                                                                                                 |
| A bug or violated invariant inside your code                                                           | `debug_assert!` / `panic!` / `expect("why")`                                                                   |
| Unrecoverable error like parsing config at startup, a port that is taken, a handler that cannot be set | `panic!` / `expect("why")`                                                                                     |
| Data from our own database that can never be wrong                                                     | `debug_assert!` for the invariant, `panic!("Invalid alarm trigger in DB")` when a match arm truly cannot exist |

**Panics are not exceptions.** A panic means "stop now". Never use one to report bad
input, never rely on `catch_unwind` for control flow (it does nothing under
`panic = "abort"` and can leave state inconsistent), and never catch a panic and continue
inside a library. Where a panicking convenience is justified, pair it with a fallible API
(`v[i]` / `v.get(i)`, `new` / `try_new`) and document it under `# Panics`.

**Every intentional panic carries a message with the relevant values:**
`assert!(buf.len() >= HEADER, "buffer too small: {} bytes", buf.len())`,
`expect("config is loaded before workers start")`. Prefer `expect("reason")` to
`unwrap()`. Don't restate the condition as the message.

**Pattern-match instead of check-then-unwrap:** `if let Some(v) = opt { .. }` or
`let Some(v) = opt else { return .. };`, never `if opt.is_some() { opt.unwrap() }`.

**Lock poisoning is fatal:** `.lock().expect("lock poisoned")`. Don't recover with
`into_inner`.

**Guard the boundaries with `debug_assert!`.** Where an invariant cannot live in the type
system — and in an application with a database it often cannot — assert it at the point
where the data enters: `debug_assert!(user_id.is_some() != group_id.is_some())`,
`debug_assert_eq!(task.id, log.task_id)`. Free in release, catches every violation in dev,
and it is the cheapest documentation of "the caller must guarantee this" there is.
An invariant a `debug_assert!` guards must never be "fixed" by a newtype the caller has to
construct — unless the same value also crosses untrusted input somewhere else.

**Data from our own database is not untrusted input.** A column value the schema says can
only be one of four things may `panic!("Invalid alarm trigger in DB")` when the DB returns
something else, and a `From<i32>` for a DB-backed enum may map the unknown case to the
default variant when a default exists. Anything arriving from outside the process — headers,
JSON, protobuf, multipart, the network — goes to `Err` instead, always.

**Destructors never fail, panic, or block.** Provide `close(self) -> Result<()>` for
fallible teardown and let `Drop` do best-effort cleanup.

**Document failure modes:** `# Errors` for any `Result`, `# Safety` for every `unsafe fn`,
and a `# Panics` block whenever a function can panic because it expects specific input, even
for private functions and in a standalone app.

### Everyday idioms

- **Iterators over hand loops** when the pipeline is short and reads top-down:
  `values.iter().filter(..).map(..).collect()`, `.any()`, `.sum()`, `.fold()`. Collect
  fallible steps into `Result<Vec<_>, E>` and apply `?`. Keep an explicit `for` loop when
  the body is large, does several jobs, or needs awkward early returns. Bind a long chain
  to a named variable rather than putting it in a `for` header, and prefer `for x in it`
  to `.for_each(|x| ..)`.
  The exception to this rule is when you already know that necessary target capacity. Instead
  of calling `.collect()` into a `Vec<_>` for instance, pre-allocate the whole capacity and
  use a traditional loop.
- **`Option` / `Result` combinators** (`map`, `and_then`, `ok_or_else`, `unwrap_or_else`,
  `as_ref`, `as_deref`) when they're clearer than `match`, and `match` when they're not.
- **`format!` / `write!`** with inline args (`format!("{name}: {count}")`) for
  concatenation and substitution, not chained `+` or `str::replace`. `write!` into an
  existing buffer in hot paths. Only inline args if **every single one** of them can be
  inlined — never mix inline and positional inside one call. Inline only while the line
  stays under 100 characters; when inlining would push it over, drop the inlining and use
  positional args — readability wins over cleverness.
- **Fail fast at the top.** Check the precondition, `return Err(..)` right there, and let
  the happy path run the rest of the function at one level of indentation. Guard clauses
  beat a pyramid of `if let Some(..) { .. }` blocks, and in a `match` the `None` / error
  arm comes before the happy one.
- **`#[inline]` on the tiny helpers** (`crc`, `is_min_len`, a `f64_to_f32` guard) that a
  hot path calls in a loop. They are the exception to the no-tiny-function rule in § 13:
  the function exists so the call site reads as intent, and `#[inline]` makes it free.
- **Drop guards for cleanup** that must run on early return or unwind. Bind them to a
  name (`let _guard = ..`), because `let _ = ..` drops immediately.
- **Closures that capture mixed ownership:** rebind in a block, then `move`:

    ```rust
    let job = {
        let cfg = cfg.clone();
        move || run(cfg, data)
    };
    ```

- **Import style:** one `use` per module path (`use foo::bar::{Bar, Baz};`, do not nest); what
  you import is everything from inside the same application and from std (see the two bullets
  below), and those are used unqualified (`Regex`); std functions stay qualified by the module
  you imported (`use std::mem;` then `mem::take`, `fs::read`). Import a trait inside the function
  that needs it unless the whole file uses it. No globs except `use super::*` in tests,
  `use Enum::*` in a function that matches on it, and a framework's prelude such as
  `use bevy::prelude::*`.
- **Import order:** own-crate imports (`use crate::...`) come first, then everything else —
  external crates and std together — in one alphabetical block with no blank lines between
  groups. That is what RustRover produces with the default layout; do not fight it.
- **Calling external code** when calling code or functions from a 3rd party crate, always
  prefix it with the module above to make it very clear where it comes from instead of using a
  direct import. E.g. `header::InvalidHeaderValue` instead of just `InvalidHeaderValue`. Imports
  from inside the same application (even cross-crate) should always be imported directly.
  Only when there can be confusing issues, like e.g. you have the `time` crate as a dependency,
  use full qualifiers like e.g. `tokio::time::`.
- **Standard is not external** anything from the std Rust library (`std`, `core`, `alloc`) should
  always be directly imported.
- **Magic numbers** become named `const`s with a doc comment saying where the value
  comes from and what changing it affects.
- **Logging:** structured events through `tracing` or `log`, not `println!` / `dbg!`,
  except for a CLI's own stdout output.

### Documentation and comments

Doc comments below are for a crate that others depend on. In a standalone app, the names carry
the story and comments stay at the exceptions: why something is not obvious, and `# Panics`.

The exception to that, in an app, is the code that is genuinely hard to follow on first read:
a parser matching an RFC, a wire- or file-format, a state machine, a protocol quirk, a
non-obvious decision taken on purpose. That code gets comments — dense ones, because the next
reader has to verify the code against the spec. Plain CRUD and route handlers get none. If a
module is written to a spec and might one day be split into its own crate (a parser like this),
document it like the library it may become.

- `///` on every public item; `//!` at the top of every public module and the crate root.
  The first sentence is a single line of about fifteen words, because it becomes the
  summary in listings.
- Sections, in this order, as applicable: `# Examples`, `# Errors`, `# Panics`,
  `# Safety`. Examples use `?` with the `fn main` hidden behind `# ` lines, never
  `unwrap()`, because users copy them verbatim. Describe parameters in prose, not a table.
- Document what the signature can't say: preconditions, side effects, ownership
  transfer, complexity, and why. Don't restate the type or the name;
  `/// Removes an element.` on `remove_element` is noise.
- Link items with ``[`Type`]``. Hide impls users never trigger
  (`From<PrivateError> for PublicError`) with `#[doc(hidden)]`.
- `//` comments explain why, not what: `// SAFETY:`, `// TODO(issue-123):`, non-obvious
  magic values, workarounds with a link. No design journals, "why we chose X" essays, or
  rule-compliance tables in code docs. Document the end state.
- **Comments outlive the issue.** A comment must read correctly the second it lands on `main`, for
  a reader who never saw the ticket. No issue or PR numbers in code comments, no internal decision
  tags ("D3", "phase 2", "point 4"), no "as decided in the plan", no "was broken before X", no
  "added in v0.36 for feature Y". History lives in git log, the commit message, and the changelog;
  the code states the constraint and the end state only. A `// TODO(issue-123):` is the exception:
  it points at work to do, not at work that was done.
- **Commented-out code and `TODO`s are deliberate** when the work is in progress or the code
  may be needed again soon — leave them alone unless asked. `#[allow(dead_code)]` on the
  same items is equally deliberate.

### Tests

- **CRUCIAL:** NEVER degrade production efficiency or performance for easier testing. Adopt testing
  to production code instead and make sure to avoid any expensive patterns!
- Unit tests in `#[cfg(test)] mod tests { use super::*; .. }` for private internals.
  Integration tests in `tests/` for anything reachable through the public API, and prefer
  that location when either would do. Doc examples are tests too.
- **A database in a test is always an integration test.** Never fake a DB inside a unit test, and
  never open a real one from a `#[cfg(test)]` module. Anything that touches a DB runs in
  `src/bin/tests/` against a running backend: via the public API, or via dev handlers when the
  public API cannot reach the case. This is the exception to the mockability rule below. The
  existing fake-DB and `RAUTHY_TEST_PG_URL` unit tests are legacy: not a template, and to be
  removed in the long run.
- Test observable behavior and properties (monotonic, round-trips, idempotent), not
  restated constants or mirrored branches.
- **Never prefix a test file name with `z` to order it.** The `src/bin/tests/zz*_` files exist
  only because a few legacy tests depend on running late; that is legacy, not a pattern. A new
  test is named after what it tests, is self-contained, and removes everything it created, so its
  result never depends on the file that ran before it.
- Write a failing test before fixing a bug. Cover each feature combination and each
  platform-specific path. When something is broken, run what already exists first and read
  the output; analyse by hand only what is still unclear afterwards.
- Anything doing I/O, time, randomness, or network must be mockable:
  `new_mocked() -> (Self, MockCtrl)` behind `test-util`, not a `Default` that performs I/O.
- Fuzz parsers of untrusted input with `cargo fuzz`. Benchmark with `criterion` or `divan`
  and `std::hint::black_box`. Run Miri over `unsafe`.
- Assertions get context so failures read well: `assert_eq!(got, want, "input {input:?}")`.
- A test that can fail returns `Result<(), Error>` and uses `?`, so the report stays clean.
  `pretty_assertions::assert_eq` in test modules where structs are compared, `rstest` for
  parameterized cases. A helper that exists only for tests is `#[cfg(debug_assertions)]` and
  says so in one line.

### Performance, in order

Keep performance in mind, always: borrow wherever possible, avoid unnecessary clones and
memory allocations, and use the correct data structure for the task. Those are free and need
no measuring. Anything bigger gets measured first; when you do optimize: `Vec::with_capacity`
or `collect()` over push loops; reuse caller-owned buffers in hot APIs
(`fn read_into(&mut self, buf: &mut Vec<u8>)`); `Box<[T]>`, `Box<str>`, `Arc<str>` for
long-lived immutable sequences; `shrink_to_fit` on long-lived collections built without a
reservation; a faster hasher (`ahash`) only for trusted keys; batch work
instead of per-item locking. Derive `Copy` for small plain types and never for large ones. When
you can re-use a buffer instead of re-allocating all the time, then do it, i.e. for a long-lived
connection task or protocol parser. Anything that is built once per process is a
`LazyLock` / `OnceLock` static (std only, no `once_cell`): compiled regexes, encoder params, channel
senders, the config.

### Code style and layout

- **Ordering inside files:** `use` imports → `mod` definitions → `static` → `const` → rest.
- **Naming that this codebase uses, keep using:** route handlers are named after the HTTP
  verb — `get_events`, `post_event`, `put_event`, `delete_event`.
  DB entities: the struct is the row, and `impl` owns its SQL (`insert`, `find`, `find_for_x`,
  `update`, `delete`); queries go in raw strings, keywords uppercase, params through the crate's
  `params!` macro. The local that receives a constructed or fetched entity is `slf`. Short domain
  abbreviations are fine when the domain uses them (`cal`, `ev`, `txn`, `res`, `err`, `typ`, `buf`,
  `uid`, `ts`); invented ones are not. A crate's own copyright header line stays on its `lib.rs` /
  `main.rs`.
- **Every function that triggers a database call is prefixed with what it does** — `insert_`,
  `upsert_`, `update_` (or `set_`), `delete_`, `find_` (always SELECT, never modifies), and
  `exists` (a SELECT is fine there). The name alone must reveal that the call is an expensive
  DB round-trip, without looking at the internals; bare intent names like `complete` or
  `pending` don't qualify — `update_complete` and `find_pending` do.
- The main `pub` function of a module comes at the top; all its helpers and sub-functions
  come AFTERWARDS, never before it.
- Multiple functions or code blocks serving one logical purpose (e.g. managing a global TLS
  CA) go into a `struct` + `impl` block, not bare functions. Single helpers/utils stay free
  functions.
- When a function has multiple params of the same type, wrap them in new `struct`s so the
  call order cannot be mixed up. A DB-model constructor that mirrors a query may keep its
  arguments and the `#[allow(clippy::too_many_arguments)]` that comes with them.
- No new function for less than 3 lines of code, and no outourcing code into a separate
  function that is only called from a single place, except for unit testing. Public crate API
  and the trait impls § 4 asks for are exempt from this — and so are the `#[inline]`
  hot-path helpers of § 6.
- Function names say everything; a comment is only acceptable when the name would be too long
  or cannot be made clear. Private functions basically never need comments, the name must tell
  the story.
- NEVER explain function internals in comments; the caller does not care how the goal is
  achieved. The exception is a function that can `panic` because it expects specific input:
  that one ALWAYS gets a `# Panics` block, even a private one.
- Doc comments for `pub` functions and `struct`s are only wanted when developing a crate
  instead of a standalone app; even then, never write how the internals work.
- ALWAYS keep performance in mind. Avoid unnecessary clones or memory allocations in general,
  make sure to use the correct data structures for the task, and borrow values wherever
  possible.

### Final checklist

Before returning Rust code, confirm:

- `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings`, and `just test` are clean
- Nothing that can fail at runtime is `unwrap()`ed; every `expect`, `assert`, and `panic` in
  production code has a message with the relevant values. `debug_assert!` guards the
  invariants the types cannot express, and unwraps in tests (or on provably infallible
  calls like static init) are fine.
- No `.clone()` whose only job is to satisfy the borrow checker.
- No globs (bar a framework's prelude), no preludes of your own, no leaked third-party types.
- No comment explains how a function works internally, and every function that can panic on
  the input it expects has a `# Panics` block, private ones included.
- No comment needs the issue, PR, plan, or roadmap point that produced it to be understood: no
  ticket numbers, no decision tags, no "as decided", no "before this change".
- Files are ordered `use` → `mod` → `static` → `const` → rest, and each module's main `pub`
  item sits above its helpers.
- No new function under 3 lines, and no helper that is called from a single place — the
  `#[inline]` helpers of § 6 being the exception.
- Multiple params of the same type were wrapped in `struct`s so they cannot be swapped.
- Anything unclear was asked about, not guessed.
