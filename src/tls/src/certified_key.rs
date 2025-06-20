use arc_swap::ArcSwap;
use notify::event::CreateKind;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rustls::crypto::CryptoProvider;
use rustls::pki_types::PrivateKeyDer;
use rustls::server::{ClientHello, ResolvesServerCert};
use rustls::sign::CertifiedKey;
use rustls_pemfile::Item;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use std::{fs, iter};
use tokio::sync::Mutex;
use tokio::task;
use tracing::{error, info};

#[derive(Debug)]
pub struct CertifiedKeyWatched {
    key_path: String,
    cert_path: String,
    cert_key: ArcSwap<CertifiedKey>,
    watcher: Mutex<Option<RecommendedWatcher>>,
}

impl ResolvesServerCert for CertifiedKeyWatched {
    fn resolve(&self, _: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        Some(self.cert_key.load_full())
    }
}

impl CertifiedKeyWatched {
    pub async fn new(key_path: String, cert_path: String) -> Result<Arc<Self>, ErrorResponse> {
        let kp_cloned = key_path.clone();
        let cp_cloned = cert_path.clone();
        let cert_key = task::spawn_blocking(move || Self::try_load(&kp_cloned, &cp_cloned))
            .await
            .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))??;

        let slf = Arc::new(Self {
            key_path,
            cert_path,
            cert_key: ArcSwap::new(cert_key),
            watcher: Mutex::new(None),
        });

        let watcher = Self::watch_files(slf.clone())?;
        *slf.watcher.lock().await = Some(watcher);

        Ok(slf)
    }

    fn try_load(key_path: &str, cert_path: &str) -> Result<Arc<CertifiedKey>, ErrorResponse> {
        let key_file = fs::read(key_path)?;
        let key = if key_path.ends_with(".der") {
            PrivateKeyDer::try_from(key_file).map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("Cannot load TLS key: {}", err),
                )
            })?
        } else {
            let mut reader = BufReader::new(key_file.as_slice());
            let mut key = None;
            for item in iter::from_fn(|| rustls_pemfile::read_one(&mut reader).transpose()).take(1)
            {
                match item? {
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
            match key {
                None => {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        "No TLS key found",
                    ));
                }
                Some(k) => k,
            }
        };

        let certs_file = fs::read(cert_path)?;
        let mut certs_reader = BufReader::new(certs_file.as_slice());
        let cert_chain = rustls_pemfile::certs(&mut certs_reader)
            .filter_map(|res| match res {
                Ok(cert) => Some(cert),
                Err(err) => {
                    error!("Error parsing certificate data: {:?}", err);
                    None
                }
            })
            .collect::<Vec<_>>();

        let provider = CryptoProvider::get_default().expect("rustls CryptoProvider not installed");
        let ck = CertifiedKey::from_der(cert_chain, key, provider).map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("Cannot build TLS CertifiedKey from Key + Cert: {:?}", err),
            )
        })?;
        ck.keys_match().map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("TLS Key and Certificate don't match: {:?}", err),
            )
        })?;

        Ok(Arc::new(ck))
    }

    fn watch_files(slf: Arc<Self>) -> Result<RecommendedWatcher, ErrorResponse> {
        let key_path = slf.key_path.clone();
        let cert_path = slf.cert_path.clone();

        let mut watcher =
            notify::recommended_watcher(move |res: notify::Result<notify::Event>| match res {
                Ok(ev) => {
                    let should_reload = matches!(
                        ev.kind,
                        EventKind::Create(CreateKind::File) | EventKind::Modify(_)
                    );

                    if should_reload {
                        match Self::try_load(&slf.key_path, &slf.cert_path) {
                            Ok(ck) => {
                                info!("Reloading TLS Key + Certificates");
                                slf.cert_key.store(ck);
                            }
                            Err(err) => {
                                error!("Error loading TLS after file watch event: {:?}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("File watch error: {:?}", err);
                }
            })
            .map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("Cannot start File Watcher: {:?}", err),
                )
            })?;

        watcher
            .watch(Path::new(&key_path), RecursiveMode::NonRecursive)
            .map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("Cannot watch path {}: {:?}", key_path, err),
                )
            })?;
        watcher
            .watch(Path::new(&cert_path), RecursiveMode::NonRecursive)
            .map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("Cannot watch path {}: {:?}", cert_path, err),
                )
            })?;

        Ok(watcher)
    }
}
