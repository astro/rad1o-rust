#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r,
        g: g,
        b: b,
    }
}

impl Color {
    pub fn blend(&self, opacity: u8, other: &Color) -> Color {
        let b = |x: u8, y: u8| {
            (x as u16 * (opacity as u16) / 255 +
             y as u16 * (255 - (opacity as u16)) / 255)
                as u8
        };

        Color {
            r: b(self.r, other.r),
            g: b(self.g, other.g),
            b: b(self.b, other.b),
        }
    }
}
