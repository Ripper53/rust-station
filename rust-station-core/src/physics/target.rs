use crate::physics::Position;

#[derive(Debug, Clone, Copy, Default)]
pub struct Target(Position);
impl Target {
    pub const fn new(position: Position) -> Self {
        Target(position)
    }
}
impl From<Target> for Position {
    fn from(target: Target) -> Self {
        target.0
    }
}
