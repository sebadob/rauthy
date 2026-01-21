# Docker

## Testing / Local Evaluation

For getting a first look at Rauthy, you can start it with docker (or any other container runtime) on
your localhost. The image contains a basic default config which is sufficient for local testing (
don't use it in production, it contains hardcoded secrets).

```
docker run -it --rm -e LOCAL_TEST=true -p 8443:8443 ghcr.io/sebadob/rauthy:0.34.0
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
    image: ghcr.io/sebadob/rauthy:0.34.0
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

```
mkdir -p rauthy/data
```

**2. Add the new config file.**

This documentation is in an early version and remote links are not available yet, they will be added
at a later point. For now, create a new file and paste the [reference config](../config/config.html)

```
vim rauthy/config.toml
```

**3. Access rights for the Database files**

The Rauthy container by default runs everything with user:group `10001:10001` for security
reasons.   
To make this work with the default values, you have 2 options:

- Change the access rights:

```
sudo chmod 0600 rauthy/config.toml
sudo chmod 0700 rauthy/data
sudo chown -R 10001:10001 rauthy
```

- The other solution, if you do not have sudo rights, would be to change the owner of the whole
  directory.

```
chmod a+w rauthy/data
```

This will make the directory writeable for everyone, so Rauthy can create the database files inside
the container with `10001:10001` again.

```admonish caution
The safest approach would be to change the owner and group for these files on the host system. This 
needs `sudo` to edit the config, which may be a bit annoying, but at the same time it makes sure, 
that you can only read the secrets inside it with `sudo` too.

You should avoid making Rauthy's data world-accessible at all cost. 
[Hiqlite](https://github.com/sebadob/hiqlite) will take care of this automatically during 
sub-directory creation, but the config includes sensitive information. 
```

**4. Adopt the config to your liking.**

Take a look at the [reference config](../config/config.html) and adopt everything to your needs, but
to not break this example, be sure to not change `cluster.data_dir`.

For an in-depth guide on a production ready config, check
the [Production Config](../config/production_config.md) section.

A **mandatory step** will be to generate proper encryption keys. Take a look
at [Encryption](../config/encryption.md).

**5. Start the container with volume mounts**

```
docker run -d \
    -v $(pwd)/rauthy/config.toml:/app/config.toml \
    -v $(pwd)/rauthy/data:/app/data \
    -p 8443:8443 \
    --name rauthy \
    ghcr.io/sebadob/rauthy:0.34.0
```

- `-v $(pwd)/rauthy/config.toml:/app/config.toml` mounts the config in the correct place
- `-v $(pwd)/rauthy/data:/app/data` mounts the volume for Hiqlite
- `-p 8443:8443` needs to match your configured `server.port_http` or `server.port_https` of course.
  If you use a reverse proxy inside a docker network, you don't need to expose any port, but you
  need to make sure to set `server.scheme = "http"`, `server.proxy_mode = true` and the correct
  value for `server.trusted_proxies`.

**6. You can now proceed with the [First Start](first_start.md) steps.**
