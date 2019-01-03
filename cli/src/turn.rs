use crate::error::TurnError;
use adm::{message::Message, parse::power_state};

pub fn turn(device: String, state: String) -> Result<Option<Message>, TurnError> {
    power_state(&state)
        .ok_or_else(|| TurnError::UnrecognizedState(state))
        .map(|target| Some(Message::Power { device, target }))
}
