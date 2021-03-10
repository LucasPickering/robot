use crate::{
    input::{InputAxis, InputMapping},
    motors::MotorPosition,
};
use gilrs::Axis;
use serde::Deserialize;

use super::InputHandler;

/// Tank drive, where one stick controls both left wheels and the other controls
/// both right wheels
#[derive(Debug, Deserialize)]
pub struct TankMapping {
    left_motor_axis: InputAxis,
    right_motor_axis: InputAxis,
}

impl InputMapping for TankMapping {
    fn motor_value(
        &self,
        handler: &InputHandler,
        motor: MotorPosition,
    ) -> Option<f32> {
        let axis = match motor {
            // TODO add abstraction layer for motor position to number
            MotorPosition::Motor1 => self.left_motor_axis,
            MotorPosition::Motor2 => self.left_motor_axis,
            MotorPosition::Motor3 => self.right_motor_axis,
            MotorPosition::Motor4 => self.right_motor_axis,
        };
        handler.read_axis(axis)
    }
}

impl Default for TankMapping {
    fn default() -> Self {
        Self {
            left_motor_axis: InputAxis {
                axis: Axis::LeftStickY,
                transformation: Default::default(),
            },
            right_motor_axis: InputAxis {
                axis: Axis::RightStickY,
                transformation: Default::default(),
            },
        }
    }
}
