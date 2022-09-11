use crate::core::context::Context;

pub trait State {
    fn handle_events(&mut self, ctx: &mut Context) -> Trans;
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
}

pub enum Trans {
    None,
    Pop,
    Set(Box<dyn State>),
    Push(Box<dyn State>),
}
