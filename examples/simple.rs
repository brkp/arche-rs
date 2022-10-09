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

    pub fn range(a: u32, b: u32) -> u32 {
        return (u32() % (b - a + 1)) + a;
    }
}

struct MainState {
    rect: (Point, u32, u32),
}
struct PauseState;

impl State for PauseState {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        draw::line(ctx, pt!(0, 0), pt!(479, 479), Color::rgba(0x00ff0060));
        draw::line(ctx, pt!(0, 479), pt!(479, 0), Color::rgba(0x00ff0060));
    }

    fn handle_events(&mut self, ctx: &mut Context, input: &WinitInputHelper) -> Trans {
        if input.key_pressed(VirtualKeyCode::Q) || input.quit() {
            return Trans::Quit;
        }
        if input.key_pressed(VirtualKeyCode::P) {
            ctx.set_show_mouse(false);
            ctx.set_grab_mouse(true);

            return Trans::Pop;
        }
        if input.key_pressed(VirtualKeyCode::LShift) {
            ctx.toggle_show_mouse();
            ctx.toggle_grab_mouse();
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

        if self.rect.0.x >= ctx.texture.w as u32 {
            self.rect.0.x = 0;
        }
        if self.rect.0.y >= ctx.texture.h as u32 {
            self.rect.0.y = 0;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        draw::fill(ctx, Color::rgb(0x212121));

        for _ in 0..50 {
            draw::triangle(
                ctx,
                pt!(rand::range(100, W as u32 - 100), rand::range(100, H as u32 - 100)),
                pt!(rand::range(100, W as u32 - 100), rand::range(100, H as u32 - 100)),
                pt!(rand::range(100, W as u32 - 100), rand::range(100, H as u32 - 100)),
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
        // .vsync(true)
        .show_mouse(false)
        .build()
        .unwrap()
        .run(|_| {
            Box::new(MainState {
                rect: (pt!(0, 0), 100, 100),
            })
        })
}
