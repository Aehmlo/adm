use adm::lifxi::http::Error as LifxiError;
use adm::{
    config::{CONFIG, MQTT_HOST, MQTT_PORT},
    message::MqttPayload,
};
use rumqtt::{error::ConnectError, *};
use std::result::Result;

const CLIENT_ID: &str = "adm-client";

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
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

enum Route {
    /// The power state of the specified device is being managed.
    Power(String),
    /// The specified device is being toggled.
    Toggle(String),
}

impl Route {
    fn try_parse(s: &str) -> Option<Self> {
        use self::Route::*;
        let mut parts = s.split('/');
        parts.next().and_then(|root| match root {
            "devices" => parts.next().and_then(|device| {
                parts.next().and_then(|action| {
                    let device = device.to_string();
                    match action {
                        "power" => Some(Power(device)),
                        "toggle" => Some(Toggle(device)),
                        _ => None,
                    }
                })
            }),
            _ => None,
        })
    }
}

fn main() -> Result<(), Error> {
    let opts = MqttOptions::new(CLIENT_ID, MQTT_HOST.to_string(), *MQTT_PORT);
    let (mut client, rx) = MqttClient::start(opts)?;
    client.subscribe("devices/+/power", QoS::ExactlyOnce)?;
    client.subscribe("devices/+/toggle", QoS::ExactlyOnce)?;
    while let Ok(message) = rx.recv() {
        if let Notification::Publish(body) = message {
            let topic = body.topic_name;
            let payload = body.payload.to_vec();
            if let Ok(payload) = String::from_utf8(payload) {
                if let Some(route) = Route::try_parse(&topic) {
                    match route {
                        Route::Power(device) => {
                            if let Some(device) = CONFIG.find(&device) {
                                if let Ok(payload) = serde_json::from_str(&payload) {
                                    let MqttPayload::Power { target: state } = payload;
                                    device.power(state, true)?;
                                }
                            }
                        }
                        Route::Toggle(device) => {
                            if let Some(device) = CONFIG.find(&device) {
                                device.toggle()?;
                            }
                        }
                    }
                }
            }
        }
    }
    Err(Error::Poll)
}
