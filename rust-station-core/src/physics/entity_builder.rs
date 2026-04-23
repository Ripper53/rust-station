use crate::physics::{BoxCollider, EntityID, Position, Velocity, World};

#[derive(Debug)]
pub struct EntityBuilder {
    world: World,
    entity_id: EntityID,
}

impl EntityBuilder {
    pub const fn new(world: World, entity_id: EntityID) -> Self {
        EntityBuilder { world, entity_id }
    }
    pub fn add_static_position(mut self, position: Position) -> Self {
        let _ = self.world.static_positions.insert(self.entity_id, position);
        self
    }
    pub fn add_position_with_velocity(mut self, position: Position, velocity: Velocity) -> Self {
        let _ = self
            .world
            .positions_and_velocities
            .insert(self.entity_id, (position, velocity));
        self
    }
    pub fn add_collider(mut self, collider: BoxCollider) -> Self {
        let _ = self.world.colliders.insert(self.entity_id, collider);
        self
    }
    pub fn finish(self) -> (World, EntityID) {
        (self.world, self.entity_id)
    }
}
