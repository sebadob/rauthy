use base64::DecodeError;
use bincode::ErrorKind;
use chacha20poly1305::Error;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RauthyError {
    #[error("BadRequest: {0}")]
    BadRequest(&'static str),
    #[error("Base64: {0}")]
    Base64(String),
    #[error("Encryption: {0}")]
    Encryption(Cow<'static, str>),
    #[error("Internal: {0}")]
    Internal(Cow<'static, str>),
    #[error("Init: {0}")]
    Init(&'static str),
    #[error("InvalidClaims: {0}")]
    InvalidClaims(&'static str),
    #[error("InvalidJwt: {0}")]
    InvalidJwt(&'static str),
    #[error("JWK: {0}")]
    JWK(Cow<'static, str>),
    #[error("MalformedJwt: {0}")]
    MalformedJwt(&'static str),
    #[error("Provider: {0}")]
    Provider(Cow<'static, str>),
    #[error("Request Error: {0}")]
    Request(Cow<'static, str>),
    #[error("Serde: {0}")]
    Serde(String),
    #[error("Token: {0}")]
    Token(Cow<'static, str>),
}

impl From<base64::DecodeError> for RauthyError {
    fn from(value: DecodeError) -> Self {
        Self::Base64(value.to_string())
    }
}

impl From<chacha20poly1305::Error> for RauthyError {
    fn from(value: Error) -> Self {
        Self::Encryption(Cow::from(value.to_string()))
    }
}

impl From<Box<bincode::ErrorKind>> for RauthyError {
    fn from(value: Box<ErrorKind>) -> Self {
        Self::Serde(value.to_string())
    }
}

impl From<reqwest::Error> for RauthyError {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(Cow::from(value.to_string()))
    }
}

impl From<jwt_simple::Error> for RauthyError {
    fn from(value: jwt_simple::Error) -> Self {
        Self::Token(Cow::from(value.to_string()))
    }
}

impl From<serde_json::Error> for RauthyError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value.to_string())
    }
}

#[cfg(feature = "qrcode")]
impl From<qrcode::types::QrError> for RauthyError {
    fn from(value: qrcode::types::QrError) -> Self {
        Self::Internal(Cow::from(value.to_string()))
    }
}
