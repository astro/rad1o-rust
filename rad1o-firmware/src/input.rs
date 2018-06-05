use embedded_hal::digital::InputPin;
use hal::gpio;

type DownPin = gpio::P5_20<gpio::Input>;
type UpPin = gpio::P5_21<gpio::Input>;
type LeftPin = gpio::P5_22<gpio::Input>;
type RightPin = gpio::P5_23<gpio::Input>;
type EnterPin = gpio::P5_24<gpio::Input>;

pub struct Input {
    down: DownPin,
    up: UpPin,
    left: LeftPin,
    right: RightPin,
    enter: EnterPin,
}

impl Input {
    pub fn setup<M1, M2, M3, M4, M5>(
        down: gpio::P5_20<M1>,
        up: gpio::P5_21<M2>,
        left: gpio::P5_22<M3>,
        right: gpio::P5_23<M4>,
        enter: gpio::P5_24<M5>,
    ) -> Self {
        Input {
            down: down.into_input(),
            up: up.into_input(),
            left: left.into_input(),
            right: right.into_input(),
            enter: enter.into_input(),
        }
    }

    pub fn down(&self) -> bool {
        self.down.is_high()
    }

    pub fn up(&self) -> bool {
        self.up.is_high()
    }

    pub fn left(&self) -> bool {
        self.left.is_high()
    }

    pub fn right(&self) -> bool {
        self.right.is_high()
    }

    pub fn enter(&self) -> bool {
        self.enter.is_high()
    }
}
