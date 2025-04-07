use chrono::{DateTime, Utc};
use rauthy_common::utils::{base64_decode, base64_encode};
use rauthy_common::{is_hiqlite, params_iter};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::database::DB;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tracing::{debug, warn};

/// If it's necessary to apply manual migrations between major versions, which are
/// not handled automatically by database migrations, put them here. This function
/// will be executed at startup after DB init and before the API start.
pub async fn manual_version_migrations() -> Result<(), ErrorResponse> {
    if !is_hiqlite() && is_existing_sqlx_database().await == Ok(true) {
        warn!("\n\n\nHandle existing sqlx migrations table properly!\n\n");
    }

    Ok(())
}

#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "_sqlx_migrations")]
struct SqlxMigration {
    version: i64,
    description: String,
    installed_on: DateTime<Utc>,
    success: bool,
    checksum: Vec<u8>,
    execution_time: i64,
}

/// Checks if the currently used Postgres database is already set up for `sqlx` from before the
/// `tokio-postgres` migration.
async fn is_existing_sqlx_database() -> Result<bool, ErrorResponse> {
    let cl = DB::pg().await?;
    let stmt = cl.prepare("SELECT * FROM _sqlx_migrations").await?;
    let mut rows = cl
        .query(&stmt, &[])
        .await?
        .into_iter()
        .map(SqlxMigration::from_row)
        .collect::<Vec<_>>();

    // in v0.28, migration no 32 is `pictures`
    if rows.len() < 32 {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "invalid database version, expected at least the state of Rauthy 0.28",
        ));
    }

    // make sure that no 32 is actually a correctly applied `pictures` migration
    let pictures = rows.swap_remove(31)?;
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
