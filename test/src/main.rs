#![no_main]
#![no_std]

extern crate panic_abort;
#[macro_use(entry)]
extern crate rad1o_l0dable as rad1o;

use core::fmt::Write;
use rad1o::{IdentifyLED, lcd, input, power, time};


entry!(main);

fn main() {
    writeln!(lcd::Stdout, "Battery is{} charging", if power::get_charging() { "" } else { " not" }).unwrap();
    writeln!(lcd::Stdout, "Voltage={} mV", power::get_voltage().as_millivolts()).unwrap();

    let mut n = 0usize;
    // let mut last_tick = time::get_ticks();
    loop {
        let input = input::get_input();
        if input.left() {
            writeln!(lcd::Stdout, "Left").unwrap();
        }
        if input.right() {
            writeln!(lcd::Stdout, "Right").unwrap();
        }
        if input.up() {
            writeln!(lcd::Stdout, "Up").unwrap();
        }
        if input.down() {
            writeln!(lcd::Stdout, "Down").unwrap();
        }
        if input.enter() {
            writeln!(lcd::Stdout, "Enter").unwrap();
            break
        }

        writeln!(lcd::Stdout, "n={}", n).unwrap();
        for led in 0usize..4 {
            led.set_led(led == n & 3);
        }
        n += 1;

        writeln!(lcd::Stdout, "time={}", time::get_seconds()).unwrap();

        // while time::get_ticks() < last_tick + 100 {}
        // last_tick = time::get_ticks();
    }
}
