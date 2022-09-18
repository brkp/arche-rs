use crate::graphics::Color;

pub struct Texture {
    pub pixel: Vec<u8>,
    pub w: usize,
    pub h: usize,
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            w: width,
            h: height,
            pixel: vec![0; 4 * width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = (x + self.w * y) * 4;
        self.pixel[index..index + 4].copy_from_slice(&color.to_bytes());
    }
}
