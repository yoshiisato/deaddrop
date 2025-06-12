use crate::types::Clue;

pub mod bb;

pub mod detector;
pub mod receiver;
pub mod setup;
pub mod submitter;
pub mod types;

#[test]
fn test_end_to_end() {
    use crate::detector::detect;
    use crate::receiver::decode;
    use crate::setup::{gen_param, keygen};
    use crate::types::{DecodeResult, Payload, PublicKey, PublicParams, SecretKey};

    // Generate public parameters
    let pp: PublicParams = gen_param(128, 0.1, 0.1);

    // Key generation
    let (sk, pk): (SecretKey, PublicKey) = keygen(&pp);

    // Simulate submission of payloads
    let payloads: Vec<Payload> = vec![vec![3u8; 32], vec![4u8; 32]];
    let bb_vals: Vec<(Clue, Payload)> = payloads
        .iter()
        .map(|x| submitter::submit(&pp, pk.pk_clue.clone(), x))
        .collect();

    // Simulate detection
    let detected_payloads: Vec<Payload> = detect(&pp, &bb_vals, &pk.pk_detect, 10);

    // Decode the detected payloads
    let decode_result = decode(&pp, detected_payloads.clone(), &sk);

    // Check the result
    match decode_result {
        DecodeResult::PayloadList(decoded) => {
            assert_eq!(decoded.len(), 2);
            assert_eq!(decoded[0], payloads[0]);
            assert_eq!(decoded[1], payloads[1]);
        }
        _ => panic!("Decoding failed"),
    }
}
