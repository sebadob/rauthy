# Backups

## SQLite

If Rauthy is using a SQLite, it does automatic backups, which can be configured with:

```
# Cron job for automatic data store backups (default: "0 0 4 * * * *")
# sec min hour day_of_month month day_of_week year
BACKUP_TASK="0 0 4 * * * *"

# The name for the data store backups. The current timestamp will always be appended automatically.
# default: rauthy-backup-
BACKUP_NAME="rauthy-backup-"

# All backups older than the specified hours will be cleaned up automatically (default: 720)
BACKUP_RETENTION_LOCAL=24
```

All these backups are written inside the pod / container into `/app/data/backup`.  
The database itself will be saved in `/app/data` by default.

This difference makes it possible, that you could add a second volume mount to the container.  
You then have the database itself on a different disk than the backups, which is the most simple and straight forward
approach to have a basic backup strategy.

```admonish info
The SQLite backups are done with `VACUUM`, which means you can just use the backups as a normal database again.
This makes it possible, to just use the [Database Migration](./db_migration.md) feature to apply backups very easily.
```

## Postgres

If you are using Postgres as the main database, Rauthy does not do any backups.  
There are a lot of way better tools out there to handle this task.
