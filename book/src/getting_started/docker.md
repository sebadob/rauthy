# Docker

## Testing / Local Evaluation

For getting a first look at Rauthy, you can start it with docker (or any other container runtime) on your localhost.
The image contains a basic default config which is sufficient for local testing. Rauthy has pretty strict cookie
settings and not all browsers treat `localhost` as being secure, therefore you should allow insecure cookies for
testing locally:

```
docker run --rm \
    -e COOKIE_MODE=danger-insecure \
    -p 8080:8080 \
    --name rauthy \
    ghcr.io/sebadob/rauthy:0.26.2-lite
```

This will start the container in interactive mode with an in-memory SQLite database. Just take a look at the log at the
logs to see the URL and first password.

If you want to test a bit more in depth, you can change to an on-disk database easily:

```
docker run -d \
    -e COOKIE_MODE=danger-insecure \
    -e DATABASE_URL=sqlite:data/rauthy.db \
    -p 8080:8080 \
    --name rauthy \
    ghcr.io/sebadob/rauthy:0.26.2-lite
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

To delete the container, if you do not need it anymore, execute

```
docker stop rauthy && docker rm rauthy
```

To proceed, go to **[First Start](first_start.md)**

## Production Setup

For going to production or to test more in-depth, you need to apply a config that matches your environment.

The first thing you might want to do is to add a volume mount for the database.  
The second thing is to provide a more custom config.

Rauthy can either be configured via environment variables only, or you can provide a config file.  
You can add environment variables to the startup command with the `-e` option, like shown in the on-disk SQLite
command.  
A better approach, when you have a bigger config file, would be to have all of them in a config file.

```admonish note
The following commands will work on Linux and Mac OS (even though not tested on Mac OS). If you are on Windows,
you might replace the `pwd` command and just paste in the path directly. Since I am no Windows user myself, I
cannot provide tested commands in this case.
```

**1. We want to create a new directory for rauthy's persistent data**

```
mkdir rauthy
```

**2. Add the new config file.**  
This documentation is in an early version and remote links are not available yet, they will be added at a later
point. For now, create a new file and paste the [reference config](../config/config.html)

```
vim rauthy/rauthy.cfg
```

**3. Create a sub-directory for the Database files**

```
mkdir rauthy/data
```

The rauthy container by default runs everything with user:group 10001:10001 for security reasons.  
To make this work with the default values, you have 2 options:

- Change the access rights:

```
chmod 0640 rauthy/rauthy.cfg
chmod 0700 -R rauthy/data
sudo chown -R 10001:10001 rauthy
```

- The other solution, if you do not have sudo rights, would be to change the owner of the whole directory.

```
chmod a+w rauthy/data
```

This will make the directory writeable for everyone, so rauthy can create the database files inside the container
with 10001:10001 again.

```admonish note
The safest approach would be to change the owner and group for these files on the host system. This needs `sudo`
to edit the config, which may be a bit annoying, but at the same time it makes sure, that you can only read
the secrets inside it with `sudo` too.
```

**4. Adopt the config to your liking.**  
Make sure to adjust the volume mount for the sqlite directory in step 5, if it differs from `sqlite:data/rauthy.db`

**5. Start the container with volume mounts**

```
docker run -d \
    -v $(pwd)/rauthy/rauthy.cfg:/app/rauthy.cfg \
    -v $(pwd)/rauthy/data:/app/data \
    -p 8080:8080 \
    --name rauthy \
    ghcr.io/sebadob/rauthy:0.26.2-lite
```

**6. Restrict DB files access even more**  
After rauthy has done the first start, you could harden the access rights of the SQLite files even more.  
This would make sure, that no one without `sudo` could just copy and read in the SQLite in some other place.
Just execute once more:

```
sudo chmod 0700 -R rauthy/data
```

**7. You can now proceed with the [First Start](first_start.md) steps.**
