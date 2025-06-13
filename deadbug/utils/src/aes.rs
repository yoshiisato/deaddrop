use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use hex::{decode, encode};
use rand::rngs::OsRng;
use rand::RngCore;

pub type Aes256Cbc = Cbc<Aes256, Pkcs7>;

/// Generate a random 32-byte AES-256 key and 16-byte IV
pub fn generate_key_iv() -> (Vec<u8>, Vec<u8>) {
    let mut key = vec![0u8; 32];
    let mut iv = vec![0u8; 16];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut iv);
    (key, iv)
}

pub fn encryption_from_string(plaintext: &str, key: &[u8], iv: &[u8]) -> String {
    let plaintext_bytes = plaintext.as_bytes();
    encryption_to_hex(plaintext_bytes, key, iv)
}

/// Encrypts a plaintext string using AES-256-CBC
pub fn encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(plaintext)
}

pub fn encryption_to_hex(plaintext: &[u8], key: &[u8], iv: &[u8]) -> String {
    let ciphertext = encrypt(plaintext, key, iv);
    encode(ciphertext)
}

/// Decrypts ciphertext into a UTF-8 string
pub fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    cipher.decrypt_vec(ciphertext).unwrap()
}

pub fn decryption_from_hex(ciphertext: String, key: &[u8], iv: &[u8]) -> String {
    let decrypted =
        decryption_to_string(&decode(ciphertext).expect("Failed to decode hex"), key, iv);
    decrypted
}

pub fn decryption_to_string(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> String {
    let decrypted = decrypt(ciphertext, key, iv);
    String::from_utf8(decrypted).expect("Decryption failed or not valid UTF-8")
}


/// Flattens a key+iv tuple to a single Vec<u8>
pub fn serialize_key_iv(key: &[u8; 32], iv: &[u8; 16]) -> Vec<u8> {
    [key.as_ref(), iv.as_ref()].concat()
}


/// Unflattens a &[u8] back to (key, iv)
pub fn deserialize_key_iv(data: &[u8]) -> ([u8; 32], [u8; 16]) {
    assert_eq!(data.len(), 48);
    let mut key = [0u8; 32];
    let mut iv = [0u8; 16];
    key.copy_from_slice(&data[..32]);
    iv.copy_from_slice(&data[32..]);
    (key, iv)
}



#[cfg(test)]
mod tests {
    use crate::pke::{decrypt_data, encrypt_data, key_gen};

    use super::*;

    #[test]
    fn test_aes_encryption_decryption() {
        let (key, iv) = generate_key_iv();
        let plaintext_str = "Hello, AES-256!";
        let plaintext = plaintext_str.to_string().into_bytes();

        let encrypted = encrypt(&plaintext, &key, &iv);
        let decrypted = String::from_utf8(decrypt(&encrypted, &key, &iv)).unwrap();

        assert_eq!(decrypted, plaintext_str);
    }

    #[test]
    fn test_aes_encryption_decryption_string_hex() {
        let (key, iv) = generate_key_iv();
        let plaintext_str = "Hello, AES-256!";
        let plaintext = plaintext_str.to_string();

        let encrypted = encryption_from_string(&plaintext, &key, &iv);
        let decrypted = decryption_from_hex(encrypted, &key, &iv);

        assert_eq!(decrypted, plaintext_str);
    }

    #[test]
    fn test_key_iv_encryption_roundtrip() {
        // Step 1: generate keys
        let (aes_key, aes_iv) = generate_key_iv();

        // Step 2: serialize to &[u8]
        let serialized = serialize_key_iv(
            &aes_key.clone().try_into().unwrap(),
            &aes_iv.clone().try_into().unwrap(),
        );

        // Step 3: generate PKE keypair
        let (identity, recipient) = key_gen();

        // Step 4: encrypt with public key
        let encrypted = encrypt_data(&serialized, &recipient);

        // Step 5: decrypt with secret key
        let decrypted = decrypt_data(&encrypted, &identity);

        // Step 6: deserialize and check
        let (recovered_key, recovered_iv) = deserialize_key_iv(&decrypted);

        assert_eq!(aes_key, recovered_key, "AES keys do not match");
        assert_eq!(aes_iv, recovered_iv, "AES IVs do not match");
    }

}
