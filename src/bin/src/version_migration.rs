use rauthy_error::ErrorResponse;

/// If it's necessary to apply manual migrations between major versions, which are
/// not handled automatically by database migrations, put them here. This function
/// will be executed at startup after DB init and before the API start.
pub async fn manual_version_migrations() -> Result<(), ErrorResponse> {
    rauthy_models::temp_migrate_federation_idp::migrate_fed_ids().await?;
    Ok(())
}
