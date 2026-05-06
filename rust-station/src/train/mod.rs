use rand::distr::uniform::SampleRange;
use rust_station_core::DeltaTime;
use web_sys::HtmlElement;

mod bounce;
mod turret;
pub use bounce::*;
pub use turret::*;

#[derive(Debug)]
pub struct TrainCartVisual<R> {
    holder: HtmlElement,
    bounce: TrainBounce<R>,
}

impl<R: SampleRange<f32> + Clone> TrainCartVisual<R> {
    pub fn new(holder: HtmlElement, bounce: TrainBounce<R>) -> Self {
        TrainCartVisual { holder, bounce }
    }
    pub fn holder(&self) -> &HtmlElement {
        &self.holder
    }
    pub fn pos_x(&self) -> f32 {
        self.holder.get_bounding_client_rect().x() as f32
    }
    pub fn pos_y(&self) -> f32 {
        self.holder.get_bounding_client_rect().y() as f32
    }
    pub fn width(&self) -> f32 {
        self.holder.get_bounding_client_rect().width() as f32
    }
    pub fn height(&self) -> f32 {
        self.holder.get_bounding_client_rect().height() as f32
    }
    pub fn update(&mut self, delta_time: DeltaTime) {
        match self.bounce.update(delta_time) {
            BounceUpdateResponse::Unchanged => {}
            BounceUpdateResponse::Idle => {
                self.holder.style().set_property("top", "0px").unwrap();
            }
            BounceUpdateResponse::Bounce => {
                self.holder.style().set_property("top", "-4px").unwrap();
            }
        }
    }
}
