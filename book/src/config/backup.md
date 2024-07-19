# Backups

## Postgres

If you are using Postgres as the main database, Rauthy does not do any backups.  
There are a lot of way better tools out there to handle this task.

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

## Remote Backups to S3 Storage

SQLite backups can be pushed to an S3 bucket after creation. This way you can keep only very low amount of local
backups and older ones on cheaper object storage.

Rauthy has been tested against MinIO and Garage S3 storage and is working fine with both, so I expect and standard S3
API to just work out of the box. You need to provide an Access Key + Secret with write access to an existing bucket
and Rauthy will take care of the rest. All backups pushed to S3 will automatically encrypted with the currently active
`ENC_KEY_ACTIVE` from the Rauthy config.

The configuration is done with the following values:

```
# The following section will only be taken into account, when
# SQLite is used as the main database. If you use Postgres, you
# should use Postgres native tooling like for instance `pgbackrest`
# to manage your backups.
# If S3 access is configured, your SQLite backups will be encrypted
# and pushed into the configured bucket.
#S3_URL=
#S3_REGION=
#S3_PATH_STYLE=false
#S3_BUCKET=my_s3_bucket_name
#S3_ACCESS_KEY=
#S3_ACCESS_SECRET=
#S3_DANGER_ACCEPT_INVALID_CERTS=false
```

## Disaster Recovery

If you really lost all your data, you can easily restore automatically from the latest backup. This works with either a
local `file` backup or with an encrypted remote backup on `s3` storage (as long as you still have the `ENC_KEY_ACTIVE`
that has been used for the remote backup).  
This, again, works only for SQLite. When you are using Postgres, you really should use native tooling which is way
better at this.

The process is really simple:

- set an environment variable before the start
- start up Rauthy
- check the logs and wait for the backup to be finished
- after a successful restore, Rauthy will start its normal operation

```admonish danger 
After a successful restore, you MUST remove the env var again!  
If you don't do it, Rauthy will re-apply the same backup with the next restart.
```

You only need to set this single value:

```
# Restores the given backup
#
# CAUTION: Be very careful with this option - it will overwrite
# any existing database! The best way to use this option is to
# provide it as an environment variable for a single start up
# and then remove it directly after success.
#
# This only works when you are using a SQLite database!
# If you are running on Postgres, you must use Postgres-native
# tooling to handle your backups.
#
# You can either restore a local backup, or an encrypted one
# from S3 storage.
#
# For restoring from a local backup, provide the folder name
# of the backup you want to restore. Local SQLite backups are
# always in `./data/backup/rauthy-backup-TIMESTAMP/` folders.
# You only provide the backup folder name itself, in this case
# it would be `rauthy-backup-TIMESTAMP` like this:
# RESTORE_BACKUP=file:rauthy-backup-TIMESTAMP
#
# If you want to restore an encrypted backup from S3 storage,
# you must provide the object name in the configured bucket.
# For instance, let's say we have an object named
# `rauthy-0.20.0-1703243039.cryptr` in our bucket, then the
# format would be:
# RESTORE_BACKUP=s3:rauthy-0.20.0-1703243039.cryptr
#
#RESTORE_BACKUP=
```
