#![feature(core_intrinsics)]
#![no_main]
#![no_std]

extern crate rad1o;

use core::intrinsics::{powif32, sqrtf32};
use rad1o::input::*;
use rad1o::display;
use rad1o::color::*;


fn powi(a: f32, x: i32) -> f32 {
    unsafe {
        powif32(a, x)
    }
}

fn sqrt(x: f32) -> f32 {
    unsafe {
        sqrtf32(x)
    }
}



#[no_mangle]
#[export_name = "ram"]
pub fn ram() {
    let lcd = display();
    
    let max_dist = sqrt(powi((lcd.width() / 2) as f32, 2) + powi((lcd.height() / 2) as f32, 2));
    while ! ENTER.get() {
        lcd.display(|x, y| {
            let x1 = (x as f32) - (lcd.width() / 2) as f32;
            let y1 = (y as f32) - (lcd.height() / 2) as f32;
            let dist = sqrt(powi(x1, 2) + powi(y1, 2)) / max_dist;
            rgb((255f32 * dist) as u8, 0, 0)
        });
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
