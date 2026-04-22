use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct EntityID(usize);

#[derive(Debug)]
pub struct World {
    last_entity_id: usize,
    colliders: HashMap<EntityID, BoxCollider>,
}

#[derive(Debug)]
pub struct BoxCollider {
    size_x: f32,
    size_y: f32,
}

impl BoxCollider {
    pub fn new(size_x: f32, size_y: f32) -> Self {
        BoxCollider { size_x, size_y }
    }
}

impl World {
    pub fn new() -> Self {
        World {
            last_entity_id: 0,
            colliders: HashMap::new(),
        }
    }
    pub fn add_collider(&mut self, collider: BoxCollider) -> EntityID {
        let entity_id = EntityID(self.last_entity_id);
        self.last_entity_id += 1;
        let _ = self.colliders.insert(entity_id, collider);
        entity_id
    }
}
