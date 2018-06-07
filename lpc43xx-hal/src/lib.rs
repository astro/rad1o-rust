#![no_std]

extern crate lpc43xx as target;
extern crate embedded_hal as hal;
extern crate vcell;

pub mod spi;
pub mod spifi;
pub mod gpio;
