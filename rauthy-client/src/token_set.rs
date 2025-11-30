use crate::jwks::{JwkKeyPairAlg, JwkPublicKey};
use crate::provider::OidcProvider;
use crate::rauthy_error::RauthyError;
use crate::{base64_url_no_pad_decode, validate_jwt};
use jwt_simple::algorithms::{EdDSAPublicKeyLike, RSAPublicKeyLike};
use jwt_simple::claims;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

/// The token set returned upon a successful login flow
#[derive(Debug, Serialize, Deserialize)]
pub struct OidcTokenSet {
    pub access_token: String,
    pub token_type: Option<String>,
    pub id_token: Option<String>,
    pub expires_in: i32,
    pub refresh_token: Option<String>,
}

impl OidcTokenSet {
    /// Returns the user claims from the `id_token`.
    /// CAUTION: This does NOT validate the signature!
    pub fn id_claims(&self) -> Result<Option<JwtIdClaims>, RauthyError> {
        if let Some(raw_token) = &self.id_token {
            let (_, rest) = raw_token.split_once('.').unwrap_or(("", ""));
            let (claims_b64, _) = rest.split_once('.').unwrap_or(("", ""));
            let bytes = base64_url_no_pad_decode(claims_b64)?;
            let s = String::from_utf8_lossy(&bytes);
            let claims = serde_json::from_str::<JwtIdClaims>(s.as_ref())?;
            Ok(Some(claims))
        } else {
            Ok(None)
        }
    }

    /// This function will return the claims from a given JWT token.
    /// CAUTION: It does NOT VALIDATE the token signature or any other values!
    /// It will only try to decode the payload into the target struct.
    pub fn danger_claims_unvalidated<T>(token: &str) -> Result<T, RauthyError>
    where
        T: Debug + for<'a> serde::de::Deserialize<'a>,
    {
        let mut split = token.split(".");

        // The first part is the header
        if split.next().is_none() {
            return Err(RauthyError::Token(Cow::from(
                "Bad format for raw token - header missing",
            )));
        }

        // The second part are the claims we care about
        let claims_b64 = match split.next() {
            None => {
                return Err(RauthyError::Token(Cow::from(
                    "Bad format for raw token - claims missing",
                )));
            }
            Some(s) => s,
        };

        let bytes = base64_url_no_pad_decode(claims_b64)?;
        let s = String::from_utf8_lossy(&bytes);
        let claims = serde_json::from_str::<T>(&s)?;

        Ok(claims)
    }
}

/// Rauthy-supported token types. This client however does currently not support `DPoP`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JwtTokenType {
    Bearer,
    DPoP,
    Id,
    Refresh,
    #[cfg(feature = "backchannel-logout")]
    #[serde(rename = "logout+jwt")]
    Logout,
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
    pub roles: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
    pub cnf: Option<JktClaim>,
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

impl JwtAccessClaims {
    pub async fn from_token_validated(token: &str) -> Result<Self, RauthyError> {
        let config = OidcProvider::config()?;

        let pubkey = JwkPublicKey::get_for_token(token).await?;
        let claims: claims::JWTClaims<Self> =
            validate_jwt!(Self, pubkey, token, config.verification_options.clone())?;

        let mut slf = claims.custom;
        if slf.typ != JwtTokenType::Bearer {
            return Err(RauthyError::Token(Cow::from("Must provide a Bearer token")));
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
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtIdClaims {
    pub sub: Option<String>,
    pub azp: String,
    pub typ: JwtTokenType,
    pub amr: Vec<String>,
    pub auth_time: i64,
    pub at_hash: Option<String>,
    pub sid: Option<String>,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub address: Option<AddressClaim>,
    pub birthdate: Option<String>,
    pub locale: Option<String>,
    pub picture: Option<String>,
    pub phone: Option<String>,
    pub roles: Vec<String>,
    pub groups: Option<Vec<String>>,
    pub cnf: Option<JktClaim>,
    pub custom: Option<HashMap<String, serde_json::Value>>,
    pub webid: Option<String>,
    pub zoneinfo: Option<String>,
}

impl JwtIdClaims {
    pub async fn from_token_validated(token: &str, nonce: &str) -> Result<Self, RauthyError> {
        let config = OidcProvider::config()?;

        let pubkey = JwkPublicKey::get_for_token(token).await?;
        let claims: claims::JWTClaims<Self> =
            validate_jwt!(Self, pubkey, token, config.verification_options.clone())?;

        let mut slf = claims.custom;
        slf.sub = claims.subject;

        if slf.typ != JwtTokenType::Id {
            return Err(RauthyError::Token(Cow::from("Must provide an id token")));
        }

        if config.email_verified && slf.email_verified != Some(true) {
            return Err(RauthyError::InvalidClaims(
                "'email_verified' is missing or false",
            ));
        }
        // we want to validate the nonce manually each time for more efficiency
        if claims.nonce.as_deref() != Some(nonce) {
            return Err(RauthyError::InvalidClaims("'nonce' is not correct"));
        }

        Ok(slf)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtRefreshClaims {
    pub iat: i64,
    pub exp: i64,
    pub nbf: i64,
    pub iss: String,
    pub aud: String,
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
                    .ok_or_else(|| RauthyError::JWK(Cow::from("Invalid 'n' for RS256 key")))?;
                let e = $pub_key
                    .e
                    .as_ref()
                    .ok_or_else(|| RauthyError::JWK(Cow::from("Invalid 'e' for RS256 key")))?;
                let pk = jwt_simple::algorithms::RS256PublicKey::from_components(
                    n.as_bytes(),
                    e.as_bytes(),
                )
                .map_err(|err| {
                    RauthyError::JWK(Cow::from(format!(
                        "Cannot build RS256 key from public key components: {:?}",
                        err,
                    )))
                })?;
                pk.verify_token::<$type>($token, Some($options))
            }

            JwkKeyPairAlg::RS384 => {
                let n = $pub_key
                    .n
                    .as_ref()
                    .ok_or_else(|| RauthyError::JWK(Cow::from("Invalid 'n' for RS384 key")))?;
                let e = $pub_key
                    .e
                    .as_ref()
                    .ok_or_else(|| RauthyError::JWK(Cow::from("Invalid 'e' for RS384 key")))?;
                let pk = jwt_simple::algorithms::RS384PublicKey::from_components(
                    n.as_bytes(),
                    e.as_bytes(),
                )
                .map_err(|err| {
                    RauthyError::JWK(Cow::from(format!(
                        "Cannot build RS384 key from public key components: {:?}",
                        err,
                    )))
                })?;
                pk.verify_token::<$type>($token, Some($options))
            }

            JwkKeyPairAlg::RS512 => {
                let n = $pub_key
                    .n
                    .as_ref()
                    .ok_or_else(|| RauthyError::JWK(Cow::from("Invalid 'n' for RS512 key")))?;
                let e = $pub_key
                    .e
                    .as_ref()
                    .ok_or_else(|| RauthyError::JWK(Cow::from("Invalid 'e' for RS512 key")))?;
                let pk = jwt_simple::algorithms::RS512PublicKey::from_components(
                    n.as_bytes(),
                    e.as_bytes(),
                )
                .map_err(|err| {
                    RauthyError::JWK(Cow::from(format!(
                        "Cannot build RS512 key from public key components: {:?}",
                        err,
                    )))
                })?;
                pk.verify_token::<$type>($token, Some($options))
            }

            JwkKeyPairAlg::EdDSA => {
                let bytes = $pub_key
                    .x_bytes
                    .as_ref()
                    .ok_or_else(|| RauthyError::JWK(Cow::from("Invalid 'x' for EdDSA key")))?;
                let pk = jwt_simple::algorithms::Ed25519PublicKey::from_bytes(bytes.as_slice())
                    .map_err(|err| {
                        RauthyError::JWK(Cow::from(format!(
                            "Cannot build EdDSA key from public key bytes: {:?}",
                            err,
                        )))
                    })?;
                pk.verify_token::<$type>($token, Some($options))
            }
        }
        .map_err(|err| RauthyError::Token(Cow::from(err.to_string())))
    };
}
