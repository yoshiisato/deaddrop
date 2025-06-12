use crate::types::{PublicKey, PublicParams, SecretKey};

pub fn gen_param(lambda: usize, epsilon_p: f64, epsilon_n: f64) -> PublicParams {
    PublicParams {
        lambda,
        epsilon_p,
        epsilon_n,
    }
}

pub fn keygen(_pp: &PublicParams) -> (SecretKey, PublicKey) {
    // For now, use random byte vectors as keys
    let sk = SecretKey {
        sk_bytes: vec![0; 32],
    };
    let pk = PublicKey {
        pk_clue: vec![1; 32],
        pk_detect: vec![1; 32],
    };
    (sk, pk)
}
