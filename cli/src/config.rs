use adm::config::CONFIG;
use structopt::StructOpt;

use crate::error::ConfigError;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Set {
    /// Set the LIFX API secret key to use.
    LifxSecret {
        /// The secret to use.
        ///
        /// If left unspecified, the user will be prompted for the secret.
        value: Option<String>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum ConfigCommand {
    /// Add a device to the list of configured devices.
    Add,
    /// Remove a device from the list of configured devices.
    Remove { device: String },
    /// Set a root-level configuration option.
    Set {
        #[structopt(subcommand)]
        key: Set,
    },
}

pub fn config(command: ConfigCommand) -> Result<(), ConfigError> {
    match command {
        ConfigCommand::Add => unimplemented!(),
        ConfigCommand::Remove { device } => {
            if let Some(device) = CONFIG.find(&device) {
                let mut config = CONFIG.clone();
                config.devices = config.devices.into_iter().filter(|d| d != device).collect();
                config.write()?;
                Ok(())
            } else {
                Err(ConfigError::DeviceNotFound(device))
            }
        }
        ConfigCommand::Set { key } => match key {
            Set::LifxSecret { value } => {
                if value.is_none() {
                    return unimplemented!();
                }
                let mut config = CONFIG.clone();
                config.lifx_secret = value;
                config.write()?;
                Ok(())
            }
        },
    }
}
