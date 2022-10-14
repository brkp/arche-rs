use crate::{Color, Context, Point};

#[inline(always)]
pub(crate) fn draw_not_clamped(ctx: &mut Context, p0: Point, p1: Point, color: Color) {
    let mut x0 = p0.x as f32;
    let mut x1 = p1.x as f32;
    let mut y0 = p0.y as f32;
    let mut y1 = p1.y as f32;

    let dx = x0 - x1;
    let dy = y0 - y1;

    if dx.abs() > dy.abs() {
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let m = dy / dx;
        let mut y = y0;

        for x in (x0 as usize)..=(x1 as usize) {
            ctx.texture.set_pixel(x, y as usize, color);
            y += m;
        }
    } else {
        if y0 > y1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let m = dx / dy;
        let mut x = x0;

        for y in (y0 as usize)..=(y1 as usize) {
            ctx.texture.set_pixel(x as usize, y, color);
            x += m;
        }
    }
}

pub fn draw(ctx: &mut Context, p0: Point, p1: Point, color: Color) {
    let p0 = p0.clamp(ctx.config.width as u32, ctx.config.height as u32);
    let p1 = p1.clamp(ctx.config.width as u32, ctx.config.height as u32);

    draw_not_clamped(ctx, p0, p1, color);
}
