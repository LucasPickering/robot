mod tank;

use std::fmt::Debug;

pub use tank::*;

use crate::{config::InputConfig, motors::Motor};
use gilrs::{Axis, Gamepad, GamepadId, Gilrs};
use log::{debug, error, info, trace, warn};
use serde::Deserialize;

/// An input mapping defines how inputs on a gamepad are mapped to values on the
/// robot.
pub trait InputMapping: Debug {
    /// Determine the necessary input value for a motor, based on the current
    /// input state. Return None if the value cannot be read from input.
    fn motor_value(&self, handler: &InputHandler, motor: Motor) -> Option<f32>;
}

/// A formula used to transform input axis values into output axis values.
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
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
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct InputAxis {
    /// The axis on the gamepad that we read
    pub axis: Axis,
    /// A transformation to be applied to any value read from this axis
    pub transformation: AxisTransformation,
}

#[derive(Debug)]
pub struct InputHandler {
    gil: Gilrs,
    mapping: Box<dyn InputMapping>,
    // Active gamepad in use. None if there is none connected.
    gamepad_id: Option<GamepadId>,
}

impl InputHandler {
    pub fn new(config: InputConfig) -> Self {
        let gil = Gilrs::new().unwrap();
        let mut rv = Self {
            gil,
            mapping: config.into_mapping(),
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
    pub fn motor_value(&self, motor: Motor) -> Option<f32> {
        self.mapping.motor_value(self, motor)
    }
}
