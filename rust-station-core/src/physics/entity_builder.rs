use crate::physics::{
    BoxCollider, Entities, EntityID, EntityTag, Position, ProjectileSpeed, RadiansAngle, Speed,
    Target, Velocity, World,
};

#[derive(Debug)]
pub struct EntityBuilder<World> {
    world: World,
    entity_id: EntityID,
    entities: Entities,
}
pub struct WithWorld(World);
pub struct WithoutWorld(());

impl<World> EntityBuilder<World> {
    pub fn add_static_position(mut self, position: Position) -> Self {
        self.entities.tag |= EntityTag::POSITION;
        self.entities.position = position;
        self
    }
    pub fn add_position(mut self, position: Position) -> Self {
        self.entities.tag |= EntityTag::POSITION;
        self.entities.position = position;
        self
    }
    pub fn add_velocity(mut self, velocity: Velocity) -> Self {
        self.entities.tag |= EntityTag::VELOCITY;
        self.entities.velocity = velocity;
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
    pub fn add_enemy(mut self) -> Self {
        self.entities.tag |= EntityTag::ENEMY;
        self
    }
    pub fn turret(mut self) -> TurretBuilderRequiresPosition<World> {
        self.entities.tag |= EntityTag::TURRET;
        TurretBuilderRequiresPosition {
            entity_builder: self,
        }
    }
    pub fn projectile(mut self) -> Self {
        self.entities.tag |= EntityTag::PROJECTILE;
        self
    }
}
impl EntityBuilder<WithWorld> {
    pub(crate) const fn new(world: World, entity_id: EntityID) -> Self {
        EntityBuilder {
            world: WithWorld(world),
            entity_id,
            entities: Entities {
                tag: EntityTag::NONE,
                position: Position::new(0.0, 0.0),
                velocity: Velocity::new(0.0, 0.0),
                box_collider: BoxCollider::new(0.0, 0.0),
                angle: RadiansAngle::new(0.0),
                target: Target::new(Position::new(0.0, 0.0)),
                speed: Speed::new(0.0),
                projectile_speed: ProjectileSpeed::new(Speed::new(0.0)),
                turret_state: super::TurretState::FollowTarget,
            },
        }
    }
    pub fn finish(self) -> (World, EntityID) {
        let mut world = self.world.0;
        if world.entities.len() == self.entity_id.0 {
            world.entities.push(self.entities);
        } else {
            let entities = world.entities.get_mut(self.entity_id.0).unwrap();
            *entities = self.entities;
        }
        (world, self.entity_id)
    }
}
impl EntityBuilder<WithoutWorld> {
    pub(crate) fn new(entity_id: EntityID) -> Self {
        EntityBuilder {
            world: WithoutWorld(()),
            entity_id,
            entities: Entities::default(),
        }
    }
    pub(crate) fn finish(self) -> (EntityID, Entities) {
        (self.entity_id, self.entities)
    }
}

#[derive(Debug)]
pub struct TurretBuilderRequiresPosition<World> {
    entity_builder: EntityBuilder<World>,
}

impl<World> TurretBuilderRequiresPosition<World> {
    pub fn add_position(mut self, position: Position) -> TurretBuilderRequiresAngle<World> {
        self.entity_builder.entities.tag |= EntityTag::POSITION;
        self.entity_builder.entities.position = position;
        TurretBuilderRequiresAngle {
            entity_builder: self.entity_builder,
        }
    }
}

#[derive(Debug)]
pub struct TurretBuilderRequiresAngle<World> {
    entity_builder: EntityBuilder<World>,
}

impl<World> TurretBuilderRequiresAngle<World> {
    pub fn add_angle(mut self, angle: RadiansAngle) -> TurretBuilderRequiresRotationSpeed<World> {
        self.entity_builder.entities.tag |= EntityTag::ANGLE;
        self.entity_builder.entities.angle = angle;
        TurretBuilderRequiresRotationSpeed {
            entity_builder: self.entity_builder,
        }
    }
}

#[derive(Debug)]
pub struct TurretBuilderRequiresRotationSpeed<World> {
    entity_builder: EntityBuilder<World>,
}

impl<World> TurretBuilderRequiresRotationSpeed<World> {
    pub fn add_rotation_speed(
        mut self,
        speed: Speed,
    ) -> TurretBuilderRequiresProjectileSpeed<World> {
        self.entity_builder.entities.speed = speed;
        TurretBuilderRequiresProjectileSpeed {
            entity_builder: self.entity_builder,
        }
    }
}

#[derive(Debug)]
pub struct TurretBuilderRequiresProjectileSpeed<World> {
    entity_builder: EntityBuilder<World>,
}

impl<World> TurretBuilderRequiresProjectileSpeed<World> {
    pub fn add_projectile_speed(
        mut self,
        projectile_speed: ProjectileSpeed,
    ) -> EntityBuilder<World> {
        self.entity_builder.entities.projectile_speed = projectile_speed;
        self.entity_builder
    }
}
