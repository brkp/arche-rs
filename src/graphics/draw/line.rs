use crate::{Color, Context, Point};

// TODO: use the Point type
pub fn draw(ctx: &mut Context, p0: Point, p1: Point, color: Color) {
    let mut x0 = p0.x;
    let mut x1 = p1.x;
    let mut y0 = p0.y;
    let mut y1 = p1.y;

    let dx = (x0 - x1) as f32;
    let dy = (y0 - y1) as f32;

    if dx.abs() > dy.abs() {
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let m = dy / dx;
        let mut y = y0 as f32;

        for x in x0..=x1 {
            ctx.texture.set_pixel(x as usize, y as usize, color);
            y += m;
        }
    } else {
        if y0 > y1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let m = dx / dy;
        let mut x = x0 as f32;

        for y in y0..=y1 {
            ctx.texture.set_pixel(x as usize, y as usize, color);
            x += m;
        }
    }
}
