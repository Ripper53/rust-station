use rust_station_core::{anim::AnimationDuration, characters::Oswin};
use web_sys::HtmlImageElement;

use crate::anim::{UpdateAnim, UpdateAnimator};

pub trait OswinUpdate {
    fn update(self, elapsed_duration: AnimationDuration, image: &mut HtmlImageElement) -> Self;
}

impl<'a> OswinUpdate for Oswin<'a> {
    fn update(mut self, elapsed_duration: AnimationDuration, image: &mut HtmlImageElement) -> Self {
        self.animator = match self.animator.update(elapsed_duration) {
            UpdateAnimator::ElapsedDuration(animator) => animator,
            UpdateAnimator::NewAnimation(new_animation) => new_animation.set_image(image),
        };
        self
    }
}
