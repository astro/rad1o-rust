use lpc43xx::peripheral::{gpio_port, ssp1, cgu, ccu1};
use color::Color;
use ssp::SSP;

pub const RESX: usize = 130;
pub const RESY: usize = 130;
const SSP_DATA_9BITS: u8  = 0x8;
const SSP_FRAME_SPI: u8 = 0x00;
const TYPE_CMD: u8 = 0;
const TYPE_DATA: u8 = 1;


pub struct PCF8833<S: SSP> {
    ssp: S,
}

// http://www.elecfreaks.com/store/download/datasheet/shield/PCF8833_1.pdf
impl<S: SSP> PCF8833<S> {
    pub fn new(ssp: S) -> Self {
        PCF8833 {
            ssp: ssp,
        }
    }
    
    fn lcd_cs_write(value: bool) {
        if value {
            gpio_port().set[4].write(|set| set.setp12(true));
        } else {
            gpio_port().clr[4].write(|clr| clr.clrp012(true));
        }
    }

    fn select(&self) {
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

        self.ssp.init(
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

        Self::lcd_cs_write(false);
    }

    fn deselect(&self) {
        Self::lcd_cs_write(true);
    }

    fn write(&self, cd: u8, data: u8) {
        let frame = ((cd as u16) << 8) | (data as u16);
        self.ssp.transfer(frame);
    }

    pub fn width(&self) -> usize {
        RESX
    }

    pub fn height(&self) -> usize {
        RESY
    }

    pub fn display<F>(&self, f: F)
        where F: Fn(usize, usize) -> Color {

        self.select();

        /* set to 12 bpp mode */
        self.write(TYPE_CMD, 0x3a);
        self.write(TYPE_DATA, 3);

        // memory write (RAMWR)
        self.write(TYPE_CMD, 0x2C);

        for y in 0..self.height() {
            for x in 0..(self.width() / 2) {
                let x1 = x * 2;
                let c1 = f(x1, y);
            
                let x2 = x * 2 + 1;
                let c2 = f(x2, y);
            
	        self.write(TYPE_DATA, ((c1.r & 0xF0) | (c1.g >> 4)) as u8);
	        self.write(TYPE_DATA, ((c1.b & 0xF0) | (c2.r >> 4)) as u8);
	        self.write(TYPE_DATA, ((c2.g & 0xF0) | (c2.b >> 4)) as u8);
            }
        }

        self.deselect();
    }
}
