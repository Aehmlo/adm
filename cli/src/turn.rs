use crate::error::TurnError;
use adm::config::CONFIG;

fn parse_state<S: ToString>(s: &S) -> Result<bool, TurnError> {
    let lower = s.to_string().to_ascii_lowercase();
    match lower.as_str() {
        "on" | "1" => Ok(true),
        "off" | "0" => Ok(false),
        _ => Err(TurnError::UnrecognizedState(lower)),
    }
}

pub fn turn(device: String, state: String, fast: bool) -> Result<(), TurnError> {
    if let Some(device) = CONFIG.find(&device) {
        let target = parse_state(&state)?;
        device.power(target, fast)?;
        Ok(())
    } else {
        if CONFIG.find(&state).is_some() && parse_state(&device).is_ok() {
            turn(state, device, fast)
        } else {
            Err(TurnError::DeviceNotFound(device))
        }
    }
}
