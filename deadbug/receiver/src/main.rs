use deadbug_receiver::receiver::Receiver;
// ylitchev: for command line inputs
use dialoguer::{theme::ColorfulTheme, Input, Select};
// ylitchev: to process payloads
use rust_omr::types::Payload;

fn main() {
    println!("Hello, world!");

        // Create an instance of the receiver
    let mut receiver = Receiver::new();

    receiver.post_info_for_submitters();

    //At start up we create the receiver and all the info
    // and we print the info for the frontend

    // Then we have a menu with multiple options: 

    // 1. Get detection key to request the digest (compress digest of pertinent messages, in our case encrypted symmetric keys for retrieving later bug reports)

    // 2. Decode the digest to get the info to retrieve the bug reports (index to query the database)

    // 3. Give the id for the file to retrieve and parse an HEX string of the CXTX to decrypt the bug report to get the bug report



    println!("Receiver is running...");

    // let detection_key: PKDetect;

    loop {
        // Menu items shown to the user
        let menu_items = [
            "1. Get detection key",
            "2. Decode digest",
            "3. Decrypt bug report",
            "4. Exit",
        ];

        // Arrow-key, highlighted selection
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option (↑/↓ then Enter)")
            .items(&menu_items)
            .default(0)
            .interact()
            .expect("terminal interaction failed");

        // Act on the chosen item
        match selection {
            0 => handle_detection_key(&mut receiver),
            1 => handle_decode_digest(&mut receiver),
            2 => handle_option("Option 3 selected: Decrypt bug report!"),
            3 => {
                println!("\nExiting. Goodbye!");
                break;
            }
            _ => unreachable!(), // `Select` guarantees 0-3 only
        }
    }
}


fn handle_detection_key(receiver: &mut Receiver) {
    println!("\nOption 1 selected: Get detection key\n");

    // let temp_detection_key: PKDetect;

    let input: String = Input::new()
        .with_prompt("Enter anything and press Enter")
        .interact_text()
        .expect("failed to read line");

    println!("You typed: {input}\n");

    receiver.public_key.pk_detect = input.clone();
    println!("Detection key set");
    
    
}

fn handle_decode_digest(receiver: &mut Receiver) {
    let digest_input: String = Input::new()
        .with_prompt("Paste digest string (press Enter to leave empty)")
        .allow_empty(true)
        .interact_text()
        .expect("failed to read digest");

    // Convert to Vec<Payload>.  Replace this stub with real parsing logic.
    let digest_vec = parse_digest(&digest_input);
    receiver.decode_digest(&digest_vec);
}



/// Helper: prompt for arbitrary user input, then echo it with a custom banner.
fn handle_option(banner: &str) {
    println!("\n{banner}");
    let input: String = Input::new()
        .with_prompt("Enter anything and press Enter")
        .interact_text()
        .expect("failed to read line");

    println!("You typed: {input}\n");
}

