use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use semver::Version;

/// Runs upgrade-only work after database migrations.
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
