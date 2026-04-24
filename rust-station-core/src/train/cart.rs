use crate::physics::World;

#[derive(Debug)]
pub struct TrainCart {
    world: World,
}

impl TrainCart {
    pub const fn new(world: World) -> Self {
        TrainCart { world }
    }
    pub fn world(&self) -> &World {
        &self.world
    }
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
