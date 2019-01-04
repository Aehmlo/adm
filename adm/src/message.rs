//! Message objects for transit over the wire.

use lifxi::http::Color;

pub enum Message {
    /// A message requesting a change in power status.
    Power { device: String, target: bool },
    /// A message requesting a power toggle.
    Toggle { device: String },
    /// A message requesting a combined brightness and color setting.
    State {
        device: String,
        color: Option<Color>,
        brightness: Option<f32>,
    },
    /// A message requesting a brightness setting.
    Brightness { device: String, brightness: f32 },
    /// A message requesting a color setting.
    Color { device: String, color: Color },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum MqttPayload {
    /// A payload encoding a change in power.
    Power { target: bool },
    /// A payload encoding a color/brightness setting.
    State {
        color: Option<Color>,
        brightness: Option<f32>,
    },
}

pub type MqttMessage = (String, Option<MqttPayload>);

impl From<Message> for MqttMessage {
    fn from(message: Message) -> Self {
        match message {
            Message::Power { device, target } => (
                format!("devices/{}/power", device),
                Some(MqttPayload::Power { target }),
            ),
            Message::Toggle { device } => (format!("devices/{}/power/toggle", device), None),
            Message::Brightness { device, brightness } => (
                format!("devices/{}/brightness", device),
                Some(MqttPayload::State {
                    brightness: Some(brightness),
                    color: None,
                }),
            ),
            Message::Color { device, color } => (
                format!("devices/{}/color", device),
                Some(MqttPayload::State {
                    color: Some(color),
                    brightness: None,
                }),
            ),
            Message::State {
                device,
                color,
                brightness,
            } => (
                format!("devices/{}/state", device),
                Some(MqttPayload::State { brightness, color }),
            ),
        }
    }
}
