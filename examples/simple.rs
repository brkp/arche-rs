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

struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

struct MenuState {
    rect: Rect,
    color: Color,
}

struct PauseState;

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
}

impl MenuState {
    fn new() -> Self {
        Self {
            rect: Rect::new(100, 100, 70, 70),
            color: Color::rgba(rand::u32()),
        }
    }
}

impl State for PauseState {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        draw::line(ctx, pt!(0, 0), pt!(479, 479), Color::default());
        draw::line(ctx, pt!(0, 479), pt!(479, 0), Color::default());
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

impl State for MenuState {
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
        self.rect.x += 1;
        self.rect.y += 1;

        if self.rect.x >= ctx.texture.w as i32 {
            self.rect.x = -self.rect.w;
        }
        if self.rect.y >= ctx.texture.h as i32 {
            self.rect.y = -self.rect.h;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        draw::fill(ctx, self.color);

        draw::triangle(
            ctx,
            pt!(480 / 2, 0),
            pt!(0, 479),
            pt!(479, 479),
            Color::rgb(0x39a7a6),
        );

        draw::triangle(
            ctx,
            pt!(31, 31),
            pt!(31, 151),
            pt!(191, 151),
            Color::rgb(0x000000),
        );

        draw::line(ctx, pt!(0, 479), pt!(100, 0), Color::default());
        draw::line(ctx, pt!(100, 0), pt!(100, 479), Color::default());
        draw::line(ctx, pt!(0, 100), pt!(479, 100), Color::default());

        for y in self.rect.y..=self.rect.y + self.rect.h {
            draw::line(
                ctx,
                pt!(self.rect.x, y),
                pt!(self.rect.x + self.rect.w, y),
                Color::default(),
            );
        }
    }
}

fn main() {
    rand::seed(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32,
    );

    ContextBuilder::new("demo".to_string(), 480, 480)
        .build()
        .unwrap()
        .run(Box::new(MenuState::new()))
}
