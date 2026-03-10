use crate::database::DB;
use crate::entity::jwk::Jwk;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::{debug, info};

mod api_key;
mod clients;
mod groups;
mod jwks;
mod rauthy_admin;
mod roles;
mod scopes;
mod types;
mod user_attrs;

macro_rules! bootstrap_data {
    ($type:ty, $e:expr) => {{
        use validator::Validate;

        let path = format!(
            "{}/{}.json",
            crate::rauthy_config::RauthyConfig::get()
                .vars
                .bootstrap
                .bootstrap_dir,
            $e
        );
        let Ok(content) = tokio::fs::read(&path).await else {
            tracing::debug!("No file for bootstrapping '{}'", path);
            return Ok(());
        };
        if content.is_empty() {
            tracing::debug!("Empty file for bootstrapping '{}'", path);
            return Ok(());
        }
        let data = serde_json::from_slice::<Vec<$type>>(&content)?;
        if data.is_empty() {
            tracing::debug!("Empty data for bootstrapping '{}'", path);
            return Ok(());
        }

        for d in &data {
            if let Err(err) = d.validate() {
                panic!(
                    "Validation error when bootstrapping data from '{}':\n\n{:?}\n\n{:?}\n",
                    path, d, err
                );
            }
        }

        data
    }};
}

pub(crate) use bootstrap_data;

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
    // - possibly bootstrap additional, optional data

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

    rauthy_admin::bootstrap().await?;
    jwks::bootstrap().await?;
    bootstrap_additional_data().await?;

    info!("Production database initialized successfully");

    Ok(())
}

// We want to be able to call this during DEV as well to always be sure that it works.
#[cold]
pub async fn bootstrap_additional_data() -> Result<(), ErrorResponse> {
    debug!("Bootstrapping additional data");

    api_key::bootstrap().await?;
    groups::bootstrap().await?;
    roles::bootstrap().await?;
    user_attrs::bootstrap().await?;
    scopes::bootstrap().await?;
    clients::bootstrap().await?;

    Ok(())
}
