pub mod aes;
pub mod db;
pub mod hashing;
pub mod pke;

pub fn serialize_omr_payload(key: &[u8; 32], iv: &[u8; 16], identifier: Vec<u8>) -> Vec<u8> {
    let mut serialized = Vec::with_capacity(48 + identifier.len());
    serialized.extend_from_slice(key);
    serialized.extend_from_slice(iv);
    serialized.extend_from_slice(&identifier);
    serialized
}

pub fn deserialize_omr_payload(data: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), String> {
    if data.len() < 48 {
        return Err("Data too short to deserialize".to_string());
    }
    let key = data[0..32].to_vec();
    let iv = data[32..48].to_vec();
    let identifier = data[48..].to_vec();
    Ok((key, iv, identifier))
}
