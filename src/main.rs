mod config;
mod input;
mod motors;
mod sensors;

use crate::{
    config::RobotConfig,
    input::InputHandler,
    motors::{Motor, Motors},
};
use anyhow::Context;
use env_logger::Env;
use log::{error, info};

// TODO fix debug derive
// #[derive(Debug)]
struct Robot {
    input_handler: InputHandler,
    motors: Motors,
}

impl Robot {
    pub fn new(config: RobotConfig) -> anyhow::Result<Self> {
        Ok(Self {
            input_handler: InputHandler::new(config.input),
            motors: Motors::new().context("Error initializing motors")?,
        })
    }

    /// A single iteration of the main loop
    fn robot_loop(&mut self) {
        // Try to connect to a gamepad. If we already have one connected, this
        // won't do anything. This allows hot-plugging
        self.input_handler.init_gamepad();

        for &motor in Motor::ALL_MOTORS {
            let speed = self.input_handler.motor_value(motor).unwrap_or(0.0);
            if let Err(err) = self.motors.set_speed(motor, speed) {
                error!("{}", err);
            }
        }
    }

    pub fn run(&mut self) {
        info!("Starting robot loop...");
        loop {
            self.robot_loop();
        }
    }
}

fn main() {
    // Initialize logger with default log level of `info`
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();
    info!("Initializing robot...");
    let config = RobotConfig::load().expect("Error loading config");
    info!("Loaded config:\n{:#?}", config);
    let mut robot = Robot::new(config).expect("Error initializing hardware");
    info!("Finished initialization");
    robot.run();
}
