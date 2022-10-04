use std::time::SystemTime;

use arche::prelude::*;
use arche::winit::event::VirtualKeyCode;
use arche::winit_input_helper::WinitInputHelper;

const W: usize = 480;
const H: usize = 480;

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

    pub fn range(a: i32, b: i32) -> i32 {
        return (u32() as i32 % (b - a + 1)) + a;
    }
}

struct MainState {
    rect: (Point, i32, i32),
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
        draw::fill(ctx, Color::rgb(0x212121));

        // for y in (0..H).step_by(C) {
        //     for x in (0..W).step_by(C) {
        //         let color = if ((x / C) + (y / C)) % 2 == 0 {
        //             Color::rgb(0xffffff)
        //         } else {
        //             Color::rgb(0x000000)
        //         };
        //         draw::rect(ctx, pt!(x as i32, y as i32), C as i32, C as i32, color);
        //     }
        // }

        for _ in 0..100 {
            draw::triangle(
                ctx,
                pt!(rand::range(100, W as i32 - 100), rand::range(100, H as i32 - 100)),
                pt!(rand::range(100, W as i32 - 100), rand::range(100, H as i32 - 100)),
                pt!(rand::range(100, W as i32 - 100), rand::range(100, H as i32 - 100)),
                Color::rgb(rand::u32()),
            );
        }

        draw::rect(
            ctx,
            self.rect.0,
            self.rect.1,
            self.rect.2,
            Color::rgba(0x6495ed80),
        );
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
        .run(|_| {
            Box::new(MainState {
                rect: (pt!(0, 0), 100, 100),
            })
        })
}
