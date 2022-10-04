mod core;
mod graphics;

pub use crate::core::{Context, ContextBuilder};
pub use crate::core::{State, StateManager, Trans};
pub use crate::graphics::{Color, Point, Texture};

// re-exports
pub use winit;
pub use winit_input_helper;

#[macro_export]
macro_rules! pt {
    ($x: expr, $y: expr) => {
        Point { x: $x, y: $y }
    };
}

pub mod prelude {
    pub use super::pt;
    pub use crate::core::{Context, ContextBuilder, State, Trans};
    pub use crate::graphics::{draw, Color, Point, Texture};
}
