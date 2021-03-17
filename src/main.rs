mod api;
mod config;
mod input;
mod motors;
mod sensors;

use crate::{
    api::Api,
    config::{DriveMotor, RobotConfig},
    input::InputHandler,
    motors::MotorHat,
};
use anyhow::Context;
use env_logger::Env;
use std::env;

const DEFAULT_CONFIG_PATH: &str = "./config/default.toml";

/// Main Robot struct. Handles initialization and operation of all robotic
/// activities, as well as processing user input.
// TODO fix debug derive
// #[derive(Debug)]
struct Robot {
    config: RobotConfig,
    input_handler: InputHandler,
    drive_motors: MotorHat,
}

impl Robot {
    pub fn new(config: RobotConfig) -> anyhow::Result<Self> {
        let input_handler = InputHandler::new(config.input);
        let drive_motors =
            MotorHat::new(&config).context("Initializing drive motors")?;

        let api = Api::new();
        async_std::task::spawn(async {
            api.run().await.unwrap(); // TODO remove unwrap
        });

        Ok(Self {
            config,
            input_handler,
            drive_motors,
        })
    }

    /// A single iteration of the main loop
    fn robot_loop(&mut self) {
        // Try to connect to a gamepad. If we already have one connected, this
        // won't do anything. This allows hot-plugging
        self.input_handler.init_gamepad();

        // Set speed for each drive motor based on the user input
        for &motor in DriveMotor::ALL {
            let speed = self.input_handler.motor_value(motor).unwrap_or(0.0);
            // Map the drive motor position to a motor channel #
            match self.config.drive.motors.get(&motor) {
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
                    log::warn!("No motor channel mapped to motor: {:?}", motor);
                }
            }
        }
    }

    /// Kick off the robot loop, after initialize is complete
    pub fn run(&mut self) {
        log::info!("Starting robot loop...");
        loop {
            self.robot_loop();
        }
    }
}

fn main() {
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

    let mut robot = Robot::new(config).expect("Error initializing hardware");
    log::info!("Finished initialization");
    robot.run();
}
