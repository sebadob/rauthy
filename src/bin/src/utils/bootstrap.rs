use crate::cli_args::{
    ArgsBootstrap, ArgsBootstrapCommand, ArgsBootstrapGet, ArgsBootstrapPurge,
    BootstrapOutputFormat,
};
use crate::utils::StdError;
use rauthy_data::email::mailer;
use rauthy_data::migration::bootstrap::generated_secrets::{
    GeneratedSecretEntry, GeneratedSecrets, read_container,
};
use rauthy_data::rauthy_config::RauthyConfig;
use std::fmt::Write as _;
use std::path::Path;
use tokio::sync::mpsc;

pub async fn run(args: ArgsBootstrap) -> Result<(), StdError> {
    match args.command {
        ArgsBootstrapCommand::Get(args) => get(args).await,
        ArgsBootstrapCommand::Purge(args) => purge(args).await,
    }
}

/// Load the existing Rauthy config. This initializes `ENC_KEYS` (so the local
/// container can be decrypted) and resolves the configured container path,
/// exactly like the server does — no keys or file path are passed to the CLI.
async fn load_config(config_file: String) -> Result<&'static RauthyConfig, StdError> {
    let (tx_email, _) = mpsc::channel::<mailer::EMail>(16);
    let (tx_events, _) = flume::unbounded();
    let (tx_events_router, _) = flume::unbounded();
    let (config, _node) = RauthyConfig::build(
        config_file,
        "secrets.toml".to_string(),
        tx_email,
        tx_events,
        tx_events_router,
    )
    .await?;
    config.init_static();
    Ok(RauthyConfig::get())
}

async fn get(args: ArgsBootstrapGet) -> Result<(), StdError> {
    let config = load_config(args.config_file.clone()).await?;
    let path = config.vars.bootstrap.generated_secrets_file.clone();

    let container = read_container(path.as_ref())
        .await
        .map_err(|err| err.message)?;
    let entries = filter_entries(container, &args);
    if entries.is_empty() {
        return Err(format!(
            "no generated bootstrap secret matched kind={:?}, id={:?}, field={:?}",
            args.kind, args.id, args.field
        )
        .into());
    }

    match args.format {
        BootstrapOutputFormat::Raw => {
            println!("{}", render_raw(&entries)?);
            Ok(())
        }
        BootstrapOutputFormat::Json => {
            println!("{}", render_json(&entries)?);
            Ok(())
        }
        BootstrapOutputFormat::Env => {
            print!("{}", render_env(&entries));
            Ok(())
        }
    }
}

async fn purge(args: ArgsBootstrapPurge) -> Result<(), StdError> {
    let config = load_config(args.config_file).await?;
    let path = config.vars.bootstrap.generated_secrets_file.clone();
    purge_path(Path::new(path.as_ref())).await
}

async fn purge_path(path: &Path) -> Result<(), StdError> {
    match tokio::fs::remove_file(path).await {
        Ok(()) => {
            eprintln!(
                "deleted generated bootstrap secret container {}",
                path.display()
            );
            Ok(())
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!(
                "generated bootstrap secret container {} was already absent",
                path.display()
            );
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

fn filter_entries(
    container: GeneratedSecrets,
    args: &ArgsBootstrapGet,
) -> Vec<GeneratedSecretEntry> {
    container
        .entries
        .into_iter()
        .filter(|entry| {
            args.kind.as_ref().is_none_or(|kind| &entry.kind == kind)
                && args.id.as_ref().is_none_or(|id| &entry.id == id)
                && args
                    .field
                    .as_ref()
                    .is_none_or(|field| &entry.field == field)
        })
        .collect()
}

fn render_raw(entries: &[GeneratedSecretEntry]) -> Result<String, StdError> {
    if entries.len() != 1 {
        return Err(format!(
            "--format raw requires exactly one matched secret, got {}",
            entries.len()
        )
        .into());
    }
    Ok(entries[0].value.clone())
}

fn render_json(entries: &[GeneratedSecretEntry]) -> Result<String, StdError> {
    if entries.len() == 1 {
        Ok(serde_json::to_string_pretty(&entries[0])?)
    } else {
        Ok(serde_json::to_string_pretty(entries)?)
    }
}

fn render_env(entries: &[GeneratedSecretEntry]) -> String {
    let mut out = String::new();
    for entry in entries {
        // write! into the buffer avoids the extra String allocation format! does.
        let _ = writeln!(out, "{}={}", entry.env_name(), entry.value);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(kind: &str, id: &str, field: &str, value: &str) -> GeneratedSecretEntry {
        GeneratedSecretEntry {
            kind: kind.to_string(),
            id: id.to_string(),
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    #[test]
    fn filter_entries_matches_exact_selector_parts() {
        let container = GeneratedSecrets {
            entries: vec![
                entry("client", "demo", "secret", "client-secret"),
                entry("user", "demo", "password", "user-password"),
            ],
        };
        let args = ArgsBootstrapGet {
            config_file: "unused".to_string(),
            format: BootstrapOutputFormat::Env,
            kind: Some("client".to_string()),
            id: Some("demo".to_string()),
            field: Some("secret".to_string()),
        };

        let filtered = filter_entries(container, &args);
        assert_eq!(
            filtered,
            vec![entry("client", "demo", "secret", "client-secret")]
        );
    }

    #[test]
    fn env_names_include_kind_id_and_field() {
        let e = entry("user", "can@example.com", "password", "pw");
        assert_eq!(
            e.env_name(),
            "RAUTHY_BOOTSTRAP_USER_CAN_EXAMPLE_COM_PASSWORD"
        );
    }

    #[test]
    fn raw_output_is_only_the_secret_value() {
        let entries = vec![entry("client", "demo", "secret", "client-secret")];
        assert_eq!(render_raw(&entries).unwrap(), "client-secret");
    }

    #[test]
    fn json_output_preserves_entry_fields() {
        let entries = vec![entry("user", "can@example.com", "password", "pw")];
        let value =
            serde_json::from_str::<serde_json::Value>(&render_json(&entries).unwrap()).unwrap();

        assert_eq!(value["kind"], "user");
        assert_eq!(value["id"], "can@example.com");
        assert_eq!(value["field"], "password");
        assert_eq!(value["value"], "pw");
    }

    #[test]
    fn env_output_uses_collision_safe_names() {
        let entries = vec![
            entry("client", "demo", "secret", "client-secret"),
            entry("user", "demo", "password", "user-password"),
        ];

        assert_eq!(
            render_env(&entries),
            "RAUTHY_BOOTSTRAP_CLIENT_DEMO_SECRET=client-secret\n\
             RAUTHY_BOOTSTRAP_USER_DEMO_PASSWORD=user-password\n"
        );
    }

    #[tokio::test]
    async fn purge_is_idempotent() {
        let path =
            std::env::temp_dir().join(format!("rauthy-bootstrap-cli-purge-{}", std::process::id()));
        tokio::fs::write(&path, b"secret container").await.unwrap();

        purge_path(&path).await.unwrap();
        purge_path(&path).await.unwrap();

        assert!(!path.exists());
    }
}
