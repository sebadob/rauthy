# Database Migrations

You can migrate easily between SQLite and Postgres, or just between different instances of them.

Let's say you started out by evaluating Rauthy with a SQLite and a single instance deployment. Later on, you want to
migrate to a HA setup, which requires you to use a Postgres.

**Solution:** `MIGRATE_DB_FROM`

If you set the `MIGRATE_DB_FROM` in Rauthy's config, it will perform a migration at the next restart.  
The way it works is the following:

1. At startup, have a look if `MIGRATE_DB_FROM` is configured
2. If yes, then connect to the given database
3. At the same time, connect to the database specified in the `DATABASE_URL`
4. Overwrite all existing data in `DATABASE_URL` with the data from the `MIGRATE_DB_FROM` database
5. Close the connection to `MIGRATE_DB_FROM`
6. Use the `DATABASE_URL` as the new database and start normal operation

```admonish danger
`MIGRATE_DB_FROM` overwrites any data in the target database! Be very careful with this option.

If you do not remove the `MIGRATE_DB_FROM` after the migration has been done, it will overwrite the target again with
the next restart of the application. Remove the config variable immediately after the migration has finished.
```

```admonish hint
The easiest to do is to just set `MIGRATE_DB_FROM` as an environmant variable, which is easier and quicker to remove 
again afterwards. 
```

```admonish info
In some cases and specific environments, this feature is currently a bit bugged. The problems are hard or almost
impossible to reproduce. If you encounter a problem using this, please let me know as much details about it as
possible.
```
