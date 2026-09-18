use crate::provider::{HTTP_CLIENT, OidcProvider};
use crate::rauthy_error::RauthyError;
use crate::{base64_url_no_pad_decode, base64_url_no_pad_decode_buf};
use arc_swap::ArcSwap;
use chrono::Utc;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use tracing::{debug, error, info, warn};

static JWKS: OnceLock<ArcSwap<BTreeMap<String, JwkPublicKey>>> = OnceLock::new();
static JWKS_LAST_SYNC: AtomicI64 = AtomicI64::new(0);
static JWKS_TX: OnceLock<mpsc::UnboundedSender<JwksMsg>> = OnceLock::new();

#[derive(Debug)]
enum JwksMsg {
    Update(oneshot::Sender<()>),
}

impl JwksMsg {
    pub(crate) fn send(self) -> Result<(), RauthyError> {
        JWKS_TX
                .get()
                .ok_or(RauthyError::Init("JWKS_TX has not been initialized"))?
                .send(self)
                .map_err(|err| RauthyError::Internal(Cow::from(err.to_string())))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub enum JwkKeyPairAlg {
    RS256,
    RS384,
    RS512,
    // Current or "old" notation for Ed25519
    #[default]
    EdDSA,
    // New notation for EdDSA (RFC 9864)
    Ed25519,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(clippy::upper_case_acronyms)] // must be uppercase by definition
pub(crate) enum JwkKeyPairType {
    RSA,
    OKP,
}

// TODO if it would still be `Send`, instead of storing the raw bytes for key parts and so on,
//  build the full ready-to-use pubkey only once after fetch.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // kty and alg are actually used, but inside a macro -> allow dead code
pub(crate) struct JwkPublicKey {
    pub kty: JwkKeyPairType,
    pub alg: JwkKeyPairAlg,
    pub kid: String,
    pub crv: Option<String>,      // Ed25519
    pub n: Option<String>,        // RSA
    pub n_bytes: Option<Vec<u8>>, // pre-decoded base64 string to speed up checking
    pub e: Option<String>,        // RSA
    pub e_bytes: Option<Vec<u8>>, // pre-decoded base64 string to speed up checking
    pub x: Option<String>,        // OCT
    pub x_bytes: Option<Vec<u8>>, // pre-decoded base64 string to speed up checking
}

impl JwkPublicKey {
    /// Validates the given raw token into the provided `buf`.
    #[inline]
    pub(crate) async fn validate_token(
        alg: JwkKeyPairAlg,
        kid: String,
        token: &str,
        buf: &mut Vec<u8>,
    ) -> Result<(), RauthyError> {
        if let Some(jwks) = JWKS.get()
                && let Some(jwk) = jwks.load().get(&kid)
        {
            jwk.validate_token_alg(alg)?;
            jwk.validate_token_signature(token, buf)?;
        } else {
            if JWKS_LAST_SYNC.load(Ordering::Relaxed) >= Utc::now().timestamp() - 1 {
                return Err(RauthyError::JWK(
                    format!("Cannot find JWK with kid {kid}").into(),
                ));
            }

            let (tx, rx) = oneshot::channel();
            JwksMsg::Update(tx).send()?;
            rx.await
                    .map_err(|err| RauthyError::Internal(Cow::from(err.to_string())))?;

            if let Some(jwks) = JWKS.get()
                    && let Some(jwk) = jwks.load().get(&kid)
            {
                jwk.validate_token_alg(alg)?;
                jwk.validate_token_signature(token, buf)?;
            } else {
                return Err(RauthyError::JWK(
                    format!("Cannot find JWK with kid {kid}").into(),
                ));
            }
        }

        Ok(())
    }

    #[cfg(feature = "rsa")]
    #[inline(always)]
    fn e(&self) -> Result<rsa::BigUint, RauthyError> {
        match &self.e_bytes {
            None => Err(RauthyError::JWK("Missing 'e' in JWK".into())),
            Some(bytes) => Ok(rsa::BigUint::from_bytes_be(bytes)),
        }
    }

    #[cfg(feature = "rsa")]
    #[inline(always)]
    fn n(&self) -> Result<rsa::BigUint, RauthyError> {
        match &self.n_bytes {
            None => Err(RauthyError::JWK("Missing 'n' in JWK".into())),
            Some(bytes) => Ok(rsa::BigUint::from_bytes_be(bytes)),
        }
    }

    #[inline(always)]
    fn x(&self) -> Result<&[u8], RauthyError> {
        match &self.x_bytes {
            None => Err(RauthyError::JWK("Missing 'x' in JWK".into())),
            Some(bytes) => Ok(bytes),
        }
    }

    #[inline(always)]
    fn validate_token_alg(&self, alg: JwkKeyPairAlg) -> Result<(), RauthyError> {
        if self.alg == alg {
            Ok(())
        } else {
            Err(RauthyError::JWK(format!("Mismatch in 'alg' for JWK with kid {}", self.kid).into()))
        }
    }

    #[inline(always)]
    pub fn validate_token_signature(
        &self,
        token: &str,
        buf: &mut Vec<u8>,
    ) -> Result<(), RauthyError> {
        let (message, sig) = token
                .rsplit_once('.')
                .ok_or(RauthyError::MalformedJwt("Malformed token"))?;

        buf.clear();
        base64_url_no_pad_decode_buf(sig, buf)?;

        match self.alg {
            JwkKeyPairAlg::RS256 => {
                #[cfg(feature = "rsa")]
                {
                    let hash = hmac_sha256::Hash::hash(message.as_bytes());
                    let rsa_pk = rsa::RsaPublicKey::new(self.n()?, self.e()?)?;
                    if rsa_pk
                            .verify(
                                rsa::Pkcs1v15Sign::new::<sha2::Sha256>(),
                                hash.as_slice(),
                                buf,
                            )
                            .is_ok()
                    {
                        return Ok(());
                    }
                }
                #[cfg(not(feature = "rsa"))]
                error!("Cannot validate RSA tokens without the `rsa` feature");
            }

            JwkKeyPairAlg::RS384 => {
                #[cfg(feature = "rsa")]
                {
                    let hash = hmac_sha512::sha384::Hash::hash(message.as_bytes());
                    let rsa_pk = rsa::RsaPublicKey::new(self.n()?, self.e()?)?;
                    if rsa_pk
                            .verify(
                                rsa::Pkcs1v15Sign::new::<sha2::Sha384>(),
                                hash.as_slice(),
                                buf,
                            )
                            .is_ok()
                    {
                        return Ok(());
                    }
                }
                #[cfg(not(feature = "rsa"))]
                error!("Cannot validate RSA tokens without the `rsa` feature");
            }

            JwkKeyPairAlg::RS512 => {
                #[cfg(feature = "rsa")]
                {
                    let hash = hmac_sha512::Hash::hash(message.as_bytes());
                    let rsa_pk = rsa::RsaPublicKey::new(self.n()?, self.e()?)?;
                    if rsa_pk
                            .verify(
                                rsa::Pkcs1v15Sign::new::<sha2::Sha512>(),
                                hash.as_slice(),
                                buf,
                            )
                            .is_ok()
                    {
                        return Ok(());
                    }
                }
                #[cfg(not(feature = "rsa"))]
                error!("Cannot validate RSA tokens without the `rsa` feature");
            }

            JwkKeyPairAlg::EdDSA | JwkKeyPairAlg::Ed25519 => {
                let pubkey = ed25519_compact::PublicKey::from_slice(self.x()?)?;
                let signature = ed25519_compact::Signature::from_slice(buf)?;
                if pubkey.verify(message, &signature).is_ok() {
                    return Ok(());
                }
            }
        };

        warn!("JWT Token validation error");
        Err(RauthyError::InvalidJwt("Invalid JWT Token signature"))
    }
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct JwksCerts {
    pub keys: Vec<JwkPublicKey>,
}

impl JwksCerts {
    pub(crate) fn spawn_update_task() {
        let (tx, rx) = mpsc::unbounded_channel();
        if JWKS_TX.set(tx).is_err() {
            error!("Error initializing JWKS_TX");
        }
        task::spawn(async move { Self::update_task(rx).await });
    }

    async fn update_task(mut rx: UnboundedReceiver<JwksMsg>) {
        let mut jwks_uri: Option<String> = None;

        loop {
            let sleep_secs = if JWKS.get().is_some() { 1800 } else { 1 };
            let msg = tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(sleep_secs)) => {
                    debug!("Updating JWKS after timeout");
                    None
                },
                msg = rx.recv() => {
                    let Some(msg) = msg else {
                        debug!("Received None in JwksCerts::update_task() - exiting");
                        break;
                    };
                    Some(msg)
                }
            };

            let uri = match &jwks_uri {
                None => {
                    let Ok(cfg) = OidcProvider::config() else {
                        debug!("OIDC Provider information missing - waiting for JWKS URI");
                        continue;
                    };
                    info!("Rauthy JWKS URI: {}", cfg.provider.jwks_uri);
                    jwks_uri = Some(cfg.provider.jwks_uri.clone());
                    jwks_uri.as_ref().unwrap()
                }
                Some(uri) => uri,
            };

            info!("Updating JWKS from Rauthy");

            let client = HTTP_CLIENT.get().expect("HTTP_CLIENT to be initialized");
            match client.get(uri).send().await {
                Ok(res) => {
                    if !res.status().is_success() {
                        error!("Error fetching JWKS from {}", uri);
                        continue;
                    }

                    let certs = match res.json::<JwksCerts>().await {
                        Ok(jwks) => jwks,
                        Err(err) => {
                            error!("Error deserializing JWKS from {}: {:?}", uri, err);
                            continue;
                        }
                    };

                    let mut keys = BTreeMap::new();

                    for mut key in certs.keys {
                        if key.alg == JwkKeyPairAlg::EdDSA || key.alg == JwkKeyPairAlg::Ed25519 {
                            // we want to pre-decode the byte string here
                            if let Some(x) = &key.x {
                                match base64_url_no_pad_decode(x) {
                                    Ok(bytes) => key.x_bytes = Some(bytes),
                                    Err(err) => {
                                        error!(
                                            "Error pre-decoding given EdDSA 'x' pub key bytes: {}",
                                            err
                                        );
                                        continue;
                                    }
                                }
                            }
                        } else {
                            if let Some(e) = &key.e {
                                match base64_url_no_pad_decode(e) {
                                    Ok(bytes) => key.e_bytes = Some(bytes),
                                    Err(err) => {
                                        error!(
                                            "Error pre-decoding given RSA 'e' pub key bytes: {}",
                                            err
                                        );
                                        continue;
                                    }
                                }
                            }
                            if let Some(n) = &key.n {
                                match base64_url_no_pad_decode(n) {
                                    Ok(bytes) => key.n_bytes = Some(bytes),
                                    Err(err) => {
                                        error!(
                                            "Error pre-decoding given RSA 'e' pub key bytes: {}",
                                            err
                                        );
                                        continue;
                                    }
                                }
                            }
                        }
                        keys.insert(key.kid.clone(), key);
                    }

                    if !keys.is_empty() {
                        if let Some(jwks) = JWKS.get() {
                            jwks.store(Arc::new(keys));
                        } else {
                            JWKS.set(ArcSwap::from_pointee(keys)).unwrap();
                        }
                    }
                    if let Some(JwksMsg::Update(ack)) = msg {
                        // just a notification that the global store was updated
                        let _ = ack.send(());
                    }
                    JWKS_LAST_SYNC.store(Utc::now().timestamp(), Ordering::Relaxed);

                    info!("Rauthy JWKS update successful");
                }
                Err(err) => {
                    error!("Error fetching JWKS from Rauthy {}: {:?}", uri, err);
                }
            }
        }
    }
}
