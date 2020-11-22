mod input;
mod motors;
mod sensors;

use crate::{
    input::{InputHandler, TankMapping},
    motors::{Motor, Motors},
};
use log::error;

#[derive(Debug)]
struct Robot {
    input_handler: InputHandler<TankMapping>,
    motors: Motors,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            input_handler: InputHandler::new(TankMapping),
            motors: Motors::new(),
        }
    }

    fn robot_loop(&mut self) {
        for &motor in Motor::ALL_MOTORS {
            let speed = self.input_handler.motor_value(motor);
            if let Err(err) = self.motors.set_speed(motor, speed) {
                error!("{}", err);
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            self.robot_loop();
        }
    }
}

fn main() {
    let mut robot = Robot::new();
    robot.run();
}
