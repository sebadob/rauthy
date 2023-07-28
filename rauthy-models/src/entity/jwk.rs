use crate::app_state::{AppState, DbPool};
use actix_web::web;
use jwt_simple::algorithms;
use rauthy_common::constants::{CACHE_NAME_12HR, IDX_JWKS, IDX_JWK_KID, IDX_JWK_LATEST};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::base64_url_encode;
use rauthy_common::utils::decrypt;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_put};
use serde::{Deserialize, Serialize};
use sqlx::any::AnyRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};
use std::str::FromStr;

#[macro_export]
macro_rules! sign_jwt {
    ($key_pair:expr, $claims:expr) => {
        match $key_pair.typ {
            JwkKeyPairType::RS256 => {
                let key =
                    jwt_simple::algorithms::RS256KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.with_key_id(&$key_pair.kid).sign($claims)
            }
            JwkKeyPairType::RS384 => {
                let key =
                    jwt_simple::algorithms::RS384KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.with_key_id(&$key_pair.kid).sign($claims)
            }
            JwkKeyPairType::RS512 => {
                let key =
                    jwt_simple::algorithms::RS512KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.with_key_id(&$key_pair.kid).sign($claims)
            }
            JwkKeyPairType::EdDSA => {
                let key =
                    jwt_simple::algorithms::Ed25519KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.with_key_id(&$key_pair.kid).sign($claims)
            }
        }
        .map_err(|_| {
            ErrorResponse::new(
                ErrorResponseType::Internal,
                "Error signing JWT Token".to_string(),
            )
        })
    };
}

#[macro_export]
macro_rules! validate_jwt {
    ($type:ty, $key_pair:expr, $token:expr, $options:expr) => {
        match $key_pair.typ {
            JwkKeyPairType::RS256 => {
                let key =
                    jwt_simple::algorithms::RS256KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.public_key()
                    .verify_token::<$type>($token, Some($options))
            }
            JwkKeyPairType::RS384 => {
                let key =
                    jwt_simple::algorithms::RS384KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.public_key()
                    .verify_token::<$type>($token, Some($options))
            }
            JwkKeyPairType::RS512 => {
                let key =
                    jwt_simple::algorithms::RS512KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.public_key()
                    .verify_token::<$type>($token, Some($options))
            }
            JwkKeyPairType::EdDSA => {
                let key =
                    jwt_simple::algorithms::Ed25519KeyPair::from_der($key_pair.bytes.as_slice())
                        .unwrap();
                key.public_key()
                    .verify_token::<$type>($token, Some($options))
            }
        }
        .map_err(|_| {
            ErrorResponse::new(ErrorResponseType::Unauthorized, "Invalid Token".to_string())
        })
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
    pub signature: JwkKeyPairType,
    pub enc_key_id: String,
    pub jwk: Vec<u8>,
}

/// CRUD
impl Jwk {
    pub async fn save(&self, db: &DbPool) -> Result<(), ErrorResponse> {
        sqlx::query(
            r#"insert into jwks (kid, created_at, signature, enc_key_id, jwk)
            values ($1, $2, $3, $4, $5)"#,
        )
        .bind(&self.kid)
        .bind(self.created_at)
        .bind(self.signature.as_str())
        .bind(&self.enc_key_id)
        .bind(&self.jwk)
        .execute(db)
        .await?;
        Ok(())
    }
}

impl Jwk {
    pub fn new(
        kid: String,
        created_at: time::OffsetDateTime,
        signature: JwkKeyPairType,
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

/// CRUD
impl JWKS {
    pub async fn find_pk(data: &web::Data<AppState>) -> Result<Self, ErrorResponse> {
        if let Some(jwks) = cache_get!(
            JWKS,
            CACHE_NAME_12HR.to_string(),
            IDX_JWKS.to_string(),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(jwks);
        }

        let res = sqlx::query_as::<_, Jwk>("select * from jwks")
            .fetch_all(&data.db)
            .await?;

        let mut jwks = JWKS::default();
        for cert in res {
            let key = data.enc_keys.get(&cert.enc_key_id).unwrap();
            let jwk_decrypted = decrypt(&cert.jwk, key)?;
            let kp = JwkKeyPair {
                kid: cert.kid.clone(),
                typ: cert.signature,
                bytes: jwk_decrypted,
            };
            jwks.add_jwk(&kp);
        }

        cache_put(
            CACHE_NAME_12HR.to_string(),
            IDX_JWKS.to_string(),
            &data.caches.ha_cache_config,
            &jwks,
        )
        .await?;

        Ok(jwks)
    }
}

impl JWKS {
    pub fn add_jwk(&mut self, key_pair: &JwkKeyPair) {
        let pub_key = JWKSPublicKey::from_key_pair(key_pair);
        self.keys.push(pub_key)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct JWKSPublicKey {
    pub kty: String,         // RSA | OKP
    pub crv: Option<String>, // Ed25519
    pub kid: String,
    pub n: Option<String>, // RSA
    pub e: Option<String>, // RSA
    pub x: Option<String>, // OKP
}

impl JWKSPublicKey {
    pub fn from_key_pair(key_pair: &JwkKeyPair) -> Self {
        let get_rsa = |kid: String, comp: algorithms::RSAPublicKeyComponents| JWKSPublicKey {
            kty: "RSA".to_string(),
            crv: None,
            kid,
            n: Some(base64_url_encode(&comp.n)),
            e: Some(base64_url_encode(&comp.e)),
            x: None,
        };

        let get_ed25519 = |kid: String, x: String| JWKSPublicKey {
            kty: "OKP".to_string(),
            crv: Some("Ed25519".to_string()),
            kid,
            n: None,
            e: None,
            x: Some(x),
        };

        match key_pair.typ {
            JwkKeyPairType::RS256 => {
                let kp = algorithms::RS256KeyPair::from_der(&key_pair.bytes).unwrap();
                let comp = kp.public_key().to_components();
                get_rsa(key_pair.kid.clone(), comp)
            }
            JwkKeyPairType::RS384 => {
                let kp = algorithms::RS384KeyPair::from_der(&key_pair.bytes).unwrap();
                let comp = kp.public_key().to_components();
                get_rsa(key_pair.kid.clone(), comp)
            }
            JwkKeyPairType::RS512 => {
                let kp = algorithms::RS384KeyPair::from_der(&key_pair.bytes).unwrap();
                let comp = kp.public_key().to_components();
                get_rsa(key_pair.kid.clone(), comp)
            }
            JwkKeyPairType::EdDSA => {
                let kp = algorithms::Ed25519KeyPair::from_der(&key_pair.bytes).unwrap();
                let x = base64_url_encode(&kp.public_key().to_der());
                get_ed25519(key_pair.kid.clone(), x)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwkKeyPair {
    pub kid: String,
    pub typ: JwkKeyPairType,
    pub bytes: Vec<u8>,
}

impl JwkKeyPair {
    /// Decrypts a Json Web Key which is in an [encrypted](encrypt) format from inside the database
    pub fn decrypt(
        data: &web::Data<AppState>,
        jwk_entity: &Jwk,
        key_pair_type: JwkKeyPairType,
    ) -> Result<Self, ErrorResponse> {
        let key = data
            .enc_keys
            .get(&jwk_entity.enc_key_id)
            .expect("JWK in Database corrupted");
        let jwk_decrypted = decrypt(&jwk_entity.jwk, key)?;

        let kid = jwk_entity.kid.clone();
        let res = match key_pair_type {
            JwkKeyPairType::RS256 => JwkKeyPair {
                kid,
                typ: JwkKeyPairType::RS256,
                bytes: jwk_decrypted,
            },
            JwkKeyPairType::RS384 => JwkKeyPair {
                kid,
                typ: JwkKeyPairType::RS384,
                bytes: jwk_decrypted,
            },
            JwkKeyPairType::RS512 => JwkKeyPair {
                kid,
                typ: JwkKeyPairType::RS512,
                bytes: jwk_decrypted,
            },
            JwkKeyPairType::EdDSA => JwkKeyPair {
                kid,
                typ: JwkKeyPairType::EdDSA,
                bytes: jwk_decrypted,
            },
        };

        Ok(res)
    }

    /// Returns a JWK by a given Key Identifier (kid)
    pub async fn find(data: &web::Data<AppState>, kid: String) -> Result<Self, ErrorResponse> {
        let idx = format!("{}{}", IDX_JWK_KID, kid);
        let jwk_opt = cache_get!(
            JwkKeyPair,
            CACHE_NAME_12HR.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if jwk_opt.is_some() {
            return Ok(jwk_opt.unwrap());
        }

        let jwk = sqlx::query_as("select * from jwks where kid = $1")
            .bind(&kid)
            .fetch_one(&data.db)
            .await?;

        let kp = JwkKeyPair::decrypt(data, &jwk, jwk.signature.clone())?;

        cache_put(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &kp,
        )
        .await?;

        Ok(kp)
    }

    /// Returns the latest JWK (especially important after a [JWK Rotation](crate::handlers::rotate_jwk)
    /// by a given algorithm.
    pub async fn find_latest(
        data: &web::Data<AppState>,
        alg: &str,
        key_pair_type: JwkKeyPairType,
    ) -> Result<Self, ErrorResponse> {
        let idx = format!("{}{}", IDX_JWK_LATEST, &alg);
        let jwk_opt = cache_get!(
            JwkKeyPair,
            CACHE_NAME_12HR.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if jwk_opt.is_some() {
            return Ok(jwk_opt.unwrap());
        }

        let mut jwks = sqlx::query_as::<_, Jwk>("select * from jwks")
            .fetch_all(&data.db)
            .await?
            .into_iter()
            .filter(|jwk| jwk.signature == key_pair_type)
            .collect::<Vec<Jwk>>();

        jwks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        if jwks.is_empty() {
            panic!("No latest JWK found - database corrupted?");
        }

        let jwk = JwkKeyPair::decrypt(data, jwks.get(0).unwrap(), key_pair_type)?;

        cache_put(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
            &jwk,
        )
        .await?;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JwkKeyPairType {
    RS256,
    RS384,
    RS512,
    EdDSA,
}

impl FromRow<'_, SqliteRow> for JwkKeyPairType {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        let sig = row.try_get("signature").unwrap();
        let slf = JwkKeyPairType::from_str(sig).expect("corrupted signature in database");
        Ok(slf)
    }
}

impl FromRow<'_, PgRow> for JwkKeyPairType {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        let sig = row.try_get("signature").unwrap();
        let slf = JwkKeyPairType::from_str(sig).expect("corrupted signature in database");
        Ok(slf)
    }
}

// impl FromRow<'_, AnyRow> for JwkKeyPairType {
//     fn from_row(row: &'_ AnyRow) -> Result<Self, Error> {
//         let sig = row.try_get("signature").unwrap();
//         let slf = JwkKeyPairType::from_str(sig).expect("corrupted signature in database");
//         Ok(slf)
//     }
// }

impl JwkKeyPairType {
    pub fn as_str(&self) -> &str {
        match self {
            JwkKeyPairType::RS256 => "RS256",
            JwkKeyPairType::RS384 => "RS384",
            JwkKeyPairType::RS512 => "RS512",
            JwkKeyPairType::EdDSA => "EdDSA",
        }
    }
}

impl ToString for JwkKeyPairType {
    fn to_string(&self) -> String {
        match self {
            JwkKeyPairType::RS256 => "RS256".to_string(),
            JwkKeyPairType::RS384 => "RS384".to_string(),
            JwkKeyPairType::RS512 => "RS512".to_string(),
            JwkKeyPairType::EdDSA => "EdDSA".to_string(),
        }
    }
}

impl FromStr for JwkKeyPairType {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RS256" => Ok(JwkKeyPairType::RS256),
            "RS384" => Ok(JwkKeyPairType::RS384),
            "RS512" => Ok(JwkKeyPairType::RS512),
            "EdDSA" => Ok(JwkKeyPairType::EdDSA),
            _ => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid JWT Token algorithm".to_string(),
            )),
        }
    }
}
