mod line;
mod triangle;

pub use line::draw as line;
pub use triangle::draw as triangle;
pub use crate::graphics::texture::{fill, clear};
