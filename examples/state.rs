use arche::prelude::*;
use arche::winit::event::VirtualKeyCode;
use arche::winit_input_helper::WinitInputHelper;

struct State0;
struct State1;
struct State2;

impl State for State0 {
    fn on_start(&mut self, _ctx: &mut Context)  { println!("state0: start"); }
    fn on_stop(&mut self, _ctx: &mut Context)   { println!("state0: stop"); }
    fn on_pause(&mut self, _ctx: &mut Context)  { println!("state0: pause"); }
    fn on_resume(&mut self, _ctx: &mut Context) { println!("state0: resume"); }

    fn handle_events(&mut self, _ctx: &mut Context, input: &WinitInputHelper) -> Trans {
        if input.quit() || input.key_pressed(VirtualKeyCode::Escape) { return Trans::Quit };
        if input.key_pressed(VirtualKeyCode::N) { return Trans::Push(Box::new(State1 {})); }
        if input.key_pressed(VirtualKeyCode::P) { return Trans::Pop; }

        Trans::None
    }
}

impl State for State1 {
    fn on_start(&mut self, _ctx: &mut Context)  { println!("state1: start"); }
    fn on_stop(&mut self, _ctx: &mut Context)   { println!("state1: stop"); }
    fn on_pause(&mut self, _ctx: &mut Context)  { println!("state1: pause"); }
    fn on_resume(&mut self, _ctx: &mut Context) { println!("state1: resume"); }

    fn handle_events(&mut self, _ctx: &mut Context, input: &winit_input_helper::WinitInputHelper) -> Trans {
        if input.quit() || input.key_pressed(VirtualKeyCode::Escape) { return Trans::Quit };
        if input.key_pressed(VirtualKeyCode::N) { return Trans::Push(Box::new(State2 {})); }
        if input.key_pressed(VirtualKeyCode::P) { return Trans::Pop; }

        Trans::None
    }
}

impl State for State2 {
    fn on_start(&mut self, _ctx: &mut Context)  { println!("state2: start"); }
    fn on_stop(&mut self, _ctx: &mut Context)   { println!("state2: stop"); }
    fn on_pause(&mut self, _ctx: &mut Context)  { println!("state2: pause"); }
    fn on_resume(&mut self, _ctx: &mut Context) { println!("state2: resume"); }

    fn handle_events(&mut self, _ctx: &mut Context, input: &winit_input_helper::WinitInputHelper) -> Trans {
        if input.quit() || input.key_pressed(VirtualKeyCode::Escape) { return Trans::Quit };
        if input.key_pressed(VirtualKeyCode::N) { println!("state2: no more state to push"); }
        if input.key_pressed(VirtualKeyCode::P) { return Trans::Pop; }

        Trans::None
    }
}

fn main() {
    ContextBuilder::new("state demo".to_owned(), 480, 320)
        .vsync(true)
        .build()
        .unwrap()
        .run(|_| Box::new(State0 {}));
}
