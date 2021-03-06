#[derive(Debug)]
pub enum Intention {
    InstantTriggerPressed,
    DelayedTriggerPressed,
    PrintTriggerPressed,
    CaptureWasInitiated,
    PhotoWasShot,
    PhotoWasDownloaded,
    PhotoWasPrinted,
    GPhotoBroken,
    CameraBroken,
    InputBroken,
}

pub struct BrokerMessage {
    pub intention: Intention,
    pub payload: Option<String>,
}
