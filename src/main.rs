use std::sync::mpsc;
use std::thread;

mod config;
mod input;

mod broker {
    pub mod broker;
    pub mod broker_message;
}

fn main() {
    // Make sure the config is loaded
    config::loadconfig::load_config();

    // Create a basic channel for our main broker
    let (tx, rx) = mpsc::channel();

    // Let the broker process incomming messages
    thread::spawn(move || {
        for msg in rx {
            broker::broker::process_message(msg)
        }
    });

    // Spawn a new thread for waiting for inputs
    thread::spawn(move || {
        input::buzz::open_device(tx);
    });

    // Blink twice to show we started successfullly
    input::buzz::blink(2, None);

    //

}
