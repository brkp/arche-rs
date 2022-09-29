use crate::Context;
use winit_input_helper::WinitInputHelper;

pub trait State {
    fn handle_events(&mut self, ctx: &mut Context, input: &WinitInputHelper) -> Trans;
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
    // TODO:
    // on start
    // on stop
    // on pause
    // on resume
}

pub enum Trans {
    None,
    Pop,
    Set(Box<dyn State>),
    Push(Box<dyn State>),
    Quit,
}

pub struct StateManager {
    stack: Vec<Box<dyn State>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn with(initial_state: Box<dyn State>) -> Self {
        Self {
            stack: vec![initial_state],
        }
    }

    pub fn pop_state(&mut self) {
        self.stack.pop();
    }

    pub fn set_state(&mut self, state: Box<dyn State>) {
        self.stack.pop();
        self.stack.push(state);
    }

    pub fn push_state(&mut self, state: Box<dyn State>) {
        self.stack.push(state);
    }

    pub fn get_active_state(&mut self) -> Option<&mut Box<dyn State>> {
        self.stack.last_mut()
    }

    pub fn get_state_count(&self) -> usize {
        self.stack.len()
    }
}
