use table::table;

/// Wraps millivolts
pub struct Voltage(pub u32);

impl Voltage {
    pub fn as_millivolts(&self) -> u32 {
        self.0
    }

    pub fn as_volts(&self) -> f32 {
        self.0 as f32 / 1000f32
    }
}

/// Get current battery voltage
pub fn get_voltage() -> Voltage {
    Voltage((table().batteryGetVoltage)())
}

/// Is the battery currently being charged?
pub fn get_charging() -> bool {
    (table().batteryCharging)()
}
