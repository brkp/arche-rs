#[derive(Debug, PartialEq, Eq)]
pub struct Color {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn rgb(color: u32) -> Self {
        Self {
            a: 255,
            r: ((color >> 16) & 0xff) as u8,
            g: ((color >>  8) & 0xff) as u8,
            b: ((color >>  0) & 0xff) as u8,
        }
    }
}
