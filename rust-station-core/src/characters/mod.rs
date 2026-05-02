use crate::anim::Animator;

mod first_minion;
mod oswin;
mod turret;
pub use first_minion::*;
pub use oswin::*;
pub use turret::*;

pub trait GetCharacterAnimator<'a> {
    fn animator(&self) -> &Animator<'a>;
}

#[derive(Debug)]
pub struct Character {
    health: HealthType,
}

impl Character {
    pub fn new(health: HealthType) -> Character {
        Character { health }
    }
    pub fn damage(&mut self, damage: Damage) {
        match self.health {
            HealthType::Normal(ref mut health) => {
                health.0 -= damage.0;
            }
            HealthType::Infinite => {}
        }
    }
    pub fn health(&self) -> &HealthType {
        &self.health
    }
}

#[derive(Debug)]
pub enum HealthType {
    Normal(Health),
    Infinite,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Health(usize);
impl Health {
    pub fn new(health: usize) -> Health {
        Health(health)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Damage(usize);
impl Damage {
    pub fn new(damage: usize) -> Damage {
        Damage(damage)
    }
}
