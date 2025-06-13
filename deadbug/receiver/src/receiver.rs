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

    pub fn decode_digest(&self, digest: &Vec<Payload>) {
        
        let dec_payload = decode(
            &self.public_params,
            digest.clone(),
            &self.secret_key,
        );

        match dec_payload {
            rust_omr::types::DecodeResult::PayloadList(decoded) => {
                println!("Decoded Payloads: {:?}", decoded);
            }
            _ => {
                println!("Decoding failed or overflow occurred.");
                return;
            }
        }

        // Add the decoded payloads to the receiver's state so that we can later retrieve the info to query the DB 

        //use a queue to do this 
        // self.decoded_paylods_queue.push_back(dec_payload);
        

        // 


        unimplemented!();
        println!("Decoding the digest...");
    }


    // Method to receive data
    // pub fn get_digest(&self, detection_pk: &[u8]) -> Digest {
    //     // Logic to receive data

    //     unimplemented!();
    //     println!("Retrieving digest of pertinent message form the server...");
    // }

    // pub fn decode_digest(&self, Digest: &Digest) {
    //     // Logic to decode the digest   
    //     unimplemented!();            
    //     println!("Decoding the digest...");
    // }

    // pub fn fetch_bug_from_storage(&self, id: &Bug) -> Bug{
    //     unimplemented!();
    //     // Logic to fetch the bug from storage
    //     println!("Fetching bug from storage...");
    // }

    // pub fn decrypt_bug_file(&self, bug_file: &Bug) {
    //     unimplemented!();
    //     // Logic to decrypt the bug file
    //     println!("Decrypting the bug file...");
    // }
}
