mod angle;
mod bounds;
mod colliders;
mod entity_builder;
mod history;
mod position;
mod speed;
mod target;
mod turret;
mod velocity;
use std::collections::HashSet;

pub use angle::*;
pub use bounds::*;
pub use colliders::*;
pub use entity_builder::*;
pub use history::*;
pub use position::*;
pub use speed::*;
pub use target::*;
pub use turret::*;
pub use velocity::*;

use crate::{DeltaTime, utility};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct EntityID(usize);

#[derive(Debug)]
pub struct World {
    bounds: Bounds,
    gravity: Gravity,
    pointer_position: Position,
    free_entity_ids: Vec<EntityID>,
    entities: Vec<Entities>,
    running_entities: Vec<Entities>,
    build_commands: Vec<BuildCommand>,
    destroy_commands: HashSet<EntityID>,
    history: std::collections::VecDeque<WorldHistory>,
}

#[derive(Debug)]
struct BuildCommand {
    entity_id: EntityID,
    entities: Entities,
}

#[derive(Debug, Default)]
pub(crate) struct Entities {
    tag: EntityTag,
    angle: RadiansAngle,
    position: Position,
    velocity: Velocity,
    box_collider: BoxCollider,
    target: Target,
    speed: Speed,
    projectile_speed: ProjectileSpeed,
    turret_state: TurretState,
}

#[derive(Debug, Default)]
struct EntityTag(usize);
impl EntityTag {
    pub const NONE: Self = EntityTag(0);
    pub const POSITION: Self = EntityTag(1 << 0);
    pub const VELOCITY: Self = EntityTag(1 << 1);
    pub const ANGLE: Self = EntityTag(1 << 2);
    pub const COLLIDER: Self = EntityTag(1 << 3);

    pub const TURRET: Self = EntityTag(1 << 4);
    pub const PROJECTILE: Self = EntityTag(1 << 5);
    pub const ENEMY: Self = EntityTag(1 << 6);
    pub const IS_DRAGGING: Self = EntityTag(1 << 7);

    pub const fn remove(&mut self, entity_tag: EntityTag) {
        self.0 &= !entity_tag.0;
    }
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

macro_rules! get_free_entity_id {
    ($world: ident, $latest_entities_length: ident) => {{
        if let Some(id) = $world.free_entity_ids.pop() {
            id
        } else {
            let entity_id = EntityID($latest_entities_length);
            $latest_entities_length += 1;
            entity_id
        }
    }};
    ($world: ident) => {{
        if let Some(id) = $world.free_entity_ids.pop() {
            id
        } else {
            let entity_id = EntityID($world.entities.len());
            $world.entities.push(Entities::default());
            entity_id
        }
    }};
}

macro_rules! entities_iter {
    ($t: ident) => {{
        let count = $t.running_entities.len();
        $t.running_entities
            .iter()
            .enumerate()
            .map(|(i, e)| (EntityID(i), e))
            .chain(
                $t.entities
                    .iter()
                    .rev()
                    .enumerate()
                    .map(move |(i, e)| (EntityID(count + i + 1), e)),
            )
    }};
}
impl World {
    pub fn new(bounds: Bounds, gravity: Gravity) -> Self {
        World {
            bounds,
            gravity,
            pointer_position: Position::new(0.0, 0.0),
            free_entity_ids: Vec::new(),
            entities: Vec::new(),
            running_entities: Vec::new(),
            build_commands: Vec::new(),
            destroy_commands: HashSet::new(),
            history: std::collections::VecDeque::new(),
        }
    }
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }
    pub fn builder(mut self) -> EntityBuilder<WithWorld> {
        let entity_id = get_free_entity_id!(self);
        EntityBuilder::<WithWorld>::new(self, entity_id)
    }
    pub fn is_dragging(&mut self, entity_id: EntityID) {
        if let Some(entities) = self.entities.get_mut(entity_id.0) {
            entities.tag |= EntityTag::IS_DRAGGING;
        }
    }
    pub fn drop(&mut self, entity_id: EntityID) {
        if let Some(entities) = self.entities.get_mut(entity_id.0) {
            entities.tag.remove(EntityTag::IS_DRAGGING)
        }
    }
    pub fn get_position_and_velocity_mut(
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
    pub fn get_position_mut(&mut self, entity_id: EntityID) -> Option<&mut Position> {
        if let Some(entities) = self.entities.get_mut(entity_id.0)
            && entities.tag.is_any(EntityTag::POSITION)
        {
            Some(&mut entities.position)
        } else {
            None
        }
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
    pub fn get_angle(&self, entity_id: EntityID) -> Option<RadiansAngle> {
        if let Some(entities) = self.entities.get(entity_id.0)
            && entities.tag.is_any(EntityTag::ANGLE)
        {
            Some(entities.angle)
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
    pub fn set_pointer_position(&mut self, position: Position) {
        self.pointer_position = position;
    }
    pub fn elapsed_duration(&mut self, delta_time: PhysicsDeltaTime) {
        if let Some(build_command) = self.build_commands.pop() {
            let entities = self.entities.get_mut(build_command.entity_id.0).unwrap();
            *entities = build_command.entities;
        }
        let mut latest_entities_length = self.entities.len();
        std::mem::swap(&mut self.entities, &mut self.running_entities);
        let mut i = self.running_entities.len();
        while let Some(mut entities) = self.running_entities.pop() {
            i -= 1;
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
                            if entities.tag.is_any(EntityTag::PROJECTILE) {
                                if self.destroy_commands.insert(EntityID(i)) {
                                    self.history
                                        .push_back(WorldHistory::DestroyProjectile(EntityID(i)));
                                }
                            }
                            if velocity.x > 0.0 {
                                velocity.x = 0.0;
                            }
                        }
                        new_pos
                    } else {
                        let mut new_pos = position.x + movement.x;
                        if new_pos <= 0.0 {
                            new_pos = 0.0;
                            if entities.tag.is_any(EntityTag::PROJECTILE) {
                                if self.destroy_commands.insert(EntityID(i)) {
                                    self.history
                                        .push_back(WorldHistory::DestroyProjectile(EntityID(i)));
                                }
                            }
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
                            if entities.tag.is_any(EntityTag::PROJECTILE) {
                                if self.destroy_commands.insert(EntityID(i)) {
                                    self.history
                                        .push_back(WorldHistory::DestroyProjectile(EntityID(i)));
                                }
                            }
                            if velocity.y > 0.0 {
                                velocity.y = 0.0;
                            }
                        }
                        new_pos
                    } else {
                        let mut new_pos = position.y + movement.y;
                        if new_pos <= 0.0 {
                            new_pos = 0.0;
                            if entities.tag.is_any(EntityTag::PROJECTILE) {
                                if self.destroy_commands.insert(EntityID(i)) {
                                    self.history
                                        .push_back(WorldHistory::DestroyProjectile(EntityID(i)));
                                }
                            }
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
                            if entities.tag.is_any(EntityTag::PROJECTILE) {
                                if self.destroy_commands.insert(EntityID(i)) {
                                    self.history
                                        .push_back(WorldHistory::DestroyProjectile(EntityID(i)));
                                }
                            }
                            if velocity.x > 0.0 {
                                velocity.x = 0.0;
                            }
                        }
                        new_pos
                    } else {
                        let mut new_pos = position.x + movement.x;
                        if new_pos <= 0.0 {
                            new_pos = 0.0;
                            if entities.tag.is_any(EntityTag::PROJECTILE) {
                                if self.destroy_commands.insert(EntityID(i)) {
                                    self.history
                                        .push_back(WorldHistory::DestroyProjectile(EntityID(i)));
                                }
                            }
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
                            if entities.tag.is_any(EntityTag::PROJECTILE) {
                                if self.destroy_commands.insert(EntityID(i)) {
                                    self.history
                                        .push_back(WorldHistory::DestroyProjectile(EntityID(i)));
                                }
                            }
                            if velocity.y > 0.0 {
                                velocity.y = 0.0;
                            }
                        }
                        new_pos
                    } else {
                        let mut new_pos = position.y + movement.y;
                        if new_pos <= 0.0 {
                            new_pos = 0.0;
                            if entities.tag.is_any(EntityTag::PROJECTILE) {
                                if self.destroy_commands.insert(EntityID(i)) {
                                    self.history
                                        .push_back(WorldHistory::DestroyProjectile(EntityID(i)));
                                }
                            }
                            if velocity.y < 0.0 {
                                velocity.y = 0.0;
                            }
                        }
                        new_pos
                    };
                    Position::new(x, y)
                };
            }
            if entities.tag.is_any(EntityTag::TURRET) {
                entities.position.y = self.bounds.height - 324.0;
                match entities.turret_state {
                    TurretState::FollowTarget {
                        mut look_around_cooldown,
                        mut shoot_cooldown,
                    } => {
                        look_around_cooldown =
                            (look_around_cooldown - delta_time.0.value()).max(0.0);
                        shoot_cooldown = (shoot_cooldown - delta_time.0.value()).max(0.0);
                        let position = entities.position;
                        let count = self.running_entities.len();
                        let target_available = if let Some(target) = entities_iter!(self)
                            .filter(|(_, entities)| entities.tag.is_any(EntityTag::ENEMY))
                            .map(move |(_, entities)| {
                                (
                                    entities.position,
                                    entities.position.distance_squared(position),
                                )
                            })
                            .reduce(|(position_a, distance_a), (position_b, distance_b)| {
                                if distance_a < distance_b {
                                    (position_a, distance_a)
                                } else {
                                    (position_b, distance_b)
                                }
                            })
                            .map(|(position, _)| position)
                        {
                            entities.target = Target::new(target + Position::new(4.0, 4.0));
                            true
                        } else {
                            if look_around_cooldown <= 0.0 {
                                look_around_cooldown = rand::random_range(1.0..4.0);
                                let angle = rand::random_range(
                                    std::f32::consts::PI..std::f32::consts::PI * 2.0,
                                );
                                entities.target = Target::new(
                                    entities.position + Position::new(angle.cos(), angle.sin()),
                                );
                            }
                            false
                        };
                        let target = entities.target;
                        let speed = entities.speed;
                        let angle = entities.angle;
                        let target_pos = Velocity::target(target.into(), position);
                        let target_angle = target_pos.y.atan2(target_pos.x);
                        entities.turret_state = if target_available
                            && (target_angle - angle.value())
                                .abs()
                                .rem_euclid(std::f32::consts::TAU)
                                < std::f32::consts::FRAC_1_PI
                            && shoot_cooldown <= 0.0
                        {
                            TurretState::Shoot
                        } else {
                            TurretState::FollowTarget {
                                look_around_cooldown,
                                shoot_cooldown,
                            }
                        };
                    }
                    TurretState::Shoot => {
                        let projectile_speed = entities.projectile_speed;
                        entities.turret_state = TurretState::FollowTarget {
                            look_around_cooldown: 0.0,
                            shoot_cooldown: 1.0 / 8.0,
                        };
                        let position = entities.position
                            + Position::new(24.0, 42.0)
                            + entities.angle.into_position() * 16.0;
                        let target_velocity =
                            entities.angle.into_position() * projectile_speed.into();
                        let target_velocity = Velocity::new(target_velocity.x, target_velocity.y);
                        let entity_id = get_free_entity_id!(self, latest_entities_length);
                        let entity_builder = EntityBuilder::<WithoutWorld>::new(entity_id);
                        let (entity_id, entities) = entity_builder
                            .projectile()
                            .add_position(position)
                            .add_collider(BoxCollider::new(16.0, 16.0))
                            .add_velocity(target_velocity)
                            .finish();
                        self.build_commands.push(BuildCommand {
                            entity_id,
                            entities,
                        });
                        self.history.push_back(WorldHistory::SpawnProjectile {
                            entity_id,
                            position,
                        });
                    }
                }
                let speed: f32 = entities.speed.into();
                let target_pos = Velocity::target(entities.target.into(), entities.position);
                let target_angle = target_pos.y.atan2(target_pos.x);
                entities.angle = RadiansAngle::new(utility::lerp_angle(
                    entities.angle.into(),
                    target_angle,
                    (delta_time.0.value() * speed).min(1.0).max(0.0),
                ));
            }

            if entities.tag.is_any(EntityTag::PROJECTILE) {
                let col = entities.box_collider.scale(2.0).with_position(
                    entities.position
                        - Position::new(
                            entities.box_collider.size_x * 0.5,
                            entities.box_collider.size_y * 0.5,
                        ),
                );
                for (ii, e) in entities_iter!(self).filter(|(_, e)| e.tag.is_any(EntityTag::ENEMY))
                {
                    if col.overlap(e.box_collider.with_position(e.position)) {
                        if self.destroy_commands.insert(EntityID(i)) {
                            self.history
                                .push_back(WorldHistory::DestroyProjectile(EntityID(i)));
                        }
                        if self.destroy_commands.insert(ii) {
                            self.history.push_back(WorldHistory::DestroyEnemy(ii));
                        }
                    }
                }
            }
            self.entities.push(entities);
        }
        self.entities.reverse();
        if self.entities.len() < latest_entities_length {
            let diff = latest_entities_length - self.entities.len();
            for _ in 0..diff {
                self.entities.push(Entities::default());
            }
        }
        for entity_id in self.destroy_commands.iter().copied() {
            self.entities[entity_id.0].tag = EntityTag::NONE;
            self.free_entity_ids.push(entity_id);
        }
        self.destroy_commands.clear();
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
    pub fn pop_history(&mut self) -> Option<WorldHistory> {
        self.history.pop_front()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct PhysicsDeltaTime(DeltaTime);
impl PhysicsDeltaTime {
    pub const fn new(duration: DeltaTime) -> Self {
        PhysicsDeltaTime(duration)
    }
}
