use crate::broker;
use broker::broker_message::BrokerMessage;

// The broker is the single receiving
// consumer for messages emitted by
// producers. It will then take care of
// distributing those to all app-parts
// that might care

pub fn process_message(msg: BrokerMessage) {
    println!(
        "The broker received a msg with the intent {:?}",
        msg.intention
    )
}
