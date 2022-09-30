use crate::{graphics::draw, pt, Color, Context, Point};

pub fn draw(ctx: &mut Context, p: Point, w: i32, h: i32, color: Color) {
    for y in p.y..=p.y + h {
        draw::line(ctx, pt!(p.x, y), pt!(p.x + w, y), color);
    }
}
