use std::time::SystemTime;

use arche::graphics::{draw, Color};
use arche::{pt, Context, ContextBuilder, Point};
use arche::{State, Trans};

use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

mod rand {
    extern "C" {
        fn rand() -> i32;
        fn srand(seed: u32);
    }

    pub fn seed(seed: u32) {
        unsafe { srand(seed) }
    }

    pub fn u32() -> u32 {
        unsafe { rand() as u32 }
    }
}

const W: usize = 480;
const H: usize = 480;
const C: usize = 40;

struct MainState {
    rect: (Point, i32, i32)
}
struct PauseState;

impl State for PauseState {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        draw::line(ctx, pt!(0, 0), pt!(479, 479), Color::rgba(0x00ff0060));
        draw::line(ctx, pt!(0, 479), pt!(479, 0), Color::rgba(0x00ff0060));
    }

    fn handle_events(&mut self, _ctx: &mut Context, input: &WinitInputHelper) -> Trans {
        if input.key_pressed(VirtualKeyCode::Q) || input.quit() {
            return Trans::Quit;
        }
        if input.key_pressed(VirtualKeyCode::P) {
            return Trans::Pop;
        }

        Trans::None
    }
}

impl State for MainState {
    fn handle_events(&mut self, _ctx: &mut Context, input: &WinitInputHelper) -> Trans {
        if input.key_pressed(VirtualKeyCode::Q) || input.quit() {
            return Trans::Quit;
        }
        if input.key_pressed(VirtualKeyCode::P) {
            return Trans::Push(Box::new(PauseState {}));
        }

        Trans::None
    }

    fn update(&mut self, ctx: &mut Context) {
        self.rect.0.x += 1;
        self.rect.0.y += 1;

        if self.rect.0.x >= ctx.texture.w as i32 {
            self.rect.0.x = -self.rect.1;
        }
        if self.rect.0.y >= ctx.texture.h as i32 {
            self.rect.0.y = -self.rect.2;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        // draw::clear(ctx);
        // draw::fill(ctx, Color::rgb(0x6495ed));

        for y in (0..H).step_by(C) {
            for x in (0..W).step_by(C) {
                let color = if ((x / C) + (y / C)) % 2 == 0 {
                    Color::rgb(0x3d4757)
                } else {
                    Color::rgb(0x486860)
                };
                draw::rect(ctx, pt!(x as i32, y as i32), C as i32, C as i32, color);
            }
        }

        draw::rect(ctx, self.rect.0, self.rect.1, self.rect.2, Color::rgba(0x6495ed80));
    }
}

fn main() {
    rand::seed(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32,
    );

    ContextBuilder::new("demo".to_string(), W, H)
        .build()
        .unwrap()
        .run(Box::new(MainState { rect: (pt!(0, 0), 100, 100) }))
}
