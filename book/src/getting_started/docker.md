# Docker

## Testing / Local Evaluation

For getting a first look at Rauthy, you can start it with docker (or any other container runtime) on your localhost.
The image contains a basic default config which is sufficient for local testing (please don't use it in production).
Rauthy has pretty strict cookie settings and not all browsers treat `localhost` as being secure, therefore you should
allow insecure cookies for testing locally:

```
docker run --rm \
    -e COOKIE_MODE=danger-insecure \
    -p 8080:8080 \
    --name rauthy \
    ghcr.io/sebadob/rauthy:0.29.0
```

This will start the container in interactive mode with the [Hiqlite](https://github.com/sebadob/hiqlite) database. Just
take a look at the log at the logs to see the Account Dashboard URL and new admin password, to get access.

If you want to test a bit more in depth, you can keep the container between restarts:

```
docker run -d \
    -e COOKIE_MODE=danger-insecure \
    -p 8080:8080 \
    --name rauthy \
    ghcr.io/sebadob/rauthy:0.29.0
```

```admonish note
The second command does not start in interactive mode and it does not delete the container on exit.  
This means the data will be persisted, as long as the container itself is not erased and you can shutdown and
restart to your liking without using test data.
```

To see the logs and the new admin password, take a look with

```
docker logs -f rauthy
```

To delete the container, if you don't need it anymore, execute

```
docker stop rauthy && docker rm rauthy
```

To proceed, go to **[First Start](first_start.md)**, or do the production setup below to have persistence.

## Production Setup

For going to production or to test more in-depth, you need to apply a config that matches your environment.

The first thing you want to do is to add a volume mount for the database. The second thing is to provide a more
appropriate config.

Rauthy can either be configured via environment variables only, or you can provide a config file. It parses both, the
config first and any env var that is set will overwrite a possibly existing one from the config. You can add environment
variables to the startup command with the `-e` option.

```admonish note
The following commands will work on Linux and Mac OS (even though not tested on Mac OS). Since I am no Windows user 
myself, I cannot provide tested commands in this case.
```

**1. We want to create a new directory for Rauthy's persistent data**

```
mkdir -p rauthy/data
```

**2. Add the new config file.**

This documentation is in an early version and remote links are not available yet, they will be added at a later
point. For now, create a new file and paste the [reference config](../config/config.html)

```
vim rauthy/rauthy.cfg
```

**3. Access rights for the Database files**

The Rauthy container by default runs everything with user:group `10001:10001` for security reasons.  
To make this work with the default values, you have 2 options:

- Change the access rights:

```
sudo chmod 0600 rauthy/rauthy.cfg
sudo chmod 0700 rauthy/data
sudo chown -R 10001:10001 rauthy
```

- The other solution, if you do not have sudo rights, would be to change the owner of the whole directory.

```
chmod a+w rauthy/data
```

This will make the directory writeable for everyone, so Rauthy can create the database files inside the container
with `10001:10001` again.

```admonish caution
The safest approach would be to change the owner and group for these files on the host system. This needs `sudo`
to edit the config, which may be a bit annoying, but at the same time it makes sure, that you can only read
the secrets inside it with `sudo` too.

You should avoid making Rauthy's data world-accessible at all cost. [Hiqlite](https://github.com/sebadob/hiqlite) will
take care of this automatically during sub-directory creation, but the config includes sensitive information. 
```

**4. Adopt the config to your liking.**

Take a look at the [reference config](../config/config.html) and adopt everything to your needs, but to not break this
example, be sure to not change `HQL_DATA_DIR`.

For an in-depth guide on a production ready config, check the [Production Config](../config/production_config.md)
section.

A **mandatory step** will be to generate proper encryption keys. Take a look at [Encryption](../config/encryption.md).

**5. Start the container with volume mounts**

```
docker run -d \
    -v $(pwd)/rauthy/rauthy.cfg:/app/rauthy.cfg \
    -v $(pwd)/rauthy/data:/app/data \
    -p 8080:8080 \
    --name rauthy \
    ghcr.io/sebadob/rauthy:0.29.0
```

- `-v $(pwd)/rauthy/rauthy.cfg:/app/rauthy.cfg` makes sure to overwrite the testing config inside the container
- `-v $(pwd)/rauthy/data:/app/data` mounts the volume for Hiqlite
- `-p 8080:8080` needs to match your configured `LISTEN_PORT_HTTP` or `LISTEN_PORT_HTTPS` of course. If you use a
  reverse proxy inside a docker network, you don't need to expose any port.

**6. You can now proceed with the [First Start](first_start.md) steps.**
