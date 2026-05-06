use crate::physics::Position;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub const fn new(x: f32, y: f32) -> Self {
        Velocity { x, y }
    }
    pub const fn target(target_position: Position, current_position: Position) -> Self {
        Velocity {
            x: target_position.x - current_position.x,
            y: target_position.y - current_position.y,
        }
    }
    pub fn normalize(mut self) -> Self {
        let magnitude = self.magnitude_squared().magnitude();
        if magnitude.0 == 0.0 {
            self.x = 0.0;
            self.y = 0.0;
        } else {
            self.x /= magnitude.0;
            self.y /= magnitude.0;
        }
        self
    }
    pub const fn magnitude_squared(&self) -> MagnitudeSquared {
        MagnitudeSquared(self.x * self.x + self.y * self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MagnitudeSquared(f32);

impl MagnitudeSquared {
    pub fn magnitude(self) -> Magnitude {
        Magnitude(self.0.sqrt())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Magnitude(f32);

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
