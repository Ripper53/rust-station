use crate::physics::{BoxCollider, Entities, EntityID, EntityTag, Position, Velocity, World};

#[derive(Debug)]
pub struct EntityBuilder {
    world: World,
    entity_id: EntityID,
    entities: Entities,
}

impl EntityBuilder {
    pub(crate) const fn new(world: World, entity_id: EntityID) -> Self {
        EntityBuilder {
            world,
            entity_id,
            entities: Entities {
                tag: EntityTag::NONE,
                position: Position::new(0.0, 0.0),
                velocity: Velocity::new(0.0, 0.0),
                box_collider: BoxCollider::new(0.0, 0.0),
            },
        }
    }
    pub fn add_static_position(mut self, position: Position) -> Self {
        self.entities.tag |= EntityTag::POSITION;
        self.entities.position = position;
        self
    }
    pub fn add_position_with_velocity(mut self, position: Position, velocity: Velocity) -> Self {
        self.entities.tag |= EntityTag::POSITION | EntityTag::VELOCITY;
        self.entities.position = position;
        self.entities.velocity = velocity;
        self
    }
    pub fn add_collider(mut self, collider: BoxCollider) -> Self {
        self.entities.tag |= EntityTag::COLLIDER;
        self.entities.box_collider = collider;
        self
    }
    pub fn finish(mut self) -> (World, EntityID) {
        if self.world.entities.len() == self.entity_id.0 {
            self.world.entities.push(self.entities);
        } else {
            let entities = self.world.entities.get_mut(self.entity_id.0).unwrap();
            *entities = self.entities;
        }
        (self.world, self.entity_id)
    }
}
