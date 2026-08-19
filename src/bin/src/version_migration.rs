use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use semver::Version;

/// If it's necessary to apply manual migrations between major versions, which are
/// not handled automatically by database migrations, put them here. This function
/// will be executed at startup after DB init and before the API start.
///
/// `previous_db_version` is the version the database was created / last upgraded
/// to, captured by `DB::migrate()` before it applied this upgrade.
pub async fn manual_version_migrations(
    previous_db_version: Option<Version>,
) -> Result<(), ErrorResponse> {
    // to avoid race conditions, only node 1 should do these one-time migrations
    if !RauthyConfig::get().is_primary_node {
        return Ok(());
    }

    rauthy_data::temp_migrations::apply_temp_migrations(previous_db_version).await?;

    Ok(())
}
