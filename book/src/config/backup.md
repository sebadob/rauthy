# Backups

## Postgres

If you are using Postgres as the main database, Rauthy does not do any backups.  
There are a lot of way better tools out there to handle this task.

## Hiqlite

If Rauthy is using Hiqlite, it does automatic backups, which can be configured with:

```toml
[cluster]
# When the auto-backup task should run.
# Accepts cron syntax:
# "sec min hour day_of_month month day_of_week year"
#
# default: "0 30 2 * * * *"
# overwritten by: HQL_BACKUP_CRON
backup_cron = "0 30 2 * * * *"

# Backups older than the configured days will be cleaned up on S3
# after the backup cron job `backup_cron`.
#
# default: 30
# overwritten by: HQL_BACKUP_KEEP_DAYS
backup_keep_days = 30

# Backups older than the configured days will be cleaned up locally
# after each `Client::backup()` and the cron job `HQL_BACKUP_CRON`.
#
# default: 3
# overwritten by: HQL_BACKUP_KEEP_DAYS_LOCAL
backup_keep_days_local = 3
```

All these backups are written inside the pod / container into `data/state_machine/backups`.

This difference makes it possible, that you could add a second volume mount to the container.  
You then have the database itself on a different disk than the backups, which is the most simple and
straight forward
approach to have a basic backup strategy. However, it is recommended to use S3 for backups,
especially for HA
deployments.

## Remote Backups to S3 Storage

Hiqlite backups can be pushed to an S3 bucket after creation. This way, you can keep only very low
amount of local
backups and older ones on cheaper object storage.

Rauthy has been tested against MinIO and Garage S3 storage and is working fine with both, so I
expect and standard S3
API to just work out of the box. You need to provide an Access Key + Secret with write access to an
existing bucket
and Rauthy will take care of the rest. All backups pushed to S3 will automatically encrypted with
the currently active
`encryption.key_active` from the Rauthy config.

The configuration is done with the following values:

```toml
[cluster]
# Access values for the S3 bucket where backups will be pushed to.
# overwritten by: HQL_S3_URL
s3_url = "https://s3.example.com"
# overwritten by: HQL_S3_BUCKET
s3_bucket = "my_bucket"
# overwritten by: HQL_S3_REGION
s3_region = "my_region"
# overwritten by: HQL_S3_PATH_STYLE
s3_path_style = true
# overwritten by: HQL_S3_KEY
s3_key = "my_key"
# overwritten by: HQL_S3_SECRET
s3_secret = "my_secret"
```

## Disaster Recovery

If you really lost all your data, you can easily restore automatically from the latest backup. This
works with either a
local `file` backup or with an encrypted remote backup on `s3` storage (as long as you still have
the `ENC_KEY_ACTIVE`
that has been used for the remote backup).  
This, again, works only for Hiqlite. When you are using Postgres, you should use Postgres native
tooling like
`pgBackRest` which is way better at this.

The process is really simple:

1. Have the cluster shut down. This is probably the case anyway, if you need to restore from a
   backup.
2. Provide a backup file name on S3 storage with the `HQL_BACKUP_RESTORE` ENV value with prefix
   `s3:object_name` (encrypted), or a file on disk (plain sqlite file) with the prefix
   `file:/path/to/backup`.
3. Start up Rauthy
4. Check the logs and wait for the backup to be finished
5. After a successful restore, Rauthy will start its normal operation
6. Make sure to remove the `HQL_BACKUP_RESTORE` env value.

```admonish danger 
After a successful restore, you MUST remove the env var again!  
If you don't do it, Rauthy will re-apply the same backup with each following restart over and over again.
```

You only need to set this single value:

```toml
# If you ever need to restore from a backup, the process is simple.
# 1. Have the cluster shut down. This is probably the case anyway, if
#    you need to restore from a backup.
# 2. Provide the backup file name on S3 storage with the
#    HQL_BACKUP_RESTORE value.
# 3. Start up the cluster again.
# 4. After the restart, make sure to remove the HQL_BACKUP_RESTORE
#    env value.
HQL_BACKUP_RESTORE = ""
```
