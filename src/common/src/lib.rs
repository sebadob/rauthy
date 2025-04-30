// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::constants::{DB_TYPE, DEV_MODE, RAUTHY_VERSION};
use reqwest::tls;
use std::env;
use std::sync::LazyLock;
use std::time::Duration;
use tracing::{debug, warn};

pub mod compression;
pub mod constants;
pub mod password_hasher;
pub mod regex;
pub mod utils;

/// TODO make sure that in (almost) all places, this single client is being used for all outgoing
/// requests to reduce duplicate Client builds and the amount of duplicate TLS handshakes.
pub static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    let connect_timeout = env::var("HTTP_CONNECT_TIMEOUT")
        .as_deref()
        .unwrap_or("10")
        .parse::<u8>()
        .expect("Cannot parse HTTP_CONNECT_TIMEOUT as u8");
    let request_timeout = env::var("HTTP_REQUEST_TIMEOUT")
        .as_deref()
        .unwrap_or("10")
        .parse::<u8>()
        .expect("Cannot parse HTTP_REQUEST_TIMEOUT as u8");
    let idle_timeout = env::var("HTTP_IDLE_TIMEOUT")
        .as_deref()
        .unwrap_or("900")
        .parse::<u32>()
        .expect("Cannot parse HTTP_IDLE_TIMEOUT as u32");

    let tls_version = match env::var("HTTP_MIN_TLS").as_deref().unwrap_or("1.3") {
        "1.3" => tls::Version::TLS_1_3,
        "1.2" => tls::Version::TLS_1_2,
        "1.1" => {
            warn!(
                r#"
    You are allowing TLS 1.1 for the global HTTP client.
    Only do this, if you know what you are doing!
    "#
            );
            tls::Version::TLS_1_1
        }
        "1.0" => {
            warn!(
                r#"
    You are allowing TLS 1.0 for the global HTTP client.
    Only do this, if you know what you are doing!
    "#
            );
            tls::Version::TLS_1_0
        }
        _ => panic!("Invalid value for HTTP_MIN_TLS, allowed: '1.3', '1.2', '1.1', '1.0'"),
    };

    let allow_plain_http = env::var("HTTP_DANGER_UNENCRYPTED")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("Cannot parse HTTP_DANGER_UNENCRYPTED as bool");
    if allow_plain_http {
        warn!("HTTP Client allows plain, unencrypted HTTP connections");
    }

    let allow_insecure = env::var("HTTP_DANGER_INSECURE")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("Cannot parse HTTP_DANGER_INSECURE as bool");
    if allow_plain_http {
        warn!("HTTP Client allows insecure TLS connections");
    }

    let mut builder = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(connect_timeout as u64))
        .timeout(Duration::from_secs(request_timeout as u64))
        .pool_idle_timeout(Duration::from_secs(idle_timeout as u64))
        .min_tls_version(tls_version)
        .user_agent(format!("Rauthy v{}", RAUTHY_VERSION))
        .https_only(!allow_plain_http || !*DEV_MODE)
        .danger_accept_invalid_certs(allow_insecure || *DEV_MODE);

    if let Ok(bundle) = env::var("HTTP_CUST_ROOT_CA_BUNDLE").as_deref() {
        let certs = reqwest::Certificate::from_pem_bundle(bundle.trim().as_bytes())
            .expect("Cannot parse given HTTP_CUST_ROOT_CA_BUNDLE");
        debug!(
            "Adding {} custom Root CA certificates to HTTP Client",
            certs.len()
        );

        for cert in certs {
            builder = builder.add_root_certificate(cert);
        }
    }

    builder.build().unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DbType {
    Postgres,
    Hiqlite,
}

impl DbType {
    fn build() -> Self {
        let use_hiqlite = env::var("HIQLITE")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .expect("Cannot parse HIQLITE as bool");

        if use_hiqlite {
            DbType::Hiqlite
        } else {
            Self::Postgres
        }
    }
}

#[inline(always)]
pub fn is_hiqlite() -> bool {
    *DB_TYPE == DbType::Hiqlite
}

#[inline(always)]
pub fn is_postgres() -> bool {
    *DB_TYPE == DbType::Postgres
}
