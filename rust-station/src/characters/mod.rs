use rust_station_core::characters::GetCharacterAnimator;
use web_sys::HtmlImageElement;

mod first_minion;
mod oswin;
pub use first_minion::*;
pub use oswin::*;

#[derive(Debug)]
pub struct AnimatedCharacter<T> {
    pub image: HtmlImageElement,
    pub character: T,
}

impl<'a, T: GetCharacterAnimator<'a>> AnimatedCharacter<T> {
    pub fn new(image: HtmlImageElement, character: T) -> Self {
        let frame = character.animator().current_frame();
        image.set_src(frame.image_source());
        AnimatedCharacter { image, character }
    }
}
