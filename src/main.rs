mod config;
mod input;
mod motors;
mod sensors;

use crate::{
    config::{DriveMotor, RobotConfig},
    input::InputHandler,
    motors::MotorHat,
};
use anyhow::Context;
use env_logger::Env;

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

        for &motor in DriveMotor::ALL {
            // TODO change unwrap_or back to 0
            let speed = self.input_handler.motor_value(motor).unwrap_or(1.0);
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
    // Initialize logger with default log level of `info`
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();
    log::info!("Initializing robot...");
    let config = RobotConfig::load().expect("Error loading config");
    log::info!("Loaded config:\n{:#?}", config);
    let mut robot = Robot::new(config).expect("Error initializing hardware");
    log::info!("Finished initialization");
    robot.run();
}
