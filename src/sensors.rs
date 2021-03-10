pub trait Sensor: Sized {
    type Config;
    type Output;

    fn new(config: Self::Config) -> anyhow::Result<Self>;

    fn read(&self) -> anyhow::Result<Self::Output>;
}
