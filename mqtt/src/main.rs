use adm::lifxi::http::Error as LifxiError;
use adm::{
    config::{CONFIG, MQTT_HOST, MQTT_PORT},
    message::MqttPayload,
};
use rumqtt::{error::ConnectError, *};
use std::result::Result;

const CLIENT_ID: &str = "adm-client";

#[derive(Debug)]
pub enum Error {
    /// An error was encountered while starting the client.
    ClientStart(ClientError),
    /// An error was encountered while subscribing to the topic.
    Subscribe(ConnectError),
    /// An error was encountered while polling for messages.
    Poll,
    /// An error occured while modifying the device power status.
    Power(LifxiError),
}

impl From<ClientError> for Error {
    fn from(err: ClientError) -> Self {
        Error::ClientStart(err)
    }
}

impl From<LifxiError> for Error {
    fn from(err: LifxiError) -> Self {
        Error::Power(err)
    }
}

impl From<ConnectError> for Error {
    fn from(err: ConnectError) -> Self {
        Error::Subscribe(err)
    }
}

fn main() -> Result<(), Error> {
    let opts = MqttOptions::new(CLIENT_ID, MQTT_HOST.to_string(), *MQTT_PORT);
    let (mut client, rx) = MqttClient::start(opts)?;
    client.subscribe("devices/+/power", QoS::ExactlyOnce)?;
    while let Ok(message) = rx.recv() {
        match message {
            Notification::Publish(body) => {
                if let Ok(payload) = String::from_utf8(body.payload.to_vec()) {
                    if let Some(device) = body.topic_name.split("/").nth(1) {
                        if let Some(device) = CONFIG.find(&device) {
                            if let Ok(payload) = serde_json::from_str(&payload) {
                                match payload {
                                    MqttPayload::Power { target: state } => {
                                        device.power(state, true)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Err(Error::Poll)
}
