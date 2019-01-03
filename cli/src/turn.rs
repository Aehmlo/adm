use crate::error::TurnError;
use adm::message::Message;

fn power_state<S: ToString>(s: &S) -> Option<bool> {
    let lower = s.to_string().to_ascii_lowercase();
    match lower.as_str() {
        "on" | "1" => Some(true),
        "off" | "0" => Some(false),
        _ => None,
    }
}

pub fn turn(device: String, state: String) -> Result<Option<Message>, TurnError> {
    power_state(&state)
        .ok_or_else(|| TurnError::UnrecognizedState(state))
        .map(|target| Some(Message::Power { device, target }))
}
