use rand::distr::uniform::SampleRange;
use web_sys::HtmlElement;

pub use bounce::*;
mod bounce;

#[derive(Debug)]
pub struct TrainCartVisual<R> {
    holder: HtmlElement,
    bounce: TrainBounce<R>,
}

impl<R: SampleRange<f32> + Clone> TrainCartVisual<R> {
    pub fn new(holder: HtmlElement, bounce: TrainBounce<R>) -> Self {
        TrainCartVisual { holder, bounce }
    }
    pub fn update(&mut self, delta_time: f32) {
        match self.bounce.update(delta_time) {
            BounceUpdateResponse::Idle => {
                self.holder.style().set_property("top", "0px").unwrap();
            }
            BounceUpdateResponse::Bounce => {
                self.holder.style().set_property("top", "-8px").unwrap();
            }
        }
    }
}
