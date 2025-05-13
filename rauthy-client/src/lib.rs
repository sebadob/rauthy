//! Minimal and safe by default client library for the [Rauthy](https://github.com/sebadob/rauthy)
//! project.
//!
//! You can of course use any generic OIDC client with [Rauthy](https://github.com/sebadob/rauthy).
//! However, the idea of this crate is to provide the simplest possible production ready setup,
//! with the least amount of overhead and secure default values, if you only use
//! [Rauthy](https://github.com/sebadob/rauthy) anyway.
//!
//! You can find examples for `actix-web`, `axum` or a fully generic framework / application in the
//! [Examples](https://github.com/sebadob/rauthy/tree/main/rauthy-client/examples) directory.
//!
//! # Features
//!
//! - `actix-web` enables actix-web specific extractors and api
//! - `axum` enables axum specific extractors and api
//! - `backchannel-logout` adds `LogoutToken` + validation functions for OIDC Backchannel Logout
//! - `device-code` adds everything oyu need to the device code flow. This will most probably be
//!   used without default features.
//! - `qrcode` brings QR Code geneation in combination with the `device-code` feature
//! - `scim` adds types and helpers to implement the client side of SCIM v2 in a Rauthy-compatible
//!   way
//! - `userinfo` adds additional types and helpers to easily fetch the `/userinfo` endpoint and
//!   actively validate against it.

use crate::provider::OidcProvider;
use base64::{engine, engine::general_purpose, Engine as _};
use rand::{distr, Rng};
use tracing::{error, warn};

use crate::handler::OidcCookieInsecure;
use crate::jwks::jwks_handler;
use crate::rauthy_error::RauthyError;
pub use reqwest::Certificate as RootCertificate;
use sha2::{Digest, Sha256};

/// OIDC Backchannel Logout
#[cfg(feature = "backchannel-logout")]
pub mod backchannel_logout;

/// Handles the encrypted OIDC state cookie for the login flow
pub mod cookie_state;

/// Device Code Flow: urn:ietf:params:oauth:grant-type:device_code
#[cfg(feature = "device-code")]
pub mod device_code;

/// The api which need to be called from your endpoints
pub mod handler;
mod jwks;
/// The Rauthy OIDC config
pub mod oidc_config;
/// The authenticated Principal, extracted from valid JWT tokens
pub mod principal;
/// Rauthy / OIDC provider config and setup
pub mod provider;
/// Provides everything necessary to extract and validate JWT token claims
pub mod token_set;

pub mod rauthy_error;

/// SCIM v2 types for full compatibility with Rauthy
#[cfg(feature = "scim")]
pub mod scim;

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

const B64_URL_SAFE_NO_PAD: engine::GeneralPurpose = general_purpose::URL_SAFE_NO_PAD;

/// Decodes a base64 value
#[allow(dead_code)]
pub(crate) fn b64_decode(value: &str) -> Result<Vec<u8>, RauthyError> {
    Ok(general_purpose::STANDARD.decode(value)?)
}

/// Returns the given input as a base64 encoded String
#[inline]
pub(crate) fn b64_encode(value: &[u8]) -> String {
    general_purpose::STANDARD.encode(value)
}

/// Returns the given input as a base64 URL encoded String
#[inline]
pub(crate) fn base64_url_encode(input: &[u8]) -> String {
    let b64 = general_purpose::STANDARD.encode(input);
    b64.chars()
        .filter_map(|c| match c {
            '=' => None,
            '+' => Some('-'),
            '/' => Some('_'),
            x => Some(x),
        })
        .collect()
}

pub(crate) fn base64_url_no_pad_decode(b64: &str) -> Result<Vec<u8>, RauthyError> {
    Ok(B64_URL_SAFE_NO_PAD.decode(b64)?)
}

#[inline]
#[allow(dead_code)]
fn build_lax_cookie_300(name: &str, value: &str, insecure: OidcCookieInsecure) -> String {
    if insecure != OidcCookieInsecure::Yes {
        format!(
            "{}={}; Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age=300",
            name, value
        )
    } else {
        warn!("Building an INSECURE cookie - DO NOT USE IN PRODUCTION");
        format!(
            "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=300",
            name, value
        )
    }
}

/// Extracts the claims from a given token into the given struct.
/// CAUTION: Does not validate the token!
pub fn extract_token_claims<T>(token: &str) -> Result<T, RauthyError>
where
    T: for<'a> serde::Deserialize<'a>,
{
    let body = match token.split_once('.') {
        None => None,
        Some((_metadata, rest)) => rest.split_once('.').map(|(body, _validation_str)| body),
    };
    if body.is_none() {
        return Err(RauthyError::MalformedJwt("Invalid or malformed JWT Token"));
    }
    let body = body.unwrap();

    let b64 = match B64_URL_SAFE_NO_PAD.decode(body) {
        Ok(values) => values,
        Err(err) => {
            error!(
                "Error decoding JWT token body '{}' from base64: {}",
                body, err
            );
            return Err(RauthyError::InvalidJwt("Invalid JWT Token body"));
        }
    };
    let s = String::from_utf8_lossy(b64.as_slice());
    let claims = match serde_json::from_str::<T>(s.as_ref()) {
        Ok(claims) => claims,
        Err(err) => {
            error!("Error deserializing JWT Token claims: {}", err);
            return Err(RauthyError::InvalidJwt("Invalid JWT Token claims"));
        }
    };

    Ok(claims)
}

/// Generates a secure random pkce s256 challenge and returns `(verifier, challenge)`
#[inline]
pub fn generate_pkce_challenge() -> (String, String) {
    let plain = secure_random(32);
    let mut hasher = Sha256::new();
    hasher.update(plain.as_bytes());
    let s256 = hasher.finalize();
    let challenge = base64_url_encode(s256.as_ref());
    (plain, challenge)
}

#[derive(Debug, Clone, PartialEq)]
pub enum RauthyHttpsOnly {
    Yes,
    No,
}

impl RauthyHttpsOnly {
    pub fn bool(&self) -> bool {
        self == &Self::Yes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DangerAcceptInvalidCerts {
    Yes,
    No,
}

impl DangerAcceptInvalidCerts {
    pub fn bool(&self) -> bool {
        self == &Self::Yes
    }
}

/// The init function must be called exactly once during your app start up before(!) the
/// OidcProvider::setup_*() function.
/// It will initialize variables, clients, cache, and validate the OIDC configuration.
///
/// # Panics
/// This will panic if it is called more than once.
pub async fn init() -> Result<(), RauthyError> {
    OidcProvider::init_client(None, RauthyHttpsOnly::Yes, DangerAcceptInvalidCerts::No)?;
    jwks_handler().await;
    Ok(())
}

/// This function must be called exactly once during your app start up before(!) the
/// OidcProvider::setup_*() function.
/// It will initialize variables, clients, cache, and validate the OIDC configuration.
///
/// # Panics
/// This will panic if it is called more than once.
pub async fn init_with(
    root_certificate: Option<RootCertificate>,
    https_only: RauthyHttpsOnly,
    danger_accept_invalid_certs: DangerAcceptInvalidCerts,
) -> Result<(), RauthyError> {
    OidcProvider::init_client(root_certificate, https_only, danger_accept_invalid_certs)?;
    jwks_handler().await;
    Ok(())
}

/// Generates a secure random alphanumeric value with the given length.
#[inline]
pub fn secure_random(count: usize) -> String {
    rand::rng()
        .sample_iter(&distr::Alphanumeric)
        .take(count)
        .map(char::from)
        .collect::<String>()
}
