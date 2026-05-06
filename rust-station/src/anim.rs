use rust_station_core::{
    DeltaTime,
    anim::{AnimationDeltaTime, Animator, AnimatorUpdateResponse},
};
use web_sys::HtmlImageElement;

pub trait UpdateAnim<'a> {
    fn update(self, elapsed_duration: AnimationDeltaTime) -> UpdateAnimator<'a>;
}

impl<'a> UpdateAnim<'a> for Animator<'a> {
    fn update(mut self, elapsed_duration: AnimationDeltaTime) -> UpdateAnimator<'a> {
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

#[derive(Debug)]
pub struct DamageFlashAnimator {
    img: HtmlImageElement,
    opacity: f32,
}

impl DamageFlashAnimator {
    pub fn new(img: HtmlImageElement) -> Self {
        DamageFlashAnimator { img, opacity: 0.0 }
    }
    pub fn play(&mut self) {
        self.opacity = 1.0;
        self.img.style().set_property("opacity", "1").unwrap();
    }
    pub fn update(&mut self, delta_time: DeltaTime) {
        self.opacity = (self.opacity - delta_time.value() * 8.0).max(0.0);
        self.img
            .style()
            .set_property("opacity", &self.opacity.to_string())
            .unwrap();
    }
}
