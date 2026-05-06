#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Speed(f32);
impl Speed {
    pub const fn new(speed: f32) -> Self {
        Speed(speed)
    }
}
impl From<Speed> for f32 {
    fn from(speed: Speed) -> Self {
        speed.0
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct ProjectileSpeed(Speed);
impl ProjectileSpeed {
    pub const fn new(speed: Speed) -> Self {
        ProjectileSpeed(speed)
    }
}
impl From<ProjectileSpeed> for Speed {
    fn from(projectile_speed: ProjectileSpeed) -> Self {
        projectile_speed.0
    }
}
impl From<ProjectileSpeed> for f32 {
    fn from(projectile_speed: ProjectileSpeed) -> Self {
        projectile_speed.0.0
    }
}
