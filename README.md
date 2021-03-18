# Robot!

Some code to run a robot I'm building. Runs on a Raspberry Pi 3.

## Setup

### Dev Machine

#### Install Dependencies

- [Docker](https://docs.docker.com/get-docker/) (For cross-compiling)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [cargo-make](https://github.com/sagiegurari/cargo-make)
  - `cargo install cargo-make`

Everything else you need will be installed automatically via Cargo on first run.

Cross-compilation setup ripped from https://capnfabs.net/posts/cross-compiling-rust-apps-raspberry-pi/, in case you need to fix something.

#### Env setup

Set your robot username and hostname like so:

```sh
echo ROBOT_HOST=username@hostname > robot.env
```

### Raspberry Pi

Some first time setup is required on the Pi:

- [Enable I2C support](https://learn.adafruit.com/adafruits-raspberry-pi-lesson-4-gpio-setup/configuring-i2c)

## Build & Execute

The easiest way to build+run code is:

```sh
cargo make watch
```

This will watch the source code and when it changes:

1. Build
2. Copy the new executable (and config files) onto the Pi
3. Run the new version on the Pi

See `Makefile.toml` for the individual steps involved in this if you don't want all of them.

## Debugging

You can increase the logging level by running with `RUST_LOG=<level>`. See https://docs.rs/log/0.4.11/log/.

## Formatting

We use a couple nightly-only Rustfmt config options, which means formatting has to be run on nightly even though the code runs on stable. Easiest way to do this is with:

```sh
cargo make fmt
```
