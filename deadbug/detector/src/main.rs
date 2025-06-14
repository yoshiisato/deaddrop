use deadbug_detector::detector::Detector;
use std::env;

use rust_omr::types::encode_payloads;


fn main() {
    println!("Begin detection!");

    let args: Vec<String> = env::args().collect();

    // If you expect exactly one argument (besides program name):
    if args.len() != 3 {
        eprintln!("Usage: {} <your_input>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let path: &str = &args[2];
    println!("pk_detector: {}", input);
    println!("path for bb: {}", path);


    let k_bound = 10;
    let detector = Detector::new(input.as_bytes(),path, k_bound);
    println!("Detection complete!");
    detector.get_payloads();
    println!("Payloads: {:?}", encode_payloads(detector.get_payloads()));

}

