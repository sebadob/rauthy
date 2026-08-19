use crate::database::{Cache, DB};
use crate::entity::db_version::DbVersion;
use rauthy_error::ErrorResponse;
use semver::Version;
use tracing::info;

/// Cache namespaces that must not survive a minor upgrade. The hiqlite cache is
/// persisted in the WAL and restored on restart: single-use claims, per-IP abuse
/// counters and ephemeral client state written by the previous version would
/// otherwise linger until the next WAL compaction.
const CACHE_WAL_CLEANUP: [Cache; 12] = [
    Cache::AuthCode,
    Cache::AuthProviderCallback,
    Cache::ClientDynamic,
    Cache::ClientEphemeral,
    Cache::ClientSecret,
    Cache::CredStuffDetect,
    Cache::DeviceCode,
    Cache::DPoPNonce,
    Cache::EmailRateLimit,
    Cache::IpBlacklist,
    Cache::IpRateLimit,
    Cache::PoW,
];

/// `previous_db_version` is the version the database was created / last upgraded
/// to (captured by [`crate::database::DB::migrate`] before the upgrade) and is
/// `None` on a fresh install.
pub async fn apply_temp_migrations(
    previous_db_version: Option<Version>,
) -> Result<(), ErrorResponse> {
    if let Some(previous) = previous_db_version {
        let app = DbVersion::app_version();
        if needs_cache_wal_cleanup(&previous, &app) {
            info!("Cleaning up cache WAL entries written by v{previous}");
            let client = DB::hql();
            for cache in CACHE_WAL_CLEANUP {
                client.clear_cache(cache).await?;
            }
        }
    }

    Ok(())
}

/// The cache WAL cleanup runs when the stored DB version predates the current
/// minor release: cache entries from an older minor are stale by definition.
fn needs_cache_wal_cleanup(previous: &Version, app: &Version) -> bool {
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
        assert!(needs_cache_wal_cleanup(
            &v("0.36.2"),
            &v("0.37.0-nightly.1")
        ));
        assert!(needs_cache_wal_cleanup(&v("0.36.0"), &v("0.37.0")));
    }

    #[test]
    fn cache_wal_cleanup_skips_same_or_newer_minor() {
        assert!(!needs_cache_wal_cleanup(
            &v("0.37.0-nightly.1"),
            &v("0.37.0-nightly.2"),
        ));
        assert!(!needs_cache_wal_cleanup(&v("0.37.0"), &v("0.37.0")));
        assert!(!needs_cache_wal_cleanup(&v("0.37.0"), &v("0.36.2")));
    }
}
