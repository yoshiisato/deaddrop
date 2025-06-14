use std::fs::File;

use rust_omr::types::OMRItem;
pub type DBEntry = (String, Vec<u8>); // (identifier, payload)


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SubmittedData {
    pub bugid: String,
    pub ciphertext: String,
    pub omr_clue: String,
    pub omr_payload: String,
    pub attestation: Option<String>
}


pub fn data_to_submitted_data(omr_item: OMRItem, db_entry: DBEntry, attestation: Option<String>) -> SubmittedData {
    let (identifier, db_payload) = db_entry;
    let encrypted_content = hex::encode(db_payload);
    let bugid = identifier.clone();
    let (omr_clue, payload) = omr_item;
    let omr_payload = hex::encode(payload);
    let clue = hex::encode(omr_clue);

    let mut att = attestation;
    if att.is_none() {
        att = Some("".to_string());
    }

    SubmittedData {
        bugid,
        ciphertext: encrypted_content,
        omr_clue: clue,
        omr_payload,
        attestation: att,
    }
}

pub fn submitted_data_to_omr_item(
    submitted_data: &SubmittedData,
) -> (OMRItem, DBEntry, Option<String>) {
    let omr_clue = hex::decode(&submitted_data.omr_clue).expect("Failed to decode omr_clue");
    let payload = hex::decode(&submitted_data.omr_payload).expect("Failed to decode omr_payload");
    let omr_item = (omr_clue, payload);

    let identifier = submitted_data.bugid.clone();
    let db_payload = hex::decode(&submitted_data.ciphertext)
        .expect("Failed to decode ciphertext");

    let db_entry = (identifier, db_payload);

    (omr_item, db_entry, submitted_data.attestation.clone())
}

pub fn write_data_to_json_file(
    data: &SubmittedData,
    file_path: &str,
) -> std::io::Result<()> {
    let json_data = serde_json::to_string(data)?;
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, &data).expect("failed to write JSON");
    Ok(())
}

pub fn read_data_from_json_file(file_path: &str) -> std::io::Result<SubmittedData> {
    let json_data = std::fs::read_to_string(file_path)?;
    let data: SubmittedData = serde_json::from_str(&json_data)?;
    Ok(data)
}