use embedded_hal::digital::OutputPin;
use hal::gpio::{Output, P0_8};

type BacklightPin<M> = P0_8<M>;

pub struct Backlight {
    pin: BacklightPin<Output>
}

impl Backlight {
    pub fn setup<M>(pin: BacklightPin<M>) -> Self {
        Backlight {
            pin: pin.into_output()
        }
    }

    pub fn off(&mut self) {
        self.pin.set_low()
    }

    pub fn on(&mut self) {
        self.pin.set_high()
    }

    pub fn set(&mut self, set: bool) {
        if set {
            self.on()
        } else {
            self.off()
        }
    }
}
