#[derive(Clone, Debug)]
pub struct BugInfo {
    pub addr: String,
    pub rules: String,
}

// Decide which scheme should we use
#[derive(Clone, Debug)]
pub struct EncKeys {
    pub pk_enc: Vec<u8>, // Public key for encryption
    sk_enc: Vec<u8>,     // Secret key for encryption
}

impl EncKeys {
    pub fn key_gen() -> Self {
        // Generate keys for encryption
        let pk_enc = vec![0u8; 32]; // Placeholder for public key
        let sk_enc = vec![1u8; 32]; // Placeholder for secret key
        EncKeys { pk_enc, sk_enc }
    }
}
