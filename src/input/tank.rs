use crate::{input::InputMapping, motors::Motor};
use gilrs::{Axis, Gamepad};
use log::warn;

#[derive(Debug)]
pub struct TankMapping;

impl InputMapping for TankMapping {
    fn motor_value(&self, gamepad: Gamepad<'_>, motor: Motor) -> f32 {
        let axis = match motor {
            Motor::RearLeft => Axis::LeftStickY,
            Motor::RearRight => Axis::RightStickY,
            Motor::FrontLeft => Axis::LeftStickY,
            Motor::FrontRight => Axis::RightStickY,
        };
        match gamepad.axis_data(axis) {
            None => {
                warn!(
                    "Could not read axis {:?} (mapped to motor {:?})",
                    axis, motor
                );
                0.0
            }
            Some(axis_data) => axis_data.value(),
        }
    }
}
