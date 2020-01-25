use crate::broker;
use broker::broker_message::BrokerMessage;
use broker::broker_message::Intention;
use evdev_rs::enums::EventCode;
use evdev_rs::enums::EventType;
use evdev_rs::enums::EV_KEY;
use evdev_rs::Device;
use evdev_rs::InputEvent;
use evdev_rs::ReadStatus;
use std::fs::File;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Hopefully the device can always be found here
const HWPATH: &str = "/dev/input/by-id/usb-Logitech_Logitech_Buzz_tm__Controller_V1-event-joystick";

// The buttons a Buzz-Controller has
#[derive(Debug)]
enum Button {
    Buzzer,
    Blue,
    Orange,
    Green,
    Yellow,
    Unknown,
}

/// Matches the given EventCode to the according button.
/// Binding results from https://github.com/torvalds/linux/blob/master/drivers/hid/hid-sony.c#L302
fn match_event_code(code: EventCode) -> Button {
    match code {
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY1) => Button::Buzzer,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY2) => Button::Yellow,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY3) => Button::Green,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY4) => Button::Orange,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY5) => Button::Blue,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY6) => Button::Buzzer,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY7) => Button::Yellow,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY8) => Button::Green,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY9) => Button::Orange,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY10) => Button::Blue,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY11) => Button::Buzzer,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY12) => Button::Yellow,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY13) => Button::Green,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY14) => Button::Orange,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY15) => Button::Blue,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY16) => Button::Buzzer,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY17) => Button::Yellow,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY18) => Button::Green,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY19) => Button::Orange,
        EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY20) => Button::Blue,
        _ => Button::Unknown,
    }
}

/// Maps the pressed buttons to their according functions.
fn match_button_to_message(btn: Button) -> Option<Intention> {
    match btn {
        Button::Buzzer => Some(Intention::DelayedTriggerPressed),
        Button::Blue => Some(Intention::InstantTriggerPressed),
        Button::Yellow => Some(Intention::PrintTriggerPressed),
        _ => None,
    }
}

/// Takes a raw evdev-event, figures out the button
/// that was pressed and its use-case to finally
/// communicate that intention to the central broker.
fn process_event(k: (ReadStatus, InputEvent), broker: mpsc::Sender<BrokerMessage>) {
    let btn = match_event_code(k.1.event_code);
    let intent = match_button_to_message(btn);
    let message: Option<BrokerMessage> = match intent {
        Some(intent) => Some(BrokerMessage {
            intention: intent,
            payload: None,
        }),
        None => None,
    };

    if let Some(msg) = message {
        broker.send(msg).unwrap();
    }
}

/// Connects to the buzz-controller and continously waits for
/// user-input which will then be processed. Thus this should
/// be executed in a dedicated thread.
pub fn open_device() {
    let f = File::open(HWPATH).unwrap();

    let mut d = Device::new().unwrap();
    d.set_fd(f).unwrap();

    loop {
        let res = d.next_event(evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING);
        // Ignore malformed events
        let event = match res {
            Ok(res) => res,
            Err(_) => continue,
        };
        // Skip non-key-events
        match event.1.event_type {
            EventType::EV_KEY => (),
            _ => continue,
        };
        // Process okayish events
        let l_broker = broker::broker::get_tx();
        process_event(event, l_broker);
    }
}

/// Let all controllers blink {n} times with breaks of
/// {duration} milliseconds.
/// The blinking will end in a turned off state.
pub fn blink(n: u8, duration: Option<u64>) {
    let d = match duration {
        None => Duration::from_millis(100),
        Some(n) => Duration::from_millis(n),
    };
    // Just to be sure
    turn_leds_off();
    // The blinking itself
    for _ in 0..n {
        thread::sleep(d);
        turn_leds_on();
        thread::sleep(d);
        turn_leds_off();
    }
}

pub fn turn_leds_on() {
    Command::new("sh")
        .arg("-c")
        .arg("echo 255 | tee /sys/class/leds/*buzz*/brightness")
        .output()
        .expect("failed to execute process");
    ();
}

pub fn turn_leds_off() {
    Command::new("sh")
        .arg("-c")
        .arg("echo 0 | tee /sys/class/leds/*buzz*/brightness")
        .output()
        .expect("failed to execute process");
    ();
}
