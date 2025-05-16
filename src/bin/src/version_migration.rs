use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use rauthy_models::database::DB;
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

    // The deletion of the legacy Rauthy logo has been forgotten for new Hiqlite deployments.
    // A permanent DB migration will be added for the next feature release. Until then, there will
    // be this dynamic query to not introduce any breaking change with a new patch version.
    if is_hiqlite() {
        DB::hql().execute(r#"
DELETE FROM client_logos
WHERE client_id = 'rauthy' AND content_type = 'image/svg+xml' AND data LIKE '%viewBox="0 0 512 138"%';
        "#, params!()).await?;
    }

    Ok(())
}
