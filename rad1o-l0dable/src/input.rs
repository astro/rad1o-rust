use table::table;

/// Wraps the input state
pub struct InputState(pub u8);

macro_rules! def_getter {
    ($name: ident, $mask: expr, $doc: expr) => (
        #[doc=$doc]
        pub fn $name(&self) -> bool {
            self.0 & $mask != 0
        }
    )
}


impl InputState {
    def_getter!(up, 1, "Pushed in the **up** direction?");
    def_getter!(down, 2, "Pushed in the **down** direction?");
    def_getter!(left, 4, "Pushed in the **left** direction?");
    def_getter!(right, 8, "Pushed in the **right** direction?");
    def_getter!(enter, 0x10, "Pressed **in**?");
}

/// Wait for and get next input
pub fn get_input() -> InputState {
    InputState((table().getInput)())
}

/// Obtain input state without delay
pub fn get_input_raw() -> InputState {
    InputState((table().getInputRaw)())
}
