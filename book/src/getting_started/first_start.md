# First Start

## Initial admin password and login

With the very first start of rauthy, or better with an empty database, when rauthy is starting, it
does not only create all the necessary schemas and initial data, but also some sensitive information
will be generated safely. This includes a set of Json Web Keys (JWKS) for the token signing and some
secrets.

The most important of these newly generated secrets is the default admin user's password.  
When this is securely generated with the very first start, it will be logged into the console. This
will only happen once and never again.

### Logs with docker

```
docker logs -f rauthy
```

### Logs with Kubernetes

```
kubectl -n rauthy logs -f rauthy-0
```

```admonish note
If you do a Kubernetes HA deployment directly, only the Pod `rauthy-0` will log the initial password.
```

```admonish note
If you missed this log entry, you will not be able to log in.  
If this is the case, you can delete the database / volume and just restart rauthy.
```

The log message contains a link to the accounts page, where you then should log in to immediately
set a new password. Follow the link, use as the default admin `admin@localhost` and as password the
copied value from the log.

- When logged into the account, click `EDIT` and `CHANGE PASSWORD` to set a new password
- Log out of the account and try to log in to the admin ui with the new password

## Custom rauthy admin user

It is a good idea, to either keep the `admin@localhost` and rename it (to never have a default
admin, which would be an attack vector) as a fallback user with just a very long password, or
disable it, after a custom admin has been added.

When logged in to the admin UI, you can add a new user. When the `SMTP` settings are correctly
configured in the config, which we can test right now, you will receive an E-Mail with the very
first password reset.

```admonish note
If you do not receive an E-Mail after the first user registration, chances are you may have a 
problem with the SMTP setup.
  
To debug this, you can set `LOG_LEVEL=debug` in the config and then watch the logs after a restart.
```

### `rauthy_admin` user role

The role, which allows a user to access the admin UI, is the `rauthy_admin`. \
If the user has this role assigned, he will be seen as an admin.

Under the hood, rauthy itself uses the OIDC roles and groups in the same way, as all clients would
do. This means you should not neither delete the `rauthy` default client, nor the `rauthy_admin`
role. There are mechanisms to prevent this from happening by accident via UI, but you could possibly
do this via a direct API call. \   
There are some anti-lockout mechanisms in place in the backend, which will be executed with every
start, but being careful at this point is a good idea anyway.
