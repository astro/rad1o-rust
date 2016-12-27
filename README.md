# Generate HW interface code from SVD

HW: NXP LPC4330
https://rad1o.badge.events.ccc.de/more_hardware

Download from https://www.lpcware.com/content/nxpfile/lpc43sxx-svd-file

```
mkdir src/peripheral
svd2rust -i LPC43xx_43Sxx.svd gpio > src/peripheral/gpio.rs
rustfmt src/peripheral/gpio.rs
```

# Building

```
xargo build --release --target thumbv7em-none-eabihf -v
arm-linux-gnueabihf-objcopy --strip-unneeded -O binary target/thumbv7em-none-eabihf/release/logo target/thumbv7em-none-eabihf/release/logo.c1d
```

## Inspect build

```
arm-linux-gnueabihf-objdump -SD target/thumbv7em-none-eabihf/release/logo | less
```

# TODO

* split into reusable crates: lpc43xx, rad1o, bins
* app.b1n?
* Target m0 processor

## Drivers

* Input
* Audio
* USB
* Flash + fatfs
* Xilinx CPLD?
* SDR: MAX2837?
* contrib to cortex-m:
  * Timers
  * MPU
