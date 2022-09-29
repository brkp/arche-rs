#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self::rgba(0x000000ff)
    }
}

impl Color {
    pub fn rgb(color: u32) -> Self {
        Self::rgba((color << 8) | 0x000000ff)
    }

    pub fn rgba(color: u32) -> Self {
        Self {
            r: ((color >> 24) & 0xff) as u8,
            g: ((color >> 16) & 0xff) as u8,
            b: ((color >>  8) & 0xff) as u8,
            a: ((color >>  0) & 0xff) as u8,
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
