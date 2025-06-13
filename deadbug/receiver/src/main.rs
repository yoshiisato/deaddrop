use deadbug_receiver::receiver::Receiver;

fn main() {
    println!("Hello, world!");

        // Create an instance of the receiver
    let receiver = Receiver::new();

    receiver.post_info_for_submitters();

    //At start up we create the receiver and all the info
    // and we print the info for the frontend

    // Then we have a menu with multiple options: 

    // 1. Get detection key to request the digest (compress digest of pertinent messages, in our case encrypted symmetric keys for retrieving later bug reports)

    // 2. Decode the digest to get the info to retrieve the bug reports (index to query the database)

    // 3. Give the id for the file to retrieve and parse an HEX string of the CXTX to decrypt the bug report to get the bug report



    println!("Receiver is running...");
}
