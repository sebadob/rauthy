use crate::base64_url_no_pad_decode;
use crate::rauthy_error::RauthyError;
use crate::tokens::claims::IdToken;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
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
    ///
    /// CAUTION: This does NOT validate the signature!
    pub fn id_claims(&self) -> Result<Option<IdToken>, RauthyError> {
        if let Some(raw_token) = &self.id_token {
            let (_, rest) = raw_token.split_once('.').unwrap_or(("", ""));
            let (claims_b64, _) = rest.split_once('.').unwrap_or(("", ""));
            let bytes = base64_url_no_pad_decode(claims_b64)?;
            let s = String::from_utf8_lossy(&bytes);
            let claims = serde_json::from_str::<IdToken>(s.as_ref())?;
            Ok(Some(claims))
        } else {
            Ok(None)
        }
    }

    /// This function will return the claims from a given JWT token.
    ///
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
