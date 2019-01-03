//! Shared utilities for validating/parsing user input.

/// Attempts to parse the (user-passed) string as a target power state.
pub fn power_state<S: ToString>(s: &S) -> Option<bool> {
    let lower = s.to_string().to_ascii_lowercase();
    match lower.as_str() {
        "on" | "1" => Some(true),
        "off" | "0" => Some(false),
        _ => None,
    }
}
