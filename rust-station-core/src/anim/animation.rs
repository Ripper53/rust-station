#[derive(Debug, Clone)]
pub struct Animation<'a> {
    frames: Vec<Frame<'a>>,
}

impl<'a> Animation<'a> {
    pub fn try_new(frames: Vec<Frame<'a>>) -> Result<Self, AnimationIsEmptyError> {
        if frames.is_empty() {
            Err(AnimationIsEmptyError)
        } else {
            Ok(Animation { frames })
        }
    }
    pub fn frames(&self) -> &[Frame<'a>] {
        self.frames.as_slice()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("animation should not be empty")]
pub struct AnimationIsEmptyError;

#[derive(Debug, Clone)]
pub struct Frame<'a> {
    src: &'a str,
    duration: AnimationDuration,
}

impl<'a> Frame<'a> {
    pub fn new(image_source: &'a str, duration: AnimationDuration) -> Self {
        Frame {
            src: image_source,
            duration,
        }
    }
    pub fn image_source(&self) -> &'a str {
        self.src
    }
    pub fn duration(&self) -> AnimationDuration {
        self.duration
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct AnimationDuration(f32);

impl AnimationDuration {
    pub const fn new(duration: f32) -> Self {
        AnimationDuration(duration)
    }
}

impl std::ops::AddAssign for AnimationDuration {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::SubAssign for AnimationDuration {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
