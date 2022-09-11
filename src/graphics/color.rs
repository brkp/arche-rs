#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self::argb(0xff000000)
    }
}

impl Color {
    pub fn rgb(color: u32) -> Self {
        Self::argb(color | 0xff000000)
    }

    pub fn argb(color: u32) -> Self {
        Self {
            a: ((color >> 24) & 0xff) as u8,
            r: ((color >> 16) & 0xff) as u8,
            g: ((color >>  8) & 0xff) as u8,
            b: ((color >>  0) & 0xff) as u8,
        }
    }
}
