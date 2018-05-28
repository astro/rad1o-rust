#![no_main]
#![no_std]

extern crate rad1o;

use rad1o::delay_nop;
use rad1o::led::*;
use rad1o::input::*;
use rad1o::display;
use rad1o::color::*;
use rad1o::m0::*;

fn m0_main() {
    loop {
        LED4.toggle();
        delay_nop(100_000);
    }
}

#[no_mangle]
#[export_name = "ram"]
pub fn ram() {
    M0App::restart(&m0_main);
    
    while ! ENTER.get() {
        // lcd.display(|x, y| {
        //     let x1 = (x as f32) - (lcd.width() / 2) as f32;
        //     let y1 = (y as f32) - (lcd.height() / 2) as f32;
        //     let dist = sqrt(powi(x1, 2) + powi(y1, 2)) / max_dist;
        //     rgb((255f32 * dist) as u8, 0, 0)
        // });
    }

    while ENTER.get() {
        LED2.toggle();
        delay_nop(10_000);
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
