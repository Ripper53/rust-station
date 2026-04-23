#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    pub const fn new(width: f32, height: f32) -> Self {
        Bounds { width, height }
    }
}
