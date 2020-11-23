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
sudo dpkg --add-architecture armhf # Maybe not necessary, need to double check
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

In `Makefile.toml`, set `ROBOT_HOST` to the `username@hostname` of your robot.

## Building

```sh
cargo make deploy
```

ez clap

## Debugging

You can increase the logging level by running with `RUST_LOG=<level>`. See https://docs.rs/log/0.4.11/log/.
