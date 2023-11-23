use rand::{distributions, Rng};

/// Application config
#[derive(Debug, Clone)]
pub struct Config {
    pub enc_key: Vec<u8>,
}

impl Config {
    pub async fn new() -> anyhow::Result<Self> {
        let enc_key = secure_random(32);

        // you will most probably do some async stuff here like open a DB pool ...

        Ok(Self {
            enc_key: enc_key.into_bytes(),
        })
    }
}

/// Generates a secure random value with the given length.
/// Used for generating temp encryption keys in this example.
#[inline]
pub fn secure_random(count: usize) -> String {
    rand::thread_rng()
        .sample_iter(&distributions::Alphanumeric)
        .take(count)
        .map(char::from)
        .collect::<String>()
}
