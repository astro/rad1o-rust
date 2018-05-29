use core::fmt;

use table::table;

/// Display width in pixels
pub const RESX: isize = 96;
/// Display height in pixels
pub const RESY: isize = 68;
/// `RESY` in bytes
pub const RESY_B: usize = 9;


/// Set/clear pixel at `x`×`y`
pub fn set_pixel(x: u8, y: u8, c: u8) {
    (table().lcdSetPixel)(x, y, c);
}

/// Get pixel at `x`×`y`
pub fn get_pixel(x: u8, y: u8) -> u8 {
    (table().lcdGetPixel)(x, y)
}

/// Print a string `s` at position `sx`×`sy`
pub fn do_string(sx: isize, sy: isize, s: &str) {
    let mut sx = sx;
    with_nul_terminated_bufs(s, |ptr, _is_last| {
        sx = (table().DoString)(sx, sy, ptr);
    });
}

fn with_nul_terminated_bufs<F>(s: &str, mut f: F)
    where F: FnMut(*const u8, bool)
{
    let mut bytes = s.as_bytes();
    while bytes.len() > 0 {
        let mut buf = [0u8; 16];
        let len = bytes.len().min(buf.len() - 1);
        buf[..len].copy_from_slice(&bytes[..len]);
        f(buf.as_ptr(), len == bytes.len());
        drop(buf);

        bytes = &bytes[len..];
    }
}

/// Print a string
///
/// A string `s` of up to 15 bytes will be copied into
/// nul-terminated buffer.
pub fn print(s: &str) {
    with_nul_terminated_bufs(s, |ptr, _is_last| {
        (table().lcdPrint)(ptr);
    });
}

/// Print a line
///
/// A string `s` of up to 15 bytes will be copied into
/// nul-terminated buffer.
pub fn println(s: &str) {
    with_nul_terminated_bufs(s, |ptr, is_last| {
        if is_last {
            (table().lcdPrintln)(ptr);
        } else {
            (table().lcdPrint)(ptr);
        }
    });
}

/// Writes onto the display
pub struct Stdout;
/// Do not use as of now. Linking in such functionality makes the l0dable exceed 2560 bytes.
pub const STDOUT: Stdout = Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.as_bytes() {
            match b {
                10 =>
                    (table().lcdNl)(),
                _ => {
                    let mut buf = [0u8; 2];
                    buf[0] = *b;
                    (table().lcdPrint)(buf.as_ptr());
                    drop(buf);
                },
            }
        }

        display();
        Ok(())
    }
}

/// Clear the display buffer
pub fn clear() {
    (table().lcdClear)();
}

/// Flush the display buffer to hardware
pub fn display() {
    (table().lcdDisplay)();
}
