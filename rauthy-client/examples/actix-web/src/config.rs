use actix_web::web::Bytes;

/// Application config
#[derive(Debug, Clone)]
pub struct Config {
    pub enc_key: Bytes,
}

impl Config {
    pub async fn new() -> anyhow::Result<Self> {
        // the encryption key used to encrypt cookies later on
        // MUST be exactly 32 bytes long
        let enc_key = rauthy_client::secure_random(32);

        // you will most probably do some async stuff here like open a DB pool ...

        Ok(Self {
            enc_key: Bytes::from(enc_key),
        })
    }
}
