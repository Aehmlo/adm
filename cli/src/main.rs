use adm::{
    config::{MQTT_HOST, MQTT_PORT},
    message::{Message, MqttMessage},
};
use structopt::StructOpt;

#[cfg(not(feature = "mqtt"))]
compile_error!(
    "mqtt is currently the only supported mode.\nPlease compile with the mqtt feature enabled."
);
use rumqtt::*;
// Sigh.
use std::result::Result;

const CLIENT_ID: &str = "adm-cli";

mod config;
mod error;
mod turn;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Command {
    #[structopt(raw(setting = "structopt::clap::AppSettings::DisableVersion"))]
    /// Manage device power states.
    Turn {
        /// The device to modify.
        device: String,
        /// The desired state of the device (on or off).
        state: String,
    },
    /// Toggle device power states.
    Toggle {
        /// The device to toggle.
        device: String,
    },
    /// Manage configuration settings/files.
    Config {
        #[structopt(subcommand)]
        command: config::ConfigCommand,
    },
}

fn main() -> Result<(), error::Error> {
    if let Some(message) = match Command::from_args() {
        Command::Turn { device, state } => turn::turn(device, state)?,
        Command::Toggle { device } => turn::toggle(device)?,
        Command::Config { command } => {
            config::config(command)?;
            None
        }
    } {
        send(message)?;
    }
    Ok(())
}

fn send(message: Message) -> Result<(), error::SendError> {
    let message: MqttMessage = message.into();
    let payload = message
        .1
        .and_then(|p| serde_json::to_string(&p).ok())
        .unwrap_or_else(|| "".to_string());
    let topic = message.0.as_str();
    let opts = MqttOptions::new(CLIENT_ID, MQTT_HOST.to_string(), *MQTT_PORT);
    if let Ok((mut client, rx)) = MqttClient::start(opts) {
        client.subscribe(topic, QoS::AtLeastOnce)?;
        client.publish(topic, QoS::ExactlyOnce, payload)?;
        let _ = rx.recv();
    }
    Ok(())
}
