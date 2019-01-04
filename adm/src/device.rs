//! Device management.
use lifxi::http::prelude::*;

// TODO: Add result wrapper.
type Result = lifxi::http::ClientResult;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Type {
    /// A [LIFX](https://lifx.com) light bulb.
    ///
    /// LIFX devices are managed using [`lifxi`](https://github.com/Aehmlo/lifxi).
    #[serde(rename = "lifx")]
    LifxBulb { selector: Selector },
}

/// The bread and butter of the device manager.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Device {
    /// The device type and any appropriate configuration.
    #[serde(flatten)]
    pub r#type: Type,
    /// The device name.
    pub name: String,
    /// A list of alternative names for the device.
    pub alternatives: Option<Vec<String>>,
}

impl Device {
    /// Changes the power state of the device.
    pub fn power(&self, on: bool, fast: bool) -> Result {
        match &self.r#type {
            Type::LifxBulb { selector } => crate::config::LIFX_CLIENT
                .select(selector.clone())
                .set_state()
                .power(on)
                .fast(fast)
                .send(),
        }
    }
    /// Toggles the device.
    pub fn toggle(&self) -> Result {
        match &self.r#type {
            Type::LifxBulb { selector } => crate::config::LIFX_CLIENT
                .select(selector.clone())
                .toggle()
                .send(),
        }
    }
    /// Sets the device color and brightness simultaneously.
    pub fn set(&self, color: Option<Color>, brightness: Option<f32>, fast: bool) -> Result {
        let (color, brightness) = match (color, brightness) {
            (None, Some(b)) => (Some(Color::Brightness(b)), None),
            p => p,
        };
        match &self.r#type {
            Type::LifxBulb { selector } => {
                if let Some(c) = color {
                    if let Some(b) = brightness {
                        crate::config::LIFX_CLIENT
                            .select(selector.clone())
                            .set_state()
                            .color(c)
                            .brightness(b)
                            .power(true)
                            .fast(fast)
                            .send()
                    } else {
                        crate::config::LIFX_CLIENT
                            .select(selector.clone())
                            .set_state()
                            .color(c)
                            .power(true)
                            .fast(fast)
                            .send()
                    }
                } else {
                    self.power(true, fast)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize() {
        let device: Device =
            toml::from_str("type = \"lifx\"\nselector = \"label:Foo\"\nname = \"foo\"").unwrap();
        assert_eq!(
            device.r#type,
            Type::LifxBulb {
                selector: Selector::Label("Foo".to_owned())
            }
        );
        assert!(toml::from_str::<Device>("").is_err());
        assert!(toml::from_str::<Device>("type = \"lifx\"").is_err());
    }
}
