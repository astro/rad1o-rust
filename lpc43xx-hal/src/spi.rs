use target::{CGU, CCU1, SSP0, SSP1, ssp0};
//TODO: use hal::spi::{FullDuplex, Mode, Phase, Polarity};
use hal::blocking::spi::{Transfer, Write};

pub trait SspExt {
    type Target;

    fn setup(self, &mut CGU, &mut CCU1) -> Self::Target;
}

impl<'a> SspExt for &'a mut SSP0 {
    type Target = SspSpi<'a>;

    fn setup(self, cgu: &mut CGU, ccu: &mut CCU1) -> Self::Target {
        /* use PLL1 as clock source for SSP0 */
        cgu.base_ssp0_clk.write(|w| {
            w
                .autoblock().enabled()
                .clk_sel().pll1()
        });
        /* Enable SSP0 Clock */
        ccu.clk_m4_ssp0_cfg.write(|w| {
            w.run().enabled()
        });

        SspSpi { ssp: self }
    }
}

impl<'a> SspExt for &'a mut SSP1 {
    type Target = SspSpi<'a>;

    fn setup(self, cgu: &mut CGU, ccu: &mut CCU1) -> Self::Target {
        /* use PLL1 as clock source for SSP1 */
        cgu.base_ssp1_clk.write(|w| {
            w
                .autoblock().enabled()
                .clk_sel().pll1()
        });
        /* Enable SSP1 Clock */
        ccu.clk_m4_ssp1_cfg.write(|w| {
            w.run().enabled()
        });

        SspSpi { ssp: self }
    }
}

pub struct SspSpi<'a> {
    ssp: &'a ssp0::RegisterBlock,
}

impl<'a> SspSpi<'a> {
    pub fn configure(self) -> SspSpiDev<'a> {
        let ssp = self.ssp;

        /*
         * The LCD requires 9-Bit frames
         * Freq = PCLK / (CPSDVSR * [SCR+1])
	 * We want 120ns / bit -> 8.3 MHz.
	 * SPI1 BASE CLOCK is expected to be 204 MHz.
         * 204 MHz / ( 12 * (1 + 1)) = 8.5 MHz
         *
         * Set CPSDVSR = 12
         */
        let serial_clock_rate = 1u8;
        let clock_prescale_rate = 12u8;

        /* Disable SSP before to configure it */
        ssp.cr1.write(|w| w.sse().disabled());

        /* Configure SSP */
        ssp.cpsr.write(|w| unsafe {
            w.cpsdvsr().bits(clock_prescale_rate)
        });
        ssp.cr0.write(|w| unsafe {
            w
                .dss()._9_bit_transfer()
                .frf().spi()
                .cpol().bus_low()
                .cpha().first_clock()
                .scr().bits(serial_clock_rate)
        });
        
        /* Enable SSP */
        ssp.cr1.write(|w| {
            w
                .lbm().normal()
                .sse().enabled()
                .ms().master()
                .sod().clear_bit()
        });

        SspSpiDev { ssp }
    }
}

pub struct SspSpiDev<'a> {
    ssp: &'a ssp0::RegisterBlock,
}

impl<'a> SspSpiDev<'a> {
    fn transfer_data(&self, data: u16) -> u16 {
        let ssp = self.ssp;
        /* Wait until Tx-fifo-Not-Full */
        while ssp.sr.read().tnf().bit_is_clear() {}

        ssp.dr.write(|w| unsafe { w.data().bits(data) });

        /* Wait for not busy, since we're controlling CS# of
         * devices manually and need to wait for the data to
         * be sent. It may also be important to wait here
         * in case we're configuring devices via SPI and also
         * with GPIO control -- we need to know when SPI
         * commands are effective before altering a device's
         * state with GPIO. I'm thinking the MAX2837, for
         * example...
         */
        while ssp.sr.read().bsy().bit_is_set() {}

        /* Wait Until Data Received (Rx FIFO not Empty) */
        while ssp.sr.read().rne().bit_is_clear() {}

        ssp.dr.read().data().bits()
    }
}

impl<'a> Write<u16> for SspSpiDev<'a> {
    type Error = ();

    fn write(&mut self, words: &[u16]) -> Result<(), Self::Error> {
        for w in words {
            self.transfer_data(*w);
        }
        Ok(())
    }
}

impl<'a> Transfer<u16> for SspSpiDev<'a> {
    type Error = ();

    fn transfer<'w>(&mut self, words: &'w mut [u16]) -> Result<&'w [u16], Self::Error> {
        for w in words.iter_mut() {
            *w = self.transfer_data(*w);
        }
        Ok(words)
    }
}
