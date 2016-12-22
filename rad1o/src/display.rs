use lpc43xx::peripheral::{gpio_port, ssp1, cgu, ccu1};
use color::Color;

pub const RESX: usize = 130;
pub const RESY: usize = 130;
const TYPE_CMD: u8 = 0;
const TYPE_DATA: u8 = 1;
const CGU_SRC_PLL1: u8 = 0x09;
const SSP_DATA_9BITS: u8  = 0x8;
const SSP_FRAME_SPI: u8 = 0x00;

pub fn ssp_init(data_size: u8, frame_format: u8, cpol_format: bool, cpha_format: bool, serial_clock_rate: u8, clk_prescale: u8, loopback: bool, ms_mode: bool, slave_out: bool) {

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

fn ssp_wait_until_not_busy() {
    while ssp1().sr.read().bsy() {}
}

fn ssp_transfer(data: u16) -> u16 {
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
    ssp_wait_until_not_busy();

    /* Wait Until Data Received (Rx FIFO not Empty) */
    while ! ssp1().sr.read().rne() {}

    ssp1().dr.read().data()
}

fn lcd_write(cd: u8, data: u8) {
    let frame = ((cd as u16) << 8) | (data as u16);
    ssp_transfer(frame);
}

fn lcd_cs_write(value: bool) {
    if value {
        gpio_port().set[4].write(|set| set.setp12(true));
    } else {
        gpio_port().clr[4].write(|clr| clr.clrp012(true));
    }
}

fn lcd_select() {
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

    ssp_init(
        SSP_DATA_9BITS,
        SSP_FRAME_SPI,
        false,
        false,
        serial_clock_rate,
        clock_prescale_rate,
        false,
        false,
        true
    );

    lcd_cs_write(false);
}

fn lcd_deselect() {
    lcd_cs_write(true);
}

// http://www.elecfreaks.com/store/download/datasheet/shield/PCF8833_1.pdf
pub fn lcd_display<F>(f: F)
    where F: Fn(usize, usize) -> Color {

    lcd_select();

    /* set to 12 bpp mode */
    lcd_write(TYPE_CMD, 0x3a);
    lcd_write(TYPE_DATA, 3);

    // memory write (RAMWR)
    lcd_write(TYPE_CMD, 0x2C);

    for y in 0..RESY {
        for x in 0..(RESX / 2) {
            let x1 = x * 2;
            let c1 = f(x1, y);
            
            let x2 = x * 2 + 1;
            let c2 = f(x2, y);
            
	    lcd_write(TYPE_DATA, ((c1.r & 0xF0) | (c1.g >> 4)) as u8);
	    lcd_write(TYPE_DATA, ((c1.b & 0xF0) | (c2.r >> 4)) as u8);
	    lcd_write(TYPE_DATA, ((c2.g & 0xF0) | (c2.b >> 4)) as u8);
        }
    }

    lcd_deselect();
}
