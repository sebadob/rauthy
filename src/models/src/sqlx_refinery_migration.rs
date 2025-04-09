use crate::database::DB;
use chrono::{DateTime, Utc};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::base64_decode;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use tracing::{debug, info};

/// This file exists only temporary during Rauth v0.29 to make the migration from sqlx to refinery
/// work seamlessly without any user action required.
/// TODO remove this file as soon as we are after v0.29
pub async fn migrate_sqlx_to_refinery() -> Result<(), ErrorResponse> {
    if !is_hiqlite() && is_existing_sqlx_database().await == Ok(true) {
        info!("Migration from old '_sqlx_migrations' to 'refinery_schema_history'");

        // insert the first refinery migration which exactly matches the state of 0.28.3
        let mut cl = DB::pg().await?;
        let txn = cl.transaction().await?;

        DB::pg_txn_append(
            &txn,
            r#"
create table refinery_schema_history
(
    version    integer not null
        primary key,
    name       varchar(255),
    applied_on varchar(255),
    checksum   varchar(255)
);"#,
            &[],
        )
        .await?;
        DB::pg_txn_append(
            &txn,
            r#"
INSERT INTO refinery_schema_history (version, name, applied_on, checksum)
VALUES (1, 'init_schema', '2025-04-09T13:11:18.383590433Z', '4468264877242552873')"#,
            &[],
        )
        .await?;
        DB::pg_txn_append(&txn, "DROP TABLE _sqlx_migrations", &[]).await?;

        txn.commit().await?;
    }

    Ok(())
}

#[derive(Debug)]
struct SqlxMigration {
    version: i64,
    description: String,
    #[allow(dead_code)]
    installed_on: DateTime<Utc>,
    #[allow(dead_code)]
    success: bool,
    checksum: Vec<u8>,
    #[allow(dead_code)]
    execution_time: i64,
}

impl From<tokio_postgres::Row> for SqlxMigration {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            version: row.get("version"),
            description: row.get("description"),
            installed_on: row.get("installed_on"),
            success: row.get("success"),
            checksum: row.get("checksum"),
            execution_time: row.get("execution_time"),
        }
    }
}

/// Checks if the currently used Postgres database is already set up for `sqlx` from before the
/// `tokio-postgres` migration.
async fn is_existing_sqlx_database() -> Result<bool, ErrorResponse> {
    let mut rows = DB::pg_query::<SqlxMigration>("SELECT * FROM _sqlx_migrations", &[], 0).await?;
    debug!("rows from pg_query!\n{:?}", rows);

    // in v0.28, migration no 32 is `pictures`
    if rows.len() < 32 {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "invalid database version, expected at least the state of Rauthy 0.28",
        ));
    }
    assert_eq!(rows.len(), 32);

    // make sure that no 32 is actually a correctly applied `pictures` migration
    let pictures = rows.swap_remove(31);
    debug!("applied pictures migration:\n{pictures:?}");
    assert_eq!(pictures.version, 32);
    assert_eq!(&pictures.description, "pictures");
    // This is the expected checksum of an already applied `pictures` checksum
    let pictures_checksum_b64 =
        base64_decode("4W2JKFehp72vT/+ilvY3pRiQ88tGgSi+lHvgDNT9ivRsa2sATFp0q3xllJ6H1/2B")?;
    assert_eq!(pictures_checksum_b64, pictures.checksum);

    Ok(true)
}
