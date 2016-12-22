#![feature(core_intrinsics)]
#![feature(const_fn)]

#![no_main]
#![no_std]

extern crate rad1o;

use core::intrinsics::powif32;

use rad1o::led::*;
use rad1o::display::*;
use rad1o::color::*;

mod rust_logo;

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

fn powi(a: f32, x: i32) -> f32 {
    unsafe {
        powif32(a, x)
    }
}

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

            let ny = 255f32 * (1f32 - powi((y as f32) / (RESY as f32), 2));
            if logo_alpha == 0 {
                bg
            } else {
                let alpha = if ny < logo_alpha as f32 {
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

