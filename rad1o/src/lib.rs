#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]

extern crate lpc43xx;

#[cfg(target_arch = "arm")]
pub mod lang_items;

pub mod led;
pub mod input;
mod ssp;
use ssp::SSP1;
mod display;
use display::PCF8833;
pub mod color;

pub fn delay_nop(duration: u32) {
    for _ in 0..duration {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

pub fn display() -> display::PCF8833<ssp::SSP1> {
    PCF8833::new(SSP1::new())
}
