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
    if let Some(target) = power_state(&state) {
        Ok(Some(Message::Power { device, target }))
    } else if power_state(&device).is_some() {
        turn(state, device)
    } else {
        Err(TurnError::UnrecognizedState(state))
    }
}

pub fn toggle(device: String) -> Result<Option<Message>, TurnError> {
    Ok(Some(Message::Toggle { device }))
}
