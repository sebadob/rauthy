use crate::jwks::{JwkKeyPairAlg, JwkPublicKey};
use crate::provider::OidcProvider;
use crate::validate_jwt;
use jwt_simple::algorithms::{EdDSAPublicKeyLike, RSAPublicKeyLike};
use jwt_simple::claims;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The token set returned upon a successful login flow
#[derive(Debug, Serialize, Deserialize)]
pub struct OidcTokenSet {
    pub access_token: String,
    pub token_type: Option<String>,
    pub id_token: Option<String>,
    pub expires_in: i32,
    pub refresh_token: Option<String>,
}

/// Rauthy-supported token types. This client however does currently not support `DPoP`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JwtTokenType {
    Bearer,
    DPoP,
    Id,
    Refresh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JktClaim {
    pub jkt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtAccessClaims {
    pub sub: Option<String>,
    pub expires_at_ts: Option<u64>,

    pub typ: JwtTokenType,
    pub azp: String,
    pub scope: String,
    pub allowed_origins: Option<Vec<String>>,
    // user part
    pub uid: Option<String>,
    pub preferred_username: Option<String>,
    pub roles: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
    pub cnf: Option<JktClaim>,
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

impl JwtAccessClaims {
    pub async fn from_token_validated(token: &str) -> anyhow::Result<Self> {
        let config = OidcProvider::config()?;

        let pubkey = JwkPublicKey::get_for_token(token).await?;
        let claims: claims::JWTClaims<Self> =
            validate_jwt!(Self, pubkey, token, config.verification_options.clone())?;

        let mut slf = claims.custom;
        // TODO should we even include DPoP here or leave it out?
        if slf.typ != JwtTokenType::Bearer {
            return Err(anyhow::Error::msg("Must provide a Bearer token"));
        }

        slf.sub = claims.subject;
        slf.expires_at_ts = claims.expires_at.map(|exp| exp.as_secs());

        Ok(slf)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressClaim {
    pub formatted: String,
    pub street_address: Option<String>,
    pub locality: Option<String>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtIdClaims {
    pub sub: Option<String>,
    pub azp: String,
    pub typ: JwtTokenType,
    pub amr: Vec<String>,
    pub auth_time: i64,
    pub preferred_username: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub address: Option<AddressClaim>,
    pub birthdate: Option<String>,
    pub locale: Option<String>,
    pub phone: Option<String>,
    pub roles: Vec<String>,
    pub groups: Option<Vec<String>>,
    pub cnf: Option<JktClaim>,
    pub custom: Option<HashMap<String, serde_json::Value>>,
    pub webid: Option<String>,
}

impl JwtIdClaims {
    pub async fn from_token_validated(token: &str, nonce: &str) -> anyhow::Result<Self> {
        let config = OidcProvider::config()?;

        let pubkey = JwkPublicKey::get_for_token(token).await?;
        let claims: claims::JWTClaims<Self> =
            validate_jwt!(Self, pubkey, token, config.verification_options.clone())?;

        let mut slf = claims.custom;
        slf.sub = claims.subject;

        if slf.typ != JwtTokenType::Id {
            return Err(anyhow::Error::msg("Must provide an id token"));
        }

        if config.email_verified && slf.email_verified != Some(true) {
            return Err(anyhow::Error::msg("'email_verified' is missing or false"));
        }
        // we want to validate the nonce manually each time for more efficiency
        if claims.nonce.as_deref() != Some(nonce) {
            return Err(anyhow::Error::msg("'nonce' is not correct"));
        }

        Ok(slf)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtRefreshClaims {
    pub azp: String,
    pub typ: JwtTokenType,
    pub uid: String,
    pub cnf: Option<JktClaim>,
}

/// Will be set to JwtAmrValue::Mfa if the user provided at least a second factor during login
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum JwtAmrValue {
    Pwd,
    Mfa,
}

#[macro_export]
macro_rules! validate_jwt {
    ($type:ty, $pub_key:expr, $token:expr, $options:expr) => {
        match $pub_key.alg {
            JwkKeyPairAlg::RS256 => {
                let n = $pub_key
                    .n
                    .as_ref()
                    .ok_or_else(|| anyhow::Error::msg("Invalid 'n' for RS256 key"))?;
                let e = $pub_key
                    .e
                    .as_ref()
                    .ok_or_else(|| anyhow::Error::msg("Invalid 'e' for RS256 key"))?;
                let pk = jwt_simple::algorithms::RS256PublicKey::from_components(
                    n.as_bytes(),
                    e.as_bytes(),
                )
                .map_err(|err| {
                    anyhow::Error::msg(format!(
                        "Cannot build RS256 key from public key components: {:?}",
                        err,
                    ))
                })?;
                pk.verify_token::<$type>($token, Some($options))
            }

            JwkKeyPairAlg::RS384 => {
                let n = $pub_key
                    .n
                    .as_ref()
                    .ok_or_else(|| anyhow::Error::msg("Invalid 'n' for RS384 key"))?;
                let e = $pub_key
                    .e
                    .as_ref()
                    .ok_or_else(|| anyhow::Error::msg("Invalid 'e' for RS384 key"))?;
                let pk = jwt_simple::algorithms::RS384PublicKey::from_components(
                    n.as_bytes(),
                    e.as_bytes(),
                )
                .map_err(|err| {
                    anyhow::Error::msg(format!(
                        "Cannot build RS384 key from public key components: {:?}",
                        err,
                    ))
                })?;
                pk.verify_token::<$type>($token, Some($options))
            }

            JwkKeyPairAlg::RS512 => {
                let n = $pub_key
                    .n
                    .as_ref()
                    .ok_or_else(|| anyhow::Error::msg("Invalid 'n' for RS512 key"))?;
                let e = $pub_key
                    .e
                    .as_ref()
                    .ok_or_else(|| anyhow::Error::msg("Invalid 'e' for RS512 key"))?;
                let pk = jwt_simple::algorithms::RS512PublicKey::from_components(
                    n.as_bytes(),
                    e.as_bytes(),
                )
                .map_err(|err| {
                    anyhow::Error::msg(format!(
                        "Cannot build RS512 key from public key components: {:?}",
                        err,
                    ))
                })?;
                pk.verify_token::<$type>($token, Some($options))
            }

            JwkKeyPairAlg::EdDSA => {
                let bytes = $pub_key
                    .x_bytes
                    .as_ref()
                    .ok_or_else(|| anyhow::Error::msg("Invalid 'x' for EdDSA key"))?;
                let pk = jwt_simple::algorithms::Ed25519PublicKey::from_bytes(bytes.as_slice())
                    .map_err(|err| {
                        anyhow::Error::msg(format!(
                            "Cannot build EdDSA key from public key bytes: {:?}",
                            err,
                        ))
                    })?;
                pk.verify_token::<$type>($token, Some($options))
            }
        }
        .map_err(|_| anyhow::Error::msg("Invalid Token"))
    };
}
