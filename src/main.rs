use std::sync::mpsc;
use std::thread;
use warp::{self, Filter};

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

    // Spawn a new thread for waiting for inputs
    thread::spawn(move || {
        input::buzz::open_device(tx);
    });

    // Blink twice to show we started successfullly
    input::buzz::blink(2, None);

    // Routes for Webserving
    let inside =
        warp::header::exact("host", "inside.box:3000").and(warp::fs::dir("./assets/inside/"));

    thread::spawn(|| {
        warp::serve(inside).run(([127, 0, 0, 1], 3000));
    });

    // Let the broker process incomming messages
    for msg in rx {
        broker::broker::process_message(msg)
    }
}
