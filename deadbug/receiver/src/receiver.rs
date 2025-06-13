use std::collections::VecDeque;

use rust_omr::types::{Payload, PublicKey, PublicParams, SecretKey};
use rust_omr::setup::{gen_param, keygen};
use rust_omr::receiver::decode;

use crate::types::{BugInfo, EncKeys};

pub struct Receiver {
    // Fields for the receiver
    public_params: PublicParams,
    public_key: PublicKey,
    secret_key: SecretKey,
    bug_info: BugInfo,
    enc_keys: EncKeys,
    decoded_paylods_queue: VecDeque<Payload>, // A queue to store retrieved info not yet fetched from the DB

}

impl Receiver {
    // Constructor for the receiver
    pub fn new() -> Self {
        let pp: PublicParams = gen_param(128, 0.1, 0.1);
        let (sk, pk) = keygen(&pp);

        let bug_info = BugInfo {
            addr: "<contract_address>".to_string(),
            rules: "<invariants of the bug>".to_string(),
        };

        let enc_keys = EncKeys::key_gen();

        let decoded_payloads = VecDeque::new();

        Receiver {
            public_params: gen_param(128, 0.1, 0.1),
            public_key: pk,
            secret_key: sk,
            bug_info,
            enc_keys,
            decoded_paylods_queue: decoded_payloads,
        }
    }

    pub fn post_info_for_submitters(&self) {

        // Logic to post the bug information for submitters

        println!("Bug Address: {}", self.bug_info.addr);
        println!("Bug Rules: {}", self.bug_info.rules);

        let clue_key = self.public_key.pk_clue.clone();
        let enc_key = self.enc_keys.pk_enc.clone();

        println!("Encryption Key: {:?}", enc_key);
        println!("Clue Key: {:?}", clue_key);
    }

    pub fn decode_digest(&mut self, digest: &Vec<Payload>) {
        
        let dec_payload = decode(
            &self.public_params,
            digest.clone(),
            &self.secret_key,
        );

        match dec_payload {
            rust_omr::types::DecodeResult::PayloadList(decoded) => {
                println!("Decoded Payloads: {:?}", decoded);

            //use a queue to do this 
            for payload in decoded.iter() {
                self.decoded_paylods_queue.push_back(payload.to_vec());
            }

            print!("Decoded Payloads added to the internal queue!");
            
            }
            _ => {
                println!("Decoding failed or overflow occurred.");
                return;
            }
        }
    }

    // Method to extact one item from the queue 
    // This will give the id to fetch the file

    pub fn get_next_decoded_payload(&mut self) -> Option<Payload> {
        if !self.decoded_paylods_queue.is_empty() {
            let decoded_payload = self.decoded_paylods_queue.pop_front();
            decoded_payload
        }else{
            println!("No decoded payloads available.");
            return None;
        }
    }

    // Method to extract id and symmetric key from the decoded payload (popped from the queue by previous function)
    pub fn extract_info_from_decoded_payload(&self, payload: &Payload) -> (String, String) {
        // Assuming the payload contains the id and enc_hex_file in a specific format
        // For now, we just return dummy values
        let id = "dummy_id".to_string();
        let symmetric_key = "dummy_enc_hex_file".to_string();

        (id, symmetric_key)
    }

    pub fn decrypt_bug_report(&self, enc_hex_file: &str, symmetric_key: &str) -> String {
        // Logic to decrypt the bug report using the symmetric key
        // For now, we just return a dummy decrypted report
        let decrypted_report = format!("Decrypted report for {} with key {}", enc_hex_file, symmetric_key);
        decrypted_report
    }

    

}
