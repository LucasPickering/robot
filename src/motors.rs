use anyhow::Context;
use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, I2cdev};
use log::trace;
use pwm_pca9685::{Channel, Pca9685};
use serde::Deserialize;
use std::collections::HashMap;

use crate::config::RobotConfig;

/// Maximum duty cycle value for one PWM channel, i.e. the last step in the
/// pulse. Based on 12-bit resolution (4096 steps).
const MAX_DUTY_CYCLE: f32 = 4095.0;

/// Controller for Adafruit's Motor HAT board. Controls up to 4 DC motors with
/// PWM.
///
/// Currently this only supports DC motors, but could easily be updated to
/// support stepper motors as well if necessary.
// #[derive(Debug)]
pub struct MotorHat {
    pwm: Pca9685<I2cdev>,
    motors: HashMap<MotorChannel, Motor>,
}

/// TODO replace this with thiserror or similar
fn convert_error(error: pwm_pca9685::Error<LinuxI2CError>) -> anyhow::Error {
    anyhow::anyhow!(format!("{:?}", error))
}

impl MotorHat {
    pub fn new(config: &RobotConfig) -> anyhow::Result<Self> {
        log::info!(
            "Initializing drive motor controller with device {} and address 0x{:2x}",
            config.i2c_device_path,
            config.drive.i2c_address
        );

        let i2c_device = I2cdev::new(&config.i2c_device_path)?;
        let mut pwm = Pca9685::new(i2c_device, config.drive.i2c_address)
            .map_err(convert_error)?;
        pwm.enable().map_err(convert_error)?;
        // TODO figure out what 4 means, it came from https://github.com/ostrosco/adafruit_motorkit/blob/master/src/dc.rs
        pwm.set_prescale(4).map_err(convert_error)?;

        let mut motors = HashMap::new();
        for &channel in MotorChannel::ALL {
            let motor = Motor::new(&mut pwm, channel)?;
            motors.insert(channel, motor);
        }

        Ok(Self { pwm, motors })
    }

    /// Set speed for a motor, -1 to 1
    pub fn set_speed(
        &mut self,
        channel: MotorChannel,
        speed: f32,
    ) -> anyhow::Result<()> {
        let motor = self.motors.get(&channel).unwrap(); // TODO no unwrap
        motor.set_speed(&mut self.pwm, speed)
    }

    /// TODO
    pub fn off(&mut self) -> anyhow::Result<()> {
        for motor in self.motors.values() {
            motor.off(&mut self.pwm).with_context(|| {
                format!("Stopping motor {:?}", motor.channel)
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

/// A reference to a single Motor on the HAT. These numbers line up with the
/// numbers printed on the HAT PCB.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotorChannel {
    Motor1,
    Motor2,
    Motor3,
    Motor4,
}

impl MotorChannel {
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
    channel: MotorChannel,
}

impl Motor {
    /// Initialize a new PWM motor on the HAT.
    fn new(
        pwm: &mut Pca9685<I2cdev>,
        channel: MotorChannel,
    ) -> anyhow::Result<Self> {
        let channels = channel.pwm_channels();
        // Set the reference channel to run at full blast.
        pwm.set_channel_on(channels.ref_channel, 0)
            .map_err(convert_error)?;
        pwm.set_channel_off(channels.ref_channel, 4095)
            .map_err(convert_error)?;

        pwm.set_channel_on(channels.forward_channel, 0)
            .map_err(convert_error)?;
        pwm.set_channel_on(channels.backward_channel, 0)
            .map_err(convert_error)?;

        Ok(Self { channel })
    }

    /// Set the speed of this motor, with a value in [-1, 1]. Invalid values
    /// will return an error. The duty cycles of this motor's PWM channels will
    /// be adjusted to achieve the desired speed.
    fn set_speed(
        &self,
        pwm: &mut Pca9685<I2cdev>,
        speed: f32,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            (-1.0..=1.0).contains(&speed),
            "Speed must be in range [-1, 1]"
        );

        trace!("Setting motor {:?} to speed {}...", self.channel, speed);

        let duty_cycle = (MAX_DUTY_CYCLE * speed.abs()) as u16;
        let pwm_channels = self.channel.pwm_channels();

        if speed > 0.0 {
            pwm.set_channel_off(pwm_channels.forward_channel, duty_cycle)
                .map_err(convert_error)?;
        } else if speed < 0.0 {
            pwm.set_channel_off(pwm_channels.backward_channel, duty_cycle)
                .map_err(convert_error)?;
        } else {
            pwm.set_channel_full_off(pwm_channels.forward_channel)
                .map_err(convert_error)?;
            pwm.set_channel_full_off(pwm_channels.backward_channel)
                .map_err(convert_error)?;
        }

        Ok(())
    }

    /// Turn of all PWM channels for this motor. Should always be called before
    /// robot shutdown.
    fn off(&self, pwm: &mut Pca9685<I2cdev>) -> anyhow::Result<()> {
        let channels = self.channel.pwm_channels();
        pwm.set_channel_full_off(channels.ref_channel)
            .map_err(convert_error)?;
        pwm.set_channel_full_off(channels.forward_channel)
            .map_err(convert_error)?;
        pwm.set_channel_full_off(channels.backward_channel)
            .map_err(convert_error)?;
        Ok(())
    }
}
