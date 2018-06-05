use embedded_hal::digital::OutputPin;
use hal::gpio::{Output, P2_1, P2_2, P2_8, P5_26};

macro_rules! def_led {
    ($LED:ident, $PIN:tt) => (
        pub struct $LED {
            pin: $PIN<Output>
        }

        impl $LED {
            pub fn setup<M>(pin: $PIN<M>) -> Self {
                $LED {
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
    )
}

def_led!(LED1, P2_1);
def_led!(LED2, P2_2);
def_led!(LED3, P2_8);
def_led!(LED4, P5_26);
