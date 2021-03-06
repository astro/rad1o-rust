//! Write **l0dable** apps for the [CCCamp 2015](https://events.ccc.de/camp/2015/) [rad1o badge](https://github.com/rad1o).
//!
//! ## Prerequisites
//!
//! ### File `.cargo/config`
//!
//! ```toml
//! [target.thumbv7em-none-eabihf]
//! runner = "arm-none-eabi-gdb"
//! rustflags = [
//!   "-C", "link-arg=-Tl0dable.x",
//!   "-C", "linker=lld",
//!   "-Z", "linker-flavor=ld.lld",
//! ]
//!
//! [build]
//! target = "thumbv7em-none-eabihf"
//! ```
//!
//! ### Enable Link-Time Optimization in `Cargo.toml`
//!
//! Optional, but strongly recommended for code size:
//!
//! ```toml
//! [profile.release]
//! lto = true
//! ```
//! ### Build script
//!
//! ```shell
//! cargo build --release
//! arm-none-eabi-objcopy -O binary --strip-unneeded target/thumbv7m-none-eabi/release/demo demo.c1d
//! ```
#![no_std]
//#![warn(missing_docs)]

extern crate lpc43xx as target;
extern crate lpc43xx_hal as hal;
extern crate r0;
extern crate embedded_hal;
extern crate vga_framebuffer;
extern crate fatfs;
extern crate core_io as io;

/// firmware startup
pub mod startup;
/// Blinking LEDs
mod led;
pub use led::*;
/// LCD
pub mod lcd;
pub use lcd::lcd;
/// Joystick
pub mod input;
pub use input::Input;
/// SPI flash storage
pub mod flash;
pub mod flash_io;

use target::{CCU1, RGU, GPIO_PORT};
use hal::gpio;
use hal::gpio::GpioExt;

pub fn gpio(ccu1: &mut CCU1, rgu: &mut RGU, gpio_port: GPIO_PORT) -> gpio::Parts {

    gpio_port.split(ccu1, rgu)
}

use target::SPIFI;
use hal::spifi::FlashInterface;
use flash::Flash;

pub fn flash(spifi: SPIFI) -> Flash {
    let iface = FlashInterface::new(spifi);
    Flash::new(iface)
}

use flash_io::FlashIO;
use fatfs::{FileSystem, FsOptions};

pub fn filesystem(flash_io: &mut FlashIO) -> FileSystem {
    let opts = FsOptions::new();
    FileSystem::new(flash_io, opts).unwrap()
}
