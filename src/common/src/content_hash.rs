use blake2::digest::consts::U8;
use blake2::{Blake2b, Digest};

type BlakeHash = Blake2b<U8>;

/// Creates a truncated content hash based on blake2. This is NOT cryptographically safe. It is only
/// useful to compare the content of 2 inputs without having a huge hash output. The output will be
/// 16 HEX characters.
///
/// CAUTION: Do NOT use in any sensitive scenario!
#[inline]
pub fn content_hash(input: impl AsRef<[u8]>) -> String {
    let mut hasher = BlakeHash::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}
