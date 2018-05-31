#![no_main]
#![no_std]

extern crate panic_abort;
#[macro_use(exception)]
extern crate cortex_m_rt;
extern crate lpc43xx;
#[macro_use(entry)]
extern crate rad1o_firmware as rad1o;

use rad1o::{IdentifyLED};


entry!(main);

fn main() {
    for n in 0usize..4 {
        n.set_led(true);
    }
    loop {}
}


exception!(*, default_handler);
fn default_handler(_interrupt: i16) {
}

exception!(HardFault, hard_fault_handler);
fn hard_fault_handler(_: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {}
}
