use linux_embedded_hal::{i2cdev::linux::LinuxI2CError, I2cdev};
use log::trace;
use pwm_pca9685::{Address, Channel, Pca9685};

/// Number of steps in the PWM pulse width (12-bit resolution)
const PWM_RESOLUTION: u16 = 4096;

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

    fn pwm_channel(self) -> Channel {
        match self {
            // TODO read these from the config
            Self::RearLeft => Channel::C0,
            Self::RearRight => Channel::C1,
            Self::FrontLeft => Channel::C2,
            Self::FrontRight => Channel::C3,
        }
    }
}

// TODO investigate the embedded-hal crate, maybe we should be using that for
// all our hardware types?
// TODO fix debug derive
// #[derive(Debug)]
pub struct Motors {
    pwm: Pca9685<I2cdev>,
}

/// TODO replace this with thiserror or similar
fn convert_error(error: pwm_pca9685::Error<LinuxI2CError>) -> anyhow::Error {
    anyhow::anyhow!(format!("{:?}", error))
}

impl Motors {
    pub fn new() -> anyhow::Result<Self> {
        let device_path = "/dev/i2c-1";
        let device = I2cdev::new(device_path)?;
        let address = Address::default();
        log::info!(
            "Initializing motor controller with device {} and address {:?}",
            device_path,
            address
        );

        let mut pwm = Pca9685::new(device, address).map_err(convert_error)?;
        pwm.set_prescale(100).map_err(convert_error)?; // 60 Hz

        Ok(Self { pwm })
    }

    /// Set speed for a motor, -1 to 1
    pub fn set_speed(
        &mut self,
        motor: Motor,
        speed: f32,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            (-1.0..=1.0).contains(&speed),
            "Speed must be in range [-1, 1]"
        );

        trace!("Setting motor {:?} to speed {}...", motor, speed);
        let channel = motor.pwm_channel();
        self.pwm.set_channel_on(channel, 0).map_err(convert_error)?;
        self.pwm
            .set_channel_off(
                channel,
                (PWM_RESOLUTION as f32 * speed.abs()) as u16,
            )
            .map_err(convert_error)?;

        Ok(())
    }
}
