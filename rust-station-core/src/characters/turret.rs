use crate::{
    DeltaTime,
    anim::{Animation, AnimationDuration, Animator, AnimatorState, Frame},
    physics::{Position, World},
    utility,
};

#[derive(Debug)]
pub struct Turret<'a> {
    pub animator: Animator<'a>,
    rotation: TurretRotation,
}

impl<'a> Turret<'a> {
    pub fn new() -> Self {
        let animation = Animation::try_new(vec![Frame::new(
            "images/train/TrainGunBarrel.png",
            AnimationDuration::new(0.0),
        )])
        .unwrap();
        Turret {
            animator: Animator::new(AnimatorState::OneShot, animation),
            rotation: TurretRotation(0.0),
        }
    }
    pub fn update_turret(
        &mut self,
        hostile_world: &mut World,
        turrent_position: Position,
        delta_time: DeltaTime,
    ) -> TurretRotation {
        if let Some(pos) = hostile_world.nearest_target(turrent_position) {
            let diff = pos - turrent_position;
            let angle = diff.y.atan2(diff.x);
            self.rotation = TurretRotation(utility::lerp(
                self.rotation.0,
                angle,
                (0.25 * delta_time.0).max(0.0).min(1.0),
            ));
        }
        self.rotation
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct TurretRotation(f32);
impl TurretRotation {
    pub fn value(&self) -> f32 {
        self.0
    }
}
