use crate::common_proto::*;

pub struct Receiver {
    // Fields for the receiver
}

impl Receiver {
    // Constructor for the receiver
    pub fn new() -> Self {
        Receiver {
            // Initialize fields if necessary
        }
    }

    // Method to receive data
    pub fn get_digest(&self, detection_pk: &[u8]) -> Digest {
        // Logic to receive data

        unimplemented!();
        println!("Retrieving digest of pertinent message form the server...");
    }

    pub fn decode_digest(&self, Digest: &Digest) {
        // Logic to decode the digest   
        unimplemented!();            
        println!("Decoding the digest...");
    }

    pub fn fetch_bug_from_storage(&self, id: &Bug) -> Bug{
        unimplemented!();
        // Logic to fetch the bug from storage
        println!("Fetching bug from storage...");
    }

    pub fn decrypt_bug_file(&self, bug_file: &Bug) {
        unimplemented!();
        // Logic to decrypt the bug file
        println!("Decrypting the bug file...");
    }
}
