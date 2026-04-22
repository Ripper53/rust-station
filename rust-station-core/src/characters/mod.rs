mod oswin;
pub use oswin::*;

use crate::anim::Animator;

pub trait GetCharacterAnimator<'a> {
    fn animator(&self) -> &Animator<'a>;
}
