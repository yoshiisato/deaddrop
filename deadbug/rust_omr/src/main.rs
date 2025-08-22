pub mod http_client;
pub mod protocol;
use std::env;
use std::io::{stdout, Write};
use std::time::Instant;

fn mb_from_bytes(n: usize) -> f64 { (n as f64) / (1024.0 * 1024.0) }
fn kb_from_bytes(n: usize) -> f64 { (n as f64) / 1024.0 }
fn bytes_len<T: AsRef<[u8]>>(v: &T) -> usize { v.as_ref().len() }
fn flush() { let _ = stdout().flush(); }

fn main()-> Result<(), Box<dyn std::error::Error>> {
    use crate::http_client::{init_deaddrop, gen_clue, submit_clue, gen_encrypted_digest, decode_digest};

    let host = env::var("OMR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("OMR_PORT").unwrap_or_else(|_| "8080".to_string());
    let base_url = format!("http://{host}:{port}");

    // 1) init_deaddrop
    println!("[1/5] init_deaddrop…"); flush();
    let t0 = Instant::now();
    let (sk_decode, pk_clue, pk_detect) = init_deaddrop(&base_url)?;
    let dt_init = t0.elapsed();
    println!(
        "    done in {:?} | sizes: sk_decode {:.3} MB, pk_clue {:.3} MB, pk_detect {:.3} MB",
        dt_init,
        mb_from_bytes(bytes_len(&sk_decode)),
        mb_from_bytes(bytes_len(&pk_clue)),
        mb_from_bytes(bytes_len(&pk_detect)),
    );
    flush();

    let expected_indices: Vec<u64> = vec![500, 1000, 1234, 6969];

    // 2 & 3) gen_clue + submit_clue in loop
    println!("[2/5] gen_clue + [3/5] submit_clue…"); flush();
    let mut dt_gen_clue_total = std::time::Duration::ZERO;
    let mut dt_submit_total   = std::time::Duration::ZERO;
    let mut last_clue_kb = 0.0;

    for &idx in &expected_indices {
        // gen_clue timing
        let t = Instant::now();
        let clue = gen_clue(&base_url, &pk_clue)?;
        let dt = t.elapsed();
        dt_gen_clue_total += dt;
        last_clue_kb = kb_from_bytes(bytes_len(&clue));
        println!("    gen_clue for idx {idx} in {:?} | clue {:.3} KB", dt, last_clue_kb);
        flush();

        // submit_clue timing
        let t = Instant::now();
        submit_clue(&base_url, &clue, idx as i32)?;
        let dt = t.elapsed();
        dt_submit_total += dt;
        println!("    submit_clue for idx {idx} in {:?}", dt);
        flush();
    }

    println!(
        "    totals -> gen_clue: {:?} | submit_clue: {:?} | last clue size: {:.3} KB",
        dt_gen_clue_total, dt_submit_total, last_clue_kb
    );
    flush();

    // 4) gen_encrypted_digest
    println!("[4/5] gen_encrypted_digest… (can be long)"); flush();
    let t0 = Instant::now();
    let encrypted_digest = gen_encrypted_digest(&base_url, &pk_detect)?;
    let dt_gen_digest = t0.elapsed();
    println!(
        "    done in {:?} | encrypted_digest {:.3} MB",
        dt_gen_digest,
        mb_from_bytes(bytes_len(&encrypted_digest))
    );
    flush();

    // 5) decode_digest
    println!("[5/5] decode_digest…"); flush();
    let t0 = Instant::now();
    let digest = decode_digest(&base_url, &encrypted_digest, &sk_decode)?;
    let dt_decode = t0.elapsed();
    let digest_bytes = digest.len() * std::mem::size_of::<u64>();
    println!("    done in {:?} | decoded_digest {:.3} MB", dt_decode, mb_from_bytes(digest_bytes));
    println!("    decoded_digest length: {}", digest.len());
    flush();

    // Evaluate decoded digest (obtain pertinent indices)
    let pertinent_indices: Vec<u64> = digest
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v != 0 { Some(i as u64) } else { None })
        .collect();
    println!("Pertinent indices: {:?}", pertinent_indices);
    flush();

    Ok(())
}
