use crate::{graphics::draw, pt, Color, Context, Point};
use std::mem::swap;

#[inline]
fn fill_bottom_flat_triangle(ctx: &mut Context, v0: Point, v1: Point, v2: Point, color: Color) {
    let im1 = (v1.x as f32 - v0.x as f32) / (v1.y as f32 - v0.y as f32);
    let im2 = (v2.x as f32 - v0.x as f32) / (v2.y as f32 - v0.y as f32);

    let mut cx1 = v0.x as f32;
    let mut cx2 = v0.x as f32;

    for y in v0.y..=v1.y {
        draw::line::draw_not_clamped(ctx, pt!(cx1 as u32, y), pt!(cx2 as u32, y), color);
        cx1 += im1;
        cx2 += im2;
    }
}

#[inline]
fn fill_top_flat_triangle(ctx: &mut Context, v0: Point, v1: Point, v2: Point, color: Color) {
    let im1 = (v2.x as f32 - v0.x as f32) / (v2.y as f32 - v0.y as f32);
    let im2 = (v2.x as f32 - v1.x as f32) / (v2.y as f32 - v1.y as f32);

    let mut cx1 = v2.x as f32;
    let mut cx2 = v2.x as f32;

    for y in (v0.y..=v2.y).rev() {
        draw::line::draw_not_clamped(ctx, pt!(cx1 as u32, y), pt!(cx2 as u32, y), color);
        cx1 -= im1;
        cx2 -= im2;
    }
}

pub fn draw(ctx: &mut Context, mut v0: Point, mut v1: Point, mut v2: Point, color: Color) {
    v0 = v0.clamp(ctx.config.width as u32, ctx.config.height as u32);
    v1 = v1.clamp(ctx.config.width as u32, ctx.config.height as u32);
    v2 = v2.clamp(ctx.config.width as u32, ctx.config.height as u32);

    draw::line::draw_not_clamped(ctx, v0, v1, color);
    draw::line::draw_not_clamped(ctx, v0, v2, color);
    draw::line::draw_not_clamped(ctx, v1, v2, color);
}

pub fn draw_filled(ctx: &mut Context, mut v0: Point, mut v1: Point, mut v2: Point, color: Color) {
    v0 = v0.clamp(ctx.config.width as u32, ctx.config.height as u32);
    v1 = v1.clamp(ctx.config.width as u32, ctx.config.height as u32);
    v2 = v2.clamp(ctx.config.width as u32, ctx.config.height as u32);

    if v1.y < v0.y { swap(&mut v1, &mut v0); }
    if v2.y < v0.y { swap(&mut v2, &mut v0); }
    if v2.y < v1.y { swap(&mut v2, &mut v1); }

    if v2.y == v1.y {
        fill_bottom_flat_triangle(ctx, v0, v1, v2, color);
    } else if v0.y == v1.y {
        fill_top_flat_triangle(ctx, v0, v1, v2, color);
    } else {
        let v3 = pt!(
            v0.x + ((v2.x as f32 - v0.x as f32) * (v1.y as f32 - v0.y as f32)
                / (v2.y as f32 - v0.y as f32)) as u32,
            v1.y
        );
        fill_bottom_flat_triangle(ctx, v0, v1, v3, color);
        fill_top_flat_triangle(ctx, v1, v3, v2, color);
    }
}
