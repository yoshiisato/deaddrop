use std::error::Error;
use crate::protocol::{read_chunk, wrap_chunk, read_u32_le, post_bytes};

pub fn init_deaddrop(base: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let resp = post_bytes(base, "/init", b"")?;
    let mut off = 0;
    let sk_decode = read_chunk(&resp, &mut off)?;
    let pk_clue  = read_chunk(&resp, &mut off)?;
    let pk_detect = read_chunk(&resp, &mut off)?;
    if off != resp.len() { return Err("extra bytes after /init".into()); }
    Ok((sk_decode, pk_clue, pk_detect))
}

pub fn gen_clue(base: &str, pk_clue: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut body = Vec::with_capacity(4 + pk_clue.len());
    body.extend_from_slice(&wrap_chunk(pk_clue));
    let resp = post_bytes(base, "/gen_clue", &body)?;
    let mut off = 0;
    let clue = read_chunk(&resp, &mut off)?;
    if off != resp.len() { return Err("extra bytes after /gen_clue payload".into()); }
    Ok(clue)
}

pub fn submit_clue(base: &str, clue: &[u8], index: i32) -> Result<(), Box<dyn Error>> {
    let mut body = Vec::with_capacity(8 + clue.len());
    body.extend_from_slice(&wrap_chunk(clue));
    body.extend_from_slice(&wrap_chunk(&index.to_le_bytes()));
    let resp = post_bytes(base, "/submit_clue", &body)?;
    let response_text = String::from_utf8(resp)?;
    if response_text == "OK" {
        Ok(())
    } else {
        Err(format!("Submit failed: {}", response_text).into())
    }
}

pub fn gen_encrypted_digest(base: &str, pk_detect: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut body = Vec::with_capacity(4 + pk_detect.len());
    body.extend_from_slice(&wrap_chunk(pk_detect)); 
    let resp = post_bytes(base, "/gen_encrypted_digest", &body)?;
    let mut off = 0;
    let digest = read_chunk(&resp, &mut off)?;
    if off != resp.len() { return Err("extra bytes after digest".into()); }
    Ok(digest)
}

pub fn decode_digest(base: &str, digest: &[u8], sk_decode: &[u8]) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut body = Vec::with_capacity(8 + digest.len() + sk_decode.len());
    body.extend_from_slice(&wrap_chunk(digest));
    body.extend_from_slice(&wrap_chunk(sk_decode));
    let resp = post_bytes(base, "/decode_digest", &body)?;
    let mut off = 0;
    let n = read_u32_le(&resp, &mut off)? as usize;
    if resp.len() != 4 + n*8 { return Err("decoded length mismatch".into()); }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let b = &resp[4 + i*8 .. 4 + (i+1)*8];
        let mut w = [0u8;8]; w.copy_from_slice(b);
        out.push(u64::from_le_bytes(w));
    }
    Ok(out)
}
