use crate::error_response::{ErrorResponse, ErrorResponseType};
use actix_web::HttpRequest;
use base64::{engine, engine::general_purpose, Engine as _};
use chacha20poly1305::aead::Aead;
use chacha20poly1305::KeyInit;
use gethostname::gethostname;
use rand::distributions::Alphanumeric;
use rand::Rng;
use tracing::error;

const B64_URL_SAFE: engine::GeneralPurpose = general_purpose::URL_SAFE;
const B64_URL_SAFE_NO_PAD: engine::GeneralPurpose = general_purpose::URL_SAFE_NO_PAD;
const B64_STD: engine::GeneralPurpose = general_purpose::STANDARD;

pub fn build_csp_header(nonce: &str) -> (&str, String) {
    let value = format!(
        "default-src 'self'; script-src 'strict-dynamic' 'nonce-{}'; style-src 'self' 'unsafe-inline'; \
        frame-ancestors 'none'; object-src 'none'; img-src 'self' data:;",
        nonce,
    );
    ("content-security-policy", value)
}

// Decrypts a `&Vec<u8>` which was [encrypted](encrypt) before with the same key.
pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

    let k = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(k);
    // 96 bits nonce is always the first bytes, if the `encrypt()` was used before
    let (n, text) = ciphertext.split_at(12);
    let nonce = Nonce::from_slice(n);
    let plaintext = cipher.decrypt(nonce, text)?;
    Ok(plaintext)
}

// TODO do encryption and decryption on a dedicated thread?
pub fn encrypt(plain: &[u8], key: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
    use chacha20poly1305::{
        aead::{AeadCore, OsRng},
        ChaCha20Poly1305, Key,
    };

    let k = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(k);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plain)?;

    let mut res = nonce.to_vec();
    res.extend(ciphertext);
    Ok(res)
}

// Returns the cache key for a given client
pub fn cache_entry_client(id: &str) -> String {
    format!("client_{}", id)
}

// Converts a given Json array / list into a Vec<String>
pub fn json_arr_to_vec(arr: &str) -> Vec<String> {
    arr.chars()
        .skip(1)
        .filter(|&c| c != '"')
        // TODO improve -> array inside array would not work
        .take_while(|&c| c != ']')
        .collect::<String>()
        .split(',')
        .map(|i| i.to_string())
        .collect()
}

pub fn get_client_ip(req: &HttpRequest) -> String {
    req.connection_info()
        .peer_addr()
        .unwrap_or("unknown")
        .to_string()
}

pub fn get_local_hostname() -> String {
    let hostname_os = gethostname();
    hostname_os
        .to_str()
        .expect("Error getting the hostname from the OS")
        .to_string()
}

// Returns an alphanumeric random String with the requested length
pub fn get_rand(count: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(count)
        .map(char::from)
        .collect::<String>()
}

pub fn base64_encode(input: &[u8]) -> String {
    B64_STD.encode(input)
}

pub fn base64_decode(b64: &str) -> Result<Vec<u8>, ErrorResponse> {
    B64_STD.decode(b64).map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "B64 decoding error".to_string(),
        )
    })
}

// Returns the given input as a base64 URL Encoded String
pub fn base64_url_encode(input: &[u8]) -> String {
    let b64 = B64_STD.encode(input);
    b64.chars()
        .filter_map(|c| match c {
            '=' => None,
            '+' => Some('-'),
            '/' => Some('_'),
            x => Some(x),
        })
        .collect()
}

pub fn base64_url_no_pad_encode(input: &[u8]) -> String {
    B64_URL_SAFE_NO_PAD.encode(input)
}

pub fn base64_url_decode(b64: &str) -> Result<Vec<u8>, ErrorResponse> {
    B64_URL_SAFE.decode(b64).map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "B64 decoding error".to_string(),
        )
    })
}

pub fn base64_url_no_pad_decode(b64: &str) -> Result<Vec<u8>, ErrorResponse> {
    B64_URL_SAFE_NO_PAD.decode(b64).map_err(|_| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "B64 decoding error".to_string(),
        )
    })
}

pub fn new_store_id() -> String {
    get_rand(24)
}

// Extracts the claims from a given token into a HashMap.
// Returns an empty HashMap if no values could be extracted at all.
// CAUTION: Does not validate the token!
pub fn extract_token_claims_unverified<T>(token: &str) -> Result<T, ErrorResponse>
where
    T: for<'a> serde::Deserialize<'a>,
{
    let body = match token.split_once('.') {
        None => None,
        Some((_metadata, rest)) => rest.split_once('.').map(|(body, _validation_str)| body),
    };
    if body.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Invalid or malformed JWT Token".to_string(),
        ));
    }
    let body = body.unwrap();

    let b64 = match B64_URL_SAFE_NO_PAD.decode(body) {
        Ok(values) => values,
        Err(err) => {
            error!(
                "Error decoding JWT token body '{}' from base64: {}",
                body, err
            );
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid JWT Token body".to_string(),
            ));
        }
    };
    let s = String::from_utf8_lossy(b64.as_slice());
    let claims = match serde_json::from_str::<T>(s.as_ref()) {
        Ok(claims) => claims,
        Err(err) => {
            error!("Error deserializing JWT Token claims: {}", err);
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid JWT Token claims".to_string(),
            ));
        }
    };

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::string::String;

    #[tokio::test]
    async fn test_encryption() -> Result<(), Box<dyn std::error::Error>> {
        let text = "PlainText";
        let plain = text.as_bytes();
        let key = b"9dJsZiqfgxoCABYziGMW2UHq7C44jvdp";

        let enc = encrypt(plain, key).unwrap();
        assert_ne!(&enc, plain);

        let dec = decrypt(enc.as_slice(), key).unwrap();
        assert_ne!(enc, dec);
        assert_eq!(&dec, plain);

        let dec_text = String::from_utf8_lossy(&dec);
        assert_eq!(dec_text, "PlainText");

        Ok(())
    }

    #[test]
    fn test_json_arr_to_vec() {
        let arr = String::from("[\"one\",\"two\",\"three\"]");
        let arr_as_vec = vec!["one", "two", "three"];
        assert_eq!(json_arr_to_vec(&arr), arr_as_vec);

        let arr = String::from("[\"one\"]");
        let arr_as_vec = vec!["one"];
        assert_eq!(json_arr_to_vec(&arr), arr_as_vec);

        let arr = String::from("[]");
        let arr_as_vec = vec![""];
        assert_eq!(json_arr_to_vec(&arr), arr_as_vec);
    }

    #[test]
    fn test_get_rand() {
        let rnd = get_rand(11);
        assert_eq!(rnd.len(), 11);

        let rnd = get_rand(0);
        assert_eq!(rnd.len(), 0);

        let rnd = get_rand(1024);
        assert_eq!(rnd.len(), 1024);
    }
}
