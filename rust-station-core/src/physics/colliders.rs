#[derive(Debug)]
pub struct BoxCollider {
    pub size_x: f32,
    pub size_y: f32,
}

impl BoxCollider {
    pub const fn new(size_x: f32, size_y: f32) -> Self {
        BoxCollider { size_x, size_y }
    }
}
