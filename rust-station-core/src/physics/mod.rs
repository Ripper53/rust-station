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
    bounds: Bounds,
    gravity: Gravity,
    free_entity_ids: Vec<EntityID>,
    entities: Vec<Entities>,
}

#[derive(Debug)]
struct Entities {
    tag: EntityTag,
    position: Position,
    velocity: Velocity,
    box_collider: BoxCollider,
}

#[derive(Debug)]
struct EntityTag(usize);
impl EntityTag {
    pub const NONE: Self = EntityTag(0);
    pub const POSITION: Self = EntityTag(1 << 0);
    pub const VELOCITY: Self = EntityTag(1 << 1);
    pub const COLLIDER: Self = EntityTag(1 << 2);
    pub const fn is_any(&self, entity_tag: EntityTag) -> bool {
        (self.0 & entity_tag.0) != 0
    }
    pub const fn is_all(&self, entity_tag: EntityTag) -> bool {
        (self.0 & entity_tag.0) == entity_tag.0
    }
}

impl std::ops::BitOr for EntityTag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        EntityTag(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for EntityTag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
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
            bounds,
            gravity,
            free_entity_ids: Vec::new(),
            entities: Vec::new(),
        }
    }
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }
    pub fn builder(mut self) -> EntityBuilder {
        let id = if let Some(id) = self.free_entity_ids.pop() {
            id
        } else {
            EntityID(self.entities.len())
        };
        EntityBuilder::new(self, id)
    }
    pub fn get_position(&self, entity_id: EntityID) -> Option<Position> {
        if let Some(entities) = self.entities.get(entity_id.0)
            && entities.tag.is_any(EntityTag::POSITION)
        {
            Some(entities.position)
        } else {
            None
        }
    }
    pub fn remove_entity(&mut self, entity_id: EntityID) {
        let entities = self.entities.get_mut(entity_id.0).unwrap();
        entities.tag = EntityTag::NONE;
        self.free_entity_ids.push(entity_id);
    }
    pub fn get_collider(&self, entity_id: EntityID) -> Option<BoxCollider> {
        if let Some(entities) = self.entities.get(entity_id.0)
            && entities.tag.is_any(EntityTag::COLLIDER)
        {
            Some(entities.box_collider)
        } else {
            None
        }
    }
    pub fn get_dynamic_position(&self, entity_id: EntityID) -> Option<(Position, Velocity)> {
        if let Some(entities) = self.entities.get(entity_id.0)
            && entities
                .tag
                .is_all(EntityTag::POSITION | EntityTag::VELOCITY)
        {
            Some((entities.position, entities.velocity))
        } else {
            None
        }
    }
    pub fn get_dynamic_positions_mut(
        &mut self,
        entity_id: EntityID,
    ) -> Option<(&mut Position, &mut Velocity)> {
        if let Some(entities) = self.entities.get_mut(entity_id.0)
            && entities
                .tag
                .is_all(EntityTag::POSITION | EntityTag::VELOCITY)
        {
            Some((&mut entities.position, &mut entities.velocity))
        } else {
            None
        }
    }
    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = bounds;
        for entities in self
            .entities
            .iter_mut()
            .filter(|e| e.tag.is_any(EntityTag::POSITION))
        {
            entities.position = if entities.tag.is_any(EntityTag::COLLIDER) {
                Position::new(
                    (entities.position.x + entities.box_collider.size_x)
                        .min(self.bounds.width)
                        .max(0.0),
                    (entities.position.y + entities.box_collider.size_y)
                        .min(self.bounds.height)
                        .max(0.0),
                )
            } else {
                Position::new(
                    entities.position.x.min(self.bounds.width).max(0.0),
                    entities.position.y.min(self.bounds.height).max(0.0),
                )
            };
        }
    }
    pub fn elapsed_duration(&mut self, delta_time: PhysicsDeltaTime) {
        for entities in self.entities.iter_mut() {
            if entities
                .tag
                .is_all(EntityTag::POSITION | EntityTag::VELOCITY)
            {
                let position = &mut entities.position;
                let velocity = &mut entities.velocity;
                *velocity += self.gravity.0 * delta_time.0.value();
                let movement = *velocity * delta_time.0.value();
                *position = if entities.tag.is_any(EntityTag::COLLIDER) {
                    let collider = &entities.box_collider;
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
    }
    pub fn nearest_target(&self, position: Position) -> Option<Position> {
        let mut nearest_target: Option<(Position, DistanceSquared)> = None;
        for entities in self
            .entities
            .iter()
            .filter(|entities| entities.tag.is_any(EntityTag::POSITION))
        {
            let new_pos = entities.position;
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
