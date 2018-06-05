use target::{CGU, CCU1, SSP1};
use hal::gpio;
use hal::spi::{SspExt, SspSpiDev};

mod pcf8833;
pub use self::pcf8833::{PCF8833, Selected,
                        ColorFormat, RGB12};
mod console;
pub use self::console::{TextConsole, ConsoleOutput};

type CsPin = gpio::P4_12<gpio::Output>;

pub fn lcd<'a>(cgu: &mut CGU, ccu: &mut CCU1, ssp: &'a mut SSP1, cs: CsPin) -> PCF8833<CsPin, SspSpiDev<'a>> {
    let spi = ssp.setup(cgu, ccu)
        .configure();
    PCF8833::new(cs, spi)
}
