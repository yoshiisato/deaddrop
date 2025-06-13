use deadbug_receiver::receiver::Receiver;

fn main() {
    println!("Hello, world!");

        // Create an instance of the receiver
    let receiver = Receiver::new();

    // Example usage of the receiver methods
    // let digest = receiver.get_digest(&[0; 32]);
    // receiver.decode_digest(&digest);
    // let bug = receiver.fetch_bug_from_storage(&db_identifier);
    // receiver.decrypt_bug_file(&bug);

    println!("Receiver is running...");
}
