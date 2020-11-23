mod tank;

pub use tank::*;

use crate::motors::Motor;
use anyhow::anyhow;
use gilrs::{Gamepad, GamepadId, Gilrs};

pub trait InputMapping {
    fn motor_value(&self, gamepad: Gamepad<'_>, motor: Motor) -> f32;
}

#[derive(Debug)]
pub struct InputHandler<T: InputMapping> {
    gil: Gilrs,
    gamepad_id: GamepadId,
    mapping: T,
}

impl<T: InputMapping> InputHandler<T> {
    pub fn new(mapping: T) -> anyhow::Result<Self> {
        let gil = Gilrs::new().unwrap();
        // Grab the first controller
        let (gamepad_id, _) = gil
            .gamepads()
            .next()
            .ok_or_else(|| anyhow!("Could not find any gamepads"))?;
        Ok(Self {
            gil,
            gamepad_id,
            mapping,
        })
    }

    fn gamepad(&self) -> Gamepad<'_> {
        self.gil.gamepad(self.gamepad_id)
    }

    pub fn motor_value(&self, motor: Motor) -> f32 {
        self.mapping.motor_value(self.gamepad(), motor)
    }
}
