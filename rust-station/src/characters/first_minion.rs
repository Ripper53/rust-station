use rust_station_core::{anim::AnimationDeltaTime, characters::FirstMinion};
use web_sys::HtmlImageElement;

use crate::anim::{UpdateAnim, UpdateAnimator};

pub trait FirstMinionUpdate {
    fn update(self, elapsed_duration: AnimationDeltaTime, image: &mut HtmlImageElement) -> Self;
}

impl<'a> FirstMinionUpdate for FirstMinion<'a> {
    fn update(
        mut self,
        elapsed_duration: AnimationDeltaTime,
        image: &mut HtmlImageElement,
    ) -> Self {
        self.animator = match self.animator.update(elapsed_duration) {
            UpdateAnimator::ElapsedDuration(animator) => animator,
            UpdateAnimator::NewAnimation(new_animation) => new_animation.set_image(image),
        };
        self
    }
}
