use std::fs::File;
use std::io::Read;

use sdl2::{event::Event, keyboard::Keycode};

use arche::graphics::{Color, draw};
use arche::{Context, ContextBuilder};
use arche::{State, Trans};

struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

struct MenuState {
    color: Color,
    counter: u32,
    rect: Rect,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
}

impl MenuState {
    fn new(counter: u32) -> Self {
        let mut file = File::open("/dev/urandom").unwrap();
        let mut cbuf = [0u8; 4];
        file.read_exact(&mut cbuf).unwrap();

        println!("current state: {}", counter);

        Self {
            color: Color::argb(u32::from_be_bytes(cbuf)),
            counter,
            rect: Rect::new(100, 100, 70, 70)
        }
    }
}

impl Drop for MenuState {
    fn drop(&mut self) {
        if self.counter > 0 {
            println!("current state: {}", self.counter - 1);
        }
    }
}

impl State for MenuState {
    fn handle_event(&mut self, _ctx: &mut Context, event: Event) -> Trans {
        match event {
            Event::Quit { .. } => Trans::Quit,
            Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            } => Trans::Pop,
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => Trans::Quit,
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => Trans::Push(Box::new(MenuState::new(self.counter + 1))),
            _ => Trans::None,
        }
    }

    fn update(&mut self, _ctx: &mut Context) {
        self.rect.x += 1;
        self.rect.y += 1;
    }

    fn draw(&mut self, ctx: &mut Context) {
        draw::fill(ctx, self.color);

        draw::line(ctx, 0, 0, 479, 479, Color::default());
        draw::line(ctx, 479, 0, 0, 479, Color::default());
        draw::line(ctx, 0, 479, 100, 0, Color::default());

        draw::line(ctx, 100, 0, 100, 479, Color::default());
        draw::line(ctx, 0, 100, 479, 100, Color::default());

        for y in self.rect.y..=self.rect.y + self.rect.h {
            draw::line(
                ctx,
                self.rect.x, y,
                self.rect.x + self.rect.w, y,
                Color::default());
        }
    }
}

fn main() {
    ContextBuilder::new("demo".to_string(), 480, 480)
        .build().unwrap()
        .run(Box::new(MenuState::new(0))).unwrap();
}
