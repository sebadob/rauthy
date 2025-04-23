use rauthy_error::ErrorResponse;
use tracing::warn;

/// If it's necessary to apply manual migrations between major versions, which are
/// not handled automatically by database migrations, put them here. This function
/// will be executed at startup after DB init and before the API start.
pub async fn manual_version_migrations() -> Result<(), ErrorResponse> {
    warn!(
        r#"

    CAUTION: If you have been using the default admin 'admin@localhost.de',
    it will no longer exist with this email. Rauthy v0.29 changes the
    default admin to 'admin@localhost', which you should use, if you are
    migrating from an older version with a still existing 'admin@localhost.de'.
"#
    );

    Ok(())
}
