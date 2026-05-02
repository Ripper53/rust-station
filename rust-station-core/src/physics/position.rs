#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
    pub fn distance_squared(&self, position: Position) -> DistanceSquared {
        let x = self.x - position.x;
        let y = self.y - position.y;
        DistanceSquared(x * x + y * y)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct DistanceSquared(f32);

impl DistanceSquared {
    pub fn distance(self) -> Distance {
        Distance(self.0.sqrt())
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Distance(f32);

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
