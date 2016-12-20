#![feature(lang_items)]
#![feature(asm)]
#![feature(concat_idents)]
#![feature(const_fn)]

#![no_main]
#![no_std]

extern crate rlibc;
extern crate cortex_m;
extern crate volatile_register;

#[cfg(target_arch = "arm")]
pub mod lang_items;

#[allow(dead_code)]
mod peripheral;

mod led;
use led::*;

mod display;

fn delay_nop(duration: u32) {
    for _ in 0..duration {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

static COLORS: &'static [u8] = &[
    display::rgb(255, 0, 0),
    display::rgb(255, 255, 0),
    display::rgb(0, 255, 0),
    display::rgb(0, 255, 255),
    display::rgb(0, 0, 255),
    display::rgb(255, 0, 255),
];


#[no_mangle]
#[export_name = "ram"]
pub fn ram() {
    let mut i = 0;
    loop {
        // delay_nop(100_000);

        led(1).set(i % 2 == 0);
        i += 1;

        display::lcd_display(|x, y| {
            if (((x + i / 2) / 16) + ((y + i) / 16)) % 2 == 0 {
                COLORS[(i as usize) / 4 % COLORS.len()]
            } else {
                display::rgb(0, 0, 0)
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

