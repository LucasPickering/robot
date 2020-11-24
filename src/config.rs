use crate::input::{InputMapping, TankMapping};
use config::{Config, File};
use log::info;
use serde::Deserialize;

const CONFIG_PATH: &str = "config/default.toml";

#[derive(Debug, Deserialize)]
pub struct RobotConfig {
    pub input: InputConfig,
}

/// All the possible input mapping types. The serialized version of this should
/// be just a single string.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputMappingType {
    Tank,
}

#[derive(Debug, Deserialize)]
pub struct InputConfig {
    /// The type of input mapping to use. Each mapping type corresponds to a
    /// particular field in this config.
    #[serde(rename = "type")]
    pub input_type: InputMappingType,
    #[serde(default)]
    pub tank: TankMapping,
}

impl RobotConfig {
    pub fn load() -> anyhow::Result<Self> {
        info!("Reading config from {}", CONFIG_PATH);
        let mut s = Config::new();

        s.merge(File::with_name(CONFIG_PATH))?;
        // may want to add more config sources here at some point

        Ok(s.try_into()?)
    }
}

impl InputConfig {
    /// Get the selected input mapping, as defined by the `input_type` field
    pub fn into_mapping(self) -> Box<dyn InputMapping> {
        match self.input_type {
            InputMappingType::Tank => Box::new(self.tank),
        }
    }
}
