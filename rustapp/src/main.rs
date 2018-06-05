#![no_main]
#![no_std]

extern crate panic_abort;
#[macro_use(exception)]
extern crate cortex_m_rt;
extern crate lpc43xx;
#[macro_use(entry)]
extern crate rad1o_firmware as rad1o;

use core::fmt::Write;
use lpc43xx::Peripherals;
use rad1o::{IdentifyLED, gpio, lcd, lcd::RGB12, lcd::TextConsole};

entry!(main);

fn main() {
    let mut p = Peripherals::take()
        .unwrap_or_else(|| {
            3usize.set_led(true);
            unsafe { Peripherals::steal() }
        });
    for led in 0usize..4 {
        led.set_led(false);
    }
    0usize.set_led(true);

    let gpio = gpio(&mut p.CCU1, &mut p.RGU, p.GPIO_PORT);
    1usize.set_led(true);
    let mut display = lcd(
        &mut p.CGU, &mut p.CCU1,
        &mut p.SSP1,
        gpio.p4_12.into_output()
    );
    2usize.set_led(true);
    let mut console = TextConsole::new();
    0usize.set_led(false);

    // extern "C" {
    //     static mut __sbss: u32;
    //     static mut __ebss: u32;
    // }

    // unsafe {
    //     writeln!(console.output(&mut display), "BSS: {:08X}..{:08X}", (&mut __sbss) as *mut _ as usize, (&mut __ebss ) as *mut _ as usize).unwrap();
    // }
    // let mut p: *mut u32 = unsafe { &mut __sbss };
    // while p < unsafe { &mut __ebss } {
    //     let v = unsafe { *p };
    //     writeln!(console.output(&mut display), "{:08X}:\n {:08X}", p as usize, v);
        
    //     p = unsafe{ p.offset(1) };
    // }

    let mut t = 0usize;
    loop {
        writeln!(console.output(&mut display), "Hello").unwrap();
        2usize.set_led(true);
        writeln!(console.output(&mut display), "t={}", t).unwrap();

        2usize.set_led(false);
        display.select().display(|x, y| {
            RGB12 {
                r: (x ^ y ^ t) as u8,
                g: (y ^ t) as u8,
                b: (y ^ t) as u8,
            }
        });
        t += 1;
    }
}


exception!(*, default_handler);
fn default_handler(_interrupt: i16) {
}

exception!(HardFault, hard_fault_handler);
fn hard_fault_handler(_: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {}
}
