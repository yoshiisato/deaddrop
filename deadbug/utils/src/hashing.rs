use hex;
use sha2::{Digest, Sha256};

/// Hashes input bytes using SHA-256 and returns a bytes vector
pub fn hash_to_bytes(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.to_vec()
}

/// Hashes input bytes using SHA-256 and returns a hex string
pub fn hash_to_string(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}
