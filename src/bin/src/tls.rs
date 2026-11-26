use rauthy_data::entity::ca_self_signed::SelfSignedCA;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_data::shutdown;
use rauthy_error::ErrorResponse;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::time::Duration;
use time::OffsetDateTime;
use tokio::fs;
use tokio::task;
use tracing::{error, info};

static SELF_SIGNED_DIR: &str = "tls";
static SELF_SIGNED_KEY: &str = "tls/self_signed_key.pem";
static SELF_SIGNED_CERT: &str = "tls/self_signed_cert.pem";

pub async fn load_tls() -> rustls::ServerConfig {
    let vars = &RauthyConfig::get().vars.tls;

    let (key_path, cert_path) = if vars.generate_self_signed {
        check_generate_tls().await;
        (SELF_SIGNED_KEY.to_string(), SELF_SIGNED_CERT.to_string())
    } else {
        // unwrap is fine here, since it should never be called with both paths set anyway
        let key_path = vars
            .key_path
            .as_deref()
            .unwrap_or("tls/tls.key")
            .to_string();
        let cert_path = vars
            .cert_path
            .as_deref()
            .unwrap_or("tls/tls.crt")
            .to_string();

        if !fs::try_exists(&key_path).await.unwrap_or(false)
            || !fs::try_exists(&cert_path).await.unwrap_or(false)
        {
            error!(
                "Cannot load TLS certificates from {key_path} / {cert_path} - using self-signed as fallback"
            );
            check_generate_tls().await;
            (SELF_SIGNED_KEY.to_string(), SELF_SIGNED_CERT.to_string())
        } else {
            (key_path, cert_path)
        }
    };

    load_server_config(key_path, cert_path).await
}

/// Loads a hot-reloading TLS config, with the poll watcher if `tls.poll_watch_interval` is set.
pub async fn load_server_config(key_path: String, cert_path: String) -> rustls::ServerConfig {
    // `tls_hot_reload` panics if it cannot load the files, which would skip the clean shutdown.
    if let Err(err) = check_tls_files(&key_path, &cert_path).await {
        shutdown::fatal(err).await;
    }

    if let Some(interval) = RauthyConfig::get().vars.tls.poll_watch_interval {
        tracing::warn!(
            "Starting TLS hot-reload with the PollWatcher backend and interval {:?}",
            interval
        );
        tls_hot_reload::load_server_config_polling(key_path, cert_path, interval).await
    } else {
        tls_hot_reload::load_server_config(key_path, cert_path).await
    }
}

async fn check_tls_files(key_path: &str, cert_path: &str) -> Result<(), String> {
    let key = fs::read(key_path)
        .await
        .map_err(|err| format!("Cannot read TLS key {key_path}: {err}"))?;
    PrivateKeyDer::from_pem_slice(&key)
        .map_err(|err| format!("Invalid PEM private key in {key_path}: {err}"))?;

    let cert = fs::read(cert_path)
        .await
        .map_err(|err| format!("Cannot read TLS certificate {cert_path}: {err}"))?;
    match CertificateDer::pem_slice_iter(&cert).collect::<Result<Vec<_>, _>>() {
        Ok(chain) if !chain.is_empty() => Ok(()),
        Ok(_) => Err(format!("No certificate found in {cert_path}")),
        Err(err) => Err(format!(
            "Invalid PEM certificate chain in {cert_path}: {err}"
        )),
    }
}

async fn check_generate_tls() {
    info!("Generating self-signed TLS certificates");

    let mut exp = match create_end_entity().await {
        Ok(exp) => exp,
        Err(err) => shutdown::fatal(format!("Cannot generate TLS certificates: {err}")).await,
    };

    task::spawn(async move {
        loop {
            let secs = exp.unix_timestamp() - OffsetDateTime::now_utc().unix_timestamp() - 3600;
            if secs > 0 {
                tokio::time::sleep(Duration::from_secs(secs as u64)).await;
            } else {
                shutdown::fatal("too short lifetime for self-signed end-entity TLS certificates")
                    .await;
            }

            exp = match create_end_entity().await {
                Ok(exp) => exp,
                Err(err) => {
                    shutdown::fatal(format!("Cannot renew the TLS certificates: {err}")).await
                }
            };
        }
    });
}

async fn create_end_entity() -> Result<OffsetDateTime, ErrorResponse> {
    let ca = SelfSignedCA::find_or_generate_new().await?;
    let end_entity = ca.create_end_entity().await?;

    fs::create_dir_all(SELF_SIGNED_DIR).await?;
    fs::write(SELF_SIGNED_KEY, end_entity.key_pem).await?;
    fs::write(SELF_SIGNED_CERT, end_entity.cert_chain_pem).await?;

    Ok(end_entity.exp)
}
