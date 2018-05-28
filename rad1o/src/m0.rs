use lpc43xx::peripheral::{creg, rgu, ccu1};
use led::*;

pub struct M0App {
}

impl M0App {
    fn setup_clock() {
        ccu1().clk_m4_m0app_cfg
            .write(|reg|
                   reg.run(true)
            );
    }
    
    pub fn restart<F: Fn()>(cm0_baseaddr: *const F) {
        Self::setup_clock();

        LED1.set(true);
	/* Set M0 memory mapping to point to start of M0 image */
        creg().m0appmemmap
            .write(|reg|
                   reg.m0appmap(cm0_baseaddr as u32));

        LED2.set(true);
        // Reset
        while ! rgu().reset_active_status1.read().m0app_rst() {
            rgu().reset_ctrl1.write(|reg|
                                    reg.m0app_rst(true));
        }
        LED3.set(true);
        // and clear
        rgu().reset_ctrl1.write(|reg|
                                reg.m0app_rst(false));
        LED2.set(false);
    }
}
