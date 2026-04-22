use rust_station_core::anim::{AnimationDuration, Animator, AnimatorUpdateResponse};
use web_sys::HtmlImageElement;

pub trait UpdateAnim<'a> {
    fn update(self, elapsed_duration: AnimationDuration) -> UpdateAnimator<'a>;
}

impl<'a> UpdateAnim<'a> for Animator<'a> {
    fn update(mut self, elapsed_duration: AnimationDuration) -> UpdateAnimator<'a> {
        match self.elapsed_duration(elapsed_duration) {
            AnimatorUpdateResponse::UnchangedAnimation => UpdateAnimator::ElapsedDuration(self),
            AnimatorUpdateResponse::NextFrame(_) => {
                UpdateAnimator::NewAnimation(PlayNewAnimation { animator: self })
            }
        }
    }
}

#[derive(Debug)]
pub enum UpdateAnimator<'a> {
    ElapsedDuration(Animator<'a>),
    NewAnimation(PlayNewAnimation<'a>),
}

#[derive(Debug)]
pub struct PlayNewAnimation<'a> {
    animator: Animator<'a>,
}

impl<'a> PlayNewAnimation<'a> {
    pub fn set_image(self, image: &mut HtmlImageElement) -> Animator<'a> {
        image.set_src(self.animator.current_frame().image_source());
        self.animator
    }
}
