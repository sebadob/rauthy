use crate::app_state::DbPool;
use rauthy_common::constants::{DATABASE_URL, RAUTHY_VERSION};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use std::env;
use std::path::Path;
use time::OffsetDateTime;
use tokio::time::Instant;
use tracing::{debug, error, info, warn};

pub mod db_migrate;
pub mod db_migrate_dev;

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

    if DATABASE_URL.starts_with("sqlite:") {
        let q = format!("vacuum main into '{}/rauthy.db'", path_base);
        sqlx::query(&q)
            .execute(db)
            .await
            .expect("VACUUM INTO sqlite");
    } else if DATABASE_URL.starts_with("postgresql://") {
        warn!("Running on a Postgres database - automatic backups are not yet implemented");
    } else {
        panic!("Unknown DATABASE_URL");
    }

    // TODO when native backups are implemented, remove this approach -> not consistent
    // // iterate over all Cfs and store each into its own file
    // for cf in Cf::iter() {
    //     let res = DATA_STORE.scan_cf(cf.clone()).await?;
    //     let bytes = bincode::serialize(&res)?;
    //     let file = cf.as_str();
    //
    //     let path_str = format!("{}{}", path_base, file);
    //     let path = Path::new(&path_str);
    //     match tokio::fs::write(path, bytes).await {
    //         Ok(_) => debug!("Backup saved successfully to {}", path_str),
    //         Err(err) => {
    //             let msg = format!("Error saving backup to file {}: {}", path_str, err);
    //             error!("{}", msg);
    //             return Err(ErrorResponse::new(ErrorResponseType::Internal, msg));
    //         }
    //     }
    // }

    // TODO rename into metadata and provide information about the used database driver at that time
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
    let retention = match env::var("BACKUP_RETENTION_LOCAL")
        .unwrap_or_else(|_| "720".to_string())
        .parse::<u32>()
    {
        Ok(v) => v,
        Err(_) => {
            error!("Error parsing BACKUP_RETENTION_LOCAL to u64, using default of 720 hours");
            720
        }
    };
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
