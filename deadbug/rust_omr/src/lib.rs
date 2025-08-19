pub mod http_client;
pub mod protocol;
use std::env;

#[test]
fn test_end_to_end()-> Result<(), Box<dyn std::error::Error>> {
    use crate::http_client::{init_deaddrop, gen_clue, submit_clue, gen_encrypted_digest, decode_digest};

    let host = env::var("OMR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("OMR_PORT").unwrap_or_else(|_| "8080".to_string());
    let base_url = format!("http://{host}:{port}");

    // generate keys and initialize clue DB
    let (sk_decode, pk_clue, pk_detect) = init_deaddrop(&base_url)?;

    // generate clue
    let clue = gen_clue(&base_url, &pk_clue)?;

    // generate payload (payload DB not implemented yet)
    // let payload = vec![vec![3u8; 32], vec![4u8; 32]];

    // submit clue
    let expected_indices: Vec<u64> = vec![500, 1000, 1234];
    for &idx in &expected_indices {
        submit_clue(&base_url, &clue, idx as i32)?; 
    }

    // generate encrypted digest
    let encrypted_digest = gen_encrypted_digest(&base_url, &pk_detect)?;

    // decode encrypted digest
    let digest = decode_digest(&base_url, &encrypted_digest, &sk_decode)?;
    
    // evaluate decoded digest (obtain pertinent indices)
    let pertinent_indices: Vec<u64> = digest
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v != 0 { Some(i as u64) } else { None })  // Cast i to u64
        .collect();
    println!("Pertinent indices: {:?}", pertinent_indices);

    // test
    assert_eq!(expected_indices, pertinent_indices, "Expected indices don't match pertinent indices");

    Ok(())
}
