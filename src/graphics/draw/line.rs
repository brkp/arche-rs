use crate::{Color, Context, Texture};

pub fn draw(ctx: &mut Context, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let mut x0 = x0;
    let mut x1 = x1;
    let mut y0 = y0;
    let mut y1 = y1;

    let dx = x0 - x1;
    let dy = y0 - y1;

    if dx.abs() > dy.abs() {
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let m = dy/dx;
        let mut y = y0;

        for x in x0..=x1 {
            ctx.texture.set_pixel(x as usize, y as usize, color);
            y += m;
        }
    }
    else {
        if y0 > y1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let m = dx/dy;
        let mut x = x0;

        for y in y0..=y1 {
            ctx.texture.set_pixel(x as usize, y as usize, color);
            x += m;
        }
    }
}
