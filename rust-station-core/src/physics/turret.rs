#[derive(Debug, Clone, Copy)]
pub enum TurretState {
    FollowTarget {
        look_around_cooldown: f32,
        shoot_cooldown: f32,
    },
    Shoot,
}

impl Default for TurretState {
    fn default() -> Self {
        TurretState::FollowTarget {
            look_around_cooldown: 0.0,
            shoot_cooldown: 0.0,
        }
    }
}
