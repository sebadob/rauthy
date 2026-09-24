use crate::entity::db_version::DbVersion;
use rauthy_error::ErrorResponse;
use semver::Version;
use tracing::info;

pub async fn apply_temp_migrations(
    previous_db_version: Option<Version>,
) -> Result<(), ErrorResponse> {
    let Some(previous) = previous_db_version else {
        return Ok(());
    };
    let app = DbVersion::app_version();
    if !needs_temp_migration(&previous, &app) {
        info!("Cache WAL state was reset before startup for upgrade from v{previous}");
    }

    Ok(())
}

fn needs_temp_migration(previous: &Version, app: &Version) -> bool {
    previous.major != app.major || previous.minor < app.minor
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Version {
        Version::parse(s).unwrap()
    }

    #[test]
    fn cache_wal_cleanup_runs_on_minor_upgrade() {
        assert!(needs_temp_migration(&v("0.36.2"), &v("0.37.0-20260819")));
        assert!(needs_temp_migration(&v("0.36.0"), &v("0.37.0")));
    }

    #[test]
    fn cache_wal_cleanup_skips_same_or_newer_minor() {
        assert!(!needs_temp_migration(
            &v("0.37.0-20260819"),
            &v("0.37.0-20260820"),
        ));
        assert!(!needs_temp_migration(&v("0.37.0"), &v("0.37.0")));
        assert!(!needs_temp_migration(&v("0.37.0"), &v("0.36.2")));
    }
}
