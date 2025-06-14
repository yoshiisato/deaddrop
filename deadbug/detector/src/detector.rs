// use std::collections::VecDeque;
// use std::path;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;


use rust_omr::types::{Payload, PublicKey, PublicParams, BulletinBoard};
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
        // println!("Detected payloads: {:?}", vec_payload);
    
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
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let data: OmrJson = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let bb: BulletinBoard = data.omr.into_iter().map(|entry| {
        let clue = entry.clue.into_bytes();
        let payload = entry.payload.into_bytes();
        // println!("Clue: {:?}", clue);
        // println!("Payload: {:?}", payload);
        (clue, payload)
    }).collect();
    // println!("Bulletin Board: {:?}", bb);
    bb

}


