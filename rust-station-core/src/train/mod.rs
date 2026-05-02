use crate::{DeltaTime, physics::PhysicsDeltaTime};

mod cart;
pub use cart::*;

#[derive(Debug)]
pub struct Train {
    carts: Vec<TrainCart>,
    active_card_index: usize,
}

impl Train {
    pub const fn new(carts: Vec<TrainCart>) -> Self {
        Train {
            carts,
            active_card_index: 0,
        }
    }
    pub fn update(&mut self, delta_time: PhysicsDeltaTime) {
        if let Some(train_cart) = self.carts.get_mut(self.active_card_index) {
            train_cart.world_mut().elapsed_duration(delta_time);
        }
    }
}

#[derive(Debug)]
pub struct TrainBackground<const BACKGROUND_LAYER_COUNT: usize> {
    background_position_x: [f32; BACKGROUND_LAYER_COUNT],
    background_max_position_x: f32,
    speed: f32,
    // Should be a value lower than 1.
    parallax_amount: f32,
}

impl<const BACKGROUND_LAYER_COUNT: usize> TrainBackground<BACKGROUND_LAYER_COUNT> {
    pub fn new(max_x: f32, speed: f32, parallax_amount: f32, start_pos: f32) -> Self {
        TrainBackground {
            background_position_x: [start_pos; BACKGROUND_LAYER_COUNT],
            background_max_position_x: max_x,
            speed,
            parallax_amount,
        }
    }
    pub fn elapsed_duration(
        &mut self,
        delta_time: DeltaTime,
    ) -> impl Iterator<Item = ParallaxUpdateResponse> {
        struct I<'a, II: Iterator<Item = (usize, &'a mut f32)>> {
            iter: II,
            delta_time: DeltaTime,
            speed: f32,
            parallax_amount: f32,
            background_max_position_x: f32,
        }
        impl<'a, II: Iterator<Item = (usize, &'a mut f32)>> Iterator for I<'a, II> {
            type Item = ParallaxUpdateResponse;
            fn next(&mut self) -> Option<Self::Item> {
                if let Some((i, x)) = self.iter.next() {
                    let new_x = *x
                        + self.speed
                            * self.delta_time.value()
                            * ((i + 1) as f32 * self.parallax_amount);
                    if new_x > self.background_max_position_x {
                        *x = new_x % self.background_max_position_x;
                        Some(ParallaxUpdateResponse::RestartAtPosition(*x))
                    } else {
                        *x = new_x;
                        Some(ParallaxUpdateResponse::UpdatePosition(*x))
                    }
                } else {
                    None
                }
            }
        }
        I {
            iter: self.background_position_x.iter_mut().enumerate(),
            delta_time,
            speed: self.speed,
            parallax_amount: self.parallax_amount,
            background_max_position_x: self.background_max_position_x,
        }
    }
    pub fn background_max_position_x(&self) -> f32 {
        self.background_max_position_x
    }
    pub fn set_background_max_position_x(&mut self, max_x: f32) {
        self.background_max_position_x = max_x;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParallaxUpdateResponse {
    UpdatePosition(f32),
    RestartAtPosition(f32),
}
