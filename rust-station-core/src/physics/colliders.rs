use crate::physics::Position;

#[derive(Debug, Copy, Clone, Default)]
pub struct BoxCollider {
    pub size_x: f32,
    pub size_y: f32,
}

impl BoxCollider {
    pub const fn new(size_x: f32, size_y: f32) -> Self {
        BoxCollider { size_x, size_y }
    }
    pub const fn with_position(self, position: Position) -> ColliderPosition<Self> {
        ColliderPosition {
            position,
            collider: self,
        }
    }
}

pub struct ColliderPosition<Collider> {
    position: Position,
    collider: Collider,
}

pub trait ColliderOverlap<Collider> {
    fn overlap(&self, other_collider: Collider) -> bool;
}

impl ColliderOverlap<Self> for ColliderPosition<BoxCollider> {
    fn overlap(&self, other_collider: Self) -> bool {
        let a_min_x = self.position.x;
        let a_max_x = self.position.x + self.collider.size_x;
        let a_min_y = self.position.y;
        let a_max_y = self.position.y + self.collider.size_y;

        let b_min_x = other_collider.position.x;
        let b_max_x = other_collider.position.x + other_collider.collider.size_x;
        let b_min_y = other_collider.position.y;
        let b_max_y = other_collider.position.y + other_collider.collider.size_y;

        a_min_x < b_max_x && a_max_x > b_min_x && a_min_y < b_max_y && a_max_y > b_min_y
    }
}
