use rauthy_models::rauthy_config::RauthyConfig;
use rustls_pemfile::Item;
use rustls_pki_types::PrivateKeyDer;
use std::io::BufReader;
use std::iter;
use tokio::fs;
use tracing::error;

/// Loads TLS key and cert file from disk and returns a `rustls::ServerConfig`
pub async fn load_tls() -> rustls::ServerConfig {
    let paths = &RauthyConfig::get().vars.tls;

    let key_path = paths.key_path.as_deref().unwrap_or("tls/tls.key");
    let cert_path = paths.cert_path.as_deref().unwrap_or("tls/tls.key");

    let key_file = fs::read(key_path).await.expect("Reading TLS private key");
    let key = if key_path.ends_with(".der") {
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
                _ => panic!("Expected a private PEM key in {}", key_path),
            }
        }
        key.expect("no valid TLS private key found")
    };

    let certs_file = fs::read(cert_path).await.expect("Reading TLS certificate");
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
