use crate::provider::OidcProvider;
use crate::rauthy_error::RauthyError;
use crate::tokens::token::JwtToken;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct JktClaim {
    pub jkt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonClaims {
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
    pub iss: String,
    pub jti: Option<String>,
    pub aud: String,
    pub sub: Option<String>,
    // pub nonce: Option<&'a str>,
    pub typ: TokenType,
    pub azp: String,
    pub scope: Option<String>,
    pub did: Option<String>,
    pub cnf: Option<JktClaim>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddressClaim {
    pub formatted: String,
    pub street_address: Option<String>,
    pub locality: Option<String>,
    // pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(flatten)]
    pub common: CommonClaims,

    // pub scope: &'a str,
    pub allowed_origins: Option<Vec<String>>,
    // user part
    pub email: Option<String>,
    pub roles: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

impl AccessToken {
    #[inline(always)]
    pub async fn from_token_validated(token: &str) -> Result<Self, RauthyError> {
        let mut buf = Vec::with_capacity(1024);
        Self::from_token_validated_buf(token, &mut buf).await
    }

    #[inline(always)]
    pub async fn from_token_validated_buf(
        token: &str,
        buf: &mut Vec<u8>,
    ) -> Result<Self, RauthyError> {
        JwtToken::validate_claims_into(token, Some(TokenType::Bearer), buf).await?;
        let slf = serde_json::from_slice::<Self>(buf)?;

        Ok(slf)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdToken {
    #[serde(flatten)]
    pub common: CommonClaims,

    pub amr: Vec<String>,
    pub auth_time: i64,
    pub at_hash: String,
    pub sid: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub preferred_username: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub address: Option<AddressClaim>,
    pub birthdate: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
    pub nonce: Option<String>,
    pub phone_number: Option<String>,
    pub phone_number_verified: Option<bool>,
    pub roles: Vec<String>,
    pub groups: Option<Vec<String>>,
    pub custom: Option<HashMap<String, serde_json::Value>>,
    pub webid: Option<String>,
    pub zoneinfo: Option<String>,
}

impl IdToken {
    #[inline(always)]
    pub async fn from_token_validated(token: &str, nonce: &str) -> Result<Self, RauthyError> {
        let mut buf = Vec::with_capacity(1024);
        Self::from_token_validated_buf(token, nonce, &mut buf).await
    }

    #[inline(always)]
    pub async fn from_token_validated_buf(
        token: &str,
        nonce: &str,
        buf: &mut Vec<u8>,
    ) -> Result<Self, RauthyError> {
        JwtToken::validate_claims_into(token, Some(TokenType::Id), buf).await?;
        let slf = serde_json::from_slice::<Self>(buf)?;

        if OidcProvider::config()?.email_verified && slf.email_verified != Some(true) {
            return Err(RauthyError::InvalidClaims("'email_verified' is false"));
        }
        // we want to validate the nonce manually each time for more efficiency
        if slf.nonce.as_deref() != Some(nonce) {
            return Err(RauthyError::InvalidClaims("'nonce' is not correct"));
        }

        Ok(slf)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    #[serde(flatten)]
    pub common: CommonClaims,

    pub uid: String,
    pub auth_time: Option<i64>,
}

/// Rauthy-supported token types. This client however does currently not support `DPoP`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Bearer,
    DPoP,
    Id,
    Refresh,
    #[cfg(feature = "backchannel-logout")]
    #[serde(rename = "logout+jwt")]
    Logout,
}

impl TokenType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bearer => "Bearer",
            Self::DPoP => "DPoP",
            Self::Id => "Id",
            #[cfg(feature = "backchannel-logout")]
            Self::Logout => "logout+jwt",
            Self::Refresh => "Refresh",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum JwtAmrValue {
    Pwd,
    Mfa,
}
