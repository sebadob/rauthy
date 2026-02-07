use crate::database::{Cache, DB};
use crate::events::event::Event;
use crate::rauthy_config::RauthyConfig;
use actix_web::web;
use cryptr::{EncKeys, EncValue};
use ed25519_compact::Noise;
use hiqlite_macros::params;
use postgres_types::Type;
use rauthy_api_types::oidc::{JWKSCerts, JWKSPublicKeyCerts};
use rauthy_common::constants::{
    APPLICATION_JSON, CACHE_TTL_APP, IDX_JWK_KID, IDX_JWK_LATEST, IDX_JWKS,
};
use rauthy_common::utils::{
    base64_url_encode, base64_url_no_pad_decode, base64_url_no_pad_decode_buf, get_rand,
};
use rauthy_common::{http_client, is_hiqlite};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::CONTENT_TYPE;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::traits::PublicKeyParts;
use rsa::{BigUint, sha2};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::{error, info, warn};

/**
The Json Web Keys are saved encrypted inside the database. The encryption is the same as for a
Client secret -> *ChaCha20Poly1305*
 */
#[derive(Serialize, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub created_at: i64,
    pub signature: JwkKeyPairAlg,
    pub enc_key_id: String,
    pub jwk: Vec<u8>,
}

impl Debug for Jwk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Jwk {{ kid: {}, created_at: {}, signature: {:?}, enc_key_id: {}, jwk: <hidden> }}",
            self.kid, self.created_at, self.signature, self.enc_key_id,
        )
    }
}

impl From<tokio_postgres::row::Row> for Jwk {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            kid: row.get("kid"),
            created_at: row.get("created_at"),
            signature: row.get("signature"),
            enc_key_id: row.get("enc_key_id"),
            jwk: row.get("jwk"),
        }
    }
}

// CRUD
impl Jwk {
    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let sig_str = self.signature.as_str();
        let sql = r#"
INSERT INTO jwks (kid, created_at, signature, enc_key_id, jwk)
VALUES ($1, $2, $3, $4, $5)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.kid.clone(),
                        self.created_at,
                        sig_str,
                        self.enc_key_id.clone(),
                        self.jwk.clone()
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.kid,
                    &self.created_at,
                    &sig_str,
                    &self.enc_key_id,
                    &self.jwk,
                ],
            )
            .await?;
        }

        Ok(())
    }
}

impl Jwk {
    pub fn new(
        kid: String,
        created_at: time::OffsetDateTime,
        signature: JwkKeyPairAlg,
        enc_key_id: String,
        jwk: Vec<u8>,
    ) -> Self {
        let ts = created_at.unix_timestamp();
        Self {
            kid,
            created_at: ts,
            signature,
            enc_key_id,
            jwk,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct JWKS {
    pub keys: Vec<JWKSPublicKey>,
}

// CRUD
impl JWKS {
    pub async fn find_pk() -> Result<JWKS, ErrorResponse> {
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::App, IDX_JWKS).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM jwks";
        let res: Vec<Jwk> = if is_hiqlite() {
            client.query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 8).await?
        };

        let mut jwks = JWKS::default();
        for cert in res {
            // let key = data.enc_keys.get(&cert.enc_key_id).unwrap();
            // let jwk_decrypted = decrypt_legacy(&cert.jwk, key)?;
            let jwk_decrypted = EncValue::try_from(cert.jwk)?.decrypt()?.to_vec();
            let kp = JwkKeyPair {
                kid: cert.kid.clone(),
                typ: cert.signature,
                bytes: jwk_decrypted,
            };
            jwks.add_jwk(&kp)?;
        }

        client
            .put(Cache::App, IDX_JWKS, &jwks, CACHE_TTL_APP)
            .await?;

        Ok(jwks)
    }
}

impl JWKS {
    pub fn add_jwk(&mut self, key_pair: &JwkKeyPair) -> Result<(), ErrorResponse> {
        let pub_key = JWKSPublicKey::from_key_pair(key_pair)?;
        self.keys.push(pub_key);
        Ok(())
    }

    /// Rotates and generates a whole new Set of JWKs for signing JWT Tokens
    pub async fn rotate() -> Result<(), ErrorResponse> {
        info!("Starting JWKS rotation - this might take some time");

        // let key = data.enc_keys.get(&data.enc_key_active).unwrap();
        let enc_key_active = &EncKeys::get_static().enc_key_active;

        // RS256
        let jwk_plain = web::block(|| {
            let mut rng = rand_08::thread_rng();
            rsa::RsaPrivateKey::new(&mut rng, 2048)
                .unwrap()
                .to_pkcs8_der()
                .unwrap()
        })
        .await?;
        let jwk = EncValue::encrypt(jwk_plain.to_bytes().as_slice())?
            .into_bytes()
            .to_vec();
        let entity = Jwk {
            kid: get_rand(24),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::RS256,
            enc_key_id: enc_key_active.to_string(),
            jwk,
        };
        entity.save().await?;

        // RS384
        let jwk_plain = web::block(|| {
            let mut rng = rand_08::thread_rng();
            rsa::RsaPrivateKey::new(&mut rng, 3072)
                .unwrap()
                .to_pkcs8_der()
                .unwrap()
        })
        .await?;
        let jwk = EncValue::encrypt(jwk_plain.to_bytes().as_slice())?
            .into_bytes()
            .to_vec();
        let entity = Jwk {
            kid: get_rand(24),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::RS384,
            enc_key_id: enc_key_active.to_string(),
            jwk,
        };
        entity.save().await?;

        // RSA512
        let jwk_plain = web::block(|| {
            let mut rng = rand_08::thread_rng();
            rsa::RsaPrivateKey::new(&mut rng, 4096)
                .unwrap()
                .to_pkcs8_der()
                .unwrap()
        })
        .await?;
        let jwk = EncValue::encrypt(jwk_plain.to_bytes().as_slice())?
            .into_bytes()
            .to_vec();
        let entity = Jwk {
            kid: get_rand(24),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::RS512,
            enc_key_id: enc_key_active.to_string(),
            jwk,
        };
        entity.save().await?;

        // Ed25519
        let jwk_plain = web::block(ed25519_compact::KeyPair::generate).await?;
        let jwk = EncValue::encrypt(jwk_plain.sk.to_der().as_slice())?
            .into_bytes()
            .to_vec();
        let entity = Jwk {
            kid: get_rand(24),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::EdDSA,
            enc_key_id: enc_key_active.to_string(),
            jwk,
        };
        entity.save().await?;

        // clear all latest_jwk from cache
        let client = DB::hql();
        client
            .delete(
                Cache::App,
                format!("{IDX_JWK_LATEST}{}", JwkKeyPairAlg::RS256.as_str()),
            )
            .await?;
        client
            .delete(
                Cache::App,
                format!("{IDX_JWK_LATEST}{}", JwkKeyPairAlg::RS384.as_str()),
            )
            .await?;
        client
            .delete(
                Cache::App,
                format!("{IDX_JWK_LATEST}{}", JwkKeyPairAlg::RS512.as_str()),
            )
            .await?;
        client
            .delete(
                Cache::App,
                format!("{IDX_JWK_LATEST}{}", JwkKeyPairAlg::EdDSA.as_str()),
            )
            .await?;

        // clear the all_certs / JWKS cache
        client.delete(Cache::App, IDX_JWKS).await?;

        info!("Finished JWKS rotation");

        RauthyConfig::get()
            .tx_events
            .send_async(Event::jwks_rotated())
            .await
            .unwrap();

        Ok(())
    }
}

impl From<JWKS> for JWKSCerts {
    fn from(jwks: JWKS) -> Self {
        let mut keys = Vec::with_capacity(jwks.keys.len());
        for k in jwks.keys {
            keys.push(JWKSPublicKeyCerts::from(k));
        }
        Self { keys }
    }
}

// Note: do not add `serde(skip_serializing_if = "Option::is_none")` here.
// This will lead to cache errors when deserializing the JWKS with bincode.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct JWKSPublicKey {
    pub kty: JwkKeyPairType,
    pub alg: Option<JwkKeyPairAlg>,
    pub crv: Option<String>, // Ed25519
    pub kid: Option<String>,
    pub n: Option<String>, // RSA
    pub e: Option<String>, // RSA
    pub x: Option<String>, // OKP
}

impl JWKSPublicKey {
    #[inline]
    pub fn alg(&self) -> Result<&JwkKeyPairAlg, ErrorResponse> {
        if let Some(alg) = &self.alg {
            Ok(alg)
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "No 'alg' in JwkKeyPublicKey",
            ))
        }
    }

    #[inline]
    pub fn n(&self) -> Result<BigUint, ErrorResponse> {
        if let Some(n) = &self.n {
            let decoded = base64_url_no_pad_decode(n)?;
            Ok(BigUint::from_bytes_be(&decoded))
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "No 'n' in JwkKeyPublicKey",
            ))
        }
    }

    #[inline]
    pub fn e(&self) -> Result<BigUint, ErrorResponse> {
        if let Some(e) = &self.e {
            let decoded = base64_url_no_pad_decode(e)?;
            Ok(BigUint::from_bytes_be(&decoded))
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "No 'e' in JwkKeyPublicKey",
            ))
        }
    }

    #[inline]
    pub fn x(&self) -> Result<Vec<u8>, ErrorResponse> {
        if let Some(x) = &self.x {
            Ok(base64_url_no_pad_decode(x)?)
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "No 'x' in JwkKeyPublicKey",
            ))
        }
    }

    #[inline]
    pub fn from_key_pair(key_pair: &JwkKeyPair) -> Result<Self, ErrorResponse> {
        let slf = match key_pair.typ {
            JwkKeyPairAlg::RS256 | JwkKeyPairAlg::RS384 | JwkKeyPairAlg::RS512 => {
                let key = rsa::RsaPrivateKey::from_pkcs8_der(&key_pair.bytes)?;
                let pubkey = key.to_public_key();
                Self {
                    kty: JwkKeyPairType::RSA,
                    alg: Some(key_pair.typ.clone()),
                    crv: None,
                    kid: Some(key_pair.kid.clone()),
                    n: Some(base64_url_encode(&pubkey.n().to_bytes_be())),
                    e: Some(base64_url_encode(&pubkey.e().to_bytes_be())),
                    x: None,
                }
            }
            JwkKeyPairAlg::EdDSA => {
                let key = ed25519_compact::SecretKey::from_der(&key_pair.bytes)?;
                let x = base64_url_encode(key.public_key().as_slice());
                Self {
                    kty: JwkKeyPairType::OKP,
                    alg: Some(key_pair.typ.clone()),
                    crv: Some("Ed25519".to_string()),
                    kid: Some(key_pair.kid.clone()),
                    n: None,
                    e: None,
                    x: Some(x),
                }
            }
        };

        Ok(slf)
    }

    /// Tries to fetch a JWKS from a remote location, parses the whole content and caches it by
    /// `kid`. If the given `kid` exists in the remote JWKS, it will be returned.
    /// This function implements a way of caching that makes a DoS impossible, because it will
    /// cache fetch / deserialization errors as well without making endless requests to invalid URLs.
    /// A JWK will be cached for 1 hour.
    pub async fn fetch_remote(jwks_uri: &str, kid: String) -> Result<Self, ErrorResponse> {
        if let Some(res) = DB::hql()
            .get::<_, _, Result<Self, ErrorResponse>>(Cache::JwksRemote, kid.clone())
            .await?
        {
            return res;
        }

        let res = match http_client()
            .get(jwks_uri)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await
        {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<JWKS>().await {
                        Ok(jwks) => {
                            if let Some(key) = jwks
                                .keys
                                .into_iter()
                                .find(|k| k.kid.as_deref() == Some(&kid))
                            {
                                if let Err(err) = key.validate_self() {
                                    Err(err)
                                } else {
                                    Ok(key)
                                }
                            } else {
                                Err(ErrorResponse::new(
                                    ErrorResponseType::NotFound,
                                    "cannot find given `kid` in JWKS",
                                ))
                            }
                        }
                        Err(err) => {
                            error!("{}", err);
                            Err(ErrorResponse::new(
                                ErrorResponseType::Connection,
                                "Error decoding JWKS from",
                            ))
                        }
                    }
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::Connection,
                        format!("Error connecting to {jwks_uri}"),
                    ))
                }
            }
            Err(err) => {
                error!("{}", err);
                Err(ErrorResponse::new(
                    ErrorResponseType::Connection,
                    format!("Error connecting to {jwks_uri}"),
                ))
            }
        };

        DB::hql()
            .put(Cache::JwksRemote, kid, &res, Some(3600))
            .await?;

        res
    }

    /// Creates a JWK thumbprint by https://datatracker.ietf.org/doc/html/rfc7638
    pub fn fingerprint(&self) -> Result<String, ErrorResponse> {
        let s = match self.kty {
            JwkKeyPairType::RSA => {
                if self.e.is_none() || self.n.is_none() {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        "Incorrect format for RSA JWK: e / n missing",
                    ));
                }

                // mandatory keys for RSA are in order: e, kty, n
                let e = self.e.as_deref().unwrap();
                let n = self.n.as_deref().unwrap();
                format!(
                    "{{\"e\":\"{e}\",\"kty\":\"{}\",\"n\":\"{n}\"}}",
                    self.kty.as_str(),
                )
            }

            JwkKeyPairType::OKP => {
                if self.crv.is_none() || self.x.is_none() {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        "Incorrect format for OKP JWK: crv / x missing",
                    ));
                }

                // mandatory keys for OKP are in order: crv, kty, x
                let crv = self.crv.as_deref().unwrap();
                let x = self.x.as_deref().unwrap();
                format!(
                    "{{\"crv\":\"{crv}\",\"kty\":\"{}\",\"x\":\"{x}\"}}",
                    self.kty.as_str(),
                )
            }
        };

        let hash = hmac_sha256::Hash::hash(s.as_bytes());
        let b64 = base64_url_encode(hash.as_slice());
        Ok(b64)
    }

    /// Validates a reconstructed key from a remote location against Rauthy's supported values.
    pub fn validate_self(&self) -> Result<(), ErrorResponse> {
        match &self.alg {
            None => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "No 'alg' for JWK",
            )),
            Some(alg) => {
                match self.kty {
                    JwkKeyPairType::RSA => {
                        if alg == &JwkKeyPairAlg::EdDSA {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "RSA kty cannot have EdDSA alg",
                            ));
                        }

                        if self.n.is_none() || self.e.is_none() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "No public key components for RSA key",
                            ));
                        }

                        if self.x.is_some() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "RSA key cannot have 'x' public key component",
                            ));
                        }
                    }

                    JwkKeyPairType::OKP => {
                        if alg != &JwkKeyPairAlg::EdDSA {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "OKP kty must have EdDSA alg",
                            ));
                        }

                        if self.crv.is_none() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "OKP kty must have 'crv'",
                            ));
                        }
                        if let Some(crv) = &self.crv
                            && crv != "Ed25519"
                        {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "Only 'Ed25519' for 'crv' is supported",
                            ));
                        }

                        if self.n.is_some() || self.e.is_some() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "EdDSA key cannot have 'n' or 'e' public key components",
                            ));
                        }

                        if self.x.is_none() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "OKP key must have 'x' public key component",
                            ));
                        }
                    }
                }

                Ok(())
            }
        }
    }
}

impl From<JWKSPublicKey> for JWKSPublicKeyCerts {
    fn from(pk: JWKSPublicKey) -> Self {
        let kty = match pk.kty {
            JwkKeyPairType::RSA => rauthy_api_types::oidc::JwkKeyPairType::RSA,
            JwkKeyPairType::OKP => rauthy_api_types::oidc::JwkKeyPairType::OKP,
        };
        let alg = match pk.alg.unwrap_or_default() {
            JwkKeyPairAlg::RS256 => rauthy_api_types::oidc::JwkKeyPairAlg::RS256,
            JwkKeyPairAlg::RS384 => rauthy_api_types::oidc::JwkKeyPairAlg::RS384,
            JwkKeyPairAlg::RS512 => rauthy_api_types::oidc::JwkKeyPairAlg::RS512,
            JwkKeyPairAlg::EdDSA => rauthy_api_types::oidc::JwkKeyPairAlg::EdDSA,
        };

        Self {
            kty,
            _use: "sig",
            alg,
            crv: pk.crv,
            kid: pk.kid,
            n: pk.n,
            e: pk.e,
            x: pk.x,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JwkKeyPair {
    pub kid: String,
    pub typ: JwkKeyPairAlg,
    pub bytes: Vec<u8>,
}

impl Debug for JwkKeyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JwkKeyPair {{ kid: {}, typ: {}, bytes: <hidden> }}",
            self.kid, self.typ
        )
    }
}

impl JwkKeyPair {
    // Decrypts a Json Web Key which is in an encrypted format from inside the database
    pub fn decrypt(jwk_entity: &Jwk, key_pair_type: JwkKeyPairAlg) -> Result<Self, ErrorResponse> {
        let jwk_decrypted = EncValue::try_from(jwk_entity.jwk.clone())?
            .decrypt()?
            .to_vec();

        let kid = jwk_entity.kid.clone();
        let res = match key_pair_type {
            JwkKeyPairAlg::RS256 => JwkKeyPair {
                kid,
                typ: JwkKeyPairAlg::RS256,
                bytes: jwk_decrypted,
            },
            JwkKeyPairAlg::RS384 => JwkKeyPair {
                kid,
                typ: JwkKeyPairAlg::RS384,
                bytes: jwk_decrypted,
            },
            JwkKeyPairAlg::RS512 => JwkKeyPair {
                kid,
                typ: JwkKeyPairAlg::RS512,
                bytes: jwk_decrypted,
            },
            JwkKeyPairAlg::EdDSA => JwkKeyPair {
                kid,
                typ: JwkKeyPairAlg::EdDSA,
                bytes: jwk_decrypted,
            },
        };

        Ok(res)
    }

    // Returns a JWK by a given Key Identifier (kid)
    pub async fn find(kid: String) -> Result<Self, ErrorResponse> {
        let idx = format!("{IDX_JWK_KID}{kid}");
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::App, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM jwks WHERE kid = $1";
        let jwk: Jwk = if is_hiqlite() {
            client.query_as_one(sql, params!(kid)).await?
        } else {
            DB::pg_query_one(sql, &[&kid]).await?
        };

        let kp = JwkKeyPair::decrypt(&jwk, jwk.signature.clone())?;
        client.put(Cache::App, idx, &kp, CACHE_TTL_APP).await?;

        Ok(kp)
    }

    // TODO add an index (signature, created_at) with the next round of DB migrations
    // Returns the latest JWK (especially important after a [JWK Rotation](crate::api::rotate_jwk)
    // by a given algorithm.
    pub async fn find_latest(key_pair_alg: JwkKeyPairAlg) -> Result<Self, ErrorResponse> {
        let idx = format!("{}{}", IDX_JWK_LATEST, key_pair_alg.as_str());
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::App, &idx).await? {
            return Ok(slf);
        }

        let signature = key_pair_alg.as_str().to_string();
        let sql = r#"
SELECT * FROM jwks
WHERE signature = $1
ORDER BY created_at DESC
LIMIT 1"#;

        let jwk_latest: Jwk = if is_hiqlite() {
            client.query_as_one(sql, params!(signature)).await?
        } else {
            DB::pg_query_one(sql, &[&signature]).await?
        };

        let jwk = JwkKeyPair::decrypt(&jwk_latest, key_pair_alg)?;
        client.put(Cache::App, idx, &jwk, CACHE_TTL_APP).await?;

        Ok(jwk)
    }
}

impl JwkKeyPair {
    pub fn sign(&self, input: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
        match self.typ {
            JwkKeyPairAlg::RS256 => {
                let hash = hmac_sha256::Hash::hash(input);
                let key = rsa::RsaPrivateKey::from_pkcs8_der(&self.bytes)?;
                let sig = key.sign(rsa::Pkcs1v15Sign::new::<sha2::Sha256>(), hash.as_slice())?;
                Ok(sig)
            }

            JwkKeyPairAlg::RS384 => {
                let hash = hmac_sha512::sha384::Hash::hash(input);
                let key = rsa::RsaPrivateKey::from_pkcs8_der(&self.bytes)?;
                let sig = key.sign(rsa::Pkcs1v15Sign::new::<sha2::Sha384>(), hash.as_slice())?;
                Ok(sig)
            }

            JwkKeyPairAlg::RS512 => {
                let hash = hmac_sha512::Hash::hash(input);
                let key = rsa::RsaPrivateKey::from_pkcs8_der(&self.bytes)?;
                let sig = key.sign(rsa::Pkcs1v15Sign::new::<sha2::Sha512>(), hash.as_slice())?;
                Ok(sig)
            }

            JwkKeyPairAlg::EdDSA => {
                let key = ed25519_compact::SecretKey::from_der(&self.bytes)?;
                let sig = key.sign(input, Some(Noise::generate()));
                Ok(sig.to_vec())
            }
        }
    }

    /// Verify the signature for a JWT token.
    ///
    /// Returns the `<header>.<claims>` string  on success.
    ///
    /// Note: This function is a little bit of duplicated code from the
    /// `JWKSPublicKey::validate_token_signature()` which is almost the same. The reason of the
    /// duplicate is that the `JWKSPublicKey` contains the public key components b64 encoded
    /// and the conversion each time would be an unnecessary overhead. `JWKSPublicKey` should
    /// only be used for validation from remote JWKs.
    pub fn verify_token<'a>(
        &self,
        token: &'a str,
        buf: &mut Vec<u8>,
    ) -> Result<&'a str, ErrorResponse> {
        debug_assert!(buf.is_empty());

        let (message, sig) = token
            .rsplit_once('.')
            .ok_or_else(|| ErrorResponse::new(ErrorResponseType::BadRequest, "Malformed token"))?;

        base64_url_no_pad_decode_buf(sig, buf)?;

        match self.typ {
            JwkKeyPairAlg::RS256 => {
                let hash = hmac_sha256::Hash::hash(message.as_bytes());
                let pk = rsa::RsaPrivateKey::from_pkcs8_der(&self.bytes)?;
                if pk
                    .as_ref()
                    .verify(
                        rsa::Pkcs1v15Sign::new::<sha2::Sha256>(),
                        hash.as_slice(),
                        buf,
                    )
                    .is_ok()
                {
                    return Ok(message);
                }
            }

            JwkKeyPairAlg::RS384 => {
                let hash = hmac_sha512::sha384::Hash::hash(message.as_bytes());
                let pk = rsa::RsaPrivateKey::from_pkcs8_der(&self.bytes)?;
                if pk
                    .as_ref()
                    .verify(
                        rsa::Pkcs1v15Sign::new::<sha2::Sha384>(),
                        hash.as_slice(),
                        buf,
                    )
                    .is_ok()
                {
                    return Ok(message);
                }
            }

            JwkKeyPairAlg::RS512 => {
                let hash = hmac_sha512::Hash::hash(message.as_bytes());
                let pk = rsa::RsaPrivateKey::from_pkcs8_der(&self.bytes)?;
                if pk
                    .as_ref()
                    .verify(
                        rsa::Pkcs1v15Sign::new::<sha2::Sha512>(),
                        hash.as_slice(),
                        buf,
                    )
                    .is_ok()
                {
                    return Ok(message);
                }
            }

            JwkKeyPairAlg::EdDSA => {
                let skey = ed25519_compact::SecretKey::from_der(&self.bytes)?;
                let signature = ed25519_compact::Signature::from_slice(buf)?;
                if skey.public_key().verify(message, &signature).is_ok() {
                    return Ok(message);
                }
            }
        };

        warn!("JWT Token validation error");
        Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Invalid JWT Token signature",
        ))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum JwkKeyPairType {
    RSA,
    #[default]
    OKP,
}

impl JwkKeyPairType {
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            JwkKeyPairType::RSA => "RSA",
            JwkKeyPairType::OKP => "OKP",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JwkKeyPairAlg {
    RS256,
    RS384,
    RS512,
    #[default]
    EdDSA,
}

impl<'r> From<hiqlite::Row<'r>> for JwkKeyPairAlg {
    fn from(mut row: hiqlite::Row<'r>) -> Self {
        let sig: String = row.get("signature");
        JwkKeyPairAlg::from_str(&sig).expect("corrupted signature in database")
    }
}

impl From<String> for JwkKeyPairAlg {
    fn from(value: String) -> Self {
        match value.as_str() {
            "RS256" => JwkKeyPairAlg::RS256,
            "RS384" => JwkKeyPairAlg::RS384,
            "RS512" => JwkKeyPairAlg::RS512,
            "EdDSA" => JwkKeyPairAlg::EdDSA,
            _ => unreachable!(),
        }
    }
}

impl postgres_types::FromSql<'_> for JwkKeyPairAlg {
    fn from_sql(_ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let sig = String::from_utf8_lossy(raw);
        let slf = JwkKeyPairAlg::from_str(sig.as_ref()).expect("corrupted signature in database");
        Ok(slf)
    }

    fn accepts(ty: &Type) -> bool {
        // TEXT or VARCHAR
        ty.eq(&Type::from_oid(tokio_postgres::types::Oid::from(25u32)).unwrap())
            || ty.eq(&Type::from_oid(tokio_postgres::types::Oid::from(1043u32)).unwrap())
    }
}

impl JwkKeyPairAlg {
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            JwkKeyPairAlg::RS256 => "RS256",
            JwkKeyPairAlg::RS384 => "RS384",
            JwkKeyPairAlg::RS512 => "RS512",
            JwkKeyPairAlg::EdDSA => "EdDSA",
        }
    }
}

impl Display for JwkKeyPairAlg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for JwkKeyPairAlg {
    type Err = ErrorResponse;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RS256" => Ok(JwkKeyPairAlg::RS256),
            "RS384" => Ok(JwkKeyPairAlg::RS384),
            "RS512" => Ok(JwkKeyPairAlg::RS512),
            "EdDSA" => Ok(JwkKeyPairAlg::EdDSA),
            _ => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid JWT Token algorithm",
            )),
        }
    }
}

impl From<JwkKeyPairAlg> for rauthy_api_types::oidc::JwkKeyPairAlg {
    #[inline]
    fn from(value: JwkKeyPairAlg) -> Self {
        match value {
            JwkKeyPairAlg::RS256 => Self::RS256,
            JwkKeyPairAlg::RS384 => Self::RS384,
            JwkKeyPairAlg::RS512 => Self::RS512,
            JwkKeyPairAlg::EdDSA => Self::EdDSA,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg, JwkKeyPairType};

    #[test]
    fn test_fingerprint() {
        // example from RFC7638 https://datatracker.ietf.org/doc/html/rfc7638
        let tp = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS256),
            crv: None,
            kid: Some("2011-04-29".to_string()),
            n: Some("0vx7agoebGcQSuuPiLJXZptN9nndrQmbXEps2aiAFbWhM78LhWx4cbbfAAtVT86zwu1RK7aPFFxuhDR1L6tSoc_BJECPebWKRXjBZCiFV4n3oknjhMstn64tZ_2W-5JsGY4Hc5n9yBXArwl93lqt7_RN5w6Cf0h4QyQ5v-65YGjQR0_FDW2QvzqY368QQMicAtaSqzs8KJZgnYb9c7d0zgdAZHzu6qMQvRL5hajrn1n91CbOpbISD08qNLyrdkt-bFTWhAI4vMQFh6WeZu0fM4lFd2NcRwr3XPksINHaQ-G_xBniIqbw0Ls1jF44-csFCur-kEgU8awapJzKnqDKgw".to_string()),
            e: Some("AQAB".to_string()),
            x: None,
        }.fingerprint().unwrap();
        assert_eq!(tp.as_str(), "NzbLsXh8uDCcd-6MNwXF4W_7noWXFZAfHkxZsRGC9Xs");

        let tp = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS384),
            crv: None,
            kid: None,
            n: Some("0OJuIbD0k90-Xod2cnqcGWu0xP4Z3Eyfi3CXBxdzlEwFHSNat6Vjts2g5Uzbdvmgm2ys-UWUaCcw2zPEbn25dtcv0MVK26J71OV0Q38yB701SniEJqLXf3OehSR7lfd9HNasZF_-2u6oJMwvKLe10qlSGYLzeUCWIV4LDPDv7lxsWFx0WntgLlHpKfVmYuvW_AQ1Q8XSO53K4Xk3n84zzAXvCUyW8Z4tmE4tc3ibriHH63AYpKbB8oDR-zhbIoGHtZnDdRo02JvS11KNINLdmMOE2zre7hPgXVbgnYS9qbpz4nsc4sPCiGclM2c2faSkwyxI60Ng6272e3fIEkBTKtYidoaG00tM1j42kD-b7bNjWJIsY92F15SdRA4stpic2KcAnyphNrLeDMKd_c-h3PC22eR-a8pb5nE1VvDSagn9g8WE3TSMEJxEmAgVcOcldSV9EDpSz4uk2CqRdytwAZOnRDEwehnRQiLNiwgyNEygLAcaVWDR8ym8ARRLWCRL".to_string()),
            e: Some("AQAB".to_string()),
            x: None,
        }.fingerprint().unwrap();
        assert_eq!(tp.as_str(), "EunK2QL42BZ2Eb4urUxXiFFomdjus4UtGB9qJ8Vnjtw");

        let tp = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS512),
            crv: None,
            kid: None,
            n: Some("1UjNug4a3OEo8saHbM14jhEqpgRHvjMaQ0lB_1rRuK4yMNPLxhdes8PcMXfEuCOYrC4jxkeVb31QgM5OFwxRtyBT-T1SmiWCtXX2beFtRrvZcGYQrd_LooKLrcjww-P8atQBBYKgf82e9aqb5I-4BFYTBdDQ5lQKQtZDwiU-lUVYP103SphHQMkkWLKsC7oFcthN2m8IliQnJ3-XeqgYt9dc6AszDEjNTDZMeC-HWwRXI9JGYjIgNIZj_u0n6UgaqhdjR1sEHxRGI_t6xQX_L9zRecdDM6-e_lNxIaeROZJ2FU-t9GmZZWyyDWUHk7tk4dS1cU5CdtwvL75dXMHsmwyTs8QK9YUvCWmLeCp6JNPOpCalwyW8YcqJphINhKgonsMinxWLPlO4jtSXKzrpGDLxOF_8xVMW3gNmnIWuUY0_29p7-DzdVm44GEYhQRNNX7yh850uYpwoi42fFvXa5wXm6Hy5QHh_Aqv3tTZgG2f20xCKOzzGzWB28BdJJa9EPu2WLrxaPbn8Qi536979UvMhlZsnUc4fW3TSy20coMb1NIatZaJCDu-uQuGFz7FHBFWjJV6fjF7gqiNqu8cZTeOedGjMitdCnMtOjCz8SASphF12_opWTvtFjq0IMNo4kR8zgZQ24Kt2o2qDhH7fYJI1cLj0RBGDCUU3AlozG_U".to_string()),
            e: Some("AQAB".to_string()),
            x: None,
        }.fingerprint().unwrap();
        assert_eq!(tp.as_str(), "rSJa_34h-WFCVMoSG7ORvEvxhF45iCvcm1FRZlxSRio");

        let tp = JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(JwkKeyPairAlg::EdDSA),
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: None,
            e: None,
            x: Some("suwfa9fyMHqS0yOh9T-Bsdkji0naFVRRGZFBNrGX_RQ".to_string()),
        }
        .fingerprint()
        .unwrap();
        assert_eq!(tp.as_str(), "lVstH-NNQsIRpUp1nMmxD3cUoDS_dUbi4Or5awQ34EQ");
    }

    #[test]
    fn test_jwk_validate_self() {
        // these should be fine
        JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS256),
            crv: None,
            kid: None,
            n: Some("r5Xn8yuwc7ekL5NLFnBw76cRUiYbIQqNgPq6XYw6_Mgle3BSJ-UTKTWjGLDoTSlFC7k2xCZNOt8pqix2R_qoGwlNo8kYXlgMpAEo00rSKoG1RO1PMj1M_--swijR8l1bnb-VfIPgT_kM3zv7RLPLEEjYHMuT7N5liFVq1Xh-So8i3X1UeWGHyJPHjF5koB_XO1vleYQCZQeGFaomJgrFJsxdmtFueJaMEMQ1-mPwuPjvSwOtMMAu0nO9DJm3-xwkygPqGmEbbDHLeEO1dEOlDdEYlYle5Pa70FGinCBqaAl7lDaJ1umAvpcLBUHtFOM7VBmt-xUjzOU7VDPareR6Ww".to_string()),
            e: Some("AQAB".to_string()),
            x: None,
        }.validate_self().unwrap();

        JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS384),
            crv: None,
            kid: None,
            n: Some("0OJuIbD0k90-Xod2cnqcGWu0xP4Z3Eyfi3CXBxdzlEwFHSNat6Vjts2g5Uzbdvmgm2ys-UWUaCcw2zPEbn25dtcv0MVK26J71OV0Q38yB701SniEJqLXf3OehSR7lfd9HNasZF_-2u6oJMwvKLe10qlSGYLzeUCWIV4LDPDv7lxsWFx0WntgLlHpKfVmYuvW_AQ1Q8XSO53K4Xk3n84zzAXvCUyW8Z4tmE4tc3ibriHH63AYpKbB8oDR-zhbIoGHtZnDdRo02JvS11KNINLdmMOE2zre7hPgXVbgnYS9qbpz4nsc4sPCiGclM2c2faSkwyxI60Ng6272e3fIEkBTKtYidoaG00tM1j42kD-b7bNjWJIsY92F15SdRA4stpic2KcAnyphNrLeDMKd_c-h3PC22eR-a8pb5nE1VvDSagn9g8WE3TSMEJxEmAgVcOcldSV9EDpSz4uk2CqRdytwAZOnRDEwehnRQiLNiwgyNEygLAcaVWDR8ym8ARRLWCRL".to_string()),
            e: Some("AQAB".to_string()),
            x: None,
        }.validate_self().unwrap();

        JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS512),
            crv: None,
            kid: None,
            n: Some("1UjNug4a3OEo8saHbM14jhEqpgRHvjMaQ0lB_1rRuK4yMNPLxhdes8PcMXfEuCOYrC4jxkeVb31QgM5OFwxRtyBT-T1SmiWCtXX2beFtRrvZcGYQrd_LooKLrcjww-P8atQBBYKgf82e9aqb5I-4BFYTBdDQ5lQKQtZDwiU-lUVYP103SphHQMkkWLKsC7oFcthN2m8IliQnJ3-XeqgYt9dc6AszDEjNTDZMeC-HWwRXI9JGYjIgNIZj_u0n6UgaqhdjR1sEHxRGI_t6xQX_L9zRecdDM6-e_lNxIaeROZJ2FU-t9GmZZWyyDWUHk7tk4dS1cU5CdtwvL75dXMHsmwyTs8QK9YUvCWmLeCp6JNPOpCalwyW8YcqJphINhKgonsMinxWLPlO4jtSXKzrpGDLxOF_8xVMW3gNmnIWuUY0_29p7-DzdVm44GEYhQRNNX7yh850uYpwoi42fFvXa5wXm6Hy5QHh_Aqv3tTZgG2f20xCKOzzGzWB28BdJJa9EPu2WLrxaPbn8Qi536979UvMhlZsnUc4fW3TSy20coMb1NIatZaJCDu-uQuGFz7FHBFWjJV6fjF7gqiNqu8cZTeOedGjMitdCnMtOjCz8SASphF12_opWTvtFjq0IMNo4kR8zgZQ24Kt2o2qDhH7fYJI1cLj0RBGDCUU3AlozG_U".to_string()),
            e: Some("AQAB".to_string()),
            x: None,
        }.validate_self().unwrap();

        JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(JwkKeyPairAlg::EdDSA),
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: None,
            e: None,
            x: Some("suwfa9fyMHqS0yOh9T-Bsdkji0naFVRRGZFBNrGX_RQ".to_string()),
        }
        .validate_self()
        .unwrap();

        // now test bad keys
        let key = JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(JwkKeyPairAlg::RS256),
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: Some("r5Xn8yuwc7ekL5NLFnBw76cRUiYbIQqNgPq6XYw6_Mgle3BSJ-UTKTWjGLDoTSlFC7k2xCZNOt8pqix2R_qoGwlNo8kYXlgMpAEo00rSKoG1RO1PMj1M_--swijR8l1bnb-VfIPgT_kM3zv7RLPLEEjYHMuT7N5liFVq1Xh-So8i3X1UeWGHyJPHjF5koB_XO1vleYQCZQeGFaomJgrFJsxdmtFueJaMEMQ1-mPwuPjvSwOtMMAu0nO9DJm3-xwkygPqGmEbbDHLeEO1dEOlDdEYlYle5Pa70FGinCBqaAl7lDaJ1umAvpcLBUHtFOM7VBmt-xUjzOU7VDPareR6Ww".to_string()),
            e: Some("AQAB".to_string()),
            x: None,
        }.validate_self();
        assert!(key.is_err());

        let key = JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(JwkKeyPairAlg::EdDSA),
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: Some("r5Xn8yuwc7ekL5NLFnBw76cRUiYbIQqNgPq6XYw6_Mgle3BSJ-UTKTWjGLDoTSlFC7k2xCZNOt8pqix2R_qoGwlNo8kYXlgMpAEo00rSKoG1RO1PMj1M_--swijR8l1bnb-VfIPgT_kM3zv7RLPLEEjYHMuT7N5liFVq1Xh-So8i3X1UeWGHyJPHjF5koB_XO1vleYQCZQeGFaomJgrFJsxdmtFueJaMEMQ1-mPwuPjvSwOtMMAu0nO9DJm3-xwkygPqGmEbbDHLeEO1dEOlDdEYlYle5Pa70FGinCBqaAl7lDaJ1umAvpcLBUHtFOM7VBmt-xUjzOU7VDPareR6Ww".to_string()),
            e: Some("AQAB".to_string()),
            x: None,
        }.validate_self();
        assert!(key.is_err());

        let key = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::EdDSA),
            crv: None,
            kid: None,
            n: Some("r5Xn8yuwc7ekL5NLFnBw76cRUiYbIQqNgPq6XYw6_Mgle3BSJ-UTKTWjGLDoTSlFC7k2xCZNOt8pqix2R_qoGwlNo8kYXlgMpAEo00rSKoG1RO1PMj1M_--swijR8l1bnb-VfIPgT_kM3zv7RLPLEEjYHMuT7N5liFVq1Xh-So8i3X1UeWGHyJPHjF5koB_XO1vleYQCZQeGFaomJgrFJsxdmtFueJaMEMQ1-mPwuPjvSwOtMMAu0nO9DJm3-xwkygPqGmEbbDHLeEO1dEOlDdEYlYle5Pa70FGinCBqaAl7lDaJ1umAvpcLBUHtFOM7VBmt-xUjzOU7VDPareR6Ww".to_string()),
            e: Some("AQAB".to_string()),
            x: None,
        }.validate_self();
        assert!(key.is_err());

        let key = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS256),
            crv: None,
            kid: None,
            n: None,
            e: Some("AQAB".to_string()),
            x: None,
        }
        .validate_self();
        assert!(key.is_err());

        let key = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS256),
            crv: None,
            kid: None,
            n: Some("r5Xn8yuwc7ekL5NLFnBw76cRUiYbIQqNgPq6XYw6_Mgle3BSJ-UTKTWjGLDoTSlFC7k2xCZNOt8pqix2R_qoGwlNo8kYXlgMpAEo00rSKoG1RO1PMj1M_--swijR8l1bnb-VfIPgT_kM3zv7RLPLEEjYHMuT7N5liFVq1Xh-So8i3X1UeWGHyJPHjF5koB_XO1vleYQCZQeGFaomJgrFJsxdmtFueJaMEMQ1-mPwuPjvSwOtMMAu0nO9DJm3-xwkygPqGmEbbDHLeEO1dEOlDdEYlYle5Pa70FGinCBqaAl7lDaJ1umAvpcLBUHtFOM7VBmt-xUjzOU7VDPareR6Ww".to_string()),
            e: None,
            x: None,
        }
            .validate_self();
        assert!(key.is_err());

        let key = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS256),
            crv: None,
            kid: None,
            n: None,
            e: None,
            x: None,
        }
        .validate_self();
        assert!(key.is_err());

        let key = JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(JwkKeyPairAlg::EdDSA),
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: None,
            e: None,
            x: None,
        }
        .validate_self();
        assert!(key.is_err());

        let key = JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(JwkKeyPairAlg::EdDSA),
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: Some("n".to_string()),
            e: None,
            x: None,
        }
        .validate_self();
        assert!(key.is_err());

        let key = JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(JwkKeyPairAlg::EdDSA),
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: Some("n".to_string()),
            e: None,
            x: Some("suwfa9fyMHqS0yOh9T-Bsdkji0naFVRRGZFBNrGX_RQ".to_string()),
        }
        .validate_self();
        assert!(key.is_err());

        let key = JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(JwkKeyPairAlg::EdDSA),
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: None,
            e: Some("e".to_string()),
            x: Some("suwfa9fyMHqS0yOh9T-Bsdkji0naFVRRGZFBNrGX_RQ".to_string()),
        }
        .validate_self();
        assert!(key.is_err());
    }
}
