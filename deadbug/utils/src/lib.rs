pub mod aes;
pub mod pke;
pub mod hashing;
pub mod db;

pub fn serialize_omr_payload(key: &[u8; 32], iv: &[u8; 16], identifier: Vec<u8>) -> Vec<u8> {
    let mut serialized = Vec::with_capacity(48 + identifier.len());
    serialized.extend_from_slice(key);
    serialized.extend_from_slice(iv);
    serialized.extend_from_slice(&identifier);
    serialized
}