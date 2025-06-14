use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

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
    contents.trim().parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // if args.len() != 4 {
    //     eprintln!("Usage: {} <bug_file> <enc_pk> <clue_key>", args[0]);
    //     std::process::exit(1);
    // }

    // let bug_file = &args[1];
    // let enc_pk = &args[2];
    // let clue_key = &args[3];

    // println!("Bug file: {}", bug_file);
    // println!("Enc PK: {}", enc_pk);
    // println!("Clue key: {}", clue_key);

    let bug_file = "test/bug_report.txt";
    let test_file = "test/test_example.txt";
    let block_num_file = "test/block_num.txt";
    let contract_addr_file = "test/contract_addr.txt";

    let block_num = read_block_num(block_num_file).unwrap();
    let contract_addr = read_contract_addr(contract_addr_file).unwrap();
    let check = check_bug_impl(
        &contract_addr,
        block_num,
        &bug_file,
        &test_file,
    );

    assert!(check, "Bug check failed");

    let mut bug_file = File::open(bug_file).unwrap();
    let mut buffer = Vec::new();
    bug_file.read_to_end(&mut buffer).unwrap();
    let bug = buffer.as_slice();

    // Rest is TODO right now
    let pp_file = "test/public_params.txt";
    let mut pp_file = File::open(pp_file).unwrap();
    let mut pp_buffer = String::new();
    pp_file.read_to_string(&mut pp_buffer).unwrap();
    let pp: PublicParams = serde_json::from_str(&pp_buffer).expect("Failed to parse public params");

    let submitter = Submitter {
        public_params: pp,
    };
    submitter.submit_bug(
        &bug,
        &contract_addr,
        block_num,
        &test_file,
    ).expect("Failed to submit bug");
    println!("Bug submitted successfully");

}
