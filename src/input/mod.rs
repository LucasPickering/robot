mod tank;

pub use tank::*;

use crate::motors::Motor;
use gilrs::{Gamepad, GamepadId, Gilrs};
use log::{debug, error, info, trace};

pub trait InputMapping {
    fn motor_value(&self, gamepad: Gamepad<'_>, motor: Motor) -> f32;
}

#[derive(Debug)]
pub struct InputHandler<T: InputMapping> {
    gil: Gilrs,
    mapping: T,
    // Active gamepad in use. None if there is none connected.
    gamepad_id: Option<GamepadId>,
}

impl<T: InputMapping> InputHandler<T> {
    pub fn new(mapping: T) -> Self {
        let gil = Gilrs::new().unwrap();
        let mut rv = Self {
            gil,
            gamepad_id: None,
            mapping,
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

    /// Get the value for a specific motor. The corresponding input value will
    /// be looked up using the input mapping. Returns `None` if we have no
    /// gamepad connected.
    pub fn motor_value(&self, motor: Motor) -> Option<f32> {
        self.gamepad()
            .map(|gamepad| self.mapping.motor_value(gamepad, motor))
    }
}
