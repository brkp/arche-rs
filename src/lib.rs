pub use sdl2;

pub mod core;
pub mod graphics;

pub use crate::core::{Context, ContextBuilder, State, Trans};
pub use crate::graphics::{Color, Point, Texture};

#[macro_export]
macro_rules! pt {
    ($x:expr, $y: expr) => {
        Point { x: $x, y: $y }
    };
}
