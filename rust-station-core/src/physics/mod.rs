use std::collections::HashMap;

mod bounds;
mod colliders;
mod entity_builder;
mod position;
mod velocity;
pub use bounds::*;
pub use colliders::*;
pub use entity_builder::*;
pub use position::*;
pub use velocity::*;

use crate::DeltaTime;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct EntityID(usize);

#[derive(Debug)]
pub struct World {
    last_entity_id: usize,
    bounds: Bounds,
    gravity: Gravity,
    static_positions: HashMap<EntityID, Position>,
    positions_and_velocities: HashMap<EntityID, (Position, Velocity)>,
    colliders: HashMap<EntityID, BoxCollider>,
}

#[derive(Debug)]
pub struct Gravity(Velocity);
impl Gravity {
    pub const fn new(velocity: Velocity) -> Self {
        Gravity(velocity)
    }
}

impl World {
    pub fn new(bounds: Bounds, gravity: Gravity) -> Self {
        World {
            last_entity_id: 0,
            bounds,
            gravity,
            static_positions: HashMap::new(),
            positions_and_velocities: HashMap::new(),
            colliders: HashMap::new(),
        }
    }
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }
    pub fn builder(mut self) -> EntityBuilder {
        let entity_id = EntityID(self.last_entity_id);
        self.last_entity_id += 1;
        EntityBuilder::new(self, entity_id)
    }
    pub fn get_position(&self, entity_id: EntityID) -> Option<Position> {
        if let Some(position) = self.static_positions.get(&entity_id) {
            Some(*position)
        } else if let Some((position, ..)) = self.positions_and_velocities.get(&entity_id) {
            Some(*position)
        } else {
            None
        }
    }
    pub fn remove_entity(&mut self, entity_id: EntityID) {
        let _ = self.static_positions.remove(&entity_id);
        let _ = self.positions_and_velocities.remove(&entity_id);
        let _ = self.colliders.remove(&entity_id);
    }
    pub fn get_collider(&self, entity_id: EntityID) -> Option<BoxCollider> {
        self.colliders.get(&entity_id).copied()
    }
    pub fn get_dynamic_position(&self, entity_id: EntityID) -> Option<(Position, Velocity)> {
        self.positions_and_velocities.get(&entity_id).copied()
    }
    pub fn get_dynamic_positions_mut(
        &mut self,
        entity_id: EntityID,
    ) -> Option<&mut (Position, Velocity)> {
        self.positions_and_velocities.get_mut(&entity_id)
    }
    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = bounds;
        for (entity_id, position) in self.static_positions.iter_mut() {
            *position = if let Some(collider) = self.colliders.get(entity_id) {
                Position::new(
                    (position.x + collider.size_x)
                        .min(self.bounds.width)
                        .max(0.0),
                    (position.y + collider.size_y)
                        .min(self.bounds.height)
                        .max(0.0),
                )
            } else {
                Position::new(
                    position.x.min(self.bounds.width).max(0.0),
                    position.y.min(self.bounds.height).max(0.0),
                )
            };
        }
        for (entity_id, (position, ..)) in self.positions_and_velocities.iter_mut() {
            *position = if let Some(collider) = self.colliders.get(entity_id) {
                Position::new(
                    position.x.min(self.bounds.width - collider.size_x).max(0.0),
                    position
                        .y
                        .min(self.bounds.height - collider.size_y)
                        .max(0.0),
                )
            } else {
                Position::new(
                    position.x.min(self.bounds.width).max(0.0),
                    position.y.min(self.bounds.height).max(0.0),
                )
            };
        }
    }
    pub fn elapsed_duration(&mut self, delta_time: PhysicsDeltaTime) {
        for (entity_id, (position, velocity)) in self.positions_and_velocities.iter_mut() {
            *velocity += self.gravity.0 * delta_time.0.value();
            let movement = *velocity * delta_time.0.value();
            *position = if let Some(collider) = self.colliders.get(entity_id) {
                let x = if movement.x > 0.0 {
                    let mut new_pos = position.x + movement.x;
                    let max = self.bounds.width - collider.size_x;
                    if new_pos >= max {
                        new_pos = max;
                        if velocity.x > 0.0 {
                            velocity.x = 0.0;
                        }
                    }
                    new_pos
                } else {
                    let mut new_pos = position.x + movement.x;
                    if new_pos <= 0.0 {
                        new_pos = 0.0;
                        if velocity.x < 0.0 {
                            velocity.x = 0.0;
                        }
                    }
                    new_pos
                };
                let y = if movement.y > 0.0 {
                    let mut new_pos = position.y + movement.y;
                    let max = self.bounds.height - collider.size_y;
                    if new_pos >= max {
                        new_pos = max;
                        if velocity.y > 0.0 {
                            velocity.y = 0.0;
                        }
                    }
                    new_pos
                } else {
                    let mut new_pos = position.y + movement.y;
                    if new_pos <= 0.0 {
                        new_pos = 0.0;
                        if velocity.y < 0.0 {
                            velocity.y = 0.0;
                        }
                    }
                    new_pos
                };
                Position::new(x, y)
            } else {
                let x = if movement.x > 0.0 {
                    let mut new_pos = position.x + movement.x;
                    if new_pos >= self.bounds.width {
                        new_pos = self.bounds.width;
                        if velocity.x > 0.0 {
                            velocity.x = 0.0;
                        }
                    }
                    new_pos
                } else {
                    let mut new_pos = position.x + movement.x;
                    if new_pos <= 0.0 {
                        new_pos = 0.0;
                        if velocity.x < 0.0 {
                            velocity.x = 0.0;
                        }
                    }
                    new_pos
                };
                let y = if movement.y > 0.0 {
                    let mut new_pos = position.y + movement.y;
                    if new_pos >= self.bounds.height {
                        new_pos = self.bounds.height;
                        if velocity.y > 0.0 {
                            velocity.y = 0.0;
                        }
                    }
                    new_pos
                } else {
                    let mut new_pos = position.y + movement.y;
                    if new_pos <= 0.0 {
                        new_pos = 0.0;
                        if velocity.y < 0.0 {
                            velocity.y = 0.0;
                        }
                    }
                    new_pos
                };
                Position::new(x, y)
            };
        }
    }
    pub fn nearest_target(&self, position: Position) -> Option<Position> {
        let mut nearest_target: Option<(Position, DistanceSquared)> = None;
        for new_pos in self.static_positions.values().copied() {
            if let Some((nt, dis)) = &mut nearest_target {
                if nt.distance_squared(new_pos) < *dis {
                    *nt = new_pos;
                }
            } else {
                nearest_target = Some((new_pos, position.distance_squared(new_pos)));
            }
        }
        for (new_pos, ..) in self.positions_and_velocities.values() {
            let new_pos = *new_pos;
            if let Some((nt, dis)) = &mut nearest_target {
                if nt.distance_squared(new_pos) < *dis {
                    *nt = new_pos;
                }
            } else {
                nearest_target = Some((new_pos, position.distance_squared(new_pos)));
            }
        }
        nearest_target.map(|(pos, _dis)| pos)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct PhysicsDeltaTime(DeltaTime);
impl PhysicsDeltaTime {
    pub const fn new(duration: DeltaTime) -> Self {
        PhysicsDeltaTime(duration)
    }
}
