use crate::app_state::{AppState, DbPool};
use crate::cache::{Cache, DB};
use crate::events::event::Event;
use actix_web::web;
use cryptr::{EncKeys, EncValue};
use jwt_simple::algorithms;
use jwt_simple::algorithms::{
    Ed25519KeyPair, EdDSAKeyPairLike, RS256KeyPair, RS384KeyPair, RS512KeyPair, RSAKeyPairLike,
};
use rauthy_api_types::oidc::{JWKSCerts, JWKSPublicKeyCerts};
use rauthy_common::constants::{CACHE_TTL_APP, IDX_JWKS, IDX_JWK_KID, IDX_JWK_LATEST};
use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_decode, get_rand};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rsa::BigUint;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};
use std::default::Default;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::info;

#[macro_export]
macro_rules! sign_jwt {
    ($key_pair:expr, $claims:expr) => {
        match $key_pair.typ {
            JwkKeyPairAlg::RS256 => {
                let key =
                    jwt_simple::algorithms::RS256KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.with_key_id(&$key_pair.kid).sign($claims)
            }
            JwkKeyPairAlg::RS384 => {
                let key =
                    jwt_simple::algorithms::RS384KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.with_key_id(&$key_pair.kid).sign($claims)
            }
            JwkKeyPairAlg::RS512 => {
                let key =
                    jwt_simple::algorithms::RS512KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.with_key_id(&$key_pair.kid).sign($claims)
            }
            JwkKeyPairAlg::EdDSA => {
                let key =
                    jwt_simple::algorithms::Ed25519KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.with_key_id(&$key_pair.kid).sign($claims)
            }
        }
        .map_err(|_| ErrorResponse::new(ErrorResponseType::Internal, "Error signing JWT Token"))
    };
}

#[macro_export]
macro_rules! validate_jwt {
    ($type:ty, $key_pair:expr, $token:expr, $options:expr) => {
        match $key_pair.typ {
            JwkKeyPairAlg::RS256 => {
                let key =
                    jwt_simple::algorithms::RS256KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.public_key()
                    .verify_token::<$type>($token, Some($options))
            }
            JwkKeyPairAlg::RS384 => {
                let key =
                    jwt_simple::algorithms::RS384KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.public_key()
                    .verify_token::<$type>($token, Some($options))
            }
            JwkKeyPairAlg::RS512 => {
                let key =
                    jwt_simple::algorithms::RS512KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.public_key()
                    .verify_token::<$type>($token, Some($options))
            }
            JwkKeyPairAlg::EdDSA => {
                let key =
                    jwt_simple::algorithms::Ed25519KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.public_key()
                    .verify_token::<$type>($token, Some($options))
            }
        }
        .map_err(|_| ErrorResponse::new(ErrorResponseType::Unauthorized, "Invalid Token"))
    };
}

/**
The Json Web Keys are saved encrypted inside the database. The encryption is the same as for a
Client secret -> *ChaCha20Poly1305*
 */
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub created_at: i64,
    #[sqlx(flatten)]
    pub signature: JwkKeyPairAlg,
    pub enc_key_id: String,
    pub jwk: Vec<u8>,
}

// CRUD
impl Jwk {
    pub async fn save(&self, db: &DbPool) -> Result<(), ErrorResponse> {
        let sig_str = self.signature.as_str();
        sqlx::query!(
            r#"INSERT INTO jwks (kid, created_at, signature, enc_key_id, jwk)
            VALUES ($1, $2, $3, $4, $5)"#,
            self.kid,
            self.created_at,
            sig_str,
            self.enc_key_id,
            self.jwk,
        )
        .execute(db)
        .await?;
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
    pub async fn find_pk(data: &web::Data<AppState>) -> Result<JWKS, ErrorResponse> {
        let client = DB::client();

        if let Some(slf) = client.get(Cache::App, IDX_JWKS).await? {
            return Ok(slf);
        }

        let res = sqlx::query_as!(Jwk, "SELECT * FROM jwks")
            .fetch_all(&data.db)
            .await?;

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
            jwks.add_jwk(&kp);
        }

        client
            .put(Cache::App, IDX_JWKS, &jwks, CACHE_TTL_APP)
            .await?;

        Ok(jwks)
    }
}

impl JWKS {
    pub fn add_jwk(&mut self, key_pair: &JwkKeyPair) {
        let pub_key = JWKSPublicKey::from_key_pair(key_pair);
        self.keys.push(pub_key)
    }

    /// Rotates and generates a whole new Set of JWKs for signing JWT Tokens
    pub async fn rotate(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        info!("Starting JWKS rotation");

        // let key = data.enc_keys.get(&data.enc_key_active).unwrap();
        let enc_key_active = &EncKeys::get_static().enc_key_active;

        // RSA256
        let jwk_plain = web::block(|| {
            RS256KeyPair::generate(2048)
                .unwrap()
                .with_key_id(&get_rand(24))
        })
        .await?;
        let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
            .into_bytes()
            .to_vec();
        let entity = Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::RS256,
            enc_key_id: enc_key_active.to_string(),
            jwk,
        };
        entity.save(&data.db).await?;

        // RS384
        let jwk_plain = web::block(|| {
            RS384KeyPair::generate(3072)
                .unwrap()
                .with_key_id(&get_rand(24))
        })
        .await?;
        let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
            .into_bytes()
            .to_vec();
        let entity = Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::RS384,
            enc_key_id: enc_key_active.to_string(),
            jwk,
        };
        entity.save(&data.db).await?;

        // RSA512
        let jwk_plain = web::block(|| {
            RS512KeyPair::generate(4096)
                .unwrap()
                .with_key_id(&get_rand(24))
        })
        .await?;
        let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
            .into_bytes()
            .to_vec();
        let entity = Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::RS512,
            enc_key_id: enc_key_active.to_string(),
            jwk,
        };
        entity.save(&data.db).await?;

        // Ed25519
        let jwk_plain =
            web::block(|| Ed25519KeyPair::generate().with_key_id(&get_rand(24))).await?;
        let jwk = EncValue::encrypt(jwk_plain.to_der().as_slice())?
            .into_bytes()
            .to_vec();
        let entity = Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::EdDSA,
            enc_key_id: enc_key_active.to_string(),
            jwk,
        };
        entity.save(&data.db).await?;

        // clear all latest_jwk from cache
        let client = DB::client();
        client
            .delete(
                Cache::App,
                format!("{}{}", IDX_JWK_LATEST, JwkKeyPairAlg::RS256.as_str()),
            )
            .await?;
        client
            .delete(
                Cache::App,
                format!("{}{}", IDX_JWK_LATEST, JwkKeyPairAlg::RS384.as_str()),
            )
            .await?;
        client
            .delete(
                Cache::App,
                format!("{}{}", IDX_JWK_LATEST, JwkKeyPairAlg::RS512.as_str()),
            )
            .await?;
        client
            .delete(
                Cache::App,
                format!("{}{}", IDX_JWK_LATEST, JwkKeyPairAlg::EdDSA.as_str()),
            )
            .await?;

        // clear the all_certs / JWKS cache
        client.delete(Cache::App, IDX_JWKS).await?;

        info!("Finished JWKS rotation");

        data.tx_events
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
// This will lead to cache errors when deserializing the JWKS
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

    pub fn from_key_pair(key_pair: &JwkKeyPair) -> Self {
        let get_rsa = |kid: String, comp: algorithms::RSAPublicKeyComponents| JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(key_pair.typ.clone()),
            crv: None,
            kid: Some(kid),
            n: Some(base64_url_encode(&comp.n)),
            e: Some(base64_url_encode(&comp.e)),
            x: None,
        };

        let get_ed25519 = |kid: String, x: String| JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(key_pair.typ.clone()),
            crv: Some("Ed25519".to_string()),
            kid: Some(kid),
            n: None,
            e: None,
            x: Some(x),
        };

        match key_pair.typ {
            JwkKeyPairAlg::RS256 => {
                let kp = algorithms::RS256KeyPair::from_der(&key_pair.bytes).unwrap();
                let comp = kp.public_key().to_components();
                get_rsa(key_pair.kid.clone(), comp)
            }
            JwkKeyPairAlg::RS384 => {
                let kp = algorithms::RS384KeyPair::from_der(&key_pair.bytes).unwrap();
                let comp = kp.public_key().to_components();
                get_rsa(key_pair.kid.clone(), comp)
            }
            JwkKeyPairAlg::RS512 => {
                let kp = algorithms::RS384KeyPair::from_der(&key_pair.bytes).unwrap();
                let comp = kp.public_key().to_components();
                get_rsa(key_pair.kid.clone(), comp)
            }
            JwkKeyPairAlg::EdDSA => {
                let kp = algorithms::Ed25519KeyPair::from_der(&key_pair.bytes).unwrap();
                let x = base64_url_encode(&kp.public_key().to_bytes());
                get_ed25519(key_pair.kid.clone(), x)
            }
        }
    }

    /// Creates a JWK thumbprint by https://datatracker.ietf.org/doc/html/rfc7638
    pub fn fingerprint(&self) -> Result<String, ErrorResponse> {
        let s = match self.kty {
            JwkKeyPairType::RSA => {
                if self.e.is_none() || self.n.is_none() {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        "Incorrect format for RSA JWK: e / n missing".to_string(),
                    ));
                }

                // mandatory keys for RSA are in order: e, kty, n
                let e = self.e.as_deref().unwrap();
                let n = self.n.as_deref().unwrap();
                format!(
                    "{{\"e\":\"{}\",\"kty\":\"{}\",\"n\":\"{}\"}}",
                    e,
                    self.kty.as_str(),
                    n
                )
            }

            JwkKeyPairType::OKP => {
                if self.crv.is_none() || self.x.is_none() {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::Internal,
                        "Incorrect format for OKP JWK: crv / x missing".to_string(),
                    ));
                }

                // mandatory keys for OKP are in order: crv, kty, x
                let crv = self.crv.as_deref().unwrap();
                let x = self.x.as_deref().unwrap();
                format!(
                    "{{\"crv\":\"{}\",\"kty\":\"{}\",\"x\":\"{}\"}}",
                    crv,
                    self.kty.as_str(),
                    x
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
                "No 'alg' for JWK".to_string(),
            )),
            Some(alg) => {
                match self.kty {
                    JwkKeyPairType::RSA => {
                        if alg == &JwkKeyPairAlg::EdDSA {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "RSA kty cannot have EdDSA alg".to_string(),
                            ));
                        }

                        if self.n.is_none() || self.e.is_none() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "No public key components for RSA key".to_string(),
                            ));
                        }

                        if self.x.is_some() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "RSA key cannot have 'x' public key component".to_string(),
                            ));
                        }
                    }

                    JwkKeyPairType::OKP => {
                        if alg != &JwkKeyPairAlg::EdDSA {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "OKP kty must have EdDSA alg".to_string(),
                            ));
                        }

                        if self.crv.is_none() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "OKP kty must have 'crv'".to_string(),
                            ));
                        }
                        if let Some(crv) = &self.crv {
                            if crv != "Ed25519" {
                                return Err(ErrorResponse::new(
                                    ErrorResponseType::BadRequest,
                                    "Only 'Ed25519' for 'crv' is supported".to_string(),
                                ));
                            }
                        }

                        if self.n.is_some() || self.e.is_some() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "EdDSA key cannot have 'n' or 'e' public key components"
                                    .to_string(),
                            ));
                        }

                        if self.x.is_none() {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "OKP key must have 'x' public key component".to_string(),
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
            alg,
            crv: pk.crv,
            kid: pk.kid,
            n: pk.n,
            e: pk.e,
            x: pk.x,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwkKeyPair {
    pub kid: String,
    pub typ: JwkKeyPairAlg,
    pub bytes: Vec<u8>,
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
    pub async fn find(data: &web::Data<AppState>, kid: String) -> Result<Self, ErrorResponse> {
        let idx = format!("{}{}", IDX_JWK_KID, kid);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::App, &idx).await? {
            return Ok(slf);
        }

        let jwk = sqlx::query_as!(Jwk, "SELECT * FROM jwks WHERE kid = $1", kid,)
            .fetch_one(&data.db)
            .await?;

        let kp = JwkKeyPair::decrypt(&jwk, jwk.signature.clone())?;
        client.put(Cache::App, idx, &kp, CACHE_TTL_APP).await?;

        Ok(kp)
    }

    // Returns the latest JWK (especially important after a [JWK Rotation](crate::api::rotate_jwk)
    // by a given algorithm.
    pub async fn find_latest(
        data: &web::Data<AppState>,
        key_pair_alg: JwkKeyPairAlg,
    ) -> Result<Self, ErrorResponse> {
        let idx = format!("{}{}", IDX_JWK_LATEST, key_pair_alg.as_str());
        let client = DB::client();

        if let Some(slf) = client.get(Cache::App, &idx).await? {
            return Ok(slf);
        }

        let mut jwks = sqlx::query_as!(Jwk, "SELECT * FROM jwks")
            .fetch_all(&data.db)
            .await?
            .into_iter()
            .filter(|jwk| jwk.signature == key_pair_alg)
            .collect::<Vec<Jwk>>();

        jwks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        if jwks.is_empty() {
            panic!("No latest JWK found - database corrupted?");
        }

        let jwk = JwkKeyPair::decrypt(jwks.first().unwrap(), key_pair_alg)?;
        client.put(Cache::App, idx, &jwk, CACHE_TTL_APP).await?;

        Ok(jwk)
    }
}

impl JwkKeyPair {
    pub fn kid_from_token(token: &str) -> Result<String, ErrorResponse> {
        let metadata_res = jwt_simple::token::Token::decode_metadata(token);
        if metadata_res.is_err() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Malformed JWT Token Header".to_string(),
            ));
        }
        let metadata = metadata_res.unwrap();

        let kid_opt = metadata.key_id();
        if let Some(kid) = kid_opt {
            Ok(kid.to_string())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Malformed JWT Token Header".to_string(),
            ))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JwkKeyPairType {
    RSA,
    OKP,
}

impl Default for JwkKeyPairType {
    fn default() -> Self {
        Self::OKP
    }
}

impl JwkKeyPairType {
    pub fn as_str(&self) -> &str {
        match self {
            JwkKeyPairType::RSA => "RSA",
            JwkKeyPairType::OKP => "OKP",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JwkKeyPairAlg {
    RS256,
    RS384,
    RS512,
    EdDSA,
}

impl Default for JwkKeyPairAlg {
    fn default() -> Self {
        Self::EdDSA
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

impl FromRow<'_, SqliteRow> for JwkKeyPairAlg {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        let sig = row.try_get("signature").unwrap();
        let slf = JwkKeyPairAlg::from_str(sig).expect("corrupted signature in database");
        Ok(slf)
    }
}

impl FromRow<'_, PgRow> for JwkKeyPairAlg {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        let sig = row.try_get("signature").unwrap();
        let slf = JwkKeyPairAlg::from_str(sig).expect("corrupted signature in database");
        Ok(slf)
    }
}

impl JwkKeyPairAlg {
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RS256" => Ok(JwkKeyPairAlg::RS256),
            "RS384" => Ok(JwkKeyPairAlg::RS384),
            "RS512" => Ok(JwkKeyPairAlg::RS512),
            "EdDSA" => Ok(JwkKeyPairAlg::EdDSA),
            _ => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid JWT Token algorithm".to_string(),
            )),
        }
    }
}

impl From<JwkKeyPairAlg> for rauthy_api_types::oidc::JwkKeyPairAlg {
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
    use crate::{JwtRefreshClaims, JwtTokenType};
    use jwt_simple::prelude::*;
    use rauthy_common::utils::base64_url_encode;

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

    // We usually want to ignore this test and only run it specifically, since it takes quite
    // a long time to generate all these RSA keys
    #[tokio::test]
    #[ignore]
    async fn test_signature_validation() {
        // generate some JWKs with third party libraries and validate them with out own logic
        // make sure they are all fine

        let claims = Claims::with_custom_claims(
            JwtRefreshClaims {
                azp: "some_azp".to_string(),
                typ: JwtTokenType::Refresh,
                uid: "user_id_13337".to_string(),
                auth_time: None,
                cnf: None,
                did: None,
            },
            coarsetime::Duration::from_secs(300),
        );

        // Ed25519
        let kp = Ed25519KeyPair::generate();
        let signed_token = kp.sign(claims.clone()).unwrap();
        let x = base64_url_encode(&kp.public_key().to_bytes());
        let jwk = JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: Some(JwkKeyPairAlg::EdDSA),
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: None,
            e: None,
            x: Some(x),
        };
        jwk.validate_token_signature(&signed_token).unwrap();

        // RS256
        let kp = RS256KeyPair::generate(2048).unwrap();
        let signed_token = kp.sign(claims.clone()).unwrap();
        let comp = kp.public_key().to_components();
        let n = base64_url_encode(&comp.n);
        let e = base64_url_encode(&comp.e);
        let jwk = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS256),
            crv: None,
            kid: None,
            n: Some(n),
            e: Some(e),
            x: None,
        };
        jwk.validate_token_signature(&signed_token).unwrap();

        // RS384
        let kp = RS384KeyPair::generate(3072).unwrap();
        let signed_token = kp.sign(claims.clone()).unwrap();
        let comp = kp.public_key().to_components();
        let n = base64_url_encode(&comp.n);
        let e = base64_url_encode(&comp.e);
        let jwk = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS384),
            crv: None,
            kid: None,
            n: Some(n),
            e: Some(e),
            x: None,
        };
        jwk.validate_token_signature(&signed_token).unwrap();

        // RS512
        let kp = RS512KeyPair::generate(4096).unwrap();
        let signed_token = kp.sign(claims.clone()).unwrap();
        let comp = kp.public_key().to_components();
        let n = base64_url_encode(&comp.n);
        let e = base64_url_encode(&comp.e);
        let jwk = JWKSPublicKey {
            kty: JwkKeyPairType::RSA,
            alg: Some(JwkKeyPairAlg::RS512),
            crv: None,
            kid: None,
            n: Some(n),
            e: Some(e),
            x: None,
        };
        jwk.validate_token_signature(&signed_token).unwrap();
    }
}
