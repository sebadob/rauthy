use crate::base64_url_no_pad_decode_buf;
use crate::provider::OidcProvider;
use crate::rauthy_error::RauthyError;
use crate::tokens::claims::TokenType;
use crate::tokens::jwks::{JwkKeyPairAlg, JwkPublicKey};
use chrono::Utc;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug, Default, Deserialize)]
pub struct JwtHeader<'a> {
    pub alg: JwkKeyPairAlg,
    pub kid: &'a str,
    // should always be JWT -> DPoP tokens have their own validation
    pub typ: &'a str,
}

pub struct JwtToken;

impl JwtToken {
    /// Validates the given, raw JWT token, deserializes and validates it. On success, the raw
    /// claims bytes will be written into `buf`.
    #[inline(always)]
    pub async fn validate_claims_into(
        token: &str,
        expected_type: Option<TokenType>,
        buf: &mut [u8],
    ) -> Result<(), RauthyError> {
        debug_assert!(buf.is_empty());

        let mut split = token.split('.');

        let Some(header) = split.next() else {
            return Err(RauthyError::InvalidJwt(
                "Cannot deserialize JWT Token header",
            ));
        };
        let Some(claims) = split.next() else {
            return Err(RauthyError::InvalidJwt(
                "Cannot deserialize JWT Token claims",
            ));
        };
        if split.next().is_none() {
            return Err(RauthyError::InvalidJwt(
                "Cannot deserialize JWT Token signature",
            ));
        };
        if split.next().is_some() {
            return Err(RauthyError::InvalidJwt("Invalid JWT token format"));
        }

        let mut buf = Vec::with_capacity(1024);
        base64_url_no_pad_decode_buf(header, &mut buf)?;
        let header = serde_json::from_slice::<JwtHeader>(&buf)?;
        if header.typ != "JWT" {
            return Err(RauthyError::InvalidJwt("Invalid JWT Header `typ`"));
        }
        let jwk = JwkPublicKey::get_for_token(token).await?;
        if jwk.alg != header.alg {
            return Err(RauthyError::JWK(
                "Invalid JWT Header `alg` does not match `kid`".into(),
            ));
        }
        jwk.validate_token_signature(token, &mut buf)?;

        buf.clear();
        base64_url_no_pad_decode_buf(claims, &mut buf)?;

        let config = OidcProvider::config()?;
        serde_json::from_slice::<ValidationClaims>(&buf)?.validate(
            &config.allowed_issuers,
            &config.allowed_audiences,
            expected_type,
            None,
        )?;

        Ok(())
    }
}

// Note: No need to include `aud` in the validation. This will never be `rauthy` itself, and all
// other possibilities are validated indirectly with a `Client` lookup afterward, which will be
// done by grabbing the `client_id` from `aud` / `azp` anyway.
#[derive(Debug, Deserialize)]
struct ValidationClaims<'a> {
    iat: i64,
    exp: i64,
    nbf: i64,
    // we need to support 2 types here: single string and array of strings
    aud: Value,
    iss: &'a str,
    typ: TokenType,
}

impl ValidationClaims<'_> {
    #[inline(always)]
    fn validate(
        &self,
        allowed_issuers: &HashSet<String>,
        allowed_audiences: &HashSet<String>,
        expected_type: Option<TokenType>,
        allowed_clock_skew_seconds: Option<u16>,
    ) -> Result<(), RauthyError> {
        if let Some(skew) = allowed_clock_skew_seconds {
            let now = Utc::now().timestamp();
            let skew = skew as i64;

            if self.iat - skew > now {
                return Err(RauthyError::InvalidJwt("Token was issued in the future"));
            }
            if self.exp + skew < now {
                return Err(RauthyError::InvalidJwt("Token has expired"));
            }
            if self.nbf - skew > now {
                return Err(RauthyError::InvalidJwt("Token is not valid yet"));
            }
        }
        if let Some(typ) = expected_type
            && self.typ != typ
        {
            return Err(RauthyError::InvalidJwt("Invalid `typ`"));
        }
        if !allowed_issuers.contains(self.iss) {
            return Err(RauthyError::InvalidJwt("Invalid `iss`"));
        }

        match &self.aud {
            Value::String(aud) => {
                if !allowed_audiences.contains(aud) {
                    return Err(RauthyError::InvalidJwt("Invalid `aud`"));
                }
            }
            Value::Array(arr) => {
                let mut found_match = false;
                for aud in arr {
                    let Value::String(aud) = aud else {
                        return Err(RauthyError::InvalidJwt("Invalid `aud` claims"));
                    };
                    if allowed_audiences.contains(aud) {
                        found_match = true;
                        break;
                    }
                }
                if !found_match {
                    return Err(RauthyError::InvalidJwt("Invalid `aud`"));
                }
            }
            _ => {
                return Err(RauthyError::InvalidJwt("Invalid `aud`"));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::rauthy_error::RauthyError;
    use crate::tokens::claims::TokenType;
    use crate::tokens::token::ValidationClaims;
    use axum::response::ErrorResponse;
    use chrono::Utc;
    use serde_json::json;
    use std::collections::HashSet;

    #[test]
    fn test_validation_claims() -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let iss = "http://localhost:8080/auth/v1";
        let iss_set = HashSet::from([iss.to_string()]);
        let aud = "client1";
        let aud_set = HashSet::from([aud.to_string()]);

        ValidationClaims {
            iat: now,
            exp: now + 60,
            nbf: now,
            aud: json!(aud),
            iss,
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Bearer), Some(0))?;

        ValidationClaims {
            iat: now,
            exp: now + 60,
            nbf: now,
            aud: json!([aud, "other_aud"]),
            iss,
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Bearer), Some(0))?;

        ValidationClaims {
            iat: now + 2,
            exp: now + 60,
            nbf: now + 2,
            aud: json!(aud),
            iss,
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Bearer), Some(2))?;

        let res = ValidationClaims {
            iat: now,
            exp: now + 60,
            nbf: now + 2,
            aud: json!(aud),
            iss,
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Bearer), Some(0));
        assert_eq!(res, Err(RauthyError::InvalidJwt("Token is not valid yet")));

        let res = ValidationClaims {
            iat: now + 2,
            exp: now + 60,
            nbf: now + 2,
            aud: json!(aud),
            iss,
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Bearer), Some(0));
        assert_eq!(
            res,
            Err(RauthyError::InvalidJwt("Token was issued in the future"))
        );

        let res = ValidationClaims {
            iat: now - 60,
            exp: now - 3,
            nbf: now - 60,
            aud: json!(aud),
            iss,
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Bearer), Some(0));
        assert_eq!(res, Err(RauthyError::InvalidJwt("Token has expired")));

        let res = ValidationClaims {
            iat: now - 60,
            exp: now - 3,
            nbf: now - 60,
            aud: json!(aud),
            iss,
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Bearer), Some(5));
        assert_eq!(res, Ok(()));

        let res = ValidationClaims {
            iat: now,
            exp: now + 10,
            nbf: now,
            aud: json!(aud),
            iss,
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Id), Some(0));
        assert_eq!(res, Err(RauthyError::InvalidJwt("Invalid `typ`")));

        let res = ValidationClaims {
            iat: now,
            exp: now + 10,
            nbf: now,
            aud: json!(aud),
            iss: "http://localhost:9090/something/else",
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Bearer), Some(0));
        assert_eq!(res, Err(RauthyError::InvalidJwt("Invalid `iss`")));

        let res = ValidationClaims {
            iat: now,
            exp: now + 10,
            nbf: now,
            aud: json!("invalid_aud"),
            iss,
            typ: TokenType::Bearer,
        }
        .validate(&iss_set, &aud_set, Some(TokenType::Bearer), Some(0));
        assert_eq!(res, Err(RauthyError::InvalidJwt("Invalid `aud`")));

        Ok(())
    }
}
