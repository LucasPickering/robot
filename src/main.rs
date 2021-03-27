mod api;
mod config;
mod input;
mod motors;
mod sensors;

use crate::{
    api::Api,
    config::{DriveMotorLocation, RobotConfig},
    input::InputHandler,
    motors::MotorHat,
};
use anyhow::Context;
use async_std::sync::RwLock;
use env_logger::Env;
use std::{env, sync::Arc};

const DEFAULT_CONFIG_PATH: &str = "./config/default.toml";

/// Main Robot struct. Handles initialization and operation of all robotic
/// activities, as well as processing user input.
// TODO fix debug derive
// #[derive(Debug)]
struct Robot {
    config: Arc<RwLock<RobotConfig>>,
    input_handler: InputHandler,
    drive_motors: MotorHat,
    api: Api,
}

impl Robot {
    pub fn new(config: RobotConfig) -> anyhow::Result<Self> {
        // Initialize hardware interfaces
        let input_handler = InputHandler::new();
        let drive_motors =
            MotorHat::new(&config).context("Initializing drive motors")?;

        // Start an HTTP API to allow reading motor state/updating config
        // Wrap the config in a rw lock so we can mutate it from the API
        let config = Arc::new(RwLock::new(config));
        let api = Api::new(Arc::clone(&config));

        Ok(Self {
            config,
            input_handler,
            drive_motors,
            api,
        })
    }

    /// Kick off the robot loop, after initialize is complete
    pub async fn run(mut self) {
        log::info!("Starting robot loop...");

        // Start the HTTP API
        // TODO cancel this task on shutdown
        let api = self.api;
        async_std::task::spawn(async {
            if let Err(err) = api.run().await {
                log::error!("Fatal API error: {}", err);
            }
        });

        loop {
            // Grab the config lock. We intentionally hold it for the whole
            // iteration so a write can't interrupt the loop mid-iteration
            let config = self.config.read().await;

            // Try to connect to a gamepad. If we already have one
            // connected, this won't do anything. This allows hot-plugging
            self.input_handler.init_gamepad();

            // Set speed for each drive motor based on the user input
            for &motor in DriveMotorLocation::ALL {
                let speed = self
                    .input_handler
                    .motor_value(config.input.drive, motor)
                    .unwrap_or(0.0);
                // Map the drive motor position to a motor channel #
                match config.drive.motors.get(&motor) {
                    Some(&motor_channel) => {
                        if let Err(err) = self
                            .drive_motors
                            .set_speed(motor_channel, speed)
                            .context("Setting motor speed")
                        {
                            log::error!("{:?}", err);
                        }
                    }
                    None => {
                        log::warn!(
                            "No motor channel mapped to motor: {:?}",
                            motor
                        );
                    }
                }
            }
        }
    }
}

#[async_std::main]
async fn main() {
    // Initialize logger with default log level
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();

    log::info!("Initializing robot...");
    // Read config path from CLI args
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_CONFIG_PATH.into());
    let config = RobotConfig::load(&config_path).expect("Error loading config");
    log::info!("Loaded config:\n{:#?}", config);

    let robot = Robot::new(config).expect("Error initializing hardware");
    log::info!("Finished initialization");
    robot.run().await;
}
