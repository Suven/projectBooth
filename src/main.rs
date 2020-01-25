use broker::broker_message::BrokerMessage;
use std::sync::mpsc;
use std::thread;
use warp::{self, Filter};

mod broker {
    pub mod broker;
    pub mod broker_message;
}

mod config {
    pub mod loadconfig;
    pub mod var;
}

mod input {
    pub mod buzz;
}

mod photo {
    pub mod camera;
}

fn main() {
    // Make sure the config is loaded
    config::loadconfig::load_config();

    // Create a central broker distribute events to their actions
    let (tx, rx) = mpsc::channel::<BrokerMessage>();
    broker::broker::init(tx);

    // Spawn a new thread for waiting for inputs
    thread::spawn(|| {
        input::buzz::open_device();
    });

    // Routes for Webserving
    let inside =
        warp::header::exact("host", "inside.box:3000").and(warp::fs::dir("./assets/inside/"));

    // Kick of the webserver
    thread::spawn(|| {
        warp::serve(inside).run(([127, 0, 0, 1], 3000));
    });

    // Blink twice to show we started successfullly
    input::buzz::blink(2, None);

    // Let the broker process incomming messages
    broker::broker::process_messages(rx);
}
