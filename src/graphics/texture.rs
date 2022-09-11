use crate::graphics::Color;

pub struct Texture {
    pixel: Vec<Color>,
    pub w: usize,
    pub h: usize,
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            w: width,
            h: height,
            pixel: vec![Color::default(); width * height]
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel_index = x + self.w * y;
        if let Some(c) = self.pixel.get_mut(pixel_index) {
            *c = color;
        }
    }
}
