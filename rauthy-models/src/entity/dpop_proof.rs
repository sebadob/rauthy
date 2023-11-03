use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg};
use actix_web::http::Uri;
use actix_web::{http, HttpRequest};
use chrono::Utc;
use rauthy_common::constants::{DPOP_TOKEN_ENDPOINT, RE_TOKEN_68, TOKEN_DPOP};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::base64_url_no_pad_decode;
use serde::{Deserialize, Serialize};

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
    #[serde(with = "http_serde::method")]
    pub htm: http::Method,
    /// The HTTP target URI (Section 7.1 of [RFC9110]) of the request
    /// to which the JWT is attached, without query and fragment parts.
    #[serde(with = "http_serde::uri")]
    pub htu: Uri,
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

    // // TODO Optional all the time or just during creating with the nonce header?
    // /// A recent nonce provided via the DPoP-Nonce HTTP header.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub nonce: Option<String>,
}

impl TryFrom<&str> for DPoPProof {
    type Error = ErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_once('.') {
            None => Err(ErrorResponse::new(
                ErrorResponseType::DPoP,
                "Invalid DPoP header format".to_string(),
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
                        ErrorResponseType::DPoP,
                        "Invalid DPoP claims format".to_string(),
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
    pub fn opt_validated_from(req: &HttpRequest) -> Result<Option<Self>, ErrorResponse> {
        match req.headers().get(TOKEN_DPOP) {
            None => Ok(None),
            Some(v) => {
                let b64 = v.to_str()?;
                if !RE_TOKEN_68.is_match(b64) {
                    Err(ErrorResponse::new(
                        ErrorResponseType::DPoP,
                        "DPoP header must be in Token68 format".to_string(),
                    ))
                } else {
                    let slf = Self::try_from(b64)?;
                    slf.validate(b64)?;

                    // TODO impl nonce check? Does Rauthy need to do this at all here?
                    // let nonce = req.headers().get(TOKEN_DPOP_NONCE);

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
    /// signature algorithm [IANA.JOSE.ALGS], is not none, is supported by the
    /// application, and is acceptable per local policy.
    /// 6. The JWT signature verifies with the public key contained in the jwk
    /// JOSE Header Parameter.
    /// 7. The jwk JOSE Header Parameter does not contain a private key.
    /// 8. The htm claim matches the HTTP method of the current request.
    /// 9. The htu claim matches the HTTP URI value for the HTTP request in
    /// which the JWT was received, ignoring any query and fragment parts.
    /// 10. If the server provided a nonce value to the client,
    /// the nonce claim matches the server-provided nonce value.
    /// 11. The creation time of the JWT, as determined by either the iat
    /// claim or a server managed timestamp via the nonce claim, is within an
    /// acceptable window (see Section 11.1).
    /// 12. If presented to a protected resource in conjunction with an access token:
    /// - ensure that the value of the ath claim equals the hash of that access token, and
    /// - confirm that the public key to which the access token is bound matches the
    ///   public key from the DPoP proof.
    pub fn validate(&self, raw_token: &str) -> Result<(), ErrorResponse> {
        // 1. we do not need to validate that there is only one head field with DPoP since
        // actix serializes into a HashMap which implies this anyway

        // 2. handled during extraction

        // 3. handled during deserialization

        // 4. The typ JOSE Header Parameter has the value dpop+jwt.The alg JOSE Header
        if &self.header.typ != "dpop+jwt" {
            return Err(ErrorResponse::new(
                ErrorResponseType::DPoP,
                "Expected header typ of 'dpop+jwt'".to_string(),
            ));
        }

        // 5. The alg JOSE Header Parameter indicates a registered asymmetric digital
        // signature algorithm [IANA.JOSE.ALGS], is not none, is supported by the
        // application, and is acceptable per local policy.
        let kp = &self.header.jwk;
        kp.validate_self()?;

        // 6. The JWT signature verifies with the public key contained in the jwk
        // JOSE Header Parameter.
        // let nonce = self.claims.nonce.clone();
        kp.validate_dpop(raw_token)?;

        // 7. The jwk JOSE Header Parameter does not contain a private key.
        // TODO ?

        // 8. The htm claim matches the HTTP method of the current request.
        if self.claims.htm != http::Method::POST {
            return Err(ErrorResponse::new(
                ErrorResponseType::DPoP,
                "The 'htm' claim from the DPoP header != POST".to_string(),
            ));
        }

        // 9. The htu claim matches the HTTP URI value for the HTTP request in
        // which the JWT was received, ignoring any query and fragment parts.
        if self.claims.htu != *DPOP_TOKEN_ENDPOINT {
            return Err(ErrorResponse::new(
                ErrorResponseType::DPoP,
                "Invalid 'htu' claim".to_string(),
            ));
        }

        // 10. If the server provided a nonce value to the client,
        // the nonce claim matches the server-provided nonce value.
        // TODO does this apply to Rauthy as the OP only? -> check!

        // 11. The creation time of the JWT, as determined by either the iat
        // claim or a server managed timestamp via the nonce claim, is within an
        // acceptable window (see Section 11.1).
        //
        // We will accept an 'iat' of 1 minute old and it must not be in the future
        let now = Utc::now().timestamp();
        let now_minus_1 = now - 60;
        if self.claims.iat < now_minus_1 || self.claims.iat > now {
            return Err(ErrorResponse::new(
                ErrorResponseType::DPoP,
                "DPoP 'iat' claim is out of range".to_string(),
            ));
        }

        // 12. If presented to a protected resource in conjunction with an access token:
        // - ensure that the value of the ath claim equals the hash of that access token, and
        // - confirm that the public key to which the access token is bound matches the
        //   public key from the DPoP proof.
        // TODO does this even apply to Rauthy? Maybe when refreshing? -> double check

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::dpop_proof::{DPoPClaims, DPoPHeader, DPoPProof};
    use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg, JwkKeyPairType};
    use actix_web::http;
    use chrono::Utc;
    use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_encode};
    // use rsa::pkcs1v15::{SigningKey};
    // use rsa::signature::{Keypair, RandomizedSigner};
    // use rsa::sha2::{Digest, Sha256};
    // use rsa::signature::SignatureEncoding;
    // use rsa::traits::{PublicKeyParts};
    use ed25519_compact::Noise;
    use rauthy_common::constants::DPOP_TOKEN_ENDPOINT;
    use std::fmt::Write;

    // Note: this test does not work anymore with the way more strict deserialization after
    // the initial testing with just Strings.
    // It was using the original Header value from the RFC, but Rauthy does not support the EC alg.
    // #[test]
    // fn test_dpop_extraction() {
    //     // test value taken out of the RFC9449
    //     let header = "eyJ0eXAiOiJkcG9wK2p3dCIsImFsZyI6IkVTMjU2IiwiandrIjp7Imt0eSI6IkVDIiwie\
    //     CI6Imw4dEZyaHgtMzR0VjNoUklDUkRZOXpDa0RscEJoRjQyVVFVZldWQVdCRnMiLCJ5IjoiOVZFNGpmX09rX282N\
    //     HpiVFRsY3VOSmFqSG10NnY5VERWclUwQ2R2R1JEQSIsImNydiI6IlAtMjU2In19.eyJqdGkiOiItQndDM0VTYzZh\
    //     Y2MybFRjIiwiaHRtIjoiUE9TVCIsImh0dSI6Imh0dHBzOi8vc2VydmVyLmV4YW1wbGUuY29tL3Rva2VuIiwiaWF0\
    //     IjoxNTYyMjYyNjE2fQ.2-GxA6T8lP4vfrg8v-FdWP0A0zdrj8igiMLvqRMUvwnQg4PtFLbdLXiOSsX0x7NVY-FNy\
    //     JK70nfbV37xRZT3Lg";
    //     println!("test header:\n{}", header);
    //
    //     let dpop = DPoPProof::try_from(header).unwrap();
    //
    //     assert_eq!(dpop.header.typ, "dpop+jwt");
    //     // assert_eq!(dpop.header.alg.as_str(), "ES256");
    //     assert_eq!(dpop.header.jwk.kty, "EC");
    //     assert_eq!(
    //         dpop.header.jwk.x.as_deref(),
    //         Some("l8tFrhx-34tV3hRICRDY9zCkDlpBhF42UQUfWVAWBFs")
    //     );
    //     // We don't validate 'y' from the example here because Rauthy does not (and will not)
    //     // support EC keys. Just took the original value to be more on the safe side with the
    //     // extraction.
    //     assert_eq!(dpop.header.jwk.crv.as_deref(), Some("P-256"));
    //
    //     assert_eq!(dpop.claims.jti, "-BwC3ESc6acc2lTc");
    //     assert_eq!(dpop.claims.htm, http::Method::POST);
    //     assert_eq!(
    //         dpop.claims.htu,
    //         Uri::from_str("https://server.example.com/token").unwrap()
    //     );
    //     assert_eq!(dpop.claims.iat, 1562262616);
    // }

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
            htm: http::Method::POST,
            htu: DPOP_TOKEN_ENDPOINT.clone(),
            iat: Utc::now().timestamp(),
            // nonce: None,
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
        let dpop = DPoPProof::try_from(token_raw.as_str()).unwrap();
        dpop.validate(&token_raw).unwrap();
    }

    // Currently, the SigningKey from the `rsa` crate makes my IDE fully crash.
    // There is a bug in the Rust plugin which makes coding basically impossible (unless I would
    // switch to another IDE).
    // Until this is solved, I will only have the RSA keys prepared, but actually only support
    // EdDSA for now.
    //
    // #[test]
    // fn test_dpop_validation_rsa() {
    //     // manually build up a dpop token
    //     let mut rng = rand::thread_rng();
    //     let rsa_sk = rsa::RsaPrivateKey::new(&mut rng, 2048).unwrap();
    //     let rsa_pk = rsa_sk.to_public_key();
    //     let n = rsa_pk.n().to_bytes_be();
    //     let e = rsa_pk.e().to_bytes_be();
    //
    //     let header = DPoPHeader {
    //         typ: "dpop+jwt".to_string(),
    //         alg: JwkKeyPairAlg::RS256,
    //         jwk: JWKSPublicKey {
    //             kty: JwkKeyPairType::RSA,
    //             // DPoP request will not have the 'alg' here but one level higher
    //             alg: None,
    //             crv: None,
    //             kid: None,
    //             n: Some(base64_url_encode(&n)),
    //             e: Some(base64_url_encode(&e)),
    //             x: None,
    //         },
    //         kid: None,
    //     };
    //
    //     let claims = DPoPClaims {
    //         jti: "-BwC3ESc6acc2lTc".to_string(),
    //         htm: http::Method::POST,
    //         htu: Uri::from_str("https://localhost:8080/auth/v1/oidc/token").unwrap(),
    //         iat: Utc::now().timestamp(),
    //         ath: None,
    //         nonce: None,
    //     };
    //
    //     // build and sign the raw token string
    //     let header_json = serde_json::to_string(&header).unwrap();
    //     let header_b64 = base64_url_no_pad_encode(header_json.as_bytes());
    //     let claims_json = serde_json::to_string(&claims).unwrap();
    //     let claims_b64 = base64_url_no_pad_encode(claims_json.as_bytes());
    //     let mut token_raw = format!("{}.{}", header_b64, claims_b64);
    //
    //     let hash = hmac_sha256::Hash::hash(token_raw.as_bytes());
    //
    //     // let signing_key: rsa::pkcs1v15::SigningKey<rsa::sha2::Sha256> = rsa::pkcs1v15::SigningKey::new(rsa_sk);
    //     let signing_key = SigningKey::new(rsa_sk);
    //     let verifying_key = signing_key.verifying_key();
    //     let signature = signing_key.sign_with_rng(&mut rng, &hash).to_bytes();
    //
    //     let sig_b64 = base64_url_no_pad_encode(&signature);
    //     write!(token_raw, ".{}", sig_b64).unwrap();
    //     println!("test signed token:\n{}", token_raw);
    //
    //     // // manual validation first
    //     // let decoded = base64_url_no_pad_decode(&header.jwk.n.clone().unwrap()).unwrap();
    //     // let n = BigUint::from_bytes_be(&decoded);
    //     // let decoded = base64_url_no_pad_decode(&header.jwk.e.clone().unwrap()).unwrap();
    //     // let e = BigUint::from_bytes_be(&decoded);
    //     // let pk = rsa::RsaPublicKey::new(n, e).unwrap();
    //     // let sig_bytes = base64_url_no_pad_decode(&sig_b64).unwrap();
    //     // let sig = rsa::pkcs1v15::Signature::try_from(sig_bytes.as_slice()).unwrap();
    //     // let hash = hmac_sha256::Hash::hash(token_raw.as_bytes());
    //     // let verifier = rsa::pkcs1v15::VerifyingKey::<sha2::Sha256>::from(pk);
    //     // verifier.verify(&hash, &sig).unwrap();
    //     //
    //     // // now we have our token like it should come in with the DPoP header -> try to verify it
    //     // let dpop = DPoPProof::try_from(token_raw.as_str()).unwrap();
    //     // dpop.validate(&token_raw).unwrap();
    // }
}
