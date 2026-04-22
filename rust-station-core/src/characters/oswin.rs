use crate::{
    anim::{Animation, AnimationDuration, Animator, AnimatorState, Frame},
    characters::GetCharacterAnimator,
};

#[derive(Debug)]
pub struct Oswin<'a> {
    pub animator: Animator<'a>,
}

impl<'a> Oswin<'a> {
    pub fn new() -> Self {
        Oswin {
            animator: Animator::new(
                AnimatorState::Loop,
                Animation::try_new(vec![
                    Frame::new(
                        "images/characters/oswin/OswinWalk0.png",
                        AnimationDuration::new(0.5),
                    ),
                    Frame::new(
                        "images/characters/oswin/OswinIdle.png",
                        AnimationDuration::new(0.5),
                    ),
                    Frame::new(
                        "images/characters/oswin/OswinWalk1.png",
                        AnimationDuration::new(0.5),
                    ),
                    Frame::new(
                        "images/characters/oswin/OswinIdle.png",
                        AnimationDuration::new(0.5),
                    ),
                ])
                .unwrap(),
            ),
        }
    }
}

impl<'a> GetCharacterAnimator<'a> for Oswin<'a> {
    fn animator(&self) -> &Animator<'a> {
        &self.animator
    }
}
