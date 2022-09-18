use std::fs::File;
use std::io::Read;

use sdl2::{event::Event, keyboard::Keycode};

use arche::graphics::Color;
use arche::{Context, ContextBuilder};
use arche::{State, Trans};

struct MenuState {
    color: Color,
    counter: u32,
}

impl MenuState {
    fn new(counter: u32) -> Self {
        let mut file = File::open("/dev/urandom").unwrap();
        let mut cbuf = [0u8; 4];
        file.read_exact(&mut cbuf).unwrap();

        println!("{}", counter);

        Self {
            color: Color::argb(u32::from_be_bytes(cbuf)),
            counter,
        }
    }
}

impl Drop for MenuState {
    fn drop(&mut self) {
        if self.counter > 0 {
            println!("{}", self.counter - 1);
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

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        for pixel in ctx.texture.pixel.chunks_exact_mut(4) {
            pixel.copy_from_slice(&self.color.to_bytes());
        }
    }
}

fn main() {
    ContextBuilder::new("demo".to_string(), 480, 480)
        .build().unwrap()
        .run(Box::new(MenuState::new(0))).unwrap();
}
