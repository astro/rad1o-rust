#![no_main]
#![no_std]

extern crate panic_abort;
#[macro_use(exception)]
extern crate cortex_m_rt;
extern crate lpc43xx;
#[macro_use(entry)]
extern crate rad1o_firmware as rad1o;

use lpc43xx::Peripherals;
use rad1o::{IdentifyLED, lcd, lcd::RGB12};


entry!(main);

fn main() {
    0usize.set_led(true);

    let p = Peripherals::take().unwrap();
    1usize.set_led(true);
    let mut display = lcd(&p);
    2usize.set_led(true);
    loop {
        4usize.set_led(true);
        display.select().display(|x, y| RGB12 {
            r: x as u8,
            g: y as u8,
            b: (x ^ y) as u8,
        });
        4usize.set_led(false);
        3usize.set_led(true);
    }
}


exception!(*, default_handler);
fn default_handler(_interrupt: i16) {
}

exception!(HardFault, hard_fault_handler);
fn hard_fault_handler(_: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {}
}
