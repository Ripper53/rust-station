use crate::physics::RadiansAngle;

#[derive(Debug)]
pub struct TrainTurrent {
    rotation: TurretRotation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TurretRotation(RadiansAngle);

impl TrainTurrent {
    pub fn new() -> Self {
        TrainTurrent {
            rotation: TurretRotation(RadiansAngle::new(0.0)),
        }
    }
    //pub fn rotation(&self) -> TurretRotation {}
    /*pub fn shoot(&mut self, world: World) -> World {
        let (world, entity_id) = world
            .builder()
            .add_position(self.position)
            .add_velocity(Velocity::from(self.rotation.0))
            .add_collider(BoxCollider::new(4.0, 4.0))
            .projectile()
            .finish();
        world
    }*/
}
