use crate::{
    config::{DriveInputMapping, DriveMotorLocation},
    motors::MotorChannel,
};
use gilrs::{Axis, Gamepad, GamepadId, Gilrs};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// An input mapping defines how inputs on a gamepad are mapped to values on the
/// robot.
pub trait InputMapping: Debug {
    /// Determine the necessary input value for a motor, based on the current
    /// input state. Return None if the value cannot be read from input.
    fn motor_value(
        &self,
        handler: &InputHandler,
        motor: MotorChannel,
    ) -> Option<f32>;
}

/// A formula used to transform input axis values into output axis values.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisTransformation {
    /// Simple transformation that makes no changes (x => x)
    Linear,
    /// Quadratic transform (x => x^2)
    Square,
}

impl AxisTransformation {
    /// Apply this transformation to the given input value.
    pub fn transform(self, input: f32) -> f32 {
        match self {
            Self::Linear => input,
            Self::Square => input.powi(2) * input.signum(),
        }
    }
}

impl Default for AxisTransformation {
    fn default() -> Self {
        Self::Linear
    }
}

/// An analog axis on a gamepad.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct InputAxis {
    /// The axis on the gamepad that we read
    pub axis: Axis,
    /// A transformation to be applied to any value read from this axis
    pub transformation: AxisTransformation,
}

#[derive(Debug)]
pub struct InputHandler {
    gil: Gilrs,
    // Active gamepad in use. None if there is none connected.
    gamepad_id: Option<GamepadId>,
}

impl InputHandler {
    pub fn new() -> Self {
        let gil = Gilrs::new().unwrap();
        let mut rv = Self {
            gil,
            gamepad_id: None,
        };

        // Try to set up the gamepad. If none is present, just log an error and
        // move on.
        rv.init_gamepad();
        if rv.gamepad_id.is_none() {
            error!("No gamepad found, initializing without one")
        }

        rv
    }

    /// Initialize the input gamepad for this handler. If the gamepad is already
    /// initialized, this does nothing. This can safely be called on every
    /// loop, to enable hot-plugging.
    pub fn init_gamepad(&mut self) {
        // If we have no input set up, attempt to connect a new one
        if self.gamepad_id.is_none() {
            // Grab the first gamepad
            match self.gil.gamepads().next() {
                None => {
                    trace!("No gamepad connected");
                }
                Some((gamepad_id, gamepad)) => {
                    info!(
                        "Using gamepad {} (id={})",
                        gamepad.name(),
                        gamepad_id
                    );
                    debug!(
                        "Gamepad mapping source: {:?}",
                        gamepad.mapping_source()
                    );
                    self.gamepad_id = Some(gamepad_id);
                }
            }
        }
    }

    fn gamepad(&self) -> Option<Gamepad<'_>> {
        self.gamepad_id.map(|id| self.gil.gamepad(id))
    }

    /// Read an input value from the given axis, and apply the axis's
    /// transformation. If the gamepad is not connected or the axis is not
    /// known, return None.
    pub fn read_axis(&self, axis: InputAxis) -> Option<f32> {
        let gamepad = self.gamepad()?;
        let raw_value = match gamepad.axis_data(axis.axis) {
            None => {
                warn!("Could not read axis {:?} ", axis.axis);
                None
            }
            Some(axis_data) => Some(axis_data.value()),
        }?;
        Some(axis.transformation.transform(raw_value))
    }

    /// Get the value for a specific motor. The corresponding input value will
    /// be looked up using the input mapping. Returns `None` if we have no
    /// gamepad connected.
    pub fn motor_value(
        &self,
        drive_input_mapping: DriveInputMapping,
        motor: DriveMotorLocation,
    ) -> Option<f32> {
        // Map the desired motor to a motor value, based on the input config
        match drive_input_mapping {
            DriveInputMapping::Tank {
                left_motor_axis,
                right_motor_axis,
            } => {
                // Map to an input axis, then read that
                let axis = match motor {
                    DriveMotorLocation::FrontLeft => left_motor_axis,
                    DriveMotorLocation::FrontRight => right_motor_axis,
                    DriveMotorLocation::BackLeft => left_motor_axis,
                    DriveMotorLocation::BackRight => right_motor_axis,
                };
                self.read_axis(axis)
            }
            DriveInputMapping::Manual {
                front_left,
                front_right,
                back_left,
                back_right,
            } => Some(match motor {
                DriveMotorLocation::FrontLeft => front_left,
                DriveMotorLocation::FrontRight => front_right,
                DriveMotorLocation::BackLeft => back_left,
                DriveMotorLocation::BackRight => back_right,
            }),
        }
    }
}
