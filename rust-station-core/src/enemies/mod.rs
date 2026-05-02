use crate::{
    anim::{Animation, AnimationDuration, Animator, AnimatorState, Frame},
    characters::{Character, Health, HealthType},
};

pub mod wave;

pub fn create_first_minion<'a>() -> (Animator<'a>, Character) {
    let animator = Animator::new(
        AnimatorState::Loop,
        Animation::try_new(vec![
            Frame::new(
                "images/characters/first_minion/FirstMinion0.png",
                AnimationDuration::new(0.25),
            ),
            Frame::new(
                "images/characters/first_minion/FirstMinion1.png",
                AnimationDuration::new(0.25),
            ),
        ])
        .unwrap(),
    );
    (animator, Character::new(HealthType::Normal(Health::new(1))))
}
