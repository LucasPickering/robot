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
