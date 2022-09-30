use crate::graphics::Color;
use crate::Context;

pub struct Texture {
    pub pixel: Vec<u8>,
    pub w: usize,
    pub h: usize,
}

pub fn fill(ctx: &mut Context, color: Color) {
    ctx.texture
        .pixel
        .chunks_exact_mut(4)
        .for_each(|c| c.copy_from_slice(&color.bytes()));
}

pub fn clear(ctx: &mut Context) {
    ctx.texture.pixel.fill(0x00);
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
        if x < self.w && y < self.h {
            let index = (x + self.w * y) * 4;
            let pixel = Color::rgba(u32::from_be_bytes(
                self.pixel[index..index + 4].try_into().unwrap(),
            )) + color;
            self.pixel[index..index + 4].copy_from_slice(&pixel.bytes());
        }
    }
}
