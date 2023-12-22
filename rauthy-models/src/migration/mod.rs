use crate::app_state::DbPool;
use chrono::Utc;
use cryptr::stream::writer::s3_writer::{Bucket, Credentials, UrlStyle};
use cryptr::{EncValue, FileReader, S3Writer, StreamReader, StreamWriter};
use rauthy_common::constants::{DATABASE_URL, DB_TYPE, RAUTHY_VERSION};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::DbType;
use rusty_s3::actions::ListObjectsV2;
use rusty_s3::S3Action;
use std::env;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::time::Instant;
use tracing::{debug, error, info, warn};

pub mod db_migrate;
pub mod db_migrate_dev;

static BUCKET: OnceLock<Bucket> = OnceLock::new();
static CREDENTIALS: OnceLock<Credentials> = OnceLock::new();
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

    let backup_file_path = format!("{}/rauthy.db", path_base);
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
    let credentials = CREDENTIALS
        .get()
        .expect("CREDENTIALS to be set up correctly");
    let danger_accept_invalid_certs = ACCEPT_INVALID_CERTS
        .get()
        .expect("ACCEPT_INVALID_CERTS to be set up correctly");

    // execute backup
    let reader = StreamReader::File(FileReader {
        path: file_path,
        print_progress: false,
    });

    let object = format!(
        "rauthy-{}-{}.cryptr",
        RAUTHY_VERSION,
        Utc::now().timestamp()
    );
    let writer = StreamWriter::S3(S3Writer {
        credentials: Some(credentials),
        bucket,
        object: &object,
        danger_accept_invalid_certs: *danger_accept_invalid_certs,
    });

    info!("Pushing backup to S3 storage {}", bucket.region());
    EncValue::encrypt_stream(reader, writer).await?;
    info!("S3 backup push successful");

    Ok(())
}

/// Initializes and tests the connection for S3 backups, if configured.
/// This will panic if anything is not configured correctly to avoid unexpected behavior at runtime.
pub async fn s3_backup_init_test() {
    let s3_url = match env::var("S3_URL") {
        Ok(url) => url,
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
    let use_path_style = env::var("S3_PATH_STYLE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .expect("Cannot parse S3_PATH_STYLE to bool\n");
    let bucket = env::var("S3_BUCKET").expect("Found S3_URL but no S3_BUCKET\n");
    let access_key = env::var("S3_ACCESS_KEY").expect("Found S3_URL but no S3_ACCESS_KEY\n");
    let secret = env::var("S3_ACCESS_SECRET").expect("Found S3_URL but no S3_ACCESS_SECRET\n");
    let danger_accept_invalid_certs = env::var("S3_DANGER_ACCEPT_INVALID_CERTS")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .expect("Cannot parse S3_DANGER_ACCEPT_INVALID_CERTS to bool\n");

    let credentials = Credentials::new(access_key, secret);
    let path_style = if use_path_style {
        UrlStyle::Path
    } else {
        UrlStyle::VirtualHost
    };
    info!("S3 backups are configured for '{}'", region);
    let bucket = Bucket::new(
        s3_url.parse().expect("Invalid format for S3_URL"),
        path_style,
        bucket,
        region,
    )
    .expect("Cannot build S3 Bucket object from given configuration");

    // test the connection to be able to panic early
    let action = ListObjectsV2::new(&bucket, Some(&credentials)).sign(Duration::from_secs(10));
    let client = if danger_accept_invalid_certs {
        cryptr::stream::http_client_insecure()
    } else {
        cryptr::stream::http_client()
    };
    match client.get(action).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                info!("S3 connection test was successful");
            } else {
                let body = resp.text().await.unwrap_or_default();
                panic!(
                    "\nCannot connect to S3 storage - check your configuration and access rights\n\n{}\n",
                    body
                );
            }
        }
        Err(err) => {
            panic!("Cannot connect to S3 storage: {}", err);
        }
    }

    // save values for backups
    BUCKET.set(bucket).expect("to set BUCKET only once");
    CREDENTIALS
        .set(credentials)
        .expect("to set CREDENTIALS only once");
    ACCEPT_INVALID_CERTS
        .set(danger_accept_invalid_certs)
        .expect("to set ACCEPT_INVALID_CERTS only once");
}

// Important: This must be executed BEFORE the rauthy store has been opened in `main`
pub async fn restore_local_backup() -> Result<(), ErrorResponse> {
    error!("Backup restores are not yet adopted for the new database drivers - exiting");

    // let start = Instant::now();
    //
    // debug!("Checking for backups to apply");
    // let path_base = "data/backup/APPLY";
    // let apply_path = Path::new(path_base);
    // let apply_res = tokio::fs::read_to_string(apply_path).await;
    // if apply_res.is_err() {
    //     debug!("No backups to apply");
    //     return Ok(());
    // }
    // let apply = apply_res.unwrap();
    // let backup_folder = apply.trim();
    //
    // let path_base = format!("data/backup/{}/", backup_folder);
    // info!(
    //     "Starting full store restore from local backup: {}",
    //     path_base
    // );
    //
    // // check the rauthy version from the backup for compatibility
    // let path_str = format!("{}RAUTHY_VERSION.txt", path_base);
    // let path = Path::new(&path_str);
    // let version = tokio::fs::read_to_string(path).await.map_err(|err| {
    //     ErrorResponse::new(
    //         ErrorResponseType::Internal,
    //         format!("Could not read in the {}: {}", path_str, err),
    //     )
    // })?;
    //
    // let regex = Regex::new(r"^0.[10-999].*$").unwrap();
    // if !regex.is_match(&version) {
    //     return Err(ErrorResponse::new(
    //         ErrorResponseType::BadRequest,
    //         "The backup was created with an incompatible version of rauthy".to_string(),
    //     ));
    // }
    //
    // // we are good to go - purge everything
    // purge_store().await;
    //
    // // apply base migration to have all necessary Cfs created again
    // Migrations::migrate_internal(StoreMode::Prod).await;
    //
    // // read in all files and PUT them into the store
    // for cf in Cf::iter() {
    //     let path_str = format!("{}{}", path_base, cf.as_str());
    //     let path = Path::new(&path_str);
    //     let bytes = tokio::fs::read(path).await.map_err(|err| {
    //         ErrorResponse::new(
    //             ErrorResponseType::Internal,
    //             format!("Error reading file {}: {}", path_str, err),
    //         )
    //     })?;
    //
    //     let columns = bincode::deserialize::<Vec<(String, Vec<u8>)>>(&bytes)?;
    //     for (k, v) in columns {
    //         DATA_STORE.put(cf.clone(), k, v).await?;
    //     }
    // }
    //
    // // delete the APPLY file to not do the same at the next restart
    // let _ = tokio::fs::remove_file(apply_path).await;
    //
    // info!(
    //     "Full store restore finished in {} ms",
    //     start.elapsed().as_millis()
    // );
    Ok(())
}
