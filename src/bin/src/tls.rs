use rauthy_data::entity::ca_self_signed::SelfSignedCA;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::fs;
use tokio::task;
use tracing::info;

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
        (key_path, cert_path)
    };

    tls_hot_reload::load_server_config(key_path, cert_path).await
}

async fn check_generate_tls() {
    info!("Generating self-signed TLS certificates");

    let mut exp = create_end_entity()
        .await
        .expect("Cannot generate TLS certificates");

    task::spawn(async move {
        loop {
            let secs = exp.unix_timestamp() - OffsetDateTime::now_utc().unix_timestamp() - 3600;
            if secs > 0 {
                tokio::time::sleep(Duration::from_secs(secs as u64)).await;
            } else {
                panic!("too short lifetime for self-signed end-entity TLS certificates");
            }

            exp = create_end_entity()
                .await
                .expect("Cannot generate TLS certificates");
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
