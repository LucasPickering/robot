FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

ENV PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y libudev-dev:armhf
