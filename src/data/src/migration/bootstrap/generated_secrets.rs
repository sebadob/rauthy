use cryptr::EncValue;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use tracing::{debug, error, info};
use zeroize::Zeroize;

const MAGIC: &str = "RAUTHY_BOOTSTRAP_GENERATED_SECRETS";
const VERSION: u16 = 1;
const HEADER_LINES: usize = 3;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContainerHeader {
    pub version: u16,
    /// Unix timestamp in seconds. `0` means the container does not expire.
    pub deadline: u64,
}

impl ContainerHeader {
    pub fn is_expired_at(self, now: SystemTime) -> bool {
        self.deadline > 0 && unix_seconds(now) >= self.deadline
    }
}

pub fn default_file_path(data_dir: &str) -> String {
    Path::new(data_dir)
        .join(DEFAULT_FILE_NAME)
        .to_string_lossy()
        .into_owned()
}

pub fn deadline_from_ttl(ttl_seconds: u64, now: SystemTime) -> u64 {
    if ttl_seconds == 0 {
        0
    } else {
        unix_seconds(now) + ttl_seconds
    }
}

pub async fn upsert_secret(
    path: impl AsRef<Path>,
    ttl_seconds: u64,
    entry: GeneratedSecretEntry,
) -> Result<(), ErrorResponse> {
    let path = path.as_ref();
    let mut container = if path.exists() {
        read_container(path).await?
    } else {
        GeneratedSecrets::default()
    };
    container.upsert(entry);
    let deadline = deadline_from_ttl(ttl_seconds, SystemTime::now());
    write_container(path, deadline, &container).await?;
    schedule_purge(path.to_path_buf(), ttl_seconds);
    Ok(())
}

pub async fn read_container(path: impl AsRef<Path>) -> Result<GeneratedSecrets, ErrorResponse> {
    let bytes = tokio::fs::read(path.as_ref()).await?;
    let (header, encrypted) = parse_header(&bytes)?;
    if header.is_expired_at(SystemTime::now()) {
        return Err(internal(format!(
            "bootstrap generated-secret container {} expired at {}",
            path.as_ref().display(),
            header.deadline
        )));
    }

    let mut decrypted = EncValue::try_from(encrypted.to_vec())?.decrypt()?.to_vec();
    let container = serde_json::from_slice(&decrypted);
    decrypted.zeroize();
    Ok(container?)
}

pub async fn write_container(
    path: impl AsRef<Path>,
    deadline: u64,
    container: &GeneratedSecrets,
) -> Result<(), ErrorResponse> {
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| {
        internal(format!(
            "bootstrap generated-secret path '{}' has no parent directory",
            path.display()
        ))
    })?;
    tokio::fs::create_dir_all(parent).await?;

    let mut payload = serde_json::to_vec(container)?;
    let encrypted = match EncValue::encrypt(&payload) {
        Ok(value) => value.into_bytes().to_vec(),
        Err(err) => {
            payload.zeroize();
            return Err(err.into());
        }
    };
    payload.zeroize();
    let mut bytes = render_header(deadline);
    bytes.extend_from_slice(&encrypted);

    let tmp = tmp_path(path)?;
    {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        options.mode(0o600);
        let mut file = options.open(&tmp)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
    }
    bytes.zeroize();
    std::fs::rename(&tmp, path)?;

    if let Ok(dir) = std::fs::File::open(parent)
        && let Err(err) = dir.sync_all()
    {
        debug!(
            "Could not fsync bootstrap generated-secret directory '{}': {err}",
            parent.display()
        );
    }

    Ok(())
}

pub async fn purge_if_expired(path: impl AsRef<Path>) -> Result<bool, ErrorResponse> {
    let path = path.as_ref();
    let bytes = match tokio::fs::read(path).await {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err.into()),
    };
    let (header, _) = parse_header(&bytes)?;
    if header.is_expired_at(SystemTime::now()) {
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

pub fn schedule_purge(path: PathBuf, ttl_seconds: u64) {
    if ttl_seconds == 0 {
        return;
    }

    tokio::spawn(async move {
        sleep(Duration::from_secs(ttl_seconds)).await;
        if let Err(err) = purge_if_expired(&path).await {
            error!(
                "Could not purge bootstrap generated-secret container '{}': {err}",
                path.display()
            );
        }
    });
}

pub fn parse_header(bytes: &[u8]) -> Result<(ContainerHeader, &[u8]), ErrorResponse> {
    let mut lines = bytes.splitn(HEADER_LINES + 1, |b| *b == b'\n');
    let magic = lines
        .next()
        .ok_or_else(|| internal("missing bootstrap generated-secret magic"))?;
    if magic != MAGIC.as_bytes() {
        return Err(internal("invalid bootstrap generated-secret magic"));
    }

    let version = parse_header_u16(lines.next(), "version")?;
    if version != VERSION {
        return Err(internal(format!(
            "unsupported bootstrap generated-secret version {version}"
        )));
    }

    let deadline = parse_header_u64(lines.next(), "deadline")?;
    let encrypted = lines
        .next()
        .ok_or_else(|| internal("missing bootstrap generated-secret payload"))?;
    Ok((ContainerHeader { version, deadline }, encrypted))
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

fn render_header(deadline: u64) -> Vec<u8> {
    format!("{MAGIC}\n{VERSION}\n{deadline}\n").into_bytes()
}

fn parse_header_u16(line: Option<&[u8]>, name: &str) -> Result<u16, ErrorResponse> {
    let raw = parse_header_line(line, name)?;
    raw.parse::<u16>()
        .map_err(|_| internal(format!("invalid bootstrap generated-secret {name}")))
}

fn parse_header_u64(line: Option<&[u8]>, name: &str) -> Result<u64, ErrorResponse> {
    let raw = parse_header_line(line, name)?;
    raw.parse::<u64>()
        .map_err(|_| internal(format!("invalid bootstrap generated-secret {name}")))
}

fn parse_header_line<'a>(line: Option<&'a [u8]>, name: &str) -> Result<&'a str, ErrorResponse> {
    let line =
        line.ok_or_else(|| internal(format!("missing bootstrap generated-secret {name}")))?;
    std::str::from_utf8(line).map_err(|_| {
        internal(format!(
            "invalid utf-8 in bootstrap generated-secret {name}"
        ))
    })
}

fn tmp_path(path: &Path) -> Result<PathBuf, ErrorResponse> {
    let file_name = path.file_name().and_then(OsStr::to_str).ok_or_else(|| {
        internal(format!(
            "invalid generated-secret path '{}'",
            path.display()
        ))
    })?;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    Ok(path.with_file_name(format!(".{file_name}.{}.{nanos}.tmp", std::process::id())))
}

fn unix_seconds(now: SystemTime) -> u64 {
    now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}

fn internal(msg: impl Into<String>) -> ErrorResponse {
    ErrorResponse::new(ErrorResponseType::Internal, msg.into())
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
    async fn bad_magic_fails_before_decrypt() {
        init_keys();
        let path = test_path("bad-magic");
        tokio::fs::write(&path, b"wrong\n1\n0\npayload")
            .await
            .unwrap();

        let err = read_container(&path).await.unwrap_err();
        assert!(err.message.contains("magic"));

        let _ = tokio::fs::remove_file(&path).await;
    }

    #[test]
    fn bad_version_fails_before_decrypt() {
        let mut bytes = b"RAUTHY_BOOTSTRAP_GENERATED_SECRETS\n2\n0\n".to_vec();
        bytes.extend_from_slice(b"payload");

        let err = parse_header(&bytes).unwrap_err();
        assert!(err.message.contains("version"));
    }

    #[tokio::test]
    async fn expired_header_fails_without_decrypting_payload() {
        init_keys();
        let path = test_path("expired");
        tokio::fs::write(
            &path,
            b"RAUTHY_BOOTSTRAP_GENERATED_SECRETS\n1\n1\nnot-encrypted",
        )
        .await
        .unwrap();

        let err = read_container(&path).await.unwrap_err();
        assert!(err.message.contains("expired"));

        let _ = tokio::fs::remove_file(&path).await;
    }

    #[tokio::test]
    async fn purge_removes_expired_file_without_decrypting_payload() {
        let path = test_path("purge");
        tokio::fs::write(
            &path,
            b"RAUTHY_BOOTSTRAP_GENERATED_SECRETS\n1\n1\nnot-encrypted",
        )
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
        let bytes = tokio::fs::read(&path).await.unwrap();
        let (header, _) = parse_header(&bytes).unwrap();

        assert_eq!(header.deadline, 0);
        assert!(!header.is_expired_at(SystemTime::now() + Duration::from_secs(3600)));

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
