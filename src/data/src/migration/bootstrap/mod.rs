use crate::database::DB;
use crate::entity::jwk::Jwk;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::info;

mod api_key;
mod jwks;
mod rauthy_admin;

/// Initializes an empty production database for a new deployment
pub async fn migrate_init_prod() -> Result<(), ErrorResponse> {
    // check if the database is un-initialized by looking at the jwks table, which should be empty
    let sql = "SELECT * FROM JWKS";
    let jwks: Vec<Jwk> = if is_hiqlite() {
        DB::hql().query_as(sql, params!()).await?
    } else {
        DB::pg_query(sql, &[], 8).await?
    };

    if !jwks.is_empty() {
        return Ok(());
    }

    info!("Initializing empty production database");

    // For initializing a prod database, we need to:
    // - delete init_admin and client
    // - set new random password for admin and log to console with the first startup
    // - generate a new set of JWKs

    // cleanup
    let sql_1 = "DELETE FROM clients WHERE id = 'init_client'";
    let sql_2 = "DELETE FROM users WHERE email IN ('init_admin@localhost', 'test_admin@localhost')";

    if is_hiqlite() {
        DB::hql().execute(sql_1, params!()).await?;
        DB::hql().execute(sql_2, params!()).await?;
    } else {
        DB::pg_execute(sql_1, &[]).await?;
        DB::pg_execute(sql_2, &[]).await?;
    }

    rauthy_admin::rauthy_admin().await?;
    api_key::api_key().await?;
    jwks::jwks().await?;

    info!("Production database initialized successfully");

    Ok(())
}
