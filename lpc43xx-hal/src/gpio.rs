use target::{Peripherals, GPIO_PORT};
use hal::digital::OutputPin;

pub struct GPIO<'a> {
    gpio_port: &'a GPIO_PORT,
    port: u8,
    pin: u8,
}

impl<'a> GPIO<'a> {
    pub fn new(p: &'a Peripherals, port: u8, pin: u8) -> Self {
        GPIO {
            gpio_port: &p.GPIO_PORT,
            port, pin,
        }
    }
    
    #[inline]
    fn port(&self) -> usize {
        self.port.into()
    }

    #[inline]
    fn mask(&self) -> u32 {
        1 << self.pin
    }

    pub fn clear(&self) {
        self.gpio_port.clr[self.port()].write(|w| unsafe {
            w.bits(self.mask())
        });
    }

    pub fn set(&self) {
        self.gpio_port.clr[self.port()].write(|w| unsafe {
            w.bits(self.mask())
        });
    }
}

impl<'a> OutputPin for GPIO<'a> {
    fn set_low(&mut self) {
        self.clear();
    }
    fn set_high(&mut self) {
        self.set();
    }
}
