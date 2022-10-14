#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn clamp(&self, cx: u32, cy: u32) -> Self {
        Self {
            x: self.x.clamp(0, cx),
            y: self.y.clamp(0, cy),
        }
    }
}
