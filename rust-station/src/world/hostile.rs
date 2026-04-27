use rust_station_core::physics::{Bounds, Gravity, PhysicsDuration, Velocity, World};

#[derive(Debug)]
pub struct HostileWorld {
    world: World,
}

impl HostileWorld {
    pub fn new(bounds: Bounds) -> Self {
        let world = World::new(bounds, Gravity::new(Velocity::new(0.0, 1028.0)));
        HostileWorld { world }
    }
    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.world.set_bounds(bounds)
    }
    pub fn update(&mut self, delta_time: f32) {
        self.world
            .elapsed_duration(PhysicsDuration::new(delta_time));
    }
}
