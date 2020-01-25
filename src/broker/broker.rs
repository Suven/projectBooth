use crate::broker;
use broker::broker_message::BrokerMessage;
use broker::broker_message::Intention;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

// The broker is the single receiving
// consumer for messages emitted by
// producers. It will then take care of
// distributing those to all app-parts
// that might care

static mut LTX: Option<Sender<BrokerMessage>> = None;

pub fn init(tx: Sender<BrokerMessage>) {
    unsafe {
        LTX = Some(tx);
    }
}

pub fn get_tx() -> Sender<BrokerMessage> {
    unsafe {
        match &LTX {
            Some(tx) => tx.clone(),
            None => panic!("Broker needs to be initialized before first access"),
        }
    }
}

pub fn send_message(msg: BrokerMessage) {
    let tx = get_tx();
    tx.send(msg);
}

pub fn process_messages(rx: Receiver<BrokerMessage>) {
    for msg in rx {
        process_message(msg);
    }
}

pub fn process_message(msg: BrokerMessage) {
    println!(
        "The broker received a msg with the intent {:?}",
        msg.intention
    )
}
