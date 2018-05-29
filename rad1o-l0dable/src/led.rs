use target::Peripherals;

pub trait IdentifyLED {
    /// Set/clear one of the four LEDs.
    fn set_led(&self, set: bool);
}

impl IdentifyLED for usize {
    fn set_led(&self, set: bool) {
        let (port, pin) = match *self & 3 {
            0 =>
                (2, 1),
            1 =>
                (2, 2),
            2 =>
                (2, 8),
            3 =>
                (5, 26),
            _ =>
                unreachable!(),
        };
        let mask = 1 << pin;
        let p: Peripherals = unsafe { Peripherals::steal() };
        if ! set {
            p.GPIO_PORT.clr[port].write(|w| unsafe {
                w.bits(mask)
            });
        } else {
            p.GPIO_PORT.set[port].write(|w| unsafe {
                w.bits(mask)
            });
        }
    }
}

impl IdentifyLED for u8 {
    fn set_led(&self, set: bool) {
        usize::from(*self).set_led(set)
    }
}

pub enum LED {
    /// Green LED in the lower left corner
    LED1,
    /// Green LED in the lower right corner
    LED2,
    /// Green LED on the left side
    LED3,
    /// Red LED on the top
    LED4,
}

impl IdentifyLED for LED {
    fn set_led(&self, set: bool) {
        match *self {
            LED::LED1 => 0usize.set_led(set),
            LED::LED2 => 1usize.set_led(set),
            LED::LED3 => 2usize.set_led(set),
            LED::LED4 => 3usize.set_led(set),
        }
    }
}
