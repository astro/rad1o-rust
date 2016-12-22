use lpc43xx::peripheral::gpio_port;

pub trait LED {
    fn toggle(&self);
    fn set(&self, value: bool);
}

pub fn led(n: usize) -> &'static LED {
    match n {
        1 => &LED1,
        2 => &LED2,
        3 => &LED3,
        4 => &LED4,
        _ => unreachable!(),
    }
}

pub struct LED1 {
}
impl LED for LED1 {
    fn toggle(&self) {
        gpio_port().not[2].write(|notw| notw.notp1(true));
    }
    fn set(&self, value: bool) {
        if value {
            gpio_port().set[2].write(|setw| setw.setp1(true));
        } else {
            gpio_port().clr[2].write(|clrw| clrw.clrp01(true));
        }
    }
}
pub static LED1: LED1 = LED1 {};

pub struct LED2 {
}
impl LED for LED2 {
    fn toggle(&self) {
        gpio_port().not[2].write(|notw| notw.notp2(true));
    }
    fn set(&self, value: bool) {
        if value {
            gpio_port().set[2].write(|setw| setw.setp2(true));
        } else {
            gpio_port().clr[2].write(|clrw| clrw.clrp02(true));
        }
    }
}
pub static LED2: LED2 = LED2 {};

pub struct LED3 {
}
impl LED for LED3 {
    fn toggle(&self) {
        gpio_port().not[2].write(|notw| notw.notp8(true));
    }
    fn set(&self, value: bool) {
        if value {
            gpio_port().set[2].write(|setw| setw.setp8(true));
        } else {
            gpio_port().clr[2].write(|clrw| clrw.clrp08(true));
        }
    }
}
pub static LED3: LED3 = LED3 {};

pub struct LED4 {
}
impl LED for LED4 {
    fn toggle(&self) {
        gpio_port().not[5].write(|notw| notw.notp26(true));
    }
    fn set(&self, value: bool) {
        if value {
            gpio_port().set[5].write(|setw| setw.setp26(true));
        } else {
            gpio_port().clr[5].write(|clrw| clrw.clrp026(true));
        }
    }
}
pub static LED4: LED4 = LED4 {};
