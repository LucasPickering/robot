# Robot!

Some code to run a robot I'm building. Runs on a Raspberry Pi 3.

## Setup

### Cross-compiling

To cross-compile for the Raspberry Pi from Ubuntu:

In `/etc/apt/sources.list`, add:

```
deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ focal main multiverse restricted universe
deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ focal-updates main multiverse restricted universe
```

Then run:

```sh
sudo dpkg --add-architecture armhf
sudo apt update
sudo apt install gcc-arm-linux-gnueabihf libudev-dev:armhf
export PKG_CONFIG_ALLOW_CROSS=1
```

^ Those steps might not actually work at all. You may have to manually download the toolchain and add it to your `PATH`.

https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/

### Cargo Make

Useful tool.

```sh
cargo install cargo-make
```

Then set your robot username and hostname like so:

```sh
echo ROBOT_HOST=username@hostname > robot.env
```

## Building

```sh
cargo make deploy
```

ez clap

## Debugging

You can increase the logging level by running with `RUST_LOG=<level>`. See https://docs.rs/log/0.4.11/log/.

## Formatting

We use a couple nightly-only Rustfmt config options, which means formatting has to be run on nightly even though the code runs on stable. Easiest way to do this is with:

```sh
cargo make fmt
```
