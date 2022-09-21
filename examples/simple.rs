use std::fs::File;
use std::io::Read;

use sdl2::{event::Event, keyboard::Keycode};

use arche::graphics::{Color, draw};
use arche::{Point, Context, ContextBuilder};
use arche::{State, Trans};

struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

struct MenuState {
    color: Color,
    rect: Rect,
}

struct PauseState;

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
}

impl MenuState {
    fn new() -> Self {
        let mut file = File::open("/dev/urandom").unwrap();
        let mut cbuf = [0u8; 4];
        file.read_exact(&mut cbuf).unwrap();

        Self {
            rect: Rect::new(100, 100, 70, 70),
            color: Color::argb(u32::from_be_bytes(cbuf)),
        }
    }
}

impl State for PauseState {
    fn handle_event(&mut self, _ctx: &mut Context, event: Event) -> Trans {
        match event {
            Event::Quit { .. } => Trans::Quit,
            Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            } => Trans::Pop,
            _ => Trans::None,
        }
    }

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
         draw::line(ctx, Point::new(0, 0), Point::new(479, 479), Color::default());
         draw::line(ctx, Point::new(0, 479), Point::new(479, 0), Color::default());
    }
}

impl State for MenuState {
    fn handle_event(&mut self, _ctx: &mut Context, event: Event) -> Trans {
        match event {
            Event::Quit { .. } => Trans::Quit,
            Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            } => Trans::Push(Box::new(PauseState {})),
            _ => Trans::None,
        }
    }

    fn update(&mut self, ctx: &mut Context) {
        self.rect.x += 1;
        self.rect.y += 1;

        if self.rect.x >= ctx.texture.w as i32 { self.rect.x = -self.rect.w; }
        if self.rect.y >= ctx.texture.h as i32 { self.rect.y = -self.rect.h; }
    }

    fn draw(&mut self, ctx: &mut Context) {
        draw::fill(ctx, self.color);

        draw::triangle(
            ctx,
            Point::new(480/2, 0),
            Point::new(0, 479),
            Point::new(479, 479),
            Color::rgb(0x39a7a6));

        draw::line(ctx, Point::new(0, 479), Point::new(100, 0), Color::default());
        draw::line(ctx, Point::new(100, 0), Point::new(100, 479), Color::default());
        draw::line(ctx, Point::new(0, 100), Point::new(479, 100), Color::default());

        for y in self.rect.y..=self.rect.y + self.rect.h {
            draw::line(
                ctx,
                Point::new(self.rect.x, y),
                Point::new(self.rect.x + self.rect.w, y),
                Color::default());
        }
    }
}

fn main() {
    ContextBuilder::new("demo".to_string(), 480, 480)
        .build().unwrap()
        .run(Box::new(MenuState::new())).unwrap();
}
