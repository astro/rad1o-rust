use core::fmt;
use embedded_hal::digital::OutputPin;
use embedded_hal::blocking::spi::{Transfer};
use vga_framebuffer::{Glyph, FONT_WIDTH, FONT_HEIGHT};
use super::{PCF8833, RGB12};

const LINES: usize = 8;
const COLS: usize = 16;

pub struct TextConsole {
    pub buffer: [[char; COLS]; LINES],
    pub line: usize,
    pub col: usize,
}

impl TextConsole {
    pub fn new() -> Self {
        TextConsole {
            buffer: [[' '; COLS]; LINES],
            line: 0,
            col: 0,
        }
    }

    fn scroll(&mut self) {
        for line in 0..(LINES - 1) {
            self.buffer[line] = self.buffer[line + 1];
        }
        self.buffer[LINES - 1] = [' '; COLS];
        self.line -= 1;
    }
    
    pub fn add_char(&mut self, ch: char) {
        if self.col >= COLS {
            self.col = 0;
            self.line += 1;
        }
        while self.line >= LINES {
            self.scroll();
        }

        self.buffer[self.line][self.col] = ch;
        self.col += 1;
    }

    pub fn add_nl(&mut self) {
        // Cause scrolling in the next `add_char()` invokation
        self.col = COLS;
    }

    pub fn output<'a, CS: OutputPin, SPI: Transfer<u16>>(&'a mut self, pcf: &'a mut PCF8833<CS, SPI>) -> ConsoleOutput<'a, CS, SPI> {
        ConsoleOutput { console: self, pcf }
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        let col = x / FONT_WIDTH;
        let line = y / FONT_HEIGHT;
        if col < COLS && line < LINES {
            let glyph = Glyph::map_char(self.buffer[line][col]);
            glyph.pixels(y % FONT_HEIGHT) & (0x80 >> (x % FONT_WIDTH)) != 0
        } else {
            false
        }
    }

    pub fn render<CS: OutputPin, SPI: Transfer<u16>>(&self, pcf: &mut PCF8833<CS, SPI>) {
        pcf.select()
            .display(|x, y| if self.get_pixel(x, y) {
                RGB12 {
                    r: 0xff,
                    g: 0xff,
                    b: 0xff,
                }
            } else {
                RGB12 {
                    r: 0,
                    g: 0,
                    b: 0,
                }
            })
    }
}


pub struct ConsoleOutput<'a, CS: 'a + OutputPin, SPI: 'a + Transfer<u16>> {
    console: &'a mut TextConsole,
    pcf: &'a mut PCF8833<CS, SPI>,
}

impl<'a, CS: OutputPin, SPI: Transfer<u16>> Drop for ConsoleOutput<'a, CS, SPI> {
    fn drop(&mut self) {
        self.console.render(&mut self.pcf);
    }
}

impl<'a, CS: OutputPin, SPI: Transfer<u16>> fmt::Write for ConsoleOutput<'a, CS, SPI> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.chars() {
            if ch == '\n' {
                self.console.add_nl();
            } else {
                self.console.add_char(ch);
            }
        }

        Ok(())
    }
}
