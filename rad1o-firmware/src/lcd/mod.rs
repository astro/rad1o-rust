use target::Peripherals;
use hal::gpio::GPIO;
use hal::spi::SspSpi;

mod pcf8833;
pub use self::pcf8833::{PCF8833, Selected,
                        ColorFormat, RGB12};
mod console;
pub use self::console::{TextConsole, ConsoleOutput};

pub fn lcd<'a>(p: &'a Peripherals) -> PCF8833<GPIO<'a>, SspSpi<'a>> {
    let cs = GPIO::new(p, 4, 12);
    let spi = SspSpi::new(p);
    PCF8833::new(cs, spi)
}
