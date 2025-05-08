use std::sync::Arc;
use tokio::sync::RwLock;

/// Application config
#[derive(Debug, Clone)]
pub struct Config {
    pub enc_key: Vec<u8>,
    pub iss: String,
    // Will contain `(Session Id, User Id)`
    pub sessions: Arc<RwLock<Vec<(String, String)>>>,
}

impl Config {
    pub async fn new(iss: String) -> anyhow::Result<Self> {
        // the encryption key used to encrypt cookies later on
        // MUST be exactly 32 bytes long
        let enc_key = rauthy_client::secure_random(32);

        // you will most probably do some async stuff here like open a DB pool ...

        Ok(Self {
            enc_key: enc_key.into_bytes(),
            iss,
            // CAUTION: This is a very simplified version of session handling only to keep this
            // example small. You should do this properly in production with expiry and and timeout!
            // Also, this `Vec<_>` is very inefficient for the job.
            sessions: Arc::new(RwLock::new(Vec::new())),
        })
    }
}
