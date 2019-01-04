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
    Power(String),
    Toggle(String),
    Brightness(String),
    Color(String),
    State(String),
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
                        "power" => match parts.next() {
                            Some("toggle") => Some(Toggle(device)),
                            Some(_) => None,
                            None => Some(Power(device)),
                        },
                        "brightness" => Some(Brightness(device)),
                        "color" => Some(Color(device)),
                        "state" => Some(State(device)),
                        _ => None,
                    }
                })
            }),
            _ => None,
        })
    }
}

const TOPICS: &[&str] = &[
    "devices/+/power",
    "devices/+/power/toggle",
    "devices/+/brightness",
    "devices/+/color",
    "devices/+/state",
];

fn main() -> Result<(), Error> {
    let opts = MqttOptions::new(CLIENT_ID, MQTT_HOST.to_string(), *MQTT_PORT);
    let (mut client, rx) = MqttClient::start(opts)?;
    for topic in TOPICS {
        client.subscribe(*topic, QoS::ExactlyOnce)?;
    }
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
                                    if let MqttPayload::Power { target: state } = payload {
                                        device.power(state, true)?;
                                    }
                                }
                            }
                        }
                        Route::Toggle(device) => {
                            if let Some(device) = CONFIG.find(&device) {
                                device.toggle()?;
                            }
                        }
                        Route::Brightness(device) => {
                            if let Some(device) = CONFIG.find(&device) {
                                if let Ok(payload) = serde_json::from_str(&payload) {
                                    if let MqttPayload::State {
                                        brightness,
                                        color: _,
                                    } = payload
                                    {
                                        device.set(None, brightness, true)?;
                                    }
                                }
                            }
                        }
                        Route::Color(device) => {
                            if let Some(device) = CONFIG.find(&device) {
                                if let Ok(payload) = serde_json::from_str(&payload) {
                                    if let MqttPayload::State {
                                        color,
                                        brightness: _,
                                    } = payload
                                    {
                                        device.set(color, None, true)?;
                                    }
                                }
                            }
                        }
                        Route::State(device) => {
                            if let Some(device) = CONFIG.find(&device) {
                                if let Ok(payload) = serde_json::from_str(&payload) {
                                    if let MqttPayload::State { color, brightness } = payload {
                                        device.set(color, brightness, true)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Err(Error::Poll)
}
