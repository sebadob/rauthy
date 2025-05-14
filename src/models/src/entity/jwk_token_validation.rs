use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg};
use rauthy_common::utils::base64_url_no_pad_decode_buf;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use ring::digest;
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
                let hash = digest::digest(&digest::SHA256, message.as_bytes());
                let pubkey = ring::signature::RsaPublicKeyComponents {
                    n: self.n()?,
                    e: self.e()?,
                };
                if pubkey
                    .verify(
                        &ring::signature::RSA_PKCS1_2048_8192_SHA256,
                        hash.as_ref(),
                        buf,
                    )
                    .is_ok()
                {
                    return Ok(());
                }
            }

            JwkKeyPairAlg::RS384 => {
                let hash = digest::digest(&digest::SHA384, message.as_bytes());
                let pubkey = ring::signature::RsaPublicKeyComponents {
                    n: self.n()?,
                    e: self.e()?,
                };
                if pubkey
                    .verify(
                        &ring::signature::RSA_PKCS1_3072_8192_SHA384,
                        hash.as_ref(),
                        buf,
                    )
                    .is_ok()
                {
                    return Ok(());
                }
            }

            JwkKeyPairAlg::RS512 => {
                let hash = digest::digest(&digest::SHA512, message.as_bytes());
                let pubkey = ring::signature::RsaPublicKeyComponents {
                    n: self.n()?,
                    e: self.e()?,
                };
                if pubkey
                    .verify(
                        &ring::signature::RSA_PKCS1_2048_8192_SHA512,
                        hash.as_ref(),
                        buf,
                    )
                    .is_ok()
                {
                    return Ok(());
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
