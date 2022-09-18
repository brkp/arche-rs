use arche::{Context, ContextBuilder};
use arche::{Event, State, Trans};

struct MenuState {
    counter: u8,
}

impl State for MenuState {
    fn handle_event(&mut self, _ctx: &mut Context, event: Event) -> Trans {
        match event {
            Event::KeyDown {
                keycode: Some(sdl2::keyboard::Keycode::P),
                ..
            } => Trans::Pop,
            Event::KeyDown {
                keycode: Some(sdl2::keyboard::Keycode::R),
                ..
            } => Trans::Push(Box::new(MenuState { counter: self.counter + 1 })),
            _ => Trans::None,
        }
    }

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, _ctx: &mut Context) {
        println!("{}", self.counter);
    }
}

fn main() {
    ContextBuilder::new("demo".to_string(), 640, 480)
        .build()
        .unwrap()
        .run(Box::new(MenuState { counter: 0 }));
}
