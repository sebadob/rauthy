use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg};
use rauthy_common::utils::base64_url_no_pad_decode_buf;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rsa::sha2::{Sha256, Sha384, Sha512};
use rsa::{Pkcs1v15Sign, RsaPublicKey};
use tracing::warn;

impl JWKSPublicKey {
    pub fn validate_token_signature(
        &self,
        token: &str,
        buf: &mut Vec<u8>,
    ) -> Result<(), ErrorResponse> {
        let (message, sig_str) = token
            .rsplit_once('.')
            .ok_or_else(|| ErrorResponse::new(ErrorResponseType::BadRequest, "Malformed token"))?;

        buf.clear();
        base64_url_no_pad_decode_buf(sig_str, buf)?;

        match self.alg()? {
            JwkKeyPairAlg::RS256 => {
                if let Ok(rsa_pk) = RsaPublicKey::new(self.n()?, self.e()?) {
                    let hash = hmac_sha256::Hash::hash(message.as_bytes());
                    if rsa_pk
                        .verify(Pkcs1v15Sign::new::<Sha256>(), hash.as_slice(), buf)
                        .is_ok()
                    {
                        return Ok(());
                    }
                }
            }

            JwkKeyPairAlg::RS384 => {
                if let Ok(rsa_pk) = RsaPublicKey::new(self.n()?, self.e()?) {
                    let hash = hmac_sha512::sha384::Hash::hash(message.as_bytes());
                    if rsa_pk
                        .verify(Pkcs1v15Sign::new::<Sha384>(), hash.as_slice(), buf)
                        .is_ok()
                    {
                        return Ok(());
                    }
                }
            }

            JwkKeyPairAlg::RS512 => {
                if let Ok(rsa_pk) = RsaPublicKey::new(self.n()?, self.e()?) {
                    let hash = hmac_sha512::Hash::hash(message.as_bytes());
                    if rsa_pk
                        .verify(Pkcs1v15Sign::new::<Sha512>(), hash.as_slice(), buf)
                        .is_ok()
                    {
                        return Ok(());
                    }
                }
            }

            JwkKeyPairAlg::EdDSA => {
                let x = self.x()?;
                if let Ok(pubkey) = ed25519_compact::PublicKey::from_slice(x.as_slice()) {
                    let signature = ed25519_compact::Signature::from_slice(buf)?;
                    if pubkey.verify(message, &signature).is_ok() {
                        return Ok(());
                    }
                }
            }
        };

        warn!("JWT Token validation error");
        Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Invalid JWT Token",
        ))
    }
}
