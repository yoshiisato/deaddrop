use std::error::Error;

pub fn u32_to_le_bytes(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}
pub fn read_u32_le(buf: &[u8], off: &mut usize) -> Result<u32, Box<dyn Error>> {
    if *off + 4 > buf.len() {
        return Err("short read for u32".into());
    }
    let mut b = [0u8; 4];
    b.copy_from_slice(&buf[*off..*off + 4]);
    *off += 4;
    Ok(u32::from_le_bytes(b))
}
pub fn read_chunk(buf: &[u8], off: &mut usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let len = read_u32_le(buf, off)? as usize;
    if *off + len > buf.len() {
        return Err("short read for chunk body".into());
    }
    let out = buf[*off..*off + len].to_vec();
    *off += len;
    Ok(out)
}
pub fn wrap_chunk(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + bytes.len());
    out.extend_from_slice(&u32_to_le_bytes(bytes.len() as u32));
    out.extend_from_slice(bytes);
    out
}
pub fn post_bytes(base: &str, path: &str, body: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let url = format!("{base}{path}");
    let resp: ureq::Response = ureq::post(&url)
        .set("Content-Type", "application/octet-stream")
        .send_bytes(body)?;

    let status = resp.status();
    if status != 200 {
        let mut s = String::new();
        let _ = resp.into_reader().read_to_string(&mut s);
        return Err(format!("HTTP {} on {}: {}", status, path, s).into());
    }

    let mut reader = resp.into_reader();
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    Ok(bytes)
}
