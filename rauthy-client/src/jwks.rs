use crate::base64_url_no_pad_decode;
use crate::provider::HTTP_CLIENT;
use crate::rauthy_error::RauthyError;
use cached::Cached;
use serde::Deserialize;
use std::borrow::Cow;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info};

static JWKS_TX: OnceLock<mpsc::UnboundedSender<JwksMsg>> = OnceLock::new();

#[derive(Debug)]
pub(crate) enum JwksMsg {
    Get((String, oneshot::Sender<Result<JwkPublicKey, RauthyError>>)),
    Update,
    NewJwksUri(String),
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) enum JwkKeyPairAlg {
    RS256,
    RS384,
    RS512,
    EdDSA,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(clippy::upper_case_acronyms)] // must be uppercase by definition
pub(crate) enum JwkKeyPairType {
    RSA,
    OKP,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // kty and alg are actually used, but inside a macro -> allow dead code
pub(crate) struct JwkPublicKey {
    pub kty: JwkKeyPairType,
    pub alg: JwkKeyPairAlg,
    pub kid: String,
    pub crv: Option<String>,      // Ed25519
    pub n: Option<String>,        // RSA
    pub e: Option<String>,        // RSA
    pub x: Option<String>,        // OCT
    pub x_bytes: Option<Vec<u8>>, // pre-decoded base64 string to speed up checking
}

impl JwkPublicKey {
    #[inline]
    pub(crate) async fn get_for_token(token: &str) -> Result<Self, RauthyError> {
        let metadata = jwt_simple::token::Token::decode_metadata(token)?;
        let kid = metadata
            .key_id()
            .ok_or(RauthyError::InvalidClaims("No 'kid' in JWT token header"))?;

        let (tx, rx) = oneshot::channel();
        JwksMsg::Get((kid.to_string(), tx)).send()?;
        rx.await
            .map_err(|err| RauthyError::Internal(Cow::from(err.to_string())))?
    }
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct JwksCerts {
    pub keys: Vec<JwkPublicKey>,
}

pub(crate) async fn jwks_handler() {
    // initialize channels
    let (tx, mut rx) = mpsc::unbounded_channel();
    if JWKS_TX.set(tx).is_err() {
        error!("Error initializing JWKS_TX");
    }

    // this task will periodically send update requests
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600));
        loop {
            if let Err(err) = JwksMsg::Update.send() {
                error!("Error Updating JWKS - this should never happen: {:?}", err);
            }
            interval.tick().await;
        }
    });

    tokio::spawn(async move {
        let mut jwks_uri = None;

        let mut recently_looked_up = cached::TimedCache::with_lifespan(300);
        // for small collections, Vec will always be faster and more efficient than a HashMap
        let mut jwks: Vec<JwkPublicKey> = Vec::with_capacity(4);

        let update = |jwks_uri: Option<String>, curr_jwks: Vec<JwkPublicKey>| async {
            let uri = if let Some(uri) = jwks_uri {
                uri
            } else {
                debug!("Cannot update JWKS with no configured OIDC provider");
                return curr_jwks;
            };

            info!("Updating JWKS from Rauthy");

            let client = HTTP_CLIENT.get().expect("HTTP_CLIENT to be initialized");
            match client.get(&uri).send().await {
                Ok(res) => {
                    if !res.status().is_success() {
                        error!("Error fetching JWKS from {}", uri);
                        return curr_jwks;
                    }

                    let certs = match res.json::<JwksCerts>().await {
                        Ok(jwks) => jwks,
                        Err(err) => {
                            error!("Error deserializing JWKS from {}: {:?}", uri, err);
                            return curr_jwks;
                        }
                    };

                    certs
                        .keys
                        .into_iter()
                        .filter_map(|mut key| {
                            if key.alg == JwkKeyPairAlg::EdDSA {
                                // we want to pre-decode the byte string here
                                if let Some(x) = &key.x {
                                    match base64_url_no_pad_decode(x) {
                                        Ok(bytes) => {
                                            key.x_bytes = Some(bytes)
                                        }
                                        Err(err) => {
                                            error!("Error pre-decoding given EdDSA 'x' pub key bytes: {}", err);
                                            return None;
                                        }
                                    }
                                }
                            }
                            Some(key)
                        })
                        .collect()
                }
                Err(err) => {
                    error!("Error fetching JWKS from Rauthy {}: {:?}", uri, err);
                    // if we got an error, just return the current JWKs unchanged
                    curr_jwks
                }
            }
        };

        'main: while let Some(msg) = rx.recv().await {
            match msg {
                JwksMsg::Get((kid, tx_ack)) => {
                    for jwk in &jwks {
                        if jwk.kid == kid {
                            tx_ack.send(Ok(jwk.clone())).unwrap();
                            continue 'main;
                        }
                    }

                    // if we get here, the kid was not present
                    // we want to cache recently looked up and failed KIDs to prevent some kind
                    // of DoS possibility with invalid tokens all over
                    if recently_looked_up.cache_get(&kid).is_some() {
                        tx_ack
                            .send(Err(RauthyError::InvalidClaims(
                                "'kid' not found and it has been recently looked up",
                            )))
                            .unwrap();
                        continue;
                    }

                    // try to do an update for the given kid
                    jwks = update(jwks_uri.clone(), jwks).await;
                    recently_looked_up.cache_set(kid.clone(), ());

                    // check again
                    for jwk in &jwks {
                        if jwk.kid == kid {
                            tx_ack.send(Ok(jwk.clone())).unwrap();
                            continue 'main;
                        }
                    }
                    tx_ack
                        .send(Err(RauthyError::InvalidClaims(
                            "'kid' not found after updating JWKs",
                        )))
                        .unwrap();
                }

                JwksMsg::Update => {
                    jwks = update(jwks_uri.clone(), jwks).await;
                }

                JwksMsg::NewJwksUri(uri) => {
                    info!("Received a new JWKS URI: {}", uri);
                    jwks_uri = Some(uri);
                    JwksMsg::Update.send().unwrap();
                }
            }
        }
    });
}
