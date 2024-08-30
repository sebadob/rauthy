use crate::cache::{Cache, DB};
use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{http, HttpRequest};
use chrono::{DateTime, Utc};
use rauthy_common::constants::{
    CACHE_TTL_DPOP_NONCE, DPOP_FORCE_NONCE, DPOP_NONCE_EXP, DPOP_TOKEN_ENDPOINT, RE_TOKEN_68,
    TOKEN_DPOP,
};
use rauthy_common::utils::{base64_url_no_pad_decode, get_rand};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};
use tracing::error;

/// A DPoP nonce that only live inside the cache to limit client's DPoP lifetimes
#[derive(Debug, Serialize, Deserialize)]
pub struct DPoPNonce {
    pub exp: DateTime<Utc>,
    pub value: String,
}

impl DPoPNonce {
    /// Creates a new DPoP nonce, inserts it into the cache and returns its value.
    pub async fn new_value() -> Result<String, ErrorResponse> {
        let exp = Utc::now().add(chrono::Duration::seconds(*DPOP_NONCE_EXP as i64));
        let slf = Self {
            exp,
            value: get_rand(32),
        };

        let client = DB::client();
        client
            .put(Cache::DPoPNonce, "latest", &slf, *CACHE_TTL_DPOP_NONCE)
            .await?;

        // we need by its own value additionally, because the "latest" may be overwritten
        // before its expiration
        client
            .put(
                Cache::DPoPNonce,
                slf.value.clone(),
                &slf,
                *CACHE_TTL_DPOP_NONCE,
            )
            .await?;

        Ok(slf.value)
    }

    /// Checks the validity of the given DPoP nonce value
    pub async fn is_valid(value: String) -> bool {
        let slf: Result<Option<Self>, hiqlite::Error> =
            DB::client().get(Cache::DPoPNonce, value).await;
        slf.is_ok()
    }

    /// Always returns the value of the latest valid DPoP nonce which is valid for at least
    /// 15 more seconds or longer.
    pub async fn get_latest() -> Result<String, ErrorResponse> {
        let slf: Option<Self> = DB::client().get(Cache::DPoPNonce, "latest").await?;
        match slf {
            None => Self::new_value().await,
            Some(slf) => {
                let now_minus_15 = Utc::now().sub(chrono::Duration::seconds(15));
                if slf.exp < now_minus_15 {
                    Self::new_value().await
                } else {
                    Ok(slf.value)
                }
            }
        }
    }
}

/// https://datatracker.ietf.org/doc/html/rfc9449
#[derive(Debug, Serialize, Deserialize)]
pub struct DPoPProof {
    pub header: DPoPHeader,
    pub claims: DPoPClaims,
    pub signature: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DPoPHeader {
    /// A field with the value dpop+jwt, which explicitly types the
    /// DPoP proof JWT as recommended in Section 3.11 of [RFC8725]
    ///
    /// In case of DPoP, must always be: 'dpop+jwt'
    pub typ: String,
    /// An identifier for a JWS asymmetric digital signature algorithm
    /// from [IANA.JOSE.ALGS]. It MUST NOT be none or an identifier for a
    /// symmetric algorithm (Message Authentication Code (MAC)).
    pub alg: JwkKeyPairAlg,
    /// Represents the public key chosen by the client in JSON Web Key
    /// (JWK) [RFC7517] format as defined in Section 4.1.3 of [RFC7515].
    /// It MUST NOT contain a private key.
    pub jwk: JWKSPublicKey,
    pub kid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DPoPClaims {
    /// Unique identifier for the DPoP proof JWT. The value MUST be
    /// assigned such that there is a negligible probability that the
    /// same value will be assigned to any other DPoP proof used in the
    /// same context during the time window of validity. Such uniqueness
    /// can be accomplished by encoding (base64url or any other suitable
    /// encoding) at least 96 bits of pseudorandom data or by using a
    /// version 4 Universally Unique Identifier (UUID) string according
    /// to [RFC4122]. The jti can be used by the server for replay
    /// detection and prevention; see Section 11.1.
    pub jti: String,
    /// The value of the HTTP method (Section 9.1 of [RFC9110]) of the
    /// request to which the JWT is attached.
    pub htm: String,
    /// The HTTP target URI (Section 7.1 of [RFC9110]) of the request
    /// to which the JWT is attached, without query and fragment parts.
    pub htu: String,
    /// Creation timestamp of the JWT (Section 4.1.6 of [RFC7519]).
    pub iat: i64,
    // The 'ath' claim does not apply to Rauthy, only used by resource servers.
    // It is just here for completeness.
    //
    // Hash of the access token. The value MUST be the result of a
    // base64url encoding (as defined in Section 2 of [RFC7515])
    // the SHA-256 [SHS] hash of the ASCII encoding of the associated
    // access token's value.
    //
    // MUST be valid when used in conjunction with an access token
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub ath: Option<String>,
    /// A recent nonce provided via the DPoP-Nonce HTTP header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

impl DPoPProof {
    fn try_from_str(origin: Option<&str>, value: &str) -> Result<Self, ErrorResponse> {
        match value.split_once('.') {
            None => Err(ErrorResponse::new(
                ErrorResponseType::DPoP(origin.map(String::from)),
                "Invalid DPoP header format",
            )),

            Some((header, rest)) => {
                let bytes = base64_url_no_pad_decode(header)?;
                let s = String::from_utf8(bytes)?;
                let mut header = serde_json::from_str::<DPoPHeader>(&s)?;

                // Our validation will fail if there is no 'alg' in the provided JWK.
                // Their "should" be always one, but in the RFC example, there was none.
                // In that case we assume the same 'alg' as in the main header itself
                // and try validating with that one.
                if header.jwk.alg.is_none() {
                    header.jwk.alg = Some(header.alg.clone());
                }

                match rest.split_once('.') {
                    None => Err(ErrorResponse::new(
                        ErrorResponseType::DPoP(origin.map(String::from)),
                        "Invalid DPoP claims format",
                    )),
                    Some((claims, signature)) => {
                        let bytes = base64_url_no_pad_decode(claims)?;
                        let s = String::from_utf8(bytes)?;
                        let claims = serde_json::from_str::<DPoPClaims>(&s)?;

                        let signature = base64_url_no_pad_decode(signature)?;

                        Ok(Self {
                            header,
                            claims,
                            signature,
                        })
                    }
                }
            }
        }
    }
}

impl DPoPProof {
    #[inline(always)]
    pub fn jwk_fingerprint(&self) -> Result<String, ErrorResponse> {
        self.header.jwk.fingerprint()
    }

    /// Tries to extract a DPoP header from the given HttpRequest and validates the given JWK
    /// if it exists.
    pub async fn opt_validated_from(
        req: &HttpRequest,
        header_origin: &Option<(HeaderName, HeaderValue)>,
    ) -> Result<Option<Self>, ErrorResponse> {
        // the String conversion is mandatory for to make it possible to build an automatic
        // ErrorResponse from the value -> could maybe be optimized in the future
        let origin = header_origin
            .as_ref()
            .map(|(_, name)| name.to_str().unwrap_or_default().to_string());

        match req.headers().get(TOKEN_DPOP) {
            None => Ok(None),
            Some(v) => {
                let b64 = v.to_str()?;
                if !RE_TOKEN_68.is_match(b64) {
                    Err(ErrorResponse::new(
                        ErrorResponseType::DPoP(origin),
                        "DPoP header must be in Token68 format",
                    ))
                } else {
                    let slf = Self::try_from_str(origin.as_deref(), b64)?;

                    if let Err(msg) = slf.validate(b64) {
                        return Err(ErrorResponse::new(ErrorResponseType::DPoP(origin), msg));
                    }
                    if let Err(nonce) = slf.validate_nonce().await {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::UseDpopNonce((origin, nonce)),
                            "DPoP 'nonce' is required in DPoP proof",
                        ));
                    }

                    Ok(Some(slf))
                }
            }
        }
    }

    /// Checking DPoP Proofs
    ///
    /// To validate a DPoP proof, the receiving server MUST ensure the following:
    ///
    /// 1. There is not more than one DPoP HTTP request header field.
    /// 2. The DPoP HTTP request header field value is a single and well-formed JWT.
    /// 3. All required claims per Section 4.2 are contained in the JWT.
    /// 4. The typ JOSE Header Parameter has the value dpop+jwt.
    /// 5. The alg JOSE Header Parameter indicates a registered asymmetric digital
    ///    signature algorithm [IANA.JOSE.ALGS], is not none, is supported by the
    ///    application, and is acceptable per local policy.
    /// 6. The JWT signature verifies with the public key contained in the jwk
    ///    JOSE Header Parameter.
    /// 7. The jwk JOSE Header Parameter does not contain a private key.
    /// 8. The htm claim matches the HTTP method of the current request.
    /// 9. The htu claim matches the HTTP URI value for the HTTP request in
    ///    which the JWT was received, ignoring any query and fragment parts.
    /// 10. If the server provided a nonce value to the client,
    ///     the nonce claim matches the server-provided nonce value.
    /// 11. The creation time of the JWT, as determined by either the iat
    ///     claim or a server managed timestamp via the nonce claim, is within an
    ///     acceptable window (see Section 11.1).
    /// 12. If presented to a protected resource in conjunction with an access token:
    /// - ensure that the value of the ath claim equals the hash of that access token, and
    /// - confirm that the public key to which the access token is bound matches the
    ///   public key from the DPoP proof.
    pub fn validate(&self, raw_token: &str) -> Result<(), String> {
        // 1. we do not need to validate that there is only one head field with DPoP since
        // actix serializes into a HashMap which implies this anyway

        // 2. handled during extraction

        // 3. handled during deserialization

        // 4. The typ JOSE Header Parameter has the value dpop+jwt.The alg JOSE Header
        if &self.header.typ != "dpop+jwt" {
            return Err("Expected header typ of 'dpop+jwt'".to_string());
        }

        // 5. The alg JOSE Header Parameter indicates a registered asymmetric digital
        // signature algorithm [IANA.JOSE.ALGS], is not none, is supported by the
        // application, and is acceptable per local policy.
        let kp = &self.header.jwk;
        kp.validate_self().map_err(|err| err.message)?;

        // 6. The JWT signature verifies with the public key contained in the jwk
        // JOSE Header Parameter.
        // let nonce = self.claims.nonce.clone();
        kp.validate_token_signature(raw_token)
            .map_err(|err| err.message)?;

        // 7. The jwk JOSE Header Parameter does not contain a private key.
        // TODO ?

        // 8. The htm claim matches the HTTP method of the current request.
        if self.claims.htm.as_str() != http::Method::POST.as_str() {
            return Err("The 'htm' claim from the DPoP header != POST".to_string());
        }

        // 9. The htu claim matches the HTTP URI value for the HTTP request in
        // which the JWT was received, ignoring any query and fragment parts.
        if self.claims.htu != DPOP_TOKEN_ENDPOINT.to_string() {
            return Err("Invalid 'htu' claim".to_string());
        }

        // 10. If the server provided a nonce value to the client,
        // the nonce claim matches the server-provided nonce value.
        //
        // The nonce is being validated in its extra function for better unit testing
        // -> Self::validate_nonce()

        // 11. The creation time of the JWT, as determined by either the iat
        // claim or a server managed timestamp via the nonce claim, is within an
        // acceptable window (see Section 11.1).
        //
        // We will accept an 'iat' of 1 minute old and it must not be in the future
        let now = Utc::now().timestamp();
        let now_minus_1 = now - 60;
        if self.claims.iat < now_minus_1 || self.claims.iat > now {
            return Err("DPoP 'iat' claim is out of range".to_string());
        }

        // 12. If presented to a protected resource in conjunction with an access token:
        // - ensure that the value of the ath claim equals the hash of that access token, and
        // - confirm that the public key to which the access token is bound matches the
        //   public key from the DPoP proof.
        // TODO does this even apply to Rauthy? Maybe when refreshing? -> double check

        Ok(())
    }

    pub async fn validate_nonce(&self) -> Result<(), String> {
        if let Some(nonce) = &self.claims.nonce {
            if !DPoPNonce::is_valid(nonce.clone()).await {
                let latest = DPoPNonce::get_latest().await.unwrap_or_else(|err| {
                    error!("Cache lookup error during DPoP nonce generation: {:?}", err);
                    err.message.to_string()
                });
                return Err(latest);
            }
        } else if *DPOP_FORCE_NONCE {
            let latest = DPoPNonce::get_latest().await.unwrap_or_else(|err| {
                error!("Cache lookup error during DPoP nonce generation: {:?}", err);
                err.message.to_string()
            });
            return Err(latest);
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::dpop_proof::{DPoPClaims, DPoPHeader, DPoPProof};
    use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg, JwkKeyPairType};
    use actix_web::http;
    use chrono::Utc;
    use ed25519_compact::Noise;
    use rauthy_common::constants::DPOP_TOKEN_ENDPOINT;
    use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_encode};
    use rsa::sha2::Sha256;
    use rsa::traits::PublicKeyParts;
    use std::fmt::Write;

    #[test]
    fn test_dpop_validation_eddsa() {
        // mandatory to read the PUB_URL for the request validation
        dotenvy::from_filename("rauthy.test.cfg").ok();

        // manually build up a dpop token
        let kp = ed25519_compact::KeyPair::generate();

        let header = DPoPHeader {
            typ: "dpop+jwt".to_string(),
            alg: JwkKeyPairAlg::EdDSA,
            jwk: JWKSPublicKey {
                kty: JwkKeyPairType::OKP,
                // DPoP request will not have the 'alg' here but one level higher
                alg: None,
                crv: Some("Ed25519".to_string()),
                kid: None,
                n: None,
                e: None,
                x: Some(base64_url_encode(kp.pk.as_slice())),
            },
            kid: None,
        };

        let claims = DPoPClaims {
            jti: "-BwC3ESc6acc2lTc".to_string(),
            htm: http::Method::POST.to_string(),
            htu: DPOP_TOKEN_ENDPOINT.clone().to_string(),
            iat: Utc::now().timestamp(),
            nonce: None,
        };

        // build and sign the raw token string
        let header_json = serde_json::to_string(&header).unwrap();
        let header_b64 = base64_url_no_pad_encode(header_json.as_bytes());
        let claims_json = serde_json::to_string(&claims).unwrap();
        let claims_b64 = base64_url_no_pad_encode(claims_json.as_bytes());
        let mut token_raw = format!("{}.{}", header_b64, claims_b64);

        let sig = kp.sk.sign(&token_raw, Some(Noise::generate()));
        let sig_b64 = base64_url_no_pad_encode(sig.as_ref());
        write!(token_raw, ".{}", sig_b64).unwrap();
        println!("test signed token:\n{}", token_raw);

        // now we have our token like it should come in with the DPoP header -> try to verify it
        let dpop = DPoPProof::try_from_str(None, token_raw.as_str()).unwrap();
        dpop.validate(&token_raw).unwrap();

        // Note: we cannot validate the nonce in this unit test because of missing AppState and
        // cache -> will be done in integration tests
    }

    #[test]
    fn test_dpop_validation_rsa() {
        // manually build up a dpop token
        let mut rng = rand::thread_rng();
        let rsa_sk = rsa::RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let rsa_pk = rsa_sk.to_public_key();
        let n = rsa_pk.n().to_bytes_be();
        let e = rsa_pk.e().to_bytes_be();

        let header = DPoPHeader {
            typ: "dpop+jwt".to_string(),
            alg: JwkKeyPairAlg::RS256,
            jwk: JWKSPublicKey {
                kty: JwkKeyPairType::RSA,
                // DPoP request will not have the 'alg' here but one level higher
                alg: None,
                crv: None,
                kid: None,
                n: Some(base64_url_encode(&n)),
                e: Some(base64_url_encode(&e)),
                x: None,
            },
            kid: None,
        };

        let claims = DPoPClaims {
            jti: "-BwC3ESc6acc2lTc".to_string(),
            htm: http::Method::POST.to_string(),
            htu: DPOP_TOKEN_ENDPOINT.clone().to_string(),
            iat: Utc::now().timestamp(),
            nonce: None,
        };

        // build and sign the raw token string
        let header_json = serde_json::to_string(&header).unwrap();
        let header_b64 = base64_url_no_pad_encode(header_json.as_bytes());
        let claims_json = serde_json::to_string(&claims).unwrap();
        let claims_b64 = base64_url_no_pad_encode(claims_json.as_bytes());
        let mut token_raw = format!("{}.{}", header_b64, claims_b64);

        let hash = hmac_sha256::Hash::hash(token_raw.as_bytes());
        let signature = rsa_sk
            .sign(rsa::Pkcs1v15Sign::new::<Sha256>(), hash.as_slice())
            .unwrap();
        let sig_b64 = base64_url_no_pad_encode(&signature);
        write!(token_raw, ".{}", sig_b64).unwrap();

        // now we have our token like it should come in with the DPoP header -> try to verify it
        let dpop = DPoPProof::try_from_str(None, token_raw.as_str()).unwrap();
        dpop.validate(&token_raw).unwrap();

        // This only tests the RS256 validation. The logic for 384 and 512 is the same, and the
        // token signature validation itself for the 2 others is tested already in
        // jwk::test_signature_validation
        // -> no need to test it again here
    }
}
