use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <bug_file> <enc_pk> <clue_key>", args[0]);
        std::process::exit(1);
    }

    let bug_file = &args[1];
    let enc_pk = &args[2];
    let clue_key = &args[3];

    println!("Bug file: {}", bug_file);
    println!("Enc PK: {}", enc_pk);
    println!("Clue key: {}", clue_key);

    let mut bug_file = File::open(bug_file).unwrap();
    let mut buffer = Vec::new();
    bug_file.read_to_end(&mut buffer).unwrap();
    let bug = buffer.as_slice();

    // Rest is TODO right now

}
