//! Message objects for transit over the wire.

pub enum Message {
    /// A message requesting a change in power status.
    Power { device: String, target: bool },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum MqttPayload {
    /// A payload encoding a change in power.
    Power { target: bool },
}

pub type MqttMessage = (String, MqttPayload);

impl From<Message> for MqttMessage {
    fn from(message: Message) -> Self {
        match message {
            Message::Power { device, target } => (
                format!("devices/{}/power", device),
                MqttPayload::Power { target },
            ),
        }
    }
}
