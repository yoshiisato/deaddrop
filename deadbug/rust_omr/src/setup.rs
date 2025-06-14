use crate::types::{PublicKey, PublicParams, SecretKey};

pub fn gen_param(lambda: usize, epsilon_p: f64, epsilon_n: f64) -> PublicParams {
    PublicParams {
        lambda,
        epsilon_p,
        epsilon_n,
    }
}

fn generate_random_32_bytes() -> [u8; 32] {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

pub fn keygen(_pp: &PublicParams) -> (SecretKey, PublicKey) {
    // For now, use random byte vectors as keys
    let sk = SecretKey {
        sk_bytes: generate_random_32_bytes().to_vec(),
    };
    let random_bytes = generate_random_32_bytes();
    // For simplicity, use the same random bytes for both public keys
    
    let pk = PublicKey {
        pk_clue: random_bytes.to_vec(),
        pk_detect: random_bytes.to_vec(),
    };
    (sk, pk)
}
