use actix_web::web;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Algorithm, Argon2, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::sync::OnceLock;
use std::thread;
use tokio::time::Instant;
use tracing::{debug, error, warn};

pub static ARGON2_PARAMS: OnceLock<argon2::Params> = OnceLock::new();
pub static HASH_CHANNELS: OnceLock<(
    flume::Sender<PasswordHashMessage>,
    flume::Receiver<PasswordHashMessage>,
)> = OnceLock::new();
pub static HASH_AWAIT_WARN_TIME: OnceLock<u32> = OnceLock::new();

pub struct HashPassword {
    plain_text: String,
    tx: flume::Sender<String>,
    created: Instant,
}

impl HashPassword {
    pub async fn hash_password(plain_text: String) -> Result<String, ErrorResponse> {
        let (tx, rx) = flume::unbounded();
        let s = Self {
            plain_text,
            tx,
            created: Instant::now(),
        };

        HASH_CHANNELS
            .get()
            .unwrap()
            .0
            .send_async(PasswordHashMessage::Hash(s))
            .await
            .expect("Send PasswordHashMessage::Hash");
        rx.recv_async()
            .await
            .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, format!("{}", err)))
    }
}

pub struct ComparePasswords {
    plain_text: String,
    hash: String,
    tx: flume::Sender<bool>,
    created: Instant,
}

impl ComparePasswords {
    pub async fn is_match(plain_text: String, hash: String) -> Result<bool, ErrorResponse> {
        let (tx, rx) = flume::unbounded();
        let c = Self {
            plain_text,
            hash,
            tx,
            created: Instant::now(),
        };

        HASH_CHANNELS
            .get()
            .unwrap()
            .0
            .send_async(PasswordHashMessage::Compare(c))
            .await
            .expect("Send PasswordHashMessage::Compare");
        rx.recv_async()
            .await
            .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, format!("{}", err)))
    }
}

pub enum PasswordHashMessage {
    Hash(HashPassword),
    Compare(ComparePasswords),
}

// This is a simple limiter for concurrent password hashes.
// The "problem" with argon2id is, that it uses more memory, the safer you want your hashes to be.
// To limit the theoretical concurrent hashes while still setting a fairly high memory for the
// operation, this simple function makes sure that at no point in time, any more than the configured
// amount of max concurrent hashes do happen to not exceed system memory.
pub async fn run() {
    while let Ok(msg) = HASH_CHANNELS.get().unwrap().1.recv_async().await {
        let res = match msg {
            PasswordHashMessage::Hash(m) => {
                check_await_threshold(&m.created);
                web::block(move || hash_password(m)).await
            }
            PasswordHashMessage::Compare(m) => {
                check_await_threshold(&m.created);
                web::block(move || compare_passwords(m)).await
            }
        };
        if let Err(err) = res {
            error!("{}", err);
        }
    }
}

#[inline]
fn check_await_threshold(instant: &Instant) {
    // This cast from u128 -> u64 is "unsafe", but in reality, this threshold can never be reached
    // in this context. Having the HASH_AWAIT_WARN_TIME as u64 is a small bonus though.
    if instant.elapsed().as_millis() as u64 > *HASH_AWAIT_WARN_TIME.get().unwrap() as u64 {
        warn!(
            "Password hash request await warn time of {} ms exceeded",
            instant.elapsed().as_millis()
        );
    }
}

fn hash_password(msg: HashPassword) {
    debug!("Starting password hash on {:?}", thread::current());

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        (*ARGON2_PARAMS.get().unwrap()).clone(),
    );
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2
        .hash_password(msg.plain_text.as_bytes(), &salt)
        .expect("Error hashing the Password")
        .to_string();

    if let Err(err) = msg.tx.send(hash) {
        error!("{}", err);
    }

    debug!("Finished with password hash on {:?}", thread::current());
}

fn compare_passwords(msg: ComparePasswords) {
    debug!("Starting password compare on {:?}", thread::current());

    let mut is_match = false;

    match PasswordHash::new(&msg.hash) {
        Ok(parsed_hash) => {
            if Argon2::default()
                .verify_password(msg.plain_text.as_bytes(), &parsed_hash)
                .is_ok()
            {
                is_match = true;
            }
        }
        Err(err) => {
            error!("Error parsing the original password hash: {}", err);
        }
    }

    if let Err(err) = msg.tx.send(is_match) {
        error!("{}", err);
    }

    debug!("Finished with password compare on {:?}", thread::current());
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::env;
    use std::time::{Duration, Instant};
    use tokio::time;

    #[tokio::test]
    async fn test_limiter() {
        unsafe { env::set_var("ARGON2_M_COST", "32768") };
        unsafe { env::set_var("ARGON2_T_COST", "3") };
        unsafe { env::set_var("ARGON2_P_COST", "2") };
        unsafe { env::set_var("MAX_HASH_THREADS", "1") };
        unsafe { env::set_var("HASH_AWAIT_WARN_TIME", "100") };

        let handle = tokio::spawn(run());
        time::sleep(Duration::from_secs(1)).await;
        assert_eq!(handle.is_finished(), false);

        // hash the password once to get the base time
        let plain = "SuperRandom1337";
        let start = Instant::now();
        HashPassword::hash_password(plain.to_string())
            .await
            .unwrap();
        let time_taken = start.elapsed().as_millis();
        assert!(time_taken > 500);

        // now do the same with 3 concurrent hashes
        let start = Instant::now();
        let handle_1 = tokio::spawn(async {
            HashPassword::hash_password(plain.to_string())
                .await
                .unwrap()
        });
        let handle_2 = tokio::spawn(async {
            HashPassword::hash_password(plain.to_string())
                .await
                .unwrap()
        });
        let handle_3 = tokio::spawn(async {
            HashPassword::hash_password(plain.to_string())
                .await
                .unwrap()
        });
        // await all 3 handles
        handle_1.await.unwrap();
        handle_2.await.unwrap();
        handle_3.await.unwrap();
        let time_taken_concurrent = start.elapsed().as_millis();

        // since the MAX_HASH_THREADS was set 1, the time for 3 concurrent hashes should be about
        // 3 times the first time_taken.
        // Allow margin of error of 15%
        let time_taken_85_percent = time_taken * 85 / 100;
        assert!(time_taken_concurrent > 3 * time_taken_85_percent);
    }
}
