#[derive(Debug, Clone, Copy, Default)]
pub enum TurretState {
    #[default]
    FollowTarget,
    Shoot,
}
