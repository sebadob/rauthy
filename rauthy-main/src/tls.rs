use rustls_pemfile::Item;
use std::io::BufReader;
use std::{env, iter};
use tokio::fs;
use tracing::error;

// Loads TLS key and cert file from disk and returns a `rustls::ServerConfig`
pub async fn load_tls() -> rustls::ServerConfig {
    let path_key = env::var("TLS_KEY").unwrap_or_else(|_| "tls/tls.key".to_string());
    let path_cert = env::var("TLS_CERT").unwrap_or_else(|_| "tls/tls.crt".to_string());

    let key_file = fs::read(&path_key).await.expect("Reading TLS private key");
    let key = if path_key.ends_with(".der") {
        rustls::PrivateKey(key_file)
    } else {
        let mut reader = BufReader::new(key_file.as_slice());
        let mut key_bytes = Vec::default();
        for item in iter::from_fn(|| rustls_pemfile::read_one(&mut reader).transpose()).take(1) {
            match item.unwrap() {
                Item::RSAKey(k) => key_bytes = k,
                Item::PKCS8Key(k) => key_bytes = k,
                Item::ECKey(k) => key_bytes = k,
                _ => panic!("Expected a private PEM key in {}", path_key),
            }
        }
        rustls::PrivateKey(key_bytes)
    };

    let certs_file = fs::read(&path_cert).await.expect("Reading TLS certificate");
    let mut certs_reader = BufReader::new(certs_file.as_slice());
    let cert_chain = rustls_pemfile::certs(&mut certs_reader)
        .expect("bad tls cert file")
        .iter()
        .map(|cert| rustls::Certificate(cert.to_owned()))
        .collect();

    rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .map_err(|err| error!("Error building rustls ServerConfig: {}", err))
        .expect("bad certificate/key")
}
