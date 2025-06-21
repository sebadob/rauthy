// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

pub mod certified_key;
pub mod certified_keys;
pub mod error;

/// Creates a `rustls::ServerConfig` with automatic hot-reloading of TLS certificates whenever
/// the given `key_path` / `cert_path` are updated.
pub async fn load_server_config(key_path: String, cert_path: String) -> rustls::ServerConfig {
    let ck = match certified_key::CertifiedKeyWatched::new(key_path, cert_path).await {
        Ok(ck) => ck,
        Err(err) => {
            panic!("Cannot load TLS certificates: {:?}", err)
        }
    };

    rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_cert_resolver(ck)
}
