#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(u32);

impl Default for Color {
    fn default() -> Self {
        Self (0x000000ff)
    }
}

impl Color {
    pub fn rgb(color: u32) -> Self {
        Self ((color << 8) | 0xff)
    }

    pub fn rgba(color: u32) -> Self {
        Self (color)
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let a = rhs.0 & 0xff;
        if  a == 0xff { return rhs; }
        if  a == 0x00 { return self; }

        let c1 = self.0 >> 8;
        let c2 =  rhs.0 >> 8;

        let mut rb = c1 & 0xff00ff;
        let mut g  = c1 & 0x00ff00;

        rb += (((c2 & 0xff00ff) - rb) * a) >> 8;
        g  += (((c2 & 0x00ff00) -  g) * a) >> 8;

        Self ((((rb & 0xff00ff) | (g & 0x00ff00)) << 8) | a)
    }
}
