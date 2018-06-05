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
use rad1o::{LED1, LED2, LED3, LED4, Input, gpio, lcd, lcd::RGB12, lcd::TextConsole, lcd::Backlight};

entry!(main);

fn main() {
    let mut stolen = false;
    let mut p = Peripherals::take()
        .unwrap_or_else(|| {
            stolen = true;
            unsafe { Peripherals::steal() }
        });
    let gpio = gpio(&mut p.CCU1, &mut p.RGU, p.GPIO_PORT);
    let mut led1 = LED1::setup(gpio.p2_1);
    let mut led2 = LED2::setup(gpio.p2_2);
    let mut led3 = LED3::setup(gpio.p2_8);
    let mut led4 = LED4::setup(gpio.p5_26);
    led1.on();
    led4.set(stolen);

    led2.on();
    let mut display = lcd(
        &mut p.CGU, &mut p.CCU1,
        &mut p.SSP1,
        gpio.p4_12.into_output()
    );
    led3.on();
    let mut console = TextConsole::new();
    led1.off();

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

    let mut backlight = Backlight::setup(gpio.p0_8);
    let input = Input::setup(
        gpio.p5_20,
        gpio.p5_21,
        gpio.p5_22,
        gpio.p5_23,
        gpio.p5_24
    );

    let mut t = 0usize;
    loop {
        backlight.set(t % 100 < 50);
        led3.on();

        let mut output = console.output(&mut display);
        writeln!(output, "t={}", t).unwrap();
        if input.down() {
            writeln!(output, "down").unwrap();
        }
        if input.up() {
            writeln!(output, "up").unwrap();
        }
        if input.left() {
            writeln!(output, "left").unwrap();
        }
        if input.right() {
            writeln!(output, "right").unwrap();
        }
        if input.enter() {
            writeln!(output, "enter").unwrap();
        }

        led3.off();
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
