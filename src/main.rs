#![feature(lang_items)]
#![feature(asm)]

#![no_main]
#![no_std]

extern crate rlibc;
extern crate cortex_m;
extern crate volatile_register;

#[cfg(target_arch = "arm")]
pub mod lang_items;

mod peripheral;
use peripheral::gpio_port;

fn delay_nop(duration: u32) {
    for _ in 0..duration {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

fn toggle_led(led: u8) {
    match led {
        1 => 
            gpio_port().not[2].write(|notw| notw.notp1(true)),
        2 => 
            gpio_port().not[2].write(|notw| notw.notp2(true)),
        3 => 
            gpio_port().not[2].write(|notw| notw.notp8(true)),
        4 => 
            gpio_port().not[5].write(|notw| notw.notp26(true)),
        _ => (),
    }
}

#[no_mangle]
#[export_name = "ram"]
pub fn ram() {
    let mut i = 0;
    loop {
        delay_nop(100_000);

        toggle_led(i + 1);
        i = (i + 1) % 4;
    }
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ! {
loop {}
}

