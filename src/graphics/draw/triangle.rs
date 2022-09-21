use crate::graphics::draw;
use crate::{Color, Context, Point, Texture};

// ugly as and inefficient hell
fn interpolate(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<i32> {
    if i0 == i1 {
        return vec![d0];
    }

    let mut values = Vec::new();
    let m = (d1 - d0) as f32 / (i1 - i0) as f32;
    let mut d = d0 as f32;

    for x in i0..=i1 {
        values.push(d as i32);
        d += m;
    }

    values
}

pub fn draw(ctx: &mut Context, mut p0: Point, mut p1: Point, mut p2: Point, color: Color) {
    // sorting the points by their `y` values
    if p1.y < p0.y { std::mem::swap(&mut p1, &mut p0); }
    if p2.y < p0.y { std::mem::swap(&mut p2, &mut p0); }
    if p2.y < p1.y { std::mem::swap(&mut p2, &mut p1); }

    let mut x01 = interpolate(p0.y, p0.x, p1.y, p1.x);
    let mut x12 = interpolate(p1.y, p1.x, p2.y, p2.x);
    let x02 = interpolate(p0.y, p0.x, p2.y, p2.x);

    x01.pop();
    x01.append(&mut x12);
    let x012 = x01;

    let m = x012.len() / 2;
    let (x_left, x_right) = if x02[m] < x012[m] {
        (x02, x012)
    }
    else {
        (x012, x02)
    };

    for y in p0.y..=p2.y {
        for x in x_left[(y - p0.y) as usize]..=x_right[(y - p0.y) as usize] {
            ctx.texture.set_pixel(x as usize, y as usize, color);
        }
    }

    draw::line(ctx, p0, p1, color);
    draw::line(ctx, p0, p2, color);
    draw::line(ctx, p1, p2, color);
}
