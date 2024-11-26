# Backups

## Postgres

If you are using Postgres as the main database, Rauthy does not do any backups.  
There are a lot of way better tools out there to handle this task.

## Hiqlite

If Rauthy is using Hiqlite, it does automatic backups, which can be configured with:

```
# When the auto-backup task should run.
# Accepts cron syntax:
# "sec min hour day_of_month month day_of_week year"
# default: "0 30 2 * * * *"
HQL_BACKUP_CRON="0 30 2 * * * *"

# Local backups older than the configured days will be cleaned up after
# the backup cron job.
# default: 30
#HQL_BACKUP_KEEP_DAYS=30

# Backups older than the configured days will be cleaned up locally
# after each `Client::backup()` and the cron job `HQL_BACKUP_CRON`.
# default: 3
#HQL_BACKUP_KEEP_DAYS_LOCAL=3
```

All these backups are written inside the pod / container into `data/state_machine/backups`.

This difference makes it possible, that you could add a second volume mount to the container.  
You then have the database itself on a different disk than the backups, which is the most simple and straight forward
approach to have a basic backup strategy. However, it is recommended to use S3 for backups, especially for HA
deployments.

## Remote Backups to S3 Storage

Hiqlite backups can be pushed to an S3 bucket after creation. This way, you can keep only very low amount of local
backups and older ones on cheaper object storage.

Rauthy has been tested against MinIO and Garage S3 storage and is working fine with both, so I expect and standard S3
API to just work out of the box. You need to provide an Access Key + Secret with write access to an existing bucket
and Rauthy will take care of the rest. All backups pushed to S3 will automatically encrypted with the currently active
`ENC_KEY_ACTIVE` from the Rauthy config.

The configuration is done with the following values:

```
# Access values for the S3 bucket where backups will be pushed to.
#HQL_S3_URL=https://s3.example.com
#HQL_S3_BUCKET=my_bucket
#HQL_S3_REGION=example
#HQL_S3_PATH_STYLE=true
#HQL_S3_KEY=s3_key
#HQL_S3_SECRET=s3_secret
```

## Disaster Recovery

If you really lost all your data, you can easily restore automatically from the latest backup. This works with either a
local `file` backup or with an encrypted remote backup on `s3` storage (as long as you still have the `ENC_KEY_ACTIVE`
that has been used for the remote backup).  
This, again, works only for Hiqlite. When you are using Postgres, you should use Postgres native tooling like
`pgBackRest` which is way better at this.

The process is really simple:

1. Have the cluster shut down. This is probably the case anyway, if you need to restore from a backup.
2. Provide a backup file name on S3 storage with the `HQL_BACKUP_RESTORE` value with prefix `s3:` (encrypted), or a file
   on disk (plain sqlite file) with the prefix `file:`.
3. Start up Rauthy
4. Check the logs and wait for the backup to be finished
5. After a successful restore, Rauthy will start its normal operation
6. Make sure to remove the HQL_BACKUP_RESTORE env value.

```admonish danger 
After a successful restore, you MUST remove the env var again!  
If you don't do it, Rauthy will re-apply the same backup with each following restart over and over again.
```

You only need to set this single value:

```
# If you ever need to restore from a backup, the process is simple.
# 1. Have the cluster shut down. This is probably the case anyway, if
#    you need to restore from a backup.
# 2. Provide the backup file name on S3 storage with the
#    HQL_BACKUP_RESTORE value.
# 3. Start up the cluster again.
# 4. After the restart, make sure to remove the HQL_BACKUP_RESTORE
#    env value.
#HQL_BACKUP_RESTORE=
```
