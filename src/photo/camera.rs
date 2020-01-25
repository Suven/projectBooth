use crate::broker;
use broker::broker::send_message;
use broker::broker_message::BrokerMessage;
use broker::broker_message::Intention;
use gphoto;

/// Tries to trigger a capture via the connected camera.
pub fn trigger_capture() {
    let mut ctx = match gphoto::Context::new() {
        Ok(ctx) => ctx,
        Err(_) => {
            return send_message(BrokerMessage {
                intention: Intention::GPhotoBroken,
                payload: Some(String::from("Couldn't get gphoto-ctx")),
            })
        }
    };

    let mut cam = match gphoto::Camera::autodetect(&mut ctx) {
        Ok(cam) => cam,
        Err(_) => {
            return send_message(BrokerMessage {
                intention: Intention::CameraBroken,
                payload: Some(String::from("Couldn't detect camera")),
            })
        }
    };

    match cam.capture_image(&mut ctx) {
        Ok(_) => send_message(BrokerMessage {
            intention: Intention::CaptureWasInitiated,
            payload: None,
        }),
        Err(_) => send_message(BrokerMessage {
            intention: Intention::CameraBroken,
            payload: Some(String::from("Couldn't capture photo")),
        }),
    };
}
