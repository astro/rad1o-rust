#![feature(lang_items)]
#![feature(asm)]
#![feature(concat_idents)]
#![feature(const_fn)]

#![no_main]
#![no_std]

// extern crate rlibc;
// extern crate cortex_m;
extern crate volatile_register;

#[cfg(target_arch = "arm")]
pub mod lang_items;

#[allow(dead_code)]
mod peripheral;

mod led;
use led::*;

mod display;
use display::*;
mod color;
use color::*;
mod rust_logo;

fn delay_nop(duration: u32) {
    for _ in 0..duration {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

static COLORS: &'static [Color] = &[
    rgb(255, 127, 127),
    rgb(255, 255, 127),
    rgb(127, 255, 127),
    rgb(127, 255, 255),
    rgb(127, 127, 255),
    rgb(255, 127, 255),
];

const SQUARE_SIZE: usize = 16;
const COLOR_DURATION: usize = 32;

#[no_mangle]
#[export_name = "ram"]
pub fn ram() {
    let mut i = 0;
    loop {
        // delay_nop(100_000);

        LED1.set(i % 2 == 0);
        i += 1;

        lcd_display(|x, y| {
            let logo_alpha = rust_logo::get_pixel(x as usize, y as usize);
            let bg = if (((x + i / 2) / SQUARE_SIZE) + ((y + i) / SQUARE_SIZE)) % 2 == 0 {
                let bg1 = COLORS[(i as usize) / COLOR_DURATION % COLORS.len()];
                let bg2 = COLORS[(i + 1 as usize) / COLOR_DURATION % COLORS.len()];
                bg2.blend(((i % COLOR_DURATION) * 255 / COLOR_DURATION) as u8, &bg1)
            } else {
                rgb(0, 0, 128)
            };

            let ny = 255 - (y as usize) * 255 / RESY;
            if logo_alpha == 0 {
                bg
            } else {
                let alpha = if ny < logo_alpha as usize {
                    ny as u8
                } else {
                    logo_alpha
                };
                rgb(0, 0, 0).blend(alpha, &bg)
            }
        });
    }
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ! {
loop {}
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr1() -> ! {
loop {}
}

