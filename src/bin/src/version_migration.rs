use rauthy_error::ErrorResponse;
use rauthy_models::rauthy_config::RauthyConfig;

/// If it's necessary to apply manual migrations between major versions, which are
/// not handled automatically by database migrations, put them here. This function
/// will be executed at startup after DB init and before the API start.
pub async fn manual_version_migrations() -> Result<(), ErrorResponse> {
    // to avoid race conditions, only node 1 should do these one-time migrations
    if !RauthyConfig::get().is_primary_node {
        return Ok(());
    }

    Ok(())
}
