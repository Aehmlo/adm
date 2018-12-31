//! Configuration file parsing.

use std::{fmt, fs::read_to_string, io};

use crate::device::*;

use lazy_static::lazy_static;
use lifxi::http::Client;

lazy_static! {
    /// The shared client to be used for all LIFX operations.
    pub static ref LIFX_CLIENT: Client = Client::new(LIFX_SECRET.to_string());
    /// The LIFX API token to be used.
    static ref LIFX_SECRET: String = CONFIG.lifx_secret.clone().expect("LIFX devices used without configuring a LIFX secret.");
    /// The parsed configuration file.
    pub static ref CONFIG: Config = Config::parse().expect("Failed to parse config file.");
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// The user's configured devices.
    pub devices: Vec<Device>,
    pub(crate) lifx_secret: Option<String>,
}

/// Represents an error encountered while reading and parsing a config file.
///
/// These errors come in two flavors: I/O errors and parsing errors.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    Io(io::Error),
    /// A TOML parsing error occured.
    Toml(toml::de::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match self {
            Io(err) => write!(f, "I/O error: {}", err),
            Toml(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl Config {
    /// Loads the config file from ~/.adm/config.toml.
    pub fn parse() -> Result<Self, Error> {
        let config_path = dirs::home_dir()
            .expect("Failed to get home directory?")
            .join(".adm/config.toml");
        let s = read_to_string(config_path)?;
        let config = toml::from_str(&s)?;
        Ok(config)
    }
    /// Finds the specified device in the list of configured devices.
    pub fn find<'a, S: ToString>(&'a self, s: S) -> Option<&'a Device> {
        let s = s.to_string();
        // TODO: Allow accessing devices by index as well.
        self.devices.iter().find(|device| {
            device.name.eq_ignore_ascii_case(&s)
                || device
                    .alternatives
                    .iter()
                    .map(|d| d.iter())
                    .flatten()
                    .any(|alt| alt.eq_ignore_ascii_case(&s))
        })
    }
}
