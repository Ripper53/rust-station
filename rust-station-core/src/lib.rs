pub mod anim;
pub mod characters;
pub mod commands;
pub mod enemies;
pub mod physics;
pub mod train;
pub mod utility;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DeltaTime(f32);
impl DeltaTime {
    pub const fn new(delta_time: f32) -> Self {
        DeltaTime(delta_time)
    }
    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Debug)]
pub struct RustStation {}
