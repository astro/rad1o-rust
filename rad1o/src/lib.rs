#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]

extern crate lpc43xx;

#[cfg(target_arch = "arm")]
pub mod lang_items;

pub mod led;
pub mod display;
pub mod color;

pub fn delay_nop(duration: u32) {
    for _ in 0..duration {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}
