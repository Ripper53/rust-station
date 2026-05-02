use crate::{
    DeltaTime,
    anim::{Animation, AnimationDuration, Animator, AnimatorState, Frame},
    characters::GetCharacterAnimator,
    physics::{EntityID, Position, Velocity, World},
};

#[derive(Debug)]
pub struct FirstMinion<'a> {
    pub animator: Animator<'a>,
}

impl<'a> FirstMinion<'a> {
    pub fn new() -> Self {
        const ANIMATION_INTERVAL: AnimationDuration = AnimationDuration::new(0.125);
        let animation = Animation::try_new(vec![
            Frame::new(
                "images/characters/first_minion/FirstMinion0.png",
                ANIMATION_INTERVAL,
            ),
            Frame::new(
                "images/characters/first_minion/FirstMinion1.png",
                ANIMATION_INTERVAL,
            ),
        ])
        .unwrap();
        FirstMinion {
            animator: Animator::new(AnimatorState::Loop, animation),
        }
    }
}

#[derive(Debug)]
pub struct FirstMinionBehavior {
    target_position: Position,
    speed: f32,
}

impl FirstMinionBehavior {
    pub fn new(target_position: Position, speed: f32) -> Self {
        FirstMinionBehavior {
            target_position,
            speed,
        }
    }
    pub fn set_target_position_y(&mut self, target_y: f32) {
        self.target_position.y = target_y;
    }
    pub fn update_first_minion(
        &mut self,
        world: &mut World,
        entity_id: EntityID,
        _delta_time: DeltaTime,
    ) {
        let Some((position, velocity)) = world.get_dynamic_positions_mut(entity_id) else {
            return;
        };
        *velocity = Velocity::target(self.target_position, *position).normalize() * self.speed;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinAttackTrainHorizontalPosition(f32);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxAttackTrainHorizontalPosition(f32);

impl<'a> GetCharacterAnimator<'a> for FirstMinion<'a> {
    fn animator(&self) -> &Animator<'a> {
        &self.animator
    }
}
