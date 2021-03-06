//! Configuration file parsing.

use std::{
    fmt,
    fs::{read_to_string, write},
    io,
    path::PathBuf,
};

use crate::device::*;

use lazy_static::lazy_static;
use lifxi::http::Client;

lazy_static! {
    /// The shared client to be used for all LIFX operations.
    pub static ref LIFX_CLIENT: Client = Client::new(LIFX_SECRET.to_string());
    /// The mqtt broker hostname.
    pub static ref MQTT_HOST: String = CONFIG.mqtt_host.clone().expect("MQTT mode used but no host configured.");
    /// The mqtt broker hostname.
    pub static ref MQTT_PORT: u16 = CONFIG.mqtt_port.unwrap_or(1883);
    /// The LIFX API token to be used.
    static ref LIFX_SECRET: String = CONFIG.lifx_secret.clone().expect("LIFX devices used without configuring a LIFX secret.");
    /// The parsed configuration file.
    pub static ref CONFIG: Config = Config::parse().expect("Failed to parse config file.");
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// The user's configured devices.
    pub devices: Vec<Device>,
    /// The user's LIFX API secret.
    pub lifx_secret: Option<String>,
    /// The user's configured MQTT broker hostname.
    pub mqtt_host: Option<String>,
    /// The user's configured MQTT broker port (1883 is used if not specified).
    pub mqtt_port: Option<u16>,
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
    fn path() -> PathBuf {
        dirs::home_dir()
            .expect("Failed to get home directory?")
            .join(".adm/config.toml")
    }
    /// Loads the config file from ~/.adm/config.toml.
    pub fn parse() -> Result<Self, Error> {
        let s = read_to_string(Self::path())?;
        let config = toml::from_str(&s)?;
        Ok(config)
    }
    /// Writes the updated config file to ~/.adm/config.toml.
    pub fn write(&self) -> Result<(), io::Error> {
        let s = toml::to_string_pretty(self).expect("Failed to serialize config as TOML.");
        write(Self::path(), s)?;
        Ok(())
    }
    /// Finds the specified device in the list of configured devices.
    pub fn find<'a, S: ToString>(&'a self, s: S) -> Option<&'a Device> {
        let s = s.to_string();
        self.devices
            .iter()
            .enumerate()
            .find(|(index, device)| {
                device.name.eq_ignore_ascii_case(&s)
                    || device
                        .alternatives
                        .iter()
                        .map(|d| d.iter())
                        .flatten()
                        .any(|alt| alt.eq_ignore_ascii_case(&s))
                    || format!("{}", (index + 1)) == s
            })
            .map(|(_, d)| d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find() {
        let config = toml::from_str::<Config>("[[devices]]\ntype=\"lifx\"\nname=\"foo\"\nselector=\"label:foo\"\n[[devices]]\ntype=\"lifx\"\nname=\"bar\"\nselector=\"label:bar\"\n[[devices]]\ntype=\"lifx\"\nname=\"baz\"\nselector=\"label:baz\"\nalternatives=[\"qux\"]\n").expect("Failed to parse config.");
        let foo = config.find("foo");
        assert_eq!(foo.map(|d| d.name.as_str()), Some("foo"));
        let bar = config.find("bar");
        assert_eq!(bar.map(|d| d.name.as_str()), Some("bar"));
        assert_eq!(config.find("2"), bar);
        let baz = config.find("baz");
        assert!(baz.is_some());
        assert_eq!(baz, config.find("qux"));
        assert!(config.find("label").is_none());
        assert!(config.find("lifx").is_none());
        assert!(config.find("").is_none());
        assert!(config.find("0").is_none());
        assert!(config.find("4").is_none());
    }
}
