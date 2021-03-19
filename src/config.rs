use crate::{input::InputAxis, motors::MotorChannel};
use config::{Config, File};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RobotConfig {
    /// User input configuration
    pub input: InputConfig,
    /// Robot drive system configuration
    pub drive: DriveConfig,
    /// HTTP API config
    pub api: ApiConfig,
    /// Stuff that doesn't fall under other categories
    pub general: GeneralConfig,
}

/// User input configuration, including button and axis mappings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputConfig {
    /// Configuration for the inputs used to control the robot drive system
    pub drive: DriveInputMapping,
}

/// The mapping of inputs used to control the robot's drive system. There are
/// multiple different drive input types, so each variant in this enum
/// represents one mapping type.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DriveInputMapping {
    /// Tank drive, in which the two motors on one side of the robot (left or
    /// right) run in sync. One axis controls the left motors and another
    /// controls the right.
    Tank {
        left_motor_axis: InputAxis,
        right_motor_axis: InputAxis,
    },
}

/// Robot drive system configuration, including motor mappings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DriveConfig {
    /// I2C address for the drive motor controller board
    pub i2c_address: u8,

    /// Mapping of motor positions as the drive train sees them (front-left,
    /// front-right, etc.) to how the motor controller sees them (motor 1,
    /// motor 2, etc.). This _should_ always have an entry for each
    /// [DriveMotor], but that isn't enforced. If one is missing, it will
    /// trigger a warning at runtime.
    pub motors: HashMap<DriveMotor, MotorChannel>,
}

/// HTTP API configuration. The API allows users to read and write robot state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    /// IP and port to bind to, e.g. 127.0.0.1:8000
    pub host: String,
}

/// General configuration fields, that don't fall under any other category
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Path to the I2C device on the system
    pub i2c_device_path: String,
}

/// TODO
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DriveMotor {
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
}

impl DriveMotor {
    /// TODO use strum
    pub const ALL: &'static [Self] = &[
        Self::FrontLeft,
        Self::FrontRight,
        Self::BackLeft,
        Self::BackRight,
    ];
}

impl RobotConfig {
    pub fn load(config_path: &str) -> anyhow::Result<Self> {
        info!("Reading config from {}", config_path);
        let mut s = Config::new();

        s.merge(File::with_name(config_path))?;
        // may want to add more config sources here at some point

        Ok(s.try_into()?)
    }
}
