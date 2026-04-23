use crate::{
    anim::{Animation, AnimationDuration, Animator, AnimatorState, Frame},
    characters::GetCharacterAnimator,
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
        let walking_animation = Animation::try_new(vec![
            Frame::new(
                "images/characters/oswin/OswinWalk0.png",
                AnimationDuration::new(0.125),
            ),
            Frame::new(
                "images/characters/oswin/OswinIdle.png",
                AnimationDuration::new(0.125),
            ),
            Frame::new(
                "images/characters/oswin/OswinWalk1.png",
                AnimationDuration::new(0.125),
            ),
            Frame::new(
                "images/characters/oswin/OswinIdle.png",
                AnimationDuration::new(0.125),
            ),
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

impl<'a> GetCharacterAnimator<'a> for Oswin<'a> {
    fn animator(&self) -> &Animator<'a> {
        &self.animator
    }
}
