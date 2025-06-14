use crate::types::{PublicKey, PublicParams, SecretKey};
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

// ylitchev: key file location
const KEYS_FILE_LOCATION: &str = "/tmp/funkeys.txt";

use crate::types::*;

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

#[derive(Serialize, Deserialize)]
struct KeyPair {
    sk: SecretKey,
    pk: PublicKey,
}

pub fn keygen(_pp: &PublicParams) -> (SecretKey, PublicKey) {
    if Path::new(KEYS_FILE_LOCATION).exists() {
        match read_keys_from_file() {
            Ok((sk, pk)) => {
                println!("Loaded keys from file.");
                return (sk, pk);
            }
            Err(e) => {
                println!("Failed to load keys from file, generating new ones: {}", e);
            }
        }
    }

    let sk = SecretKey {
        sk_bytes: generate_random_32_bytes().to_vec(),
    };
    let random_bytes = generate_random_32_bytes();
    // For simplicity, use the same random bytes for both public keys
    
    let pk = PublicKey {
        pk_clue: random_bytes.to_vec(),
        pk_detect: random_bytes.to_vec(),
    };

    if let Err(e) = write_keys_to_file(&sk, &pk) {
        eprintln!("Failed to write keys to file: {}", e);
    }

    (sk, pk)
}

fn write_keys_to_file(sk: &SecretKey, pk: &PublicKey) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(KEYS_FILE_LOCATION)?;

    writeln!(file, "{}", encode_sk_to_hex(sk))?;
    writeln!(file, "{}", encode_pk_clue_to_hex(&pk.pk_clue))?;
    writeln!(file, "{}", encode_pk_detect_to_hex(&pk.pk_detect))?;
    Ok(())
}

fn read_keys_from_file() -> Result<(SecretKey, PublicKey), String> {
    let mut file = File::open(KEYS_FILE_LOCATION).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    let mut lines = contents.lines();
    let sk_hex = lines.next().ok_or("Missing secret key line")?;
    let clue_hex = lines.next().ok_or("Missing clue key line")?;
    let detect_hex = lines.next().ok_or("Missing detect key line")?;

    let sk = decode_sk_from_hex(sk_hex)?;
    let pk_clue = decode_pk_clue_from_hex(clue_hex)?;
    let pk_detect = decode_pk_detect_from_hex(detect_hex)?;

    let pk = PublicKey {
        pk_clue,
        pk_detect,
    };

    Ok((sk, pk))
}
