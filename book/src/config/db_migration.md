# Database Migrations

You can migrate easily between Hiqlite and Postgres, or just between different instances of the same database.

Let's say you started out by evaluating Rauthy with the default Hiqlite and a single instance deployment. Later on, you
want to migrate to Postgres for whatever reason. Or you started with Postgres and yuo want to reduce your memory
footprint by switching to Hiqlite. All of this is easily possible.

**Solution:** `MIGRATE_DB_FROM`

If you set the `MIGRATE_DB_FROM` ENV var, it will perform a migration at the next restart.   
The way it works is the following:

1. At startup, have a look if `MIGRATE_DB_FROM` is given
2. If yes, then connect to the given database
3. At the same time, connect to the database specified via the normal config values
4. Overwrite all existing data in the target database with the data from the `MIGRATE_DB_FROM` source
5. Close the connection to `MIGRATE_DB_FROM`
6. Start normal operation

```admonish danger
`MIGRATE_DB_FROM` overwrites any data in the target database! Be very careful with this option.

If you do not remove the `MIGRATE_DB_FROM` after the migration has been done, it will overwrite the target again with
the next restart of the application. Remove the config variable immediately after the migration has finished.
```
