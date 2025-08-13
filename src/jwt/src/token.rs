use crate::claims::JwtTokenType;
use chrono::Utc;
use rauthy_common::utils::{base64_url_no_pad_decode_buf, base64_url_no_pad_encode_buf};
use rauthy_data::entity::jwk::{JwkKeyPair, JwkKeyPairAlg};
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::warn;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JwtHeader<'a> {
    pub alg: JwkKeyPairAlg,
    pub kid: &'a str,
    // should always be JWT -> DPoP tokens have their own validation
    pub typ: &'a str,
}

pub struct JwtToken;

impl JwtToken {
    pub fn build<C: Debug + Serialize>(
        jwk: &JwkKeyPair,
        claims: &C,
    ) -> Result<String, ErrorResponse> {
        let mut token = String::with_capacity(1024);

        let header = format!(
            "{{\"alg\":\"{}\",\"kid\":\"{}\",\"typ\":\"JWT\"}}",
            jwk.typ.as_str(),
            jwk.kid
        );
        base64_url_no_pad_encode_buf(header.as_bytes(), &mut token);
        token.push('.');

        let claims = serde_json::to_string(claims)?;
        base64_url_no_pad_encode_buf(claims.as_bytes(), &mut token);

        let sig = jwk.sign(token.as_bytes())?;
        token.push('.');
        base64_url_no_pad_encode_buf(&sig, &mut token);

        Ok(token)
    }

    /// Validates the given, raw JWT token, deserializes and validates it. On success, the raw
    /// claims bytes will be written into `buf`.
    pub async fn validate_claims_into(
        token: &str,
        expected_type: Option<JwtTokenType>,
        allowed_clock_skew_seconds: u16,
        buf: &mut Vec<u8>,
    ) -> Result<(), ErrorResponse> {
        debug_assert!(buf.is_empty());

        if token.len() > RauthyConfig::get().vars.access.token_len_limit as usize {
            warn!(
                "Received a JWT token above the size limit TOKEN_LEN_LIMIT. Either this is an \
                exhaustion attack, or you create very big tokens and might need to increase the \
                limit"
            );
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "JWT token input too long",
            ));
        }

        let mut split = token.split('.');

        let Some(header) = split.next() else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Cannot deserialize JWT Token header",
            ));
        };
        let Some(claims) = split.next() else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Cannot deserialize JWT Token claims",
            ));
        };
        if split.next().is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Cannot deserialize JWT Token signature",
            ));
        };
        if split.next().is_some() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid JWT token format",
            ));
        }

        base64_url_no_pad_decode_buf(header, buf)?;
        let header = serde_json::from_slice::<JwtHeader>(buf)?;
        if header.typ != "JWT" {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid JWT Header `typ`",
            ));
        }
        let jwk = JwkKeyPair::find(header.kid.to_string()).await?;
        if jwk.typ != header.alg {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid JWT Header `alg` does not match `kid`",
            ));
        }
        buf.clear();
        jwk.verify_token(token, buf)?;

        buf.clear();
        base64_url_no_pad_decode_buf(claims, buf)?;
        serde_json::from_slice::<ValidationClaims>(buf)?.validate(
            &RauthyConfig::get().issuer,
            expected_type,
            allowed_clock_skew_seconds,
        )?;

        Ok(())
    }
}

// Note: No need to include `aud` in the validation. This will never be `rauthy` itself, and all
// other possibilities are validated indirectly with a `Client` lookup afterward, which will be
// done ba grabbing the `client_id` from `aud` / `azp` anyway.
#[derive(Debug, Deserialize)]
struct ValidationClaims<'a> {
    iat: i64,
    exp: i64,
    nbf: i64,
    iss: &'a str,
    typ: JwtTokenType,
}

impl ValidationClaims<'_> {
    #[inline]
    fn validate(
        &self,
        issuer: &str,
        expected_type: Option<JwtTokenType>,
        allowed_clock_skew_seconds: u16,
    ) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let skew = allowed_clock_skew_seconds as i64;

        if self.iat - skew > now {
            return Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Token was issued in the future",
            ));
        }
        if self.exp + skew < now {
            return Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Token has expired",
            ));
        }
        if self.nbf - skew > now {
            return Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Token is not valid yet",
            ));
        }
        if let Some(typ) = expected_type
            && self.typ != typ
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Invalid `typ`",
            ));
        }
        if self.iss != issuer {
            return Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Invalid `iss`",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::claims::JwtTokenType;
    use crate::token::ValidationClaims;
    use chrono::Utc;
    use rauthy_error::{ErrorResponse, ErrorResponseType};

    #[test]
    fn test_validation_claims() -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();
        let iss = "http://localhost:8080/auth/v1";

        ValidationClaims {
            iat: now,
            exp: now + 60,
            nbf: now,
            iss,
            typ: JwtTokenType::Bearer,
        }
        .validate(iss, Some(JwtTokenType::Bearer), 0)?;

        ValidationClaims {
            iat: now + 2,
            exp: now + 60,
            nbf: now + 2,
            iss,
            typ: JwtTokenType::Bearer,
        }
        .validate(iss, Some(JwtTokenType::Bearer), 2)?;

        let res = ValidationClaims {
            iat: now,
            exp: now + 60,
            nbf: now + 2,
            iss,
            typ: JwtTokenType::Bearer,
        }
        .validate(iss, Some(JwtTokenType::Bearer), 0);
        assert_eq!(
            res,
            Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Token is not valid yet"
            ))
        );

        let res = ValidationClaims {
            iat: now + 2,
            exp: now + 60,
            nbf: now + 2,
            iss,
            typ: JwtTokenType::Bearer,
        }
        .validate(iss, Some(JwtTokenType::Bearer), 0);
        assert_eq!(
            res,
            Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Token was issued in the future"
            ))
        );

        let res = ValidationClaims {
            iat: now - 60,
            exp: now - 3,
            nbf: now - 60,
            iss,
            typ: JwtTokenType::Bearer,
        }
        .validate(iss, Some(JwtTokenType::Bearer), 0);
        assert_eq!(
            res,
            Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Token has expired"
            ))
        );

        let res = ValidationClaims {
            iat: now - 60,
            exp: now - 3,
            nbf: now - 60,
            iss,
            typ: JwtTokenType::Bearer,
        }
        .validate(iss, Some(JwtTokenType::Bearer), 5);
        assert_eq!(res, Ok(()));

        let res = ValidationClaims {
            iat: now,
            exp: now + 10,
            nbf: now,
            iss,
            typ: JwtTokenType::Bearer,
        }
        .validate(iss, Some(JwtTokenType::Id), 0);
        assert_eq!(
            res,
            Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Invalid `typ`"
            ))
        );

        let res = ValidationClaims {
            iat: now,
            exp: now + 10,
            nbf: now,
            iss: "http://localhost:9090/something/else",
            typ: JwtTokenType::Bearer,
        }
        .validate(iss, Some(JwtTokenType::Bearer), 0);
        assert_eq!(
            res,
            Err(ErrorResponse::new(
                ErrorResponseType::JwtToken,
                "Invalid `iss`"
            ))
        );

        Ok(())
    }
}
