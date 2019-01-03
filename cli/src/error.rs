//! Error handling.
use std::{error::Error as ErrorT, fmt, io};

// TODO: Add wrapper type around this so we can expose a generic client error instead of lifxi's.
use adm::lifxi::http::Error as LifxiClientError;

/// Represents an error encountered while using the `turn` subcommand.
#[derive(Debug)]
pub enum TurnError {
    /// No devices matched the given specifier.
    DeviceNotFound(String),
    /// The target state could not be parsed.
    ///
    /// To turn *on* a device, use a state of `on` or `1`; to turn *off* a device, use a state of
    /// `off` or `0`.
    UnrecognizedState(String),
    /// The input was parsed correctly, but the client encountered an error.
    LifxiClientError(LifxiClientError),
}

impl From<LifxiClientError> for TurnError {
    fn from(err: LifxiClientError) -> Self {
        TurnError::LifxiClientError(err)
    }
}

impl fmt::Display for TurnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TurnError::*;
        match self {
            DeviceNotFound(device) => write!(f, "No devices found matching specifier {}", device),
            UnrecognizedState(state) => write!(f, "Unrecognized target state {}", state),
            LifxiClientError(err) => write!(f, "lifxi error: {}", err),
        }
    }
}

impl ErrorT for TurnError {}

/// Represents an error encountered while using the `config` subcommand.
#[derive(Debug)]
pub enum ConfigError {
    /// No devices matched the given specifier.
    DeviceNotFound(String),
    /// An I/O error occured while saving the config.
    Io(io::Error),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ConfigError::*;
        match self {
            DeviceNotFound(device) => write!(f, "No devices found matching specifier {}", device),
            Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl ErrorT for ConfigError {}

/// A general error type.
#[derive(Debug)]
pub enum Error {
    /// An error encountered when using the `turn` subcommand.
    Turn(TurnError),
    /// An error encountered when using the `config` subcommand.
    Config(ConfigError),
}

impl From<TurnError> for Error {
    fn from(err: TurnError) -> Self {
        Error::Turn(err)
    }
}

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        Error::Config(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Turn(err) => write!(f, "{}", err),
            Error::Config(err) => write!(f, "{}", err),
        }
    }
}

impl ErrorT for Error {}
