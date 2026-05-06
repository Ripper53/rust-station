use crate::physics::{EntityID, Position};

#[derive(Debug)]
pub enum WorldHistory {
    SpawnProjectile {
        entity_id: EntityID,
        position: Position,
    },
    DestroyProjectile(EntityID),
    DestroyEnemy(EntityID),
}
