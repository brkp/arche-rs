use crate::{Color, Context, Texture};

// TODO: implement a point type
// TODO: vertical lines will cause a `panic`
pub fn draw(ctx: &mut Context, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let mut x0 = x0;
    let mut x1 = x1;
    let mut y0 = y0;
    let mut y1 = y1;

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let a = (y1 - y0) / (x1 - x0);
    let b = y0 - a * x0;

    for x in x0..x1 {
        let x = x as i32;
        let y = a*x + b;
        ctx.texture.set_pixel(x as usize, y as usize, color);
    }
}
