use crate::{
    anim::{Animation, AnimationDuration, Animator, AnimatorState, Frame},
    characters::GetCharacterAnimator,
    physics::{EntityID, World},
};

#[derive(Debug)]
pub struct Oswin<'a> {
    state: OswinState,
    pub animator: Animator<'a>,
    idle_animation: Animation<'a>,
    walking_animation: Animation<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OswinState {
    Idle,
    Walking,
}

impl<'a> Oswin<'a> {
    pub fn new() -> Self {
        let idle_animation = Animation::try_new(vec![Frame::new(
            "images/characters/oswin/OswinIdle.png",
            AnimationDuration::new(0.0),
        )])
        .unwrap();
        const ANIMATION_INTERVAL: AnimationDuration = AnimationDuration::new(0.125);
        let walking_animation = Animation::try_new(vec![
            Frame::new("images/characters/oswin/OswinWalk0.png", ANIMATION_INTERVAL),
            Frame::new("images/characters/oswin/OswinIdle.png", ANIMATION_INTERVAL),
            Frame::new("images/characters/oswin/OswinWalk1.png", ANIMATION_INTERVAL),
            Frame::new("images/characters/oswin/OswinIdle.png", ANIMATION_INTERVAL),
        ])
        .unwrap();
        Oswin {
            state: OswinState::Idle,
            animator: Animator::new(AnimatorState::Loop, idle_animation.clone()),
            idle_animation,
            walking_animation,
        }
    }
    pub fn set_state(&mut self, state: OswinState) {
        if self.state == state {
            return;
        }
        self.state = state;
        match self.state {
            OswinState::Idle => {
                self.animator.set_animation(self.idle_animation.clone());
            }
            OswinState::Walking => {
                self.animator.set_animation(self.walking_animation.clone());
            }
        }
    }
}

#[derive(Debug)]
pub struct OswinBehavior {
    state: OswinBehaviorState,
    min_speed: f32,
    max_speed: f32,
    speed: f32,
}

impl OswinBehavior {
    pub fn new(min_speed: f32, max_speed: f32) -> Self {
        OswinBehavior {
            state: OswinBehaviorState::Idle {
                wait_timer: rand::random_range(0.0..2.0),
            },
            min_speed,
            max_speed,
            speed: 0.0,
        }
    }
}

#[derive(Debug)]
enum OswinBehaviorState {
    Idle {
        wait_timer: f32,
    },
    Walking {
        walking_timer: f32,
        direction: WalkingDirection,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum WalkingDirection {
    Right,
    Left,
}

impl OswinBehavior {
    pub fn oswin_behavior(
        &mut self,
        world: &mut World,
        entity_id: EntityID,
        delta_time: f32,
    ) -> (OswinState, Option<WalkingDirection>) {
        let width = world.bounds().width;
        let Some((position, velocity)) = world.get_dynamic_positions_mut(entity_id) else {
            return (OswinState::Idle, None);
        };
        match &mut self.state {
            OswinBehaviorState::Idle { wait_timer } => {
                if *wait_timer > 0.0 {
                    *wait_timer -= delta_time;
                    (OswinState::Idle, None)
                } else {
                    let direction = if rand::random_bool(0.5) {
                        WalkingDirection::Right
                    } else {
                        WalkingDirection::Left
                    };
                    self.speed = rand::random_range(self.min_speed..self.max_speed);
                    self.state = OswinBehaviorState::Walking {
                        walking_timer: rand::random_range(1.0..4.0),
                        direction,
                    };
                    (OswinState::Walking, Some(direction))
                }
            }
            OswinBehaviorState::Walking {
                walking_timer,
                direction,
            } => {
                if *walking_timer > 0.0 {
                    *walking_timer -= delta_time;
                    let x = position.x;
                    match direction {
                        WalkingDirection::Right => {
                            if x + 18.0 > width {
                                *direction = WalkingDirection::Left;
                                velocity.x = -self.speed;
                            } else {
                                velocity.x = self.speed;
                            }
                        }
                        WalkingDirection::Left => {
                            if x < f32::EPSILON {
                                *direction = WalkingDirection::Right;
                                velocity.x = self.speed;
                            } else {
                                velocity.x = -self.speed;
                            }
                        }
                    }
                    (OswinState::Walking, Some(*direction))
                } else {
                    velocity.x = 0.0;
                    self.state = OswinBehaviorState::Idle {
                        wait_timer: rand::random_range(0.0..2.0),
                    };
                    (OswinState::Idle, None)
                }
            }
        }
    }
}

impl<'a> GetCharacterAnimator<'a> for Oswin<'a> {
    fn animator(&self) -> &Animator<'a> {
        &self.animator
    }
}
