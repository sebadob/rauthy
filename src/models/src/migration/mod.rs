use crate::app_state::DbPool;
use chrono::Utc;
use cryptr::stream::s3::{
    AccessKeyId, AccessKeySecret, Bucket, BucketOptions, Credentials, Region,
};
use cryptr::{EncValue, FileReader, FileWriter, S3Reader, S3Writer, StreamReader, StreamWriter};
use rauthy_common::constants::{DATABASE_URL, DB_TYPE, RAUTHY_VERSION};
use rauthy_common::DbType;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use sqlx::pool::PoolOptions;
use std::env;
use std::path::Path;
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::fs;
use tokio::time::Instant;
use tracing::{debug, error, info, warn};

pub mod db_migrate;
pub mod db_migrate_dev;

static BUCKET: OnceLock<Bucket> = OnceLock::new();
static ACCEPT_INVALID_CERTS: OnceLock<bool> = OnceLock::new();

pub async fn backup_db(db: &DbPool) -> Result<(), ErrorResponse> {
    let start = Instant::now();
    info!("Starting database backup");

    // make sure the backups folder does exist
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let backup_name = env::var("BACKUP_NAME").unwrap_or_else(|_| String::from("rauthy-backup-"));
    let path_base = format!("data/backup/{}{}/", backup_name, now);
    tokio::fs::create_dir_all(&path_base).await.map_err(|e| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Error creating Backups folder path: {:?}", e),
        )
    })?;

    let backup_file_path = format!("{}rauthy.db", path_base);
    if DATABASE_URL.starts_with("sqlite:") {
        let q = format!("VACUUM main INTO '{}'", backup_file_path);
        sqlx::query(&q)
            .execute(db)
            .await
            .expect("VACUUM INTO sqlite");
    } else if DATABASE_URL.starts_with("postgresql://") {
        debug!("Running on a Postgres database - use native tools to handle backups");
        return Ok(());
    } else {
        panic!("Unknown DATABASE_URL");
    }

    // when everything was successful, store a version information
    let path_str = format!("{}RAUTHY_VERSION.txt", path_base);
    let path = Path::new(&path_str);
    match tokio::fs::write(path, RAUTHY_VERSION).await {
        Ok(_) => debug!("RAUTHY_VERSION appended to full store backup"),
        Err(err) => {
            let msg = format!("Error saving RAUTHY_VERSION to file {}: {}", path_str, err);
            error!("{}", msg);
            return Err(ErrorResponse::new(ErrorResponseType::Internal, msg));
        }
    }

    // TODO encrypt and push backup to S3 storage
    s3_backup(&backup_file_path).await?;

    // cleanup old backups
    let path_base = "data/backup/";
    let path = Path::new(&path_base);
    let mut all = tokio::fs::read_dir(path).await.map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Error reading content of data/backup/ for cleanup: {}", err),
        )
    })?;

    let ts_low = 1670000000u32;
    let ts_high = 3000000000u32;
    let retention = env::var("BACKUP_RETENTION_LOCAL")
        .unwrap_or_else(|_| "720".to_string())
        .parse::<u32>()
        .unwrap_or_else(|_| {
            error!("Error parsing BACKUP_RETENTION_LOCAL to u64, using default of 720 hours");
            720
        });
    let ts_now = now as u32;
    let max_diff = retention * 60 * 60;

    while let Ok(Some(entry)) = all.next_entry().await {
        let name = entry.file_name();
        let s = name.to_str().unwrap();
        let (_, stripped) = s.split_at(s.len() - 10);
        if let Ok(ts) = stripped.parse::<u32>() {
            if ts > ts_low && ts < ts_high && (ts_now - ts) > max_diff {
                debug!("Cleaning up backup {}", s);
                let p = format!("{}{}", path_base, s);
                let _ = tokio::fs::remove_dir_all(p).await;
            }
        }
    }

    info!(
        "Database backup finished in {} ms",
        start.elapsed().as_millis()
    );
    Ok(())
}

async fn s3_backup(file_path: &str) -> Result<(), ErrorResponse> {
    let bucket = match BUCKET.get() {
        None => {
            return Ok(());
        }
        Some(b) => b,
    };
    // execute backup
    let reader = StreamReader::File(FileReader {
        path: file_path,
        print_progress: true,
    });

    let object = format!(
        "rauthy-{}-{}.cryptr",
        RAUTHY_VERSION,
        Utc::now().timestamp()
    );
    let writer = StreamWriter::S3(S3Writer {
        bucket,
        object: &object,
    });

    info!(
        "Pushing backup to S3 storage {} / {}",
        bucket.name,
        bucket.region.as_str()
    );
    EncValue::encrypt_stream(reader, writer).await?;
    info!("S3 backup push successful");

    Ok(())
}

/// Initializes and tests the connection for S3 backups, if configured.
/// This will panic if anything is not configured correctly to avoid unexpected behavior at runtime.
pub async fn s3_backup_init_test() {
    let s3_url = match env::var("S3_URL") {
        Ok(url) => {
            if url.is_empty() && *DB_TYPE == DbType::Sqlite {
                info!("S3 backups are not configured, 'S3_URL' is empty");
                return;
            }
            url
        }
        Err(_) => {
            if *DB_TYPE == DbType::Sqlite {
                info!("S3 backups are not configured, 'S3_URL' not found");
            }
            return;
        }
    };
    if *DB_TYPE == DbType::Postgres {
        warn!(
            r#"

    Found S3 config. This will be ignored, since you are using a Postgres.
    Postgres backups must be managed in a Postgres-native way with proper tooling like for instance pgbackrest
    "#
        );
        return;
    }

    // read env vars
    let region = env::var("S3_REGION").expect("Found S3_URL but no S3_REGION\n");
    let path_style = env::var("S3_PATH_STYLE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .expect("Cannot parse S3_PATH_STYLE to bool\n");
    let bucket = env::var("S3_BUCKET").expect("Found S3_URL but no S3_BUCKET\n");
    let access_key = env::var("S3_ACCESS_KEY").expect("Found S3_URL but no S3_ACCESS_KEY\n");
    let secret = env::var("S3_ACCESS_SECRET").expect("Found S3_URL but no S3_ACCESS_SECRET\n");

    // TODO deprecate and remove after v0.24
    let danger_accept_invalid_certs = env::var("S3_DANGER_ACCEPT_INVALID_CERTS")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .expect("Cannot parse S3_DANGER_ACCEPT_INVALID_CERTS to bool\n");

    if danger_accept_invalid_certs {
        warn!(
            "`S3_DANGER_ACCEPT_INVALID_CERTS` has been deprecated and renamed to \
        `S3_DANGER_ALLOW_INSECURE`. You need to adopt your config for versions afters v0.24"
        );
        env::set_var("S3_DANGER_ALLOW_INSECURE", "true");
    }

    info!("S3 backups are configured for '{}'", region);
    let options = BucketOptions {
        path_style,
        list_objects_v2: true,
    };
    let bucket = Bucket::new(
        s3_url.parse().expect("Invalid format for S3_URL"),
        bucket,
        Region(region),
        Credentials {
            access_key_id: AccessKeyId(access_key),
            access_key_secret: AccessKeySecret(secret),
        },
        Some(options),
    )
    .expect("Cannot build S3 Bucket object from given configuration");

    // test the connection
    if let Err(err) = bucket.head("").await {
        error!(
            "Error connecting to configured S3 bucket {}: {}",
            bucket.name, err
        );
    }

    // save values for backups
    let _ = BUCKET.set(bucket);
    let _ = ACCEPT_INVALID_CERTS.set(danger_accept_invalid_certs);
}

/// Important: Must be executed BEFORE any database connection has been opened
pub async fn check_restore_backup() -> Result<(), ErrorResponse> {
    let backup_name = match env::var("RESTORE_BACKUP").ok() {
        None => return Ok(()),
        Some(name) => {
            if name.is_empty() {
                return Ok(());
            }
            name
        }
    };

    if *DB_TYPE == DbType::Postgres {
        error!(
            r#"

Found a backup which should be restored: {}
You are using a Postgres database - this only works when using SQLite!
Please use Postgres-native tooling like `pgbackrest` for Postgres.
"#,
            backup_name
        );
        panic!();
    }

    warn!(
        r#"

Found a backup which should be restored: {}
This will override any existing database!
Proceeding in 10 seconds ...
"#,
        backup_name
    );
    tokio::time::sleep(Duration::from_secs(10)).await;

    // mv current DB file
    let db_url = DATABASE_URL.as_str()[7..].to_string();
    let restore_path = format!("{}.restore", &db_url);

    // copy backup in place
    if let Some(folder) = backup_name.strip_prefix("file:") {
        let backup_path = format!("data/backup/{}/rauthy.db", folder.trim_end());
        fs::copy(&backup_path, &restore_path).await?;
    } else if let Some(object) = backup_name.strip_prefix("s3:") {
        s3_backup_init_test().await;

        let reader = StreamReader::S3(S3Reader {
            bucket: BUCKET.get().unwrap(),
            object: object.trim_end(),
            print_progress: true,
        });
        let writer = StreamWriter::File(FileWriter {
            path: &restore_path,
            overwrite_target: true,
        });
        EncValue::decrypt_stream(reader, writer).await?;
    } else {
        error!("\nInvalid format for RESTORE_BACKUP. Please check the documentation.\n");
        panic!();
    }

    // move the current file out of the way and store it temp
    let current_bkp = format!("{}.bkp", db_url);
    let _ = fs::copy(&db_url, &current_bkp).await;
    let current_shm = format!("{}-shm", db_url);
    let current_shm_bkp = format!("{}.bkp", current_shm);
    let _ = fs::copy(&current_shm, &current_shm_bkp).await;
    let current_wal = format!("{}-wal", db_url);
    let current_wal_bkp = format!("{}.bkp", current_wal);
    let _ = fs::copy(&current_wal, &current_wal_bkp).await;

    let _ = fs::remove_file(&db_url).await;
    let _ = fs::remove_file(&current_shm).await;
    let _ = fs::remove_file(&current_wal).await;

    // switch places with the restored file
    fs::copy(restore_path, &db_url).await?;

    // check file integrity and try to open DB
    let opts = sqlx::sqlite::SqliteConnectOptions::from_str(DATABASE_URL.as_str())?
        .foreign_keys(true)
        .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
    match PoolOptions::<sqlx::Sqlite>::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(opts)
        .await
    {
        Ok(_pool) => {
            info!("Successfully connected to restored backup database");
        }
        Err(err) => {
            error!(
                "Error connecting to restored backup database:\n{}\n\nRestoring original file ...",
                err
            );
            // copy back the original file
            fs::copy(current_bkp, &db_url).await?;
            fs::copy(current_shm_bkp, current_shm).await?;
            fs::copy(current_wal_bkp, current_wal).await?;
            panic!();
        }
    }

    // when the restored file is in place - rm old DB file
    info!("Cleaning up after backup restore");
    let _ = fs::remove_file(&current_bkp).await;
    let _ = fs::remove_file(&current_shm_bkp).await;
    let _ = fs::remove_file(&current_wal_bkp).await;

    info!("Database backup has been restored successfully");

    Ok(())
}
