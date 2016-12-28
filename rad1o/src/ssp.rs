use lpc43xx::peripheral::{gpio_port, ssp1, cgu, ccu1};
use display::SelectsDisplay;

pub trait SSP {
    fn init(&self, data_size: u8, frame_format: u8, cpol_format: bool, cpha_format: bool, serial_clock_rate: u8, clk_prescale: u8, loopback: bool, ms_mode: bool, slave_out: bool);
    fn wait_until_not_busy(&self);
    fn transfer(&self, data: u16) -> u16;
}


const CGU_SRC_PLL1: u8 = 0x09;

pub struct SSP1 {
}

impl SSP1 {
    pub fn new() -> Self {
        SSP1 {}
    }
}

impl SelectsDisplay for SSP1 {
    fn display_cs_write(&self, selected: bool) {
        if selected {
            gpio_port().clr[4].write(|clr| clr.clrp012(true));
        } else {
            gpio_port().set[4].write(|set| set.setp12(true));
        }
    }
}

impl SSP for SSP1 {
    fn init(&self, data_size: u8, frame_format: u8, cpol_format: bool, cpha_format: bool, serial_clock_rate: u8, clk_prescale: u8, loopback: bool, ms_mode: bool, slave_out: bool) {

        /* use PLL1 as clock source for SSP1 */
        cgu().base_ssp1_clk.write(|clk| clk
                                  .autoblock(true)
                                  .clk_sel(CGU_SRC_PLL1));
        /* Enable SSP1 Clock */
        ccu1().clk_m4_ssp1_cfg.write(|cfg| cfg
                                     .run(true));

        /* Disable SSP before to configure it */
        ssp1().cr1.write(|cr1| cr1.sse(false));

        /* Configure SSP */
        ssp1().cpsr.write(|cpsr| cpsr
                          .cpsdvsr(clk_prescale));
        ssp1().cr0.write(|cr0| cr0
                         .dss(data_size)
                         .frf(frame_format)
                         .cpol(cpol_format)
                         .cpha(cpha_format)
                         .scr(serial_clock_rate));
        
        /* Enable SSP */
        ssp1().cr1.write(|cr1| cr1
                         .lbm(loopback)
                         .sse(true)
                         .ms(ms_mode)
                         .sod(! slave_out));
    }

    fn wait_until_not_busy(&self) {
        while ssp1().sr.read().bsy() {}
    }

    fn transfer(&self, data: u16) -> u16 {
        /* Wait until Tx-fifo-Not-Full */
        while ! ssp1().sr.read().tnf() {}

        ssp1().dr.write(|dr| dr.data(data));

        /* Wait for not busy, since we're controlling CS# of
         * devices manually and need to wait for the data to
         * be sent. It may also be important to wait here
         * in case we're configuring devices via SPI and also
         * with GPIO control -- we need to know when SPI
         * commands are effective before altering a device's
         * state with GPIO. I'm thinking the MAX2837, for
         * example...
         */
        self.wait_until_not_busy();

        /* Wait Until Data Received (Rx FIFO not Empty) */
        while ! ssp1().sr.read().rne() {}

        ssp1().dr.read().data()
    }
}
