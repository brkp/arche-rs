use crate::Context;
use winit_input_helper::WinitInputHelper;

#[allow(unused_variables)]
pub trait State {
    fn on_start(&mut self, ctx: &mut Context) {}
    fn on_stop(&mut self, ctx: &mut Context) {}
    fn on_pause(&mut self, ctx: &mut Context) {}
    fn on_resume(&mut self, ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context) {}
    fn draw(&mut self, ctx: &mut Context) {}
    fn handle_events(&mut self, ctx: &mut Context, input: &WinitInputHelper) -> Trans { Trans::None }
}

pub enum Trans {
    None,
    Pop,
    Set(Box<dyn State>),
    Push(Box<dyn State>),
    Quit,
}

#[derive(Default)]
pub struct StateManager {
    stack: Vec<Box<dyn State>>,
}

impl StateManager {
    pub fn with(ctx: &mut Context, mut initial_state: Box<dyn State>) -> Self {
        initial_state.on_start(ctx);

        Self {
            stack: vec![initial_state],
        }
    }

    pub fn pop_state(&mut self, ctx: &mut Context) {
        if let Some(mut state) = self.stack.pop() {
            state.on_stop(ctx);
        }
        if let Some(new_state) = self.get_active_state() {
            new_state.on_resume(ctx);
        }
    }

    pub fn set_state(&mut self, ctx: &mut Context, mut state: Box<dyn State>) {
        if let Some(mut prev_state) = self.stack.pop() {
            prev_state.on_stop(ctx);
        }
        state.on_start(ctx);
        self.stack.push(state);

    }

    pub fn push_state(&mut self, ctx: &mut Context, mut state: Box<dyn State>) {
        if let Some(prev_state) = self.get_active_state() {
            prev_state.on_pause(ctx);
        }
        state.on_start(ctx);
        self.stack.push(state);
    }

    pub fn get_active_state(&mut self) -> Option<&mut Box<dyn State>> {
        self.stack.last_mut()
    }

    pub fn get_state_count(&self) -> usize {
        self.stack.len()
    }
}
