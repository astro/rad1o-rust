#![feature(core_intrinsics)]
#![feature(const_fn)]

#![no_main]
#![no_std]

extern crate rad1o;

use core::intrinsics::powif32;

use rad1o::led::*;
use rad1o::display;
use rad1o::color::*;
use rad1o::input::*;

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
const COLOR_DURATION: usize = 8;

fn powi(a: f32, x: i32) -> f32 {
    unsafe {
        powif32(a, x)
    }
}

#[no_mangle]
#[export_name = "ram"]
pub fn ram() {
    let lcd = display();
    
    let mut i = 0;
    let mut bx = 0x00010000;
    let mut by = 0x00010000;
    let mut dx = 0i32;
    let mut dy = 0i32;
    while ! ENTER.get() {
        if UP.get() {
            dy = -1;
        }
        else if DOWN.get() {
            dy = 1;
        }
        if LEFT.get() {
            dx = -1;
        } else if RIGHT.get() {
            dx = 1;
        }
        bx = (bx as i32 + dx) as usize;
        by = (by as i32 + dy) as usize;

        lcd.display(|x, y| {
            let logo_alpha = rust_logo::get_pixel(x as usize, y as usize);
            let bg = if (((bx + x) / SQUARE_SIZE) + ((by + y) / SQUARE_SIZE)) % 2 == 0 {
                let bg1 = COLORS[(i as usize) / COLOR_DURATION % COLORS.len()];
                let bg2 = COLORS[((i / COLOR_DURATION) + 1 as usize) % COLORS.len()];
                bg2.blend(((i % COLOR_DURATION) * 255 / COLOR_DURATION) as u8, &bg1)
            } else {
                let bg1 = COLORS[((i as usize) / COLOR_DURATION + 3) % COLORS.len()];
                let bg2 = COLORS[((i / COLOR_DURATION) + 4 as usize) % COLORS.len()];
                bg2.blend(((i % COLOR_DURATION) * 255 / COLOR_DURATION) as u8, &bg1)
            };

            let ny = 255f32 * (1f32 - powi((y as f32) / (lcd.height() as f32), 2));
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

        i += 1;
    }

    while ENTER.get() {}
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ! {
loop {}
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr1() -> ! {
loop {}
}
