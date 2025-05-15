use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg};
use rauthy_common::utils::base64_url_no_pad_decode_buf;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rsa::sha2;
use tracing::warn;

impl JWKSPublicKey {
    pub fn validate_token_signature(
        &self,
        token: &str,
        buf: &mut Vec<u8>,
    ) -> Result<(), ErrorResponse> {
        let (message, sig) = token
            .rsplit_once('.')
            .ok_or_else(|| ErrorResponse::new(ErrorResponseType::BadRequest, "Malformed token"))?;

        buf.clear();
        base64_url_no_pad_decode_buf(sig, buf)?;

        match self.alg()? {
            JwkKeyPairAlg::RS256 => {
                let hash = hmac_sha256::Hash::hash(message.as_bytes());
                let rsa_pk = rsa::RsaPublicKey::new(self.n()?, self.e()?)?;
                if rsa_pk
                    .verify(
                        rsa::Pkcs1v15Sign::new::<sha2::Sha256>(),
                        hash.as_slice(),
                        buf,
                    )
                    .is_ok()
                {
                    return Ok(());
                }
            }

            JwkKeyPairAlg::RS384 => {
                let hash = hmac_sha512::sha384::Hash::hash(message.as_bytes());
                let rsa_pk = rsa::RsaPublicKey::new(self.n()?, self.e()?)?;
                if rsa_pk
                    .verify(
                        rsa::Pkcs1v15Sign::new::<sha2::Sha384>(),
                        hash.as_slice(),
                        buf,
                    )
                    .is_ok()
                {
                    return Ok(());
                }
            }

            JwkKeyPairAlg::RS512 => {
                let hash = hmac_sha512::Hash::hash(message.as_bytes());
                let rsa_pk = rsa::RsaPublicKey::new(self.n()?, self.e()?)?;
                if rsa_pk
                    .verify(
                        rsa::Pkcs1v15Sign::new::<sha2::Sha512>(),
                        hash.as_slice(),
                        buf,
                    )
                    .is_ok()
                {
                    return Ok(());
                }
            }

            JwkKeyPairAlg::EdDSA => {
                let x = self.x()?;
                let pubkey = ed25519_compact::PublicKey::from_slice(x.as_slice())?;
                let signature = ed25519_compact::Signature::from_slice(buf)?;
                if pubkey.verify(message, &signature).is_ok() {
                    return Ok(());
                }
            }
        };

        warn!("JWT Token validation error");
        Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Invalid JWT Token signature",
        ))
    }
}
