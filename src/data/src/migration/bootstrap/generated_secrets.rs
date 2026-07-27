use cryptr::EncValue;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use tracing::{error, info, warn};
use zeroize::Zeroize;

const VERSION: u16 = 1;
const DEFAULT_FILE_NAME: &str = "bootstrap.secrets.enc";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GeneratedSecretKey {
    pub kind: String,
    pub id: String,
    pub field: String,
}

impl GeneratedSecretKey {
    pub fn new(kind: impl Into<String>, id: impl Into<String>, field: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            id: id.into(),
            field: field.into(),
        }
    }

    pub fn env_name(&self) -> String {
        sanitize_env_name(format!(
            "RAUTHY_BOOTSTRAP_{}_{}_{}",
            self.kind, self.id, self.field
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedSecretEntry {
    pub kind: String,
    pub id: String,
    pub field: String,
    pub value: String,
}

impl GeneratedSecretEntry {
    pub fn new(key: GeneratedSecretKey, value: impl Into<String>) -> GeneratedSecretEntry {
        Self {
            kind: key.kind,
            id: key.id,
            field: key.field,
            value: value.into(),
        }
    }

    pub fn key(&self) -> GeneratedSecretKey {
        GeneratedSecretKey {
            kind: self.kind.clone(),
            id: self.id.clone(),
            field: self.field.clone(),
        }
    }

    pub fn env_name(&self) -> String {
        self.key().env_name()
    }
}

impl Drop for GeneratedSecretEntry {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeneratedSecrets {
    pub entries: Vec<GeneratedSecretEntry>,
}

impl GeneratedSecrets {
    fn upsert(&mut self, entry: GeneratedSecretEntry) {
        if let Some(existing) = self.entries.iter_mut().find(|e| e.key() == entry.key()) {
            *existing = entry;
        } else {
            self.entries.push(entry);
        }
    }
}

/// On-disk container payload. The whole struct is encrypted as a single
/// `cryptr` AEAD value, so its integrity is guaranteed by decryption itself —
/// no separate magic/version header on disk is needed. The expiry `deadline`
/// lives inside the encrypted payload; the server (and the CLI) hold `ENC_KEYS`,
/// so reading it back to check expiry is a cheap decrypt.
#[derive(Serialize)]
struct ContainerPayloadRef<'a> {
    version: u16,
    /// UTC unix timestamp in seconds. `0` means the container does not expire.
    deadline: i64,
    entries: &'a [GeneratedSecretEntry],
}

#[derive(Deserialize)]
struct ContainerPayload {
    #[serde(default)]
    version: u16,
    #[serde(default)]
    deadline: i64,
    entries: Vec<GeneratedSecretEntry>,
}

pub fn default_file_path(data_dir: &str) -> String {
    Path::new(data_dir)
        .join(DEFAULT_FILE_NAME)
        .to_string_lossy()
        .into_owned()
}

/// Current UTC unix timestamp in seconds. Always work in UTC for stored
/// timestamps so a shared volume mounted on multiple hosts compares the same
/// instant regardless of local time zone.
fn now_utc() -> i64 {
    chrono::Utc::now().timestamp()
}

fn is_expired(deadline: i64, now: i64) -> bool {
    deadline > 0 && now >= deadline
}

pub fn deadline_from_ttl(ttl_seconds: u32, now: i64) -> i64 {
    if ttl_seconds == 0 {
        0
    } else {
        now.saturating_add(i64::from(ttl_seconds))
    }
}

pub async fn upsert_secret(
    path: impl AsRef<Path>,
    ttl_seconds: u32,
    entry: GeneratedSecretEntry,
) -> Result<(), ErrorResponse> {
    let path = path.as_ref();
    let mut container = if tokio::fs::try_exists(path).await.unwrap_or(false) {
        match read_container(path).await {
            Ok(container) => container,
            Err(err) => {
                warn!(
                    "Could not read existing bootstrap generated-secret container '{}': {err}; starting with a fresh container",
                    path.display()
                );
                GeneratedSecrets::default()
            }
        }
    } else {
        GeneratedSecrets::default()
    };
    container.upsert(entry);
    let deadline = deadline_from_ttl(ttl_seconds, now_utc());
    write_container(path, deadline, &container).await?;
    schedule_purge(path.to_path_buf(), ttl_seconds);
    Ok(())
}

pub async fn read_container(path: impl AsRef<Path>) -> Result<GeneratedSecrets, ErrorResponse> {
    let path = path.as_ref();
    let bytes = match tokio::fs::read(path).await {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(ErrorResponse::internal(format!(
                "no generated bootstrap secrets stored at {}",
                path.display()
            )));
        }
        Err(err) => return Err(err.into()),
    };
    let payload = decrypt_container(bytes, path)?;
    if is_expired(payload.deadline, now_utc()) {
        return Err(ErrorResponse::internal(format!(
            "bootstrap generated-secret container {} expired at {}",
            path.display(),
            payload.deadline
        )));
    }
    Ok(GeneratedSecrets {
        entries: payload.entries,
    })
}

pub async fn write_container(
    path: impl AsRef<Path>,
    deadline: i64,
    container: &GeneratedSecrets,
) -> Result<(), ErrorResponse> {
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        ErrorResponse::internal(format!(
            "bootstrap generated-secret path '{}' has no parent directory",
            path.display()
        ))
    })?;
    tokio::fs::create_dir_all(parent).await?;

    let mut payload = serde_json::to_vec(&ContainerPayloadRef {
        version: VERSION,
        deadline,
        entries: &container.entries,
    })?;
    let encrypted = match EncValue::encrypt(&payload) {
        Ok(value) => value.into_bytes(),
        Err(err) => {
            payload.zeroize();
            return Err(err.into());
        }
    };
    payload.zeroize();

    let tmp = tmp_path(path)?;
    {
        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);
        #[cfg(unix)]
        options.mode(0o600);
        let mut file = options.open(&tmp).await?;
        file.write_all(&encrypted).await?;
        file.sync_all().await?;
    }
    #[cfg(unix)]
    tokio::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o600)).await?;
    tokio::fs::rename(&tmp, path).await?;

    Ok(())
}

pub async fn purge_if_expired(path: impl AsRef<Path>) -> Result<bool, ErrorResponse> {
    let path = path.as_ref();
    let bytes = match tokio::fs::read(path).await {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err.into()),
    };
    let payload = decrypt_container(bytes, path)?;
    if is_expired(payload.deadline, now_utc()) {
        tokio::fs::remove_file(path).await?;
        info!(
            "Purged expired bootstrap generated-secret container '{}'",
            path.display()
        );
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn schedule_purge(path: PathBuf, ttl_seconds: u32) {
    if ttl_seconds == 0 {
        return;
    }

    tokio::spawn(async move {
        sleep(Duration::from_secs(u64::from(ttl_seconds))).await;
        if let Err(err) = purge_if_expired(&path).await {
            error!(
                "Could not purge bootstrap generated-secret container '{}': {err}",
                path.display()
            );
        }
    });
}

fn decrypt_container(bytes: Vec<u8>, path: &Path) -> Result<ContainerPayload, ErrorResponse> {
    if bytes.is_empty() {
        return Err(ErrorResponse::internal(format!(
            "bootstrap generated-secret container {} is empty",
            path.display()
        )));
    }
    // cryptr uses AEAD (ChaCha20Poly1305): a malformed or tampered file fails to
    // decrypt, which is the integrity check. No manual magic/version on disk.
    let decrypted = EncValue::try_from(bytes)?.decrypt()?;
    let payload: ContainerPayload = match serde_json::from_slice(&decrypted) {
        Ok(payload) => payload,
        Err(err) => {
            if let Ok(mut decrypted) = decrypted.try_into_mut() {
                decrypted.as_mut().zeroize();
            }
            return Err(err.into());
        }
    };
    if let Ok(mut decrypted) = decrypted.try_into_mut() {
        decrypted.as_mut().zeroize();
    }
    if payload.version > VERSION {
        return Err(ErrorResponse::internal(format!(
            "unsupported bootstrap generated-secret container version {} in {}",
            payload.version,
            path.display()
        )));
    }
    Ok(payload)
}

pub fn sanitize_env_name(input: impl AsRef<str>) -> String {
    let mut out = String::with_capacity(input.as_ref().len());
    let mut last_was_underscore = false;
    for c in input.as_ref().chars() {
        let next = if c.is_ascii_alphanumeric() {
            c.to_ascii_uppercase()
        } else {
            '_'
        };
        if next == '_' {
            if !last_was_underscore {
                out.push(next);
            }
            last_was_underscore = true;
        } else {
            out.push(next);
            last_was_underscore = false;
        }
    }

    let out = out.trim_matches('_').to_string();
    if out.is_empty() {
        "_".to_string()
    } else if out.as_bytes()[0].is_ascii_digit() {
        format!("_{out}")
    } else {
        out
    }
}

fn tmp_path(path: &Path) -> Result<PathBuf, ErrorResponse> {
    let file_name = path.file_name().and_then(OsStr::to_str).ok_or_else(|| {
        ErrorResponse::internal(format!(
            "invalid generated-secret path '{}'",
            path.display()
        ))
    })?;
    // Editor-style sibling temp file. Only one Rauthy process runs and
    // bootstrapping is single-threaded, so no pid/timestamp uniqueness is needed.
    Ok(path.with_file_name(format!("{file_name}~")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cryptr::EncKeys;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::Once;

    static INIT_KEYS: Once = Once::new();

    fn init_keys() {
        INIT_KEYS.call_once(|| {
            let keys = EncKeys::generate_with_id("test".to_string()).unwrap();
            let _ = keys.init();
        });
    }

    fn test_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rauthy-bootstrap-generated-secrets-{name}-{}",
            std::process::id()
        ))
    }

    #[tokio::test]
    async fn roundtrip_writes_mode_0600_and_reads_entries() {
        init_keys();
        let path = test_path("roundtrip");
        let _ = tokio::fs::remove_file(&path).await;

        upsert_secret(
            &path,
            0,
            GeneratedSecretEntry::new(
                GeneratedSecretKey::new("client", "demo-app", "secret"),
                "generated-secret",
            ),
        )
        .await
        .unwrap();

        let meta = tokio::fs::metadata(&path).await.unwrap();
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);

        let container = read_container(&path).await.unwrap();
        assert_eq!(container.entries.len(), 1);
        assert_eq!(container.entries[0].value, "generated-secret");

        let _ = tokio::fs::remove_file(&path).await;
    }

    #[tokio::test]
    async fn garbage_file_fails_to_decrypt() {
        init_keys();
        let path = test_path("garbage");
        tokio::fs::write(&path, b"not-a-valid-enc-container")
            .await
            .unwrap();

        let err = read_container(&path).await.unwrap_err();
        assert!(!err.message.is_empty());

        let _ = tokio::fs::remove_file(&path).await;
    }

    #[tokio::test]
    async fn missing_container_error_names_path() {
        init_keys();
        let path = test_path("missing");
        let _ = tokio::fs::remove_file(&path).await;

        let err = read_container(&path).await.unwrap_err();

        assert!(
            err.message
                .contains("no generated bootstrap secrets stored at")
        );
        assert!(err.message.contains(path.to_str().unwrap()));
    }

    #[tokio::test]
    async fn upsert_recovers_from_unreadable_existing_container() {
        init_keys();
        let path = test_path("recover-garbage");
        tokio::fs::write(&path, b"not-a-valid-enc-container")
            .await
            .unwrap();

        upsert_secret(
            &path,
            0,
            GeneratedSecretEntry::new(
                GeneratedSecretKey::new("client", "demo-app", "secret"),
                "generated-secret",
            ),
        )
        .await
        .unwrap();

        let container = read_container(&path).await.unwrap();
        assert_eq!(container.entries.len(), 1);
        assert_eq!(container.entries[0].value, "generated-secret");

        let _ = tokio::fs::remove_file(&path).await;
    }

    #[tokio::test]
    async fn write_container_overwrites_stale_temp_file() {
        init_keys();
        let path = test_path("stale-temp");
        let tmp = tmp_path(&path).unwrap();
        let _ = tokio::fs::remove_file(&path).await;
        tokio::fs::write(&tmp, b"stale partial write")
            .await
            .unwrap();

        let mut container = GeneratedSecrets::default();
        container.upsert(GeneratedSecretEntry::new(
            GeneratedSecretKey::new("user", "can", "password"),
            "pw",
        ));
        write_container(&path, 0, &container).await.unwrap();

        let container = read_container(&path).await.unwrap();
        assert_eq!(container.entries.len(), 1);
        assert_eq!(container.entries[0].value, "pw");
        assert!(!tmp.exists());

        let _ = tokio::fs::remove_file(&path).await;
    }

    #[tokio::test]
    async fn expired_container_is_rejected() {
        init_keys();
        let path = test_path("expired");
        let _ = tokio::fs::remove_file(&path).await;

        let mut container = GeneratedSecrets::default();
        container.upsert(GeneratedSecretEntry::new(
            GeneratedSecretKey::new("client", "demo", "secret"),
            "s",
        ));
        write_container(&path, now_utc() - 10, &container)
            .await
            .unwrap();

        let err = read_container(&path).await.unwrap_err();
        assert!(err.message.contains("expired"));

        let _ = tokio::fs::remove_file(&path).await;
    }

    #[tokio::test]
    async fn purge_removes_expired_file() {
        init_keys();
        let path = test_path("purge");
        let _ = tokio::fs::remove_file(&path).await;

        let mut container = GeneratedSecrets::default();
        container.upsert(GeneratedSecretEntry::new(
            GeneratedSecretKey::new("user", "can", "password"),
            "pw",
        ));
        write_container(&path, now_utc() - 10, &container)
            .await
            .unwrap();

        assert!(purge_if_expired(&path).await.unwrap());
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn ttl_zero_disables_expiry_and_auto_purge() {
        init_keys();
        let path = test_path("ttl-zero");
        let _ = tokio::fs::remove_file(&path).await;

        upsert_secret(
            &path,
            0,
            GeneratedSecretEntry::new(GeneratedSecretKey::new("user", "can", "password"), "pw"),
        )
        .await
        .unwrap();

        let container = read_container(&path).await.unwrap();
        assert_eq!(container.entries.len(), 1);
        assert!(!purge_if_expired(&path).await.unwrap());

        let _ = tokio::fs::remove_file(&path).await;
    }

    #[test]
    fn env_names_are_sanitized() {
        let key = GeneratedSecretKey::new("api-key", "provision/key", "client secret");
        assert_eq!(
            key.env_name(),
            "RAUTHY_BOOTSTRAP_API_KEY_PROVISION_KEY_CLIENT_SECRET"
        );
        assert_eq!(sanitize_env_name("123 !thing"), "_123_THING");
    }
}
