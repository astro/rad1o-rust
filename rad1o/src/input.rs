use lpc43xx::peripheral::gpio_port;

pub trait Key {
    fn get(&self) -> bool;
}

macro_rules! key {
    ($name:ident, $constname:ident, $num:expr, $port:ident) => (
        pub struct $name {
        }

        impl Key for $name {
            fn get(&self) -> bool {
                gpio_port().pin[$num].read().$port()
            }
        }

        pub static $constname: $name = $name {};
    )
}
key!(Down, DOWN, 5, port20);
key!(Up, UP, 5, port21);
key!(Left, LEFT, 5, port22);
key!(Right, RIGHT, 5, port23);
key!(Enter, ENTER, 5, port24);
