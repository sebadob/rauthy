use rauthy_common::utils::base64_decode;
use rauthy_models::rauthy_config::RauthyConfig;
use rauthy_models::vault_config::VaultConfig;
use rustls_pemfile::Item;
use rustls_pki_types::PrivateKeyDer;
use std::io::BufReader;
use std::iter;
use tokio::fs;
use tracing::error;

/// Loads TLS key and cert file from disk and returns a `rustls::ServerConfig`
pub async fn load_tls() -> rustls::ServerConfig {
    let paths = &RauthyConfig::get().vars.tls;

    if paths.cert_path_vault.is_some() && paths.key_path_vault.is_some() {
        return load_tls_vault().await;
    }

    let key_path = paths
        .key_path
        .as_deref()
        .unwrap_or("tls/tls.key")
        .to_string();
    let cert_path = paths
        .cert_path
        .as_deref()
        .unwrap_or("tls/tls.crt")
        .to_string();

    rauthy_tls::load_server_config(key_path, cert_path).await
}

pub async fn load_tls_vault() -> rustls::ServerConfig {
    println!("#### [load_tls_vault]");

    let paths = &RauthyConfig::get().vars.tls;

    let mut key_path_vault = paths.key_path_vault.as_deref().unwrap_or("cert.key");
    println!("#### key_path_vault:");
    println!("{}",&key_path_vault);

    let cert_path_vault = paths.cert_path_vault.as_deref().unwrap_or("cert.pem");

    let vault_source_certs = &VaultConfig::get().vault_rauthy;

    let vault_secrets = vault_source_certs.get_certs().await.unwrap();

    let key_file = vault_secrets[key_path_vault].as_str().expect("Cannot read TLS private key");
    
    //if using DER it needs to be saved base64 eoncoded in the vault
    let key = if key_path_vault.ends_with(".der") {
        let key_bytes = base64_decode(key_file).ok().unwrap();
        println!("#### [load_tls_vault] got private key DER from vault");
        PrivateKeyDer::try_from(key_bytes).expect("TLS private key to be valid")
    } else {
        let mut reader = BufReader::new(key_file.as_bytes());
        let mut key = None;
        for item in iter::from_fn(|| rustls_pemfile::read_one(&mut reader).transpose()).take(1) {
            match item.unwrap() {
                Item::Pkcs1Key(k) => {
                    key = Some(PrivateKeyDer::Pkcs1(k));
                }
                Item::Pkcs8Key(k) => {
                    key = Some(PrivateKeyDer::Pkcs8(k));
                }
                Item::Sec1Key(k) => {
                    key = Some(PrivateKeyDer::Sec1(k));
                }
                _ => panic!("Expected a private PEM key in {}", key_path_vault),
            }
        }
        key.expect("no valid TLS private key found")
    };

    let certs_file = vault_secrets[cert_path_vault].as_str().expect("Cannot read TLS certificate");
    let mut certs_reader = BufReader::new(certs_file.as_bytes());
    let cert_chain = rustls_pemfile::certs(&mut certs_reader)
        .map(|cert| cert.expect("Invalid TLS certificate file"))
        .collect();

    println!("#### [load_tls_vault] Certs loaded.");

    rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .map_err(|err| error!("Error building rustls ServerConfig: {}", err))
        .expect("bad certificate/key")
}