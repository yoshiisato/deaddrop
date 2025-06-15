// use std::collections::VecDeque;
// use std::path;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use hex;
use utils::db::read_data_from_json_file;


use rust_omr::types::{decode_payloads, decode_pk_clue_from_hex, BulletinBoard, Payload, PublicKey, PublicParams};
use rust_omr::setup::{gen_param};
use rust_omr::detector::detect;
// use crate::detector::decode;

// use deadbug_receiver::types::{BugInfo, EncKeys};

pub struct Detector <'a>{
    // Fields for the detector
    public_params: PublicParams,
    pk_detect: &'a [u8],
    bb: BulletinBoard  ,
    vec_payload: Vec<Payload>,
    // enc_keys: EncKeys,
}


impl<'a> Detector<'a> {
    pub fn new(
        pk: &'a [u8],
        path: &str,
        k_bound: usize,
    ) -> Self {

        let pp: PublicParams = gen_param(128, 0.1, 0.1);
        let bb = read_bulletin_board_from_json(path); 
        // println!("Bulletin Board read from {}: {:?}", path, bb);

        // Use "OMR" to detect the payloads corresponding to pk_detect from the bulletin board
        let vec_payload: Vec<Payload> = detect(&pp, &bb, pk, k_bound);
    
    Detector { public_params: pp, 
        pk_detect: pk, 
        bb: bb, 
        vec_payload: vec_payload,
    }
    }


    pub fn get_payloads(&self) -> &Vec<Payload> {
        &self.vec_payload
    }
}


#[derive(Deserialize)]
struct OmrEntry {
    clue: String,
    payload: String,
}

#[derive(Deserialize)]
struct OmrJson {
    omr: Vec<OmrEntry>,
}

pub fn read_bulletin_board_from_json(path: &str) -> BulletinBoard {
    let json_data = std::fs::read_to_string(path).expect("Failed to read JSON file");
    let data: OmrJson = serde_json::from_str(&json_data).expect("Failed to parse JSON");

    let bb = data.omr.into_iter().map(|entry| {
        let clue = hex::decode(&entry.clue).expect("Failed to decode omr_clue");
        let payload = hex::decode(&entry.payload).expect("Failed to decode omr_payload");
        // println!("Clue: {:?}", clue);
        // println!("Payload: {:?}", payload);
        (clue, payload)
    }).collect();

    bb

}


