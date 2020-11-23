mod input;
mod motors;
mod sensors;

use crate::{
    input::{InputHandler, TankMapping},
    motors::{Motor, Motors},
};
use env_logger::Env;
use log::{error, info};

#[derive(Debug)]
struct Robot {
    input_handler: Option<InputHandler<TankMapping>>,
    motors: Motors,
}

impl Robot {
    pub fn new() -> Self {
        let input_handler = match InputHandler::new(TankMapping) {
            Ok(handler) => Some(handler),
            Err(err) => {
                error!("Error initializing input handler: {}", err);
                None
            }
        };
        Self {
            input_handler,
            motors: Motors::new(),
        }
    }

    /// A single iteration of the main loop
    fn robot_loop(&mut self) {
        for &motor in Motor::ALL_MOTORS {
            let speed = self
                .input_handler
                .as_ref()
                .map(|handler| handler.motor_value(motor))
                .unwrap_or(0.0);
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
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();
    info!("Initializing robot...");
    let mut robot = Robot::new();
    info!("Finished initialization");
    robot.run();
}
