use rand::{RngExt, distr::uniform::SampleRange};

#[derive(Debug)]
pub struct TrainBounce<R> {
    state: State,
    pause_time: R,
    pause_timer: f32,
}

#[derive(Debug)]
enum State {
    Idle,
    Bounce,
}

impl<R: SampleRange<f32> + Clone> TrainBounce<R> {
    pub fn new(pause_time: R) -> Self {
        TrainBounce {
            state: State::Idle,
            pause_time,
            pause_timer: 0.0,
        }
    }
    pub fn update(&mut self, delta_time: f32) -> BounceUpdateResponse {
        match self.state {
            State::Idle => {
                if self.pause_timer > 0.0 {
                    self.pause_timer -= delta_time;
                    BounceUpdateResponse::Idle
                } else {
                    self.state = State::Bounce;
                    self.pause_timer = 0.1;
                    BounceUpdateResponse::Bounce
                }
            }
            State::Bounce => {
                if self.pause_timer > 0.0 {
                    self.pause_timer -= delta_time;
                    BounceUpdateResponse::Bounce
                } else {
                    self.state = State::Idle;
                    self.pause_timer = rand::rng().random_range(self.pause_time.clone());
                    BounceUpdateResponse::Idle
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BounceUpdateResponse {
    Idle,
    Bounce,
}
