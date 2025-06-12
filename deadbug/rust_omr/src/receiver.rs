use crate::types::{DecodeResult, Payload, PublicParams, SecretKey};

pub fn decode(_pp: &PublicParams, m: Vec<Payload>, _sk: &SecretKey) -> DecodeResult {
    if m.len() > 5 {
        DecodeResult::Overflow
    } else {
        DecodeResult::PayloadList(m)
    }
}
