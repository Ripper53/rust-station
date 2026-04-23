#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub const fn new(x: f32, y: f32) -> Self {
        Velocity { x, y }
    }
}

impl std::ops::AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Mul<f32> for Velocity {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Velocity::new(self.x * rhs, self.y * rhs)
    }
}
