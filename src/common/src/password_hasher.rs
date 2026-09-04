use actix_web::web;
use argon2_rust::{Algorithm, Argon2, Version};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::sync::OnceLock;
use std::thread;
use tokio::time::Instant;
use tracing::{debug, error, warn};
use zeroize::Zeroize;

pub static ARGON2_PARAMS: OnceLock<argon2_rust::Params> = OnceLock::new();
pub static HASH_CHANNELS: OnceLock<(
    flume::Sender<PasswordHashMessage>,
    flume::Receiver<PasswordHashMessage>,
)> = OnceLock::new();
/// Concurrent argon2 worker loops; set from `max_hash_threads`, defaults to 1 (tests).
pub static HASH_THREADS: OnceLock<usize> = OnceLock::new();
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
            .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))
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
            .map_err(|err| ErrorResponse::new(ErrorResponseType::Internal, err.to_string()))
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
#[inline]
fn worker_count() -> usize {
    // defaults to 1 when unset (tests / before init), matching the historical single-consumer
    // behavior; set from hashing.max_hash_threads in init_static_vars
    HASH_THREADS.get().copied().unwrap_or(1)
}

pub async fn run() {
    let workers = worker_count();
    let rx = HASH_CHANNELS.get().unwrap().1.clone();

    let mut handles = Vec::with_capacity(workers);
    for _ in 0..workers {
        handles.push(tokio::spawn(hash_worker(rx.clone())));
    }
    for handle in handles {
        let _ = handle.await;
    }
}

async fn hash_worker(rx: flume::Receiver<PasswordHashMessage>) {
    while let Ok(msg) = rx.recv_async().await {
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

fn hash_password(mut msg: HashPassword) {
    debug!("Starting password hash on {:?}", thread::current());

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        *ARGON2_PARAMS.get().unwrap(),
    );

    let hash = argon2
        .hash_password_with_random_salt(msg.plain_text.as_bytes())
        .expect("Error hashing the Password");

    msg.plain_text.as_mut().zeroize();

    if let Err(err) = msg.tx.send(hash) {
        error!("{}", err);
    }

    debug!("Finished with password hash on {:?}", thread::current());
}

fn compare_passwords(mut msg: ComparePasswords) {
    debug!("Starting password compare on {:?}", thread::current());

    let mut is_match = false;

    if Argon2::verify_password(&msg.hash, msg.plain_text.as_bytes(), Algorithm::Argon2id).is_ok() {
        is_match = true;
    }
    msg.plain_text.as_mut().zeroize();

    if let Err(err) = msg.tx.send(is_match) {
        error!("{}", err);
    }

    debug!("Finished with password compare on {:?}", thread::current());
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2_rust::Params;
    use argon2_rust::params::{Memory, TagLen};
    use pretty_assertions::assert_eq;
    use std::time::{Duration, Instant};
    use tokio::time;

    // End-to-end: 2 workers drain a bounded channel; low cost, no timing asserts.
    #[tokio::test]
    async fn test_run_workers_process_all_messages() {
        let params = Params::builder()
            .memory(Memory::kib(8 * 1024))
            .passes(1)
            .lanes(1)
            .tag_len(TagLen::bytes(32))
            .build_or_panic();
        let _ = ARGON2_PARAMS.set(params);
        let _ = HASH_AWAIT_WARN_TIME.set(10_000);
        let _ = HASH_CHANNELS.set(flume::bounded(2));
        let _ = HASH_THREADS.set(2);

        assert_eq!(worker_count(), 2);

        let handle = tokio::spawn(run());

        let mut hashes = Vec::with_capacity(4);
        for _ in 0..4 {
            hashes.push(
                HashPassword::hash_password("test-password-123".to_string())
                    .await
                    .unwrap(),
            );
        }
        for h in &hashes {
            assert!(h.len() >= 32, "expected a non-empty argon2 hash string");
        }
        assert!(
            !handle.is_finished(),
            "workers keep running after processing"
        );
    }

    // pretty intensive test -> ignored by default
    #[tokio::test]
    #[ignore]
    async fn test_limiter() {
        let argon2_params = Params::builder()
            .memory(Memory::kib(32768))
            .passes(3)
            .lanes(2)
            .tag_len(TagLen::bytes(32))
            .build_or_panic();
        let _ = ARGON2_PARAMS.set(argon2_params);
        let _ = HASH_CHANNELS.set(flume::bounded(1));
        let _ = HASH_AWAIT_WARN_TIME.set(100);

        let handle = tokio::spawn(run());
        time::sleep(Duration::from_secs(1)).await;
        assert_eq!(handle.is_finished(), false);

        // hash the password once to get the base time
        // get base time taken to hash a single password
        let plain = "SuperRandom1337";

        let start = Instant::now();
        HashPassword::hash_password(plain.to_string())
            .await
            .unwrap();
        let mut time_taken = start.elapsed().as_millis();

        for _ in 0..2 {
            let start = Instant::now();
            HashPassword::hash_password(plain.to_string())
                .await
                .unwrap();
            time_taken += start.elapsed().as_millis();
            time_taken /= 2;
        }
        println!("time_taken: {}", time_taken);

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
