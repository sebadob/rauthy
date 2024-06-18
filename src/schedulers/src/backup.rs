use crate::sleep_schedule_next;
use rauthy_common::constants::DB_TYPE;
use rauthy_common::DbType;
use rauthy_models::app_state::DbPool;
use rauthy_models::migration::backup_db;
use std::env;
use std::str::FromStr;
use tracing::{debug, error, info};

/// Creates a backup of the data store
pub async fn db_backup(db: DbPool) {
    if *DB_TYPE == DbType::Postgres {
        debug!("Using Postgres as the main database - automatic backups disabled");
        return;
    }

    let mut cron_task = env::var("BACKUP_TASK").unwrap_or_else(|_| "0 0 4 * * * *".to_string());

    // sec min hour day_of_month month day_of_week year
    let schedule = cron::Schedule::from_str(&cron_task).unwrap_or_else(|err| {
        error!(
            "Error creating a cron scheduler with the given BACKUP_TASK input: {} - using default \"0 0 4 * * * *\": {}",
            cron_task, err
        );
        cron_task = "0 0 4 * * * *".to_string();
        cron::Schedule::from_str(&cron_task).unwrap()
    });

    info!("Database backups are scheduled for: {}", cron_task);

    loop {
        sleep_schedule_next(&schedule).await;
        debug!("Running db_backup scheduler");

        // VACUUM main INTO 'data/rauthy.db.backup-'

        if let Err(err) = backup_db(&db).await {
            error!("{}", err.message);
        }
    }
}
