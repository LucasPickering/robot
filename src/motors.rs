use log::trace;

#[derive(Copy, Clone, Debug)]
pub enum Motor {
    RearLeft,
    RearRight,
    FrontLeft,
    FrontRight,
}

impl Motor {
    pub const ALL_MOTORS: &'static [Self] = &[
        Self::RearLeft,
        Self::RearRight,
        Self::FrontLeft,
        Self::FrontRight,
    ];
}

#[derive(Debug)]
pub struct Motors {}

impl Motors {
    pub fn new() -> Self {
        Self {}
    }

    // Set speed for a motor, -1 to 1
    pub fn set_speed(
        &mut self,
        motor: Motor,
        speed: f32,
    ) -> anyhow::Result<()> {
        trace!("Set {:?} to {}", motor, speed);
        Ok(())
    }
}
