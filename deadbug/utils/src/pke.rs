use age::{
    secrecy::ExposeSecret,
    x25519::{Identity, Recipient},
    Decryptor, Encryptor,
};
use std::io::{Read, Write};
use std::str::FromStr;

pub type PublicKey = age::x25519::Recipient;
pub type PrivateKey = age::x25519::Identity;

/// Encrypt arbitrary bytes using a recipient's public key
pub fn encrypt_data(data: &[u8], recipient: &PublicKey) -> Vec<u8> {
    let encryptor = Encryptor::with_recipients(vec![Box::new(recipient.clone())])
        .expect("failed to create encryptor");
    let mut encrypted = vec![];

    let mut writer = encryptor
        .wrap_output(&mut encrypted)
        .expect("encryption setup failed");
    writer.write_all(data).expect("encryption failed");
    writer.finish().expect("finalizing encryption failed");

    encrypted
}

/// Decrypt bytes using your private identity
pub fn decrypt_data(encrypted: &[u8], identity: &PrivateKey) -> Vec<u8> {
    let decryptor = Decryptor::new(encrypted).expect("failed to parse encrypted data");
    let mut reader = match decryptor {
        Decryptor::Recipients(d) => d
            .decrypt(std::iter::once(&*identity as &dyn age::Identity))
            .expect("decryption setup failed"),
        _ => panic!("unsupported decryptor variant"),
    };

    let mut decrypted = vec![];
    reader
        .read_to_end(&mut decrypted)
        .expect("decryption failed");

    decrypted
}

pub fn encrypt_to_hex(data: &[u8], recipient: &PublicKey) -> String {
    let encrypted = encrypt_data(data, recipient);
    hex::encode(encrypted)
}

pub fn decrypt_from_hex(encrypted_hex: &str, identity: &PrivateKey) -> Vec<u8> {
    let encrypted = hex::decode(encrypted_hex).expect("failed to decode hex");
    decrypt_data(&encrypted, identity)
}

/// Generate a new X25519 identity (private key) and its corresponding public key
pub fn key_gen() -> (PrivateKey, PublicKey) {
    let identity = PrivateKey::generate();
    let recipient = identity.to_public();
    (identity, recipient)
}

/// Convert public key to string (e.g., "age1xyz...")
pub fn pk_to_string(pk: &PublicKey) -> String {
    pk.to_string()
}

/// Parse public key from string
pub fn pk_from_string(s: &str) -> PublicKey {
    PublicKey::from_str(s).expect("invalid public key string")
}

/// Convert private key (identity) to string (e.g., "AGE-SECRET-KEY-...")
pub fn sk_to_string(sk: &PrivateKey) -> String {
    sk.to_string().expose_secret().clone()
}

/// Parse private key (identity) from string
pub fn sk_from_string(s: &str) -> PrivateKey {
    PrivateKey::from_str(s).expect("invalid private key string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_gen() {
        let (private_key, public_key) = key_gen();
        assert!(!public_key.to_string().is_empty());
    }

    #[test]
    fn test_encrypt_decrypt() {
        let (private_key, public_key) = key_gen();
        let data = b"Hello, world!";

        let encrypted = encrypt_data(data, &public_key);
        let decrypted = decrypt_data(&encrypted, &private_key);

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_hex() {
        let (private_key, public_key) = key_gen();
        let data = b"Hello, world!";

        let encrypted_hex = encrypt_to_hex(data, &public_key);
        let decrypted = decrypt_from_hex(&encrypted_hex, &private_key);

        assert_eq!(data.to_vec(), decrypted);
    }
}
