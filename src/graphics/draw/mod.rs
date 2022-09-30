mod line;
mod rect;
mod triangle;

pub use crate::graphics::texture::{clear, fill};

pub use line::draw as line;
pub use rect::draw as rect;
pub use triangle::draw as triangle;
