use chrono::Utc;
use cryptr::utils::secure_random_alnum;
use rauthy_common::utils::{base64_url_no_pad_decode, base64_url_no_pad_decode_buf};
use rauthy_error::ErrorResponse;
use rauthy_error::ErrorResponseType;
use rauthy_jwt::claims::{JwtCommonClaims, JwtLogoutClaims, JwtTokenType};
use rauthy_models::entity::auth_providers::AuthProvider;
use rauthy_models::entity::jwk::{JWKSPublicKey, JwkKeyPair, JwkKeyPairAlg};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;
use std::str::FromStr;
use std::string::ToString;
use std::sync::LazyLock;
use tracing::error;

static EVENT: &str = "http://schemas.openid.net/event/backchannel-logout";

static LOGOUT_TOKEN_LIFETIME: LazyLock<u8> = LazyLock::new(|| {
    env::var("LOGOUT_TOKEN_LIFETIME")
        .unwrap_or_else(|_| "30".to_string())
        .parse::<u8>()
        .expect("Cannot parse LOGOUT_TOKEN_LIFETIME as u8")
});
static LOGOUT_TOKEN_ALLOW_CLOCK_SKEW: LazyLock<u8> = LazyLock::new(|| {
    env::var("LOGOUT_TOKEN_ALLOW_CLOCK_SKEW")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<u8>()
        .expect("Cannot parse LOGOUT_TOKEN_ALLOW_CLOCK_SKEW as u8")
});
static LOGOUT_TOKEN_ALLOWED_LIFETIME: LazyLock<u16> = LazyLock::new(|| {
    env::var("LOGOUT_TOKEN_ALLOWED_LIFETIME")
        .unwrap_or_else(|_| "120".to_string())
        .parse::<u16>()
        .expect("Cannot parse LOGOUT_TOKEN_ALLOWED_LIFETIME as u16")
});

/// Logout Token for OIDC backchannel logout specified in
/// https://openid.net/specs/openid-connect-backchannel-1_0.html#LogoutToken
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutToken<'a> {
    // default claims
    pub iss: &'a str,
    pub aud: &'a str,
    pub exp: i64,

    // MUST always either not exist or be `logout+jwt`
    typ: Option<JwtTokenType>,
    // alg: JwkKeyPairAlg,
    pub iat: i64,
    pub jti: Cow<'a, str>,
    // MUST always contain `"http://schemas.openid.net/event/backchannel-logout": {}`
    events: serde_json::Value,

    pub sub: Option<&'a str>,
    pub sid: Option<&'a str>,

    // The `nonce` MUST NOT exist in this token. We try to deserialize into an `Option<_>` for easy
    // `.is_none()` validation.
    nonce: Option<&'a str>,
}

impl LogoutToken<'_> {
    pub fn new<'a>(
        issuer: &'a str,
        aud: &'a str,
        sub: Option<&'a str>,
        sid: Option<&'a str>,
    ) -> LogoutToken<'a> {
        let iat = Utc::now().timestamp();
        let exp = iat + *LOGOUT_TOKEN_LIFETIME as i64;

        let events = serde_json::Value::Object(serde_json::Map::from_iter([(
            EVENT.to_string(),
            serde_json::Value::Object(serde_json::Map::new()),
        )]));

        LogoutToken {
            iss: issuer,
            aud,
            exp,
            typ: Some(JwtTokenType::Logout),
            iat,
            jti: secure_random_alnum(8).into(),
            events,
            sub,
            sid,
            nonce: None,
        }
    }

    #[inline]
    pub async fn into_token(self, alg: JwkKeyPairAlg) -> Result<String, ErrorResponse> {
        let kp = JwkKeyPair::find_latest(alg).await?;
        self.into_token_with_kp(&kp)
    }

    #[inline]
    pub fn into_token_with_kp(self, kp: &JwkKeyPair) -> Result<String, ErrorResponse> {
        let claims = JwtLogoutClaims {
            common: JwtCommonClaims {
                iat: self.iat,
                nbf: self.iat,
                exp: self.exp,
                iss: self.iss,
                jti: Some(self.jti.as_ref()),
                aud: Cow::Borrowed(self.aud),
                sub: self.sub,
                typ: self.typ.clone().unwrap_or(JwtTokenType::Logout),
                azp: self.aud,
                scope: None,
                preferred_username: None,
                did: None,
                cnf: None,
            },
            events: self.events,
            sid: self.sid,
            nonce: None,
        };
        rauthy_jwt::token::JwtToken::build(kp, &claims)
    }

    /// Parse and validate the token as specified in
    /// https://openid.net/specs/openid-connect-backchannel-1_0.html#Validation
    pub async fn from_str_validated<'a>(
        logout_token: &'a str,
        buf: &'a mut Vec<u8>,
    ) -> Result<LogoutToken<'a>, ErrorResponse> {
        let (header, slf) = LogoutToken::build_from_str(logout_token, buf)?;
        let (kid, alg) = slf.validate_claims(header)?;

        let provider = AuthProvider::find_by_iss(slf.iss.to_string())
            .await
            .map_err(|err| {
                error!(
                    "Error looking up AuthProvider by issuer during backchannel logout: {:?}",
                    err
                );
                ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "invalid / unknown `issuer` for the given `logout_token`",
                )
            })?;
        if provider.jwks_endpoint.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "this provider has no configured `jwks_endpoint` for token validation",
            ));
        }

        if slf.aud != provider.client_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`aud` claim does not match our `client_id`",
            ));
        }

        // If we get here, the token we received seems to come from an actual configured upstream
        // auth provider, and we can try to fetch the public key and validate the signature.
        // Caching tokens by `jti` would be a waste of resources at this point.

        // validates signature
        let jwk =
            JWKSPublicKey::fetch_remote(provider.jwks_endpoint.as_ref().unwrap(), kid).await?;
        if jwk.alg != Some(alg) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`alg` mismatch between token header and fetched JWK",
            ));
        }
        let mut buf = Vec::with_capacity(256);
        jwk.validate_token_signature(logout_token, &mut buf)?;

        Ok(slf)
    }

    /// CAUTION: DO NOT use this function directly outside of tests. It DOES NOT VALIDATE the input.
    /// Only use `from_str_validated()`!
    ///
    /// Returns `(jwt_header, Self)`
    #[inline]
    pub fn build_from_str<'a>(
        logout_token: &'a str,
        buf: &'a mut Vec<u8>,
    ) -> Result<(serde_json::Value, LogoutToken<'a>), ErrorResponse> {
        // Before we can actually validate the token, we need to decode it and take a peek at the
        // issuer, so we can validate that we do have the issuer configured as an upstream provider.
        let mut split = logout_token.split(".");

        let header = match split.next() {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "invalid logout token format - header is missing",
                ));
            }
            Some(b64) => {
                let bytes = base64_url_no_pad_decode(b64)?;
                let s = String::from_utf8_lossy(&bytes);
                serde_json::from_str::<serde_json::Value>(&s)?
            }
        };

        match split.next() {
            None => Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "invalid logout token format - claims are missing",
            )),
            Some(b64) => {
                base64_url_no_pad_decode_buf(b64, buf)?;
                Ok((header, serde_json::from_slice::<LogoutToken>(buf)?))
            }
        }
    }

    #[inline]
    fn validate_claims(
        &self,
        header: serde_json::Value,
    ) -> Result<(String, JwkKeyPairAlg), ErrorResponse> {
        let alg = JwkKeyPairAlg::from_str(
            header
                .get("alg")
                .map(|a| a.as_str().unwrap_or_default())
                .unwrap_or_default(),
        )?;

        let kid = header
            .get("kid")
            .map(|a| a.as_str().unwrap_or_default())
            .unwrap_or_default()
            .to_string();
        if kid.is_empty() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`kid` is missing in token header",
            ));
        }

        if self.typ.is_some() && self.typ != Some(JwtTokenType::Logout) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "invalid token `typ`",
            ));
        }

        let now = Utc::now().timestamp();
        let iat_limit =
            now - *LOGOUT_TOKEN_ALLOWED_LIFETIME as i64 - *LOGOUT_TOKEN_ALLOW_CLOCK_SKEW as i64;
        let exp_limit = now + *LOGOUT_TOKEN_ALLOW_CLOCK_SKEW as i64;

        if self.iat < iat_limit {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`iat` is too long ago",
            ));
        }
        if self.exp < exp_limit {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "token has expired",
            ));
        }
        if self.exp <= self.iat {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`exp` must be greater than `iat`",
            ));
        }

        if self.sub.is_none() && self.sid.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "at least one of `sub` or `sid` must be given",
            ));
        }
        match self.events.get(EVENT) {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "`events.http://schemas.openid.net/event/backchannel-logout` is missing",
                ));
            }
            Some(value) => {
                if value != &serde_json::Value::Object(serde_json::Map::new()) {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "invalid value for `events.http://schemas.openid.net/event/backchannel-logout`",
                    ));
                }
            }
        }
        if self.nonce.is_some() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`nonce` MUST NOT be given",
            ));
        }

        // We don't really need to care about `jti` caching and validation. Replay attacks can only
        // happen for unencrypted connections, which Rauthy will never work with anyway.

        Ok((kid, alg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rauthy_common::utils::base64_decode;

    #[test]
    fn test_logout_token_validation() {
        // pre-generated keys to speed up testing

        // let kid = get_rand(8);
        // let ed25519 = Ed25519KeyPair::generate().with_key_id(&kid);
        // let der = base64_encode(&ed25519.to_der());
        // eprintln!("ed25519\nkid:\n{}\nder:\n{}", kid, der);
        let (kid_ed25519, b64_ed25519) = (
            "udzkw5Sh",
            "MC4CAQAwBQYDK2VwBCIEILBn5/9V9dehFRFcHM72yGhb/20l/HfFQVTa2avBxyDv",
        );
        let kp_25519 = JwkKeyPair {
            kid: kid_ed25519.to_string(),
            typ: JwkKeyPairAlg::EdDSA,
            bytes: base64_decode(b64_ed25519).unwrap(),
        };

        // let kid = get_rand(8);
        // let rsakp = RS256KeyPair::generate(2048).unwrap().with_key_id(&kid);
        // let der = base64_encode(rsakp.to_der().as_ref().unwrap());
        // eprintln!("rsa384\nkid:\n{}\nder:\n{}", kid, der);
        let (kid_rs256, b64_rs256) = (
            "sLgwc2ZJ",
            "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCqE1JsKbnE+xSj6N7zZ1ct8SsTPwOkO7/nyO2diOibd4VPUN34j7b931IIE7rtVBJPz8qdWgvf+Hinqxk7+kYdjO2t+UM1IKA8yDVFX/dco4q4aLAmNHugCzfUP05otPIR1l0YQQfTOd4BugYJE0d7yAT3dpUN43PUnT967ic9ODWY5CCSFDK1g43sAJ/KCeezfl3sHYwx6BVs7cB4jUv43BmCf+NS5tLNFPRtDLKSai9IRG1DIWJmAw2cCdKjQE9WSLwRwION8kkHZZptpaDLcQVkmhDgu/u5ZIZrxXd7eWwXacURqRghKznm3Gym7ss3qF36VxTFuQDCQduQ69fBAgMBAAECggEBAItYH1ytZx4D1SpiPmDQl3X1tSQkqdgieB3w2vYKWQIfj+KnT+Y54WTpVq11dN7D1ej8R3Zac4ZCbqB5J8ijSlUlxQqYQj9CBucl5bd8ZA/Lknl6lHAI6gz7g/drEtPpBLgmmMmpZj7il4l0n0NKP3QkMiWKS59AbwOQ/pS6kP2vPGeMCZJ4QiuqpwckZ6tGGZbaSak7hrmxc9zygzaw0XKlvomIo6DiJrZm3sJjK8Veec70syjDEmZowQyfASfWheHCLPcjXiHwoRELBzHtAIPJijsAZNbzXXwLLVa45/nzB6uaSERc+BhtbnYDen0L2K96AUaDjeq4RPt+TfGGX9ECgYEA3kwlPaSNr81vOCxMia1UWSXMFgxKn/M6qQl3kAPUhnJw1pQMl4wjXBIiXExYcLLOStAHH2MMYhXRRnCHAHWtBZOw7fHF1clfz71/ZwEEWpDDkJP/6rCBIz9HSiwK65PGpXxCwQUSv0vx67JPCTKJKnJv9NO2zsYX/SZtUVqzqsUCgYEAw9xToKtLNUGB6DBT+wiHXRHxTpDNvZZeSq8YHOSqnte+tr8QTKHR5b9g+EQ+H42feN24vk3mQOJgDjr0B8bkDIvzUS/ITpvFhqT8Lvd/N35MEmI8tpDqzwVRRJLinY0IE3P+6mLxZdPi6nVwyOwni3Fs3t8nnbBhP3kCtTn9OM0CgYAXUc+aTtfaOGfRcbw5uSgfDfsB2Ro++0oND4QuZGh4DitOJ0vG3hmbf/+AxTWF5QwFBxuoLEDRTU6d4BIZdz7GVf76hyFk0vItHcjuqkmloogRc6UmHov9gZbd7g7bNa5iD84E555W2QWw3wyxf06qWETsXwVPXbKIiO5w2V39rQKBgE3kwh2UAkUb2F5PWhbsPWl2SVHFpQ1RUA7DtANJrdGeW+qx8fmIIcINarNR8KL74ABhraeBs6D6ZtlB+Ote3jNfNVWEe1VH/jaeRu0m4SFgyUBGcX7qA3hx1PBAun+OJzgVWbw9vWOR8Rztt/0wbkm2wmWgulCsZQCpT7pjZaHBAoGAYMn/XpRTEhr9yN0zO8IUEHaDbJhVcgdoUAR9Rv2Sfrjwdn2I0gY7uJZ6niaz5M1o7blhEwhT7FMjFcK3r9y9IGKTBApSpJZNXwF4ReMhz0kxP57n4atquHhMmoZjO5nyzmtGrWIEZqIJALDvIeZkVCvQSiyefosO7CaUN4PlAC8=",
        );
        let kp_256 = JwkKeyPair {
            kid: kid_rs256.to_string(),
            typ: JwkKeyPairAlg::RS256,
            bytes: base64_decode(b64_rs256).unwrap(),
        };

        // let kid = get_rand(8);
        // let rsakp = RS384KeyPair::generate(3072).unwrap().with_key_id(&kid);
        // let der = base64_encode(rsakp.to_der().as_ref().unwrap());
        // eprintln!("rsa384\nkid:\n{}\nder:\n{}", kid, der);
        let (kid_rs384, b64_rs384) = (
            "ywUbbLPO",
            "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",
        );
        let kp_384 = JwkKeyPair {
            kid: kid_rs384.to_string(),
            typ: JwkKeyPairAlg::RS384,
            bytes: base64_decode(b64_rs384).unwrap(),
        };

        // let kid = get_rand(8);
        // let rsakp = RS512KeyPair::generate(4096).unwrap().with_key_id(&kid);
        // let der = base64_encode(rsakp.to_der().as_ref().unwrap());
        // eprintln!("rsa512\nkid:\n{}\nder:\n{}", kid, der);
        let (kid_rs512, b64_rs512) = (
            "1nhv22ov",
            "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",
        );
        let kp_512 = JwkKeyPair {
            kid: kid_rs512.to_string(),
            typ: JwkKeyPairAlg::RS512,
            bytes: base64_decode(b64_rs512).unwrap(),
        };

        let iss = "http://localhost:8080/auth/v1";
        let aud = "test_client";
        let sub = "admin";
        let sid = "sid_1337";

        // ed25519 with sub and no sid
        let token = LogoutToken::new(iss, aud, Some(sub), None)
            .into_token_with_kp(&kp_25519)
            .unwrap();
        let mut buf = Vec::with_capacity(256);

        let (header, lt) = LogoutToken::build_from_str(&token, &mut buf).unwrap();
        lt.validate_claims(header).unwrap();
        assert_eq!(lt.typ.unwrap().as_str(), "logout+jwt");
        assert_eq!(lt.aud, aud);
        assert_eq!(lt.sub.unwrap(), sub);
        assert!(lt.sid.is_none());

        // rs256 with sid and no sub
        let token = LogoutToken::new(iss, aud, None, Some(sid))
            .into_token_with_kp(&kp_256)
            .unwrap();

        buf.clear();
        let (header, lt) = LogoutToken::build_from_str(&token, &mut buf).unwrap();
        lt.validate_claims(header).unwrap();
        assert_eq!(lt.sid.unwrap(), sid);
        assert!(lt.sub.is_none());

        // make sure building + validation is fine with rs384 and rs512 as well
        let token = LogoutToken::new(iss, aud, None, Some(sid))
            .into_token_with_kp(&kp_384)
            .unwrap();
        buf.clear();
        let (header, lt) = LogoutToken::build_from_str(&token, &mut buf).unwrap();
        lt.validate_claims(header).unwrap();

        let token = LogoutToken::new(iss, aud, None, Some(sid))
            .into_token_with_kp(&kp_512)
            .unwrap();
        buf.clear();
        let (header, lt) = LogoutToken::build_from_str(&token, &mut buf).unwrap();
        lt.validate_claims(header).unwrap();

        // no sub + sid - expect failure
        let token = LogoutToken::new(iss, aud, None, None)
            .into_token_with_kp(&kp_25519)
            .unwrap();
        buf.clear();
        let (header, lt) = LogoutToken::build_from_str(&token, &mut buf).unwrap();
        lt.validate_claims(header).expect_err("empty sub and sid");
    }
}
