use crate::{b64_decode, b64_encode, generate_pkce_challenge, secure_random};
use axum_extra::extract::CookieJar;
use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{error, warn};

pub(crate) static OIDC_STATE_COOKIE: &str = "OIDC_STATE";

#[derive(Serialize, Deserialize)]
pub struct OidcCookieState {
    pub nonce: String,
    pub pkce_verifier: String,
    pub state: String,
    pub timestamp: DateTime<Utc>,
}

impl OidcCookieState {
    pub fn generate() -> (Self, String) {
        let (pkce_verifier, challenge) = generate_pkce_challenge();
        let slf = Self {
            nonce: secure_random(32),
            pkce_verifier,
            state: secure_random(24),
            timestamp: Utc::now(),
        };
        (slf, challenge)
    }

    pub fn from_cookie_value(jar: &CookieJar, key: &[u8]) -> anyhow::Result<Self> {
        match jar.get(OIDC_STATE_COOKIE) {
            None => {
                warn!("STATE_COOKIE is missing - Request may have expired");
                Err(anyhow::Error::msg("Request has expired"))
            }
            Some(cookie) => {
                let enc = b64_decode(cookie.value())?;
                let dec = Self::decrypt(&enc, key)?;
                let slf = bincode::deserialize::<Self>(&dec)?;
                Ok(slf)
            }
        }
    }

    pub fn to_cookie_value(&self, key: &[u8]) -> String {
        let ser = bincode::serialize(self).unwrap();
        let enc = Self::encrypt(&ser, key).unwrap();
        b64_encode(&enc)
    }

    pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            error!("Invalid ciphertext for decryption: {:?}", ciphertext);
            return Err(anyhow::Error::msg("Invalid ciphertext for decryption"));
        }
        let k = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(k);
        // 96 bits nonce is always the first bytes, if the `encrypt()` was used before
        let (n, text) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(n);
        let plaintext = cipher.decrypt(nonce, text)?;

        Ok(plaintext)
    }

    pub fn encrypt(plain: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>> {
        let k = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(k);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plain)?;

        let mut res = nonce.to_vec();
        res.extend(ciphertext);

        Ok(res)
    }
}
