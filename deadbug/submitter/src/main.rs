use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use submitter::submitter::Submitter;
use submitter::submitter::*;
// use crate::check_bug_impl; // Uncomment and fix this if check_bug_impl exists elsewhere
fn read_contract_addr(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

fn read_block_num(file_path: &str) -> io::Result<u32> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents
        .trim()
        .parse::<u32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn main() {
    // Get everthing you need for the bug check
    let bug_file = "bug/bug_report.sol";
    let block_num_file = "test/block_num.txt";
    let test_file = "test/test_example.sol";
    let contract_addr_file = "test/contract_addr.txt";

    let block_num = read_block_num(block_num_file).unwrap();
    let contract_addr = read_contract_addr(contract_addr_file).unwrap();
    // Check the correctness of the bug
    let check = check_bug_impl(&contract_addr, block_num, &bug_file, &test_file);

    assert!(check, "Bug check failed");

    // Now read the bug file, encryption stuff and submit the bug

    let mut bug_file = File::open(bug_file).unwrap();
    let mut buffer = Vec::new();
    bug_file.read_to_end(&mut buffer).unwrap();
    let bug = buffer.as_slice();

    // let pp_file = "test/public_params.txt";
    // let mut pp_file = File::open(pp_file).unwrap();
    // let mut pp_buffer = String::new();
    // pp_file.read_to_string(&mut pp_buffer).unwrap();
    // let pp: PublicParams = serde_json::from_str(&pp_buffer).expect("Failed to parse public params");

    // let submitter = Submitter { public_params: pp };

    // let enc_pk_file = "test/enc_pk.txt";
    // let enc_pk = File::open(enc_pk_file).expect("Failed to open enc_pk file");

    // let clue_key_file = "test/clue_key.txt";
    // let clue_key = File::open(clue_key_file).expect("Failed to open clue_key file");

    // submitter.submit_bug(
    //     enc_pk,
    //     clue_key,
    //     bug
    // ).expect("Failed to submit bug");
    // println!("Bug submitted successfully");
}
