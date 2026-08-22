use crate::entity::db_version::DbVersion;
use rauthy_error::ErrorResponse;
use semver::Version;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{info, warn};

const CACHE_WAL_VERSION_FILE: &str = "rauthy-cache-wal-version";
const CACHE_WAL_DIRS: [&str; 2] = ["logs_cache", "state_machine_cache"];

/// Removes incompatible cache Raft state before Hiqlite restores it.
pub async fn prepare_cache_wal(
    data_dir: &str,
    cache_storage_disk: bool,
) -> Result<(), ErrorResponse> {
    if !cache_storage_disk {
        return Ok(());
    }

    let marker = cache_wal_version_path(data_dir);
    let should_reset = match fs::read_to_string(&marker).await {
        Ok(version) => Version::parse(version.trim())
            .map(|previous| needs_cache_wal_cleanup(&previous, &DbVersion::app_version()))
            .unwrap_or(true),
        Err(err) if err.kind() == ErrorKind::NotFound => true,
        Err(err) => return Err(err.into()),
    };

    if should_reset {
        warn!(
            "Resetting cache WAL state; this also removes manual IP blacklist entries and failed-login counters"
        );
        for dir in CACHE_WAL_DIRS {
            let path = Path::new(data_dir).join(dir);
            match fs::remove_dir_all(path).await {
                Ok(()) => {}
                Err(err) if err.kind() == ErrorKind::NotFound => {}
                Err(err) => return Err(err.into()),
            }
        }
    }

    Ok(())
}

/// Records the cache format after database and manual migrations succeed.
pub async fn mark_cache_wal_current(
    data_dir: &str,
    cache_storage_disk: bool,
) -> Result<(), ErrorResponse> {
    if !cache_storage_disk {
        return Ok(());
    }

    let marker = cache_wal_version_path(data_dir);
    let temporary = marker.with_extension("tmp");
    fs::write(&temporary, DbVersion::app_version().to_string()).await?;
    fs::rename(temporary, marker).await?;
    Ok(())
}

fn cache_wal_version_path(data_dir: &str) -> PathBuf {
    Path::new(data_dir).join(CACHE_WAL_VERSION_FILE)
}

pub async fn apply_temp_migrations(
    previous_db_version: Option<Version>,
) -> Result<(), ErrorResponse> {
    if let Some(previous) = previous_db_version {
        let app = DbVersion::app_version();
        if needs_cache_wal_cleanup(&previous, &app) {
            info!("Cache WAL state was reset before startup for upgrade from v{previous}");
        }
    }

    Ok(())
}

/// Returns whether cache state from `previous` must be discarded for `app`.
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
        assert!(needs_cache_wal_cleanup(&v("0.36.2"), &v("0.37.0-20260819")));
        assert!(needs_cache_wal_cleanup(&v("0.36.0"), &v("0.37.0")));
    }

    #[test]
    fn cache_wal_cleanup_skips_same_or_newer_minor() {
        assert!(!needs_cache_wal_cleanup(
            &v("0.37.0-20260819"),
            &v("0.37.0-20260820"),
        ));
        assert!(!needs_cache_wal_cleanup(&v("0.37.0"), &v("0.37.0")));
        assert!(!needs_cache_wal_cleanup(&v("0.37.0"), &v("0.36.2")));
    }

    fn test_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("rauthy-cache-wal-{name}-{}", std::process::id()))
    }

    async fn setup_cache_wal(name: &str, marker: Option<&str>) -> PathBuf {
        let dir = test_dir(name);
        let _ = fs::remove_dir_all(&dir).await;
        fs::create_dir_all(dir.join("logs_cache")).await.unwrap();
        fs::create_dir_all(dir.join("state_machine_cache"))
            .await
            .unwrap();
        if let Some(marker) = marker {
            fs::write(cache_wal_version_path(dir.to_str().unwrap()), marker)
                .await
                .unwrap();
        }
        dir
    }

    #[tokio::test]
    async fn cache_wal_reset_clears_old_state_and_records_current_version() {
        let dir = setup_cache_wal("old", Some("0.36.2")).await;
        prepare_cache_wal(dir.to_str().unwrap(), true)
            .await
            .unwrap();
        assert!(!dir.join("logs_cache").exists());
        assert!(!dir.join("state_machine_cache").exists());

        fs::create_dir_all(dir.join("logs_cache")).await.unwrap();
        mark_cache_wal_current(dir.to_str().unwrap(), true)
            .await
            .unwrap();
        let marker = fs::read_to_string(cache_wal_version_path(dir.to_str().unwrap()))
            .await
            .unwrap();
        assert_eq!(marker, DbVersion::app_version().to_string());
        fs::remove_dir_all(dir).await.unwrap();
    }

    #[tokio::test]
    async fn cache_wal_reset_keeps_current_state_and_handles_fresh_install() {
        let current = DbVersion::app_version().to_string();
        let dir = setup_cache_wal("current", Some(&current)).await;
        prepare_cache_wal(dir.to_str().unwrap(), true)
            .await
            .unwrap();
        assert!(dir.join("logs_cache").exists());
        assert!(dir.join("state_machine_cache").exists());
        fs::remove_dir_all(&dir).await.unwrap();

        let dir = setup_cache_wal("invalid", Some("not-a-version")).await;
        prepare_cache_wal(dir.to_str().unwrap(), true)
            .await
            .unwrap();
        assert!(!dir.join("logs_cache").exists());
        fs::remove_dir_all(&dir).await.unwrap();

        let dir = setup_cache_wal("disabled", Some("0.36.2")).await;
        prepare_cache_wal(dir.to_str().unwrap(), false)
            .await
            .unwrap();
        assert!(dir.join("logs_cache").exists());
        fs::remove_dir_all(&dir).await.unwrap();

        let dir = setup_cache_wal("fresh", None).await;
        prepare_cache_wal(dir.to_str().unwrap(), true)
            .await
            .unwrap();
        assert!(!dir.join("logs_cache").exists());
        assert!(!dir.join("state_machine_cache").exists());
        fs::remove_dir_all(dir).await.unwrap();
    }
}
