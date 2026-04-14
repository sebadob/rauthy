# Docker

## Testing / Local Evaluation

For getting a first look at Rauthy, you can start it with docker (or any other container runtime) on
your localhost. The image contains a basic default config which is sufficient for local testing (
don't use it in production, it contains hardcoded secrets).

```
docker run -it --rm -e LOCAL_TEST=true -p 8443:8443 ghcr.io/sebadob/rauthy:0.35.1
```

```admonish caution
When you add the `LOCAL_TEST=true` var, it will ONLY work on `localhost`! Under the hood, Rauthy
loads a very minimal (and unsafe) demo config. If you want to test anywhere else than `localhost`,
you need to set up a proper config.
```

```admonish note
Some browsers like Firefox do not allow the registration of Passkeys when using self-signed TLS
certificates. To be able to do this during testing, you would need to add the generated CA
certificate to your trust store.
```

```admonish hint
This command starts an HTTPS server with self-signed certificates.  
Make sure to add the `https://` scheme if you open the URL manually.
```

To proceed, go to **[First Start](first_start.md)**, or do the production setup below to have
persistence.

## Testing / Evaluation with E-Mail

You can do anything you like with the default <code>admin@localhost</code> account. Rauthy does not
have any special accounts. It is an account like any other. The only reason it is a Rauthy admin, is
because it is assigned to the `rauthy_admin` role.

If you like to test creating new accounts or password reset flows though, you need to have at least
a minimal setup that is able to send E-Mails. The easiest way (works on localhost only) is the below
`docker-compose.yaml`:

```yaml
services:

  mailcrab:
    image: marlonb/mailcrab:latest
    ports:
      - "1080:1080"

  rauthy:
    image: ghcr.io/sebadob/rauthy:0.35.1
    environment:
      - LOCAL_TEST=true
      - SMTP_URL=mailcrab
      - SMTP_PORT=1025
      - SMTP_DANGER_INSECURE=true
    volumes:
      - data:/app/data
    ports:
      - "8443:8443"
    depends_on:
      - mailcrab

volumes:
  data:
```

```admonish hint
You can create a `.env` file with `COMPOSE_PROJECT_NAME=myapp` for custom container/volume name 
prefix.
```

Save this into `docker-compose.yaml` and start with:

```
docker compose up -d
```

You then need the logs output from Rauthy to read the random password for the `admin@localhost`
account:

```
docker compose logs -f rauthy
```

Any outgoing E-Mail will be caught by `mailcrab`, which can be accessed
via [http://localhost:1080](http://localhost:1080) .
When you are done testing, shut down with

```
docker compose down
```

## Production Setup

For going to production or to test more in-depth, you need to create a config that matches your
environment.

The first thing you want to do is to add a volume mount for the database. The second thing is to
provide a more appropriate config.

Rauthy expects at least the very basics in a config file. Most other values could be set via ENV
vars. It parses both, the config first and any env var that is set will overwrite a possibly
existing one from the config. You can add environment variables to the startup command with the `-e`
option.

```admonish note
The following commands will work on Linux and Mac OS (even though not tested on Mac OS). Since I am 
no Windows user myself, I cannot provide tested commands in this case.
```

**1. We want to create a new directory for Rauthy's persistent data**

```bash
mkdir -p rauthy/data rauthy/tls
```

**2. Add the new config file.**

Rauthy (since v0.35) comes with a CLI. You can use it to generate a (basic) config file to get
you started. We will use it from inside the container. However, since it's rootless and runs with
user `10001:10001` inside, we need to modify access rights.

```bash
sudo chown -R 10001:$(id -g) rauthy
``` 

Next, we set an `alias` to be able to work with the CLI comfortably.

```bash
alias rauthy='docker run -it --rm -v $(pwd)/rauthy/data:/app/data -p 8080:8080 ghcr.io/sebadob/rauthy'
```

```admonish note
The alias expects you to expose on port `8080` afterwards. If you plan on doing something else
(for testing), adopt the `-p` option.
```

Make sure it's working:

```bash
rauthy --help
```

We can then use it to generate a config:

```bash
rauthy generate-config -o data/config-generated.toml
```

Once finished, you will have a new config file at `rauthy/data/config-generated.toml`. You can only
access it as `root` though, because as mentioned above, the binary inside the container runs as
`10001:10001`.

```bash
sudo cat rauthy/data/config-generated.toml
```

You can either use it directly, or you can evaluate it first. The final config will be expected in
`rauthy/config.toml` by default. However, let's start with the generated one and make sure
everything works as expected:

```bash
rauthy serve -c data/config-generated.toml
```

```admonish note
If you answered during the config generation that you want to run Rauthy with your own TLS 
certificates, make sure to copy them in place. By default in `rauthy/tls/*`.
```

When everything is working as expected, move the generated config in the final location:

```bash
sudo mv rauthy/data/config-generated.toml rauthy/config.toml
```

**3. Access rights for the Database files**

Finalize / harden access rights:

```
sudo chmod 0600 rauthy/config.toml && \
sudo chmod 0700 rauthy/data && \
sudo chmod 0700 rauthy/tls && \
sudo chown -R 10001:10001 rauthy
```

**4. Adopt the config to your liking.**

The config-generator only gets you started and asks for the very basic requirements. There is a lot
more you can configure. Take a look at the [reference config](../config/config.html) and adopt
everything to your needs.

For an in-depth guide on a production ready config, check
the [Production Config](../config/production_config.md) section.

**5. Start the container with volume mounts**

```
docker run -d \
    -v $(pwd)/rauthy/config.toml:/app/config.toml \
    -v $(pwd)/rauthy/data:/app/data \
    -p 8443:8443 \
    --name rauthy \
    ghcr.io/sebadob/rauthy:0.35.1
```

- `-v $(pwd)/rauthy/config.toml:/app/config.toml` mounts the config in the correct place
- `-v $(pwd)/rauthy/data:/app/data` mounts the volume for Hiqlite
- `-p 8443:8443` needs to match your configured `server.port_http` or `server.port_https` of course.
  If you use a reverse proxy inside a docker network, you don't need to expose any port, but you
  need to make sure to set `server.scheme = "http"`, `server.proxy_mode = true` and the correct
  value for `server.trusted_proxies`.-

```admonish note
If you provided your own TLS Certificates, make sure to mount them as well with an additional \
`-v $(pwd)/rauthy/tls:/app/tls`.
```

**6. You can now proceed with the [First Start](first_start.md) steps.**
