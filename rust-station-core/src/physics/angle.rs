use crate::physics::Velocity;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct RadiansAngle(f32);

impl RadiansAngle {
    pub const fn new(angle: f32) -> Self {
        RadiansAngle(angle)
    }
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl From<RadiansAngle> for Velocity {
    fn from(angle: RadiansAngle) -> Self {
        Velocity::new(f32::cos(angle.0), f32::sin(angle.0))
    }
}

impl From<RadiansAngle> for f32 {
    fn from(angle: RadiansAngle) -> Self {
        angle.0
    }
}
