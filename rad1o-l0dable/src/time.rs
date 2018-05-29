use table::table;
use hal::blocking::delay::DelayMs;

/// Get current time in seconds
pub fn get_seconds() -> u32 {
    get_time() / 1000
}

/// Get current time in milliiseconds
pub fn get_time() -> u32 {
    unsafe { *table()._timectr }
}

/// Delay execution for `ms` milliseconds
pub fn delay_ms(ms: u32) {
    (table().delayms)(ms)
}

/// Use this as an **embedded-hal** `DelayMs` instance
pub struct Delay;
impl<T: Into<u32>> DelayMs<T> for Delay {
    fn delay_ms(&mut self, ms: T) {
        self::delay_ms(ms.into())
    }
}
