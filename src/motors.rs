use std::collections::HashMap;

use anyhow::Context;
use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, I2cdev};
use log::trace;
use pwm_pca9685::{Channel, Pca9685};

/// Maximum duty cycle value, i.e. the last step in the pulse. Based on 12-bit
/// resolution (4096 steps).
const MAX_DUTY_CYCLE: f32 = 4095.0;

// TODO investigate the embedded-hal crate, maybe we should be using that for
// all our hardware types?
// TODO fix debug derive
// #[derive(Debug)]
pub struct MotorHat {
    pwm: Pca9685<I2cdev>,
    motors: HashMap<MotorPosition, Motor>,
}

/// TODO replace this with thiserror or similar
fn convert_error(error: pwm_pca9685::Error<LinuxI2CError>) -> anyhow::Error {
    anyhow::anyhow!(format!("{:?}", error))
}

impl MotorHat {
    pub fn new() -> anyhow::Result<Self> {
        let device_path = "/dev/i2c-1";
        let device = I2cdev::new(device_path)?;
        let address = 0x60;
        log::info!(
            "Initializing motor controller with device {} and address {:x}",
            device_path,
            address
        );

        let mut pwm = Pca9685::new(device, address).map_err(convert_error)?;
        pwm.enable().map_err(convert_error)?;
        // TODO figure out what 4 means, it came from https://github.com/ostrosco/adafruit_motorkit/blob/master/src/dc.rs
        pwm.set_prescale(4).map_err(convert_error)?;

        let mut motors = HashMap::new();
        for &position in MotorPosition::ALL {
            let motor = Motor::new(&mut pwm, position)?;
            motors.insert(position, motor);
        }

        Ok(Self { pwm, motors })
    }

    /// Set speed for a motor, -1 to 1
    pub fn set_speed(
        &mut self,
        position: MotorPosition,
        speed: f32,
    ) -> anyhow::Result<()> {
        let motor = self.motors.get(&position).unwrap(); // TODO no unwrap
        motor.set_speed(&mut self.pwm, speed)
    }

    /// TODO
    pub fn off(&mut self) -> anyhow::Result<()> {
        for motor in self.motors.values() {
            motor.off(&mut self.pwm).with_context(|| {
                format!("Stopping motor {:?}", motor.position)
            })?;
        }
        self.pwm.disable().map_err(convert_error)?;
        Ok(())
    }
}

// Turn off all motors on drop
impl Drop for MotorHat {
    fn drop(&mut self) {
        // We can't propagate this error, so just log it
        let result = self.off().context("Stopping motors during drop");
        if let Err(error) = result {
            log::error!("{:?}", error);
        }
    }
}

/// TODO
struct MotorHatChannels {
    ref_channel: Channel,
    forward_channel: Channel,
    backward_channel: Channel,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MotorPosition {
    Motor1,
    Motor2,
    Motor3,
    Motor4,
}

impl MotorPosition {
    /// TODO use strum
    pub const ALL: &'static [Self] =
        &[Self::Motor1, Self::Motor2, Self::Motor3, Self::Motor4];

    fn pwm_channels(self) -> MotorHatChannels {
        match self {
            // TODO find docs for this on adafruit and link here
            Self::Motor1 => MotorHatChannels {
                ref_channel: Channel::C8,
                forward_channel: Channel::C9,
                backward_channel: Channel::C10,
            },
            Self::Motor2 => MotorHatChannels {
                ref_channel: Channel::C13,
                forward_channel: Channel::C11,
                backward_channel: Channel::C12,
            },
            Self::Motor3 => MotorHatChannels {
                ref_channel: Channel::C2,
                forward_channel: Channel::C3,
                backward_channel: Channel::C4,
            },
            Self::Motor4 => MotorHatChannels {
                ref_channel: Channel::C7,
                forward_channel: Channel::C5,
                backward_channel: Channel::C6,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Motor {
    position: MotorPosition,
}

impl Motor {
    /// TODO
    fn new(
        pwm: &mut Pca9685<I2cdev>,
        position: MotorPosition,
    ) -> anyhow::Result<Self> {
        let channels = position.pwm_channels();
        // Set the reference channel to run at full blast.
        pwm.set_channel_on(channels.ref_channel, 0)
            .map_err(convert_error)?;
        pwm.set_channel_off(channels.ref_channel, 4095)
            .map_err(convert_error)?;

        pwm.set_channel_on(channels.forward_channel, 0)
            .map_err(convert_error)?;
        pwm.set_channel_on(channels.backward_channel, 0)
            .map_err(convert_error)?;

        Ok(Self { position })
    }

    /// TODO
    fn set_speed(
        &self,
        pwm: &mut Pca9685<I2cdev>,
        speed: f32,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            (-1.0..=1.0).contains(&speed),
            "Speed must be in range [-1, 1]"
        );

        trace!("Setting motor {:?} to speed {}...", self.position, speed);

        let duty_cycle = (MAX_DUTY_CYCLE * speed.abs()) as u16;
        let channels = self.position.pwm_channels();

        if speed > 0.0 {
            pwm.set_channel_off(channels.forward_channel, duty_cycle)
                .map_err(convert_error)?;
        } else if speed < 0.0 {
            pwm.set_channel_off(channels.backward_channel, duty_cycle)
                .map_err(convert_error)?;
        } else {
            pwm.set_channel_full_off(channels.forward_channel)
                .map_err(convert_error)?;
            pwm.set_channel_full_off(channels.backward_channel)
                .map_err(convert_error)?;
        }

        Ok(())
    }

    /// TODO
    fn off(&self, pwm: &mut Pca9685<I2cdev>) -> anyhow::Result<()> {
        let channels = self.position.pwm_channels();
        pwm.set_channel_full_off(channels.ref_channel)
            .map_err(convert_error)?;
        pwm.set_channel_full_off(channels.forward_channel)
            .map_err(convert_error)?;
        pwm.set_channel_full_off(channels.backward_channel)
            .map_err(convert_error)?;
        Ok(())
    }
}
