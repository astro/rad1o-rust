use core::mem::replace;
use embedded_hal::digital::OutputPin;
use embedded_hal::blocking::spi::{Transfer, Write};

pub const RESX: usize = 130;
pub const RESY: usize = 130;
const SSP_DATA_9BITS: u8  = 0x8;
const SSP_FRAME_SPI: u8 = 0x00;
const TYPE_CMD: u8 = 0;
const TYPE_DATA: u8 = 1;
const CMD_COLMOD: u8 = 0x3A;


pub struct PCF8833<CS: OutputPin, SPI: Transfer<u16>> {
    cs: CS,
    spi: SPI,
}

// http://www.elecfreaks.com/store/download/datasheet/shield/PCF8833_1.pdf
impl<CS: OutputPin, SPI: Transfer<u16>> PCF8833<CS, SPI> {
    pub fn new(cs: CS, spi: SPI) -> Self {
        PCF8833 {
            cs, spi,
        }
    }

    /// Unwrap so that the SPI peripheral can be used by other drivers
    pub fn into_inner(self) -> (CS, SPI) {
        (self.cs, self.spi)
    }

    pub fn select<'a>(&'a mut self) -> Selected<'a, CS, SPI> {
        Selected::new(self)
    }

    pub fn width(&self) -> usize {
        RESX
    }

    pub fn height(&self) -> usize {
        RESY
    }
}

pub struct Selected<'a, CS: 'a + OutputPin, SPI: 'a + Transfer<u16>> {
    pcf: &'a mut PCF8833<CS, SPI>,
}

impl<'a, CS: OutputPin, SPI: Transfer<u16>> Selected<'a, CS, SPI> {
    pub fn new(pcf: &'a mut PCF8833<CS, SPI>) -> Self {
        pcf.cs.set_low();
        Selected { pcf }
    }

    // TODO: return Result<(), SPI::Error>
    fn write(&mut self, cd: u8, data: u8) {
        let mut buf = [((cd as u16) << 8) | (data as u16)];
        self.pcf.spi.transfer(&mut buf);
    }


    pub fn display<F, C>(&mut self, f: F)
        where C: ColorFormat,
              F: Fn(usize, usize) -> C
    {
        let width = self.pcf.width();
        let height = self.pcf.height();

        /* set color mode */
        self.write(TYPE_CMD, CMD_COLMOD);
        self.write(TYPE_DATA, C::colmod());

        // memory write (RAMWR)
        self.write(TYPE_CMD, 0x2C);
        let mut write_data = move |byte| self.write(TYPE_DATA, byte);
        let mut buf = C::Buf::new();
        for y in 0..height {
            for x in 0..width {
                buf.push(f(x, y));

                if buf.is_full() {
                    C::write(buf, &mut write_data);

                    // Reset buf
                    buf = C::Buf::new();
                }
            }
        }
    }
}

impl<'a, CS: OutputPin, SPI: Transfer<u16>> Drop for Selected<'a, CS, SPI> {
    fn drop(&mut self) {
        self.pcf.cs.set_high();
    }
}

pub trait ColorBuf<C> {
    fn new() -> Self;
    fn push(&mut self, color: C);
    fn is_full(&self) -> bool;
}

pub enum TwoColorsBuf<C> {
    Empty,
    One(C),
    Two(C, C),
}

impl<C> ColorBuf<C> for TwoColorsBuf<C> {
    fn new() -> Self {
        TwoColorsBuf::Empty
    }

    fn push(&mut self, color: C) {
        let this = replace(self, TwoColorsBuf::Empty);
        match this {
            TwoColorsBuf::Empty => {
                replace(self, TwoColorsBuf::One(color));
            }
            TwoColorsBuf::One(prev) => {
                replace(self, TwoColorsBuf::Two(prev, color));
            }
            TwoColorsBuf::Two(_, _) =>
                unreachable!()
        }
    }

    fn is_full(&self) -> bool {
        match *self {
            TwoColorsBuf::Two(_, _) => true,
            _ =>  false,
        }
    }
}


pub trait ColorFormat: Sized {
    /// How many bytes to write at once
    type Buf: ColorBuf<Self>;
    /// COLMOD Register value
    fn colmod() -> u8;
    /// Write color data
    fn write<F>(colors: Self::Buf, f: F)
        where F: FnMut(u8),
              Self: Sized;
}

#[derive(Clone)]
pub struct RGB12 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorFormat for RGB12 {
    type Buf = TwoColorsBuf<Self>;

    fn colmod() -> u8 {
        3
    }

    fn write<F>(colors: Self::Buf, mut f: F)
        where F: FnMut(u8),
              Self: Sized
    {
        let (c1, c2) = match colors {
            TwoColorsBuf::Two(c1, c2) => (c1, c2),
            _ => unreachable!(),
        };

	f((c1.r & 0xF0) | (c1.g >> 4));
	f((c1.b & 0xF0) | (c2.r >> 4));
	f((c2.g & 0xF0) | (c2.b >> 4));
    }
}
