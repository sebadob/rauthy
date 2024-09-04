use rustls_pemfile::Item;
use rustls_pki_types::PrivateKeyDer;
use std::io::BufReader;
use std::{env, iter};
use tokio::fs;
use tracing::error;

/// Loads TLS key and cert file from disk and returns a `rustls::ServerConfig`
pub async fn load_tls() -> rustls::ServerConfig {
    let path_key = env::var("TLS_KEY").unwrap_or_else(|_| "tls/tls.key".to_string());
    let path_cert = env::var("TLS_CERT").unwrap_or_else(|_| "tls/tls.crt".to_string());

    let key_file = fs::read(&path_key).await.expect("Reading TLS private key");
    let key = if path_key.ends_with(".der") {
        PrivateKeyDer::try_from(key_file).expect("TLS private key to be valid")
    } else {
        let mut reader = BufReader::new(key_file.as_slice());
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
                _ => panic!("Expected a private PEM key in {}", path_key),
            }
        }
        key.expect("no valid TLS private key found")
    };

    let certs_file = fs::read(&path_cert).await.expect("Reading TLS certificate");
    let mut certs_reader = BufReader::new(certs_file.as_slice());
    let cert_chain = rustls_pemfile::certs(&mut certs_reader)
        .map(|cert| cert.expect("Invalid TLS certificate file"))
        .collect();

    rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .map_err(|err| error!("Error building rustls ServerConfig: {}", err))
        .expect("bad certificate/key")
}
