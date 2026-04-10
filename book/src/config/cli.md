# Rauthy CLI

The Rauthy binary comes with a CLI. You can use it either from inside the default container, or
directly when you have access to the binary. It does NOT have functionality to control a Rauthy
instance or anything like that. It does not have it now and never will. It helps you getting started
with config generation, or it can generate encryption keys, secrets, or hash passwords.

Let's make sure that you can run it first.

## From inside the Container

This is by far the easiest and quickest method. We are setting an `alias` to make it even easier.

```bash
alias rauthy='docker run -it --rm ghcr.io/sebadob/rauthy'
```

This will be it already for generating secrets or hashing passwords. If you want to use it to
generate a config as well, we need to make sure to mount a volume, so you can access it later:

```bash
mkdir rauthy_data && sudo chown -R 10001:$(id -g) rauthy_data
```

And then update the alias:

```bash
alias rauthy='docker run -it --rm -v $(pwd)/rauthy_data:/app/data ghcr.io/sebadob/rauthy'
```

## Direct Access

Usually, this is the easiest and quickest method, because there's nothing to do if you have the
final binary. The only issue for most people is probably, that they cannot get it easily. There are
no installers or binary downloads available right now. However, you will get it by either copying
it out of a container image, or by building from source.

## From DEV Env

When you have set up a development environment, you can access it pretty much directly as well.
Just make sure you always append the ` -- ` after `cargo run`.

```bash
cargo run -- --help
```

```admonish hint
To make sure it's working simply execute `rauthy --help` or `./rauthy --help` in case you have the
binary.
```

## What it can do

As mentioned already, you will not be able to control a Rauthy instance with this CLI. You can use
it to generate config files, secrets, encryption keys, or create password hashes for bootstrapping.

## Generate a Config

For generating a config, make sure you created the `alias` above with the volume mount. You can then
execute:

```bash
rauthy generate-config -o data/config-generated.toml
```

It will ask you a bunch of questions and then create a `config-generated.toml` file in the `data`
dir inside the container, which will be in your `rauthy_data` dir later on, if you use the `alias`
above. This config is only there to get you started though. There is a lot more you can configure,
and it would be absolutely insane to ask for every little thing. The
full [Reference Config](../config/config.md) with comments explaining the values has almost 3000
lines by now.

## Generate Encryption Keys

Be it either for your very first setup, or when want to rotate your keys, you can generate new ones
with the CLI pretty easily:

```bash
rauthy generate-enc-key
```

By default, it will use the current day as date as the ID. This makes it easy to know when a key was
created. If you like, you can set your own ID with the following flag:

```bash
-k, --with-key-id <WITH_KEY_ID>
```

The key ID must match this regex: `[a-zA-Z0-9:_-]{2,20}`

## Generate Secrets

This probably the most basic task ever, but it will generate you random alphanumeric,
cryptographically secure secrets. You can generate them anywhere else as well, but make sure the
generation is actually cryptographically secure, and not just a JS generator in a random web app,
where you cannot be sure.

```bash
rauthy generate-secrets
```

There are no flags for this one. It will simply generate 48 secure random characters. This length
is very safe while still not being way over the top. If we wanted the same level of security as a
256-bit random bytes key, in alphanumeric form it would be ~43 characters. We have 48 here, which
even exceeds it. Anything bigger would be a waste of resources.

## Hash Passwords

If you want to hash a password for bootstrapping, you can do it like so:

```bash
rauthy hash-password
```

You can configure the parameters for password hashing, but this tool provides safe defaults. If you
really need to, you could lower the `m_cost` though, if you have a good reason, or bump it up even
more if you want to be extra secure.

## Serve Rauthy

The last option here is `rauthy serve`. This will actually launch Rauthy. 