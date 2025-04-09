use chrono::{DateTime, Utc};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::base64_decode;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::database::DB;

use tracing::{debug, warn};

/// If it's necessary to apply manual migrations between major versions, which are
/// not handled automatically by database migrations, put them here. This function
/// will be executed at startup after DB init and before the API start.
pub async fn manual_version_migrations() -> Result<(), ErrorResponse> {
    if !is_hiqlite() && is_existing_sqlx_database().await == Ok(true) {
        // TODO
        warn!("\n\n\nHandle existing sqlx migrations table properly!\n\n");
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

    // make sure that no 32 is actually a correctly applied `pictures` migration
    let pictures = rows.swap_remove(31);
    debug!("applied pictures migration:\n{pictures:?}");
    assert_eq!(pictures.version, 32);
    assert_eq!(&pictures.description, "pictures");
    // This is the expected checksum of an already applied `pictures` checksum
    let pictures_checksum_b64 =
        base64_decode("4W2JKFehp72vT/+ilvY3pRiQ88tGgSi+lHvgDNT9ivRsa2sATFp0q3xllJ6H1/2B")?;
    assert_eq!(pictures_checksum_b64, pictures.checksum);

    warn!(
        "\n\n\nTODO after we are finished dropping sqlx, assert that the length is exactly 32!\n\n"
    );

    Ok(true)
}
