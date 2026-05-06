use rust_station_core::{
    DeltaTime,
    anim::AnimationDeltaTime,
    characters::{Oswin, OswinBehavior, WalkingDirection},
    physics::{
        Bounds, BoxCollider, EntityID, Gravity, PhysicsDeltaTime, Position, Velocity, World,
    },
};
use web_sys::{HtmlElement, HtmlImageElement};

use crate::characters::{AnimatedCharacter, OswinUpdate};

pub mod hostile;
mod projectile;
pub use projectile::*;

pub fn create_world<'a, 'r>(
    body: &'a HtmlElement,
    bounds: Bounds,
    offset: Position,
) -> WorldRenderer<'r> {
    let world = World::new(bounds, Gravity::new(Velocity::new(0.0, 1028.0)));
    WorldRenderer::new(world, body, offset)
}

pub struct WorldRenderer<'a> {
    world: World,
    oswins: Vec<(EntityID, AnimatedCharacter<Oswin<'a>>, OswinBehavior)>,
    oswins_switch: Vec<(EntityID, AnimatedCharacter<Oswin<'a>>, OswinBehavior)>,
    offset: Position,
}

impl<'a> WorldRenderer<'a> {
    pub fn new(mut world: World, body: &HtmlElement, offset: Position) -> Self {
        const OSWINS_COUNT: usize = 10;
        let mut oswins = Vec::with_capacity(OSWINS_COUNT);
        for _ in 0..OSWINS_COUNT {
            let i = HtmlImageElement::new().unwrap();
            i.class_list().add_2("character", "oswin").unwrap();
            body.append_child(&i).unwrap();
            let oswin = AnimatedCharacter::new(i, Oswin::new());
            let (w, entity_id) = world
                .builder()
                .add_position_with_velocity(Position::new(0.0, 0.0), Velocity::new(0.0, 0.0))
                .add_collider(BoxCollider::new(16.0, 32.0))
                .finish();
            world = w;
            oswins.push((entity_id, oswin, OswinBehavior::new(96.0, 128.0)));
        }
        let oswins_switch = Vec::with_capacity(OSWINS_COUNT);
        WorldRenderer {
            world,
            oswins,
            oswins_switch,
            offset,
        }
    }
    pub fn set_offset(&mut self, position: Position) {
        self.offset = position;
    }
    pub fn update(&mut self, delta_time: DeltaTime) {
        self.world
            .elapsed_duration(PhysicsDeltaTime::new(delta_time));
        while let Some((
            entity_id,
            AnimatedCharacter {
                mut image,
                mut character,
            },
            mut behavior,
        )) = self.oswins.pop()
        {
            character = character.update(AnimationDeltaTime::new(delta_time), &mut image);
            if let Some(mut position) = self.world.get_position(entity_id) {
                position += self.offset;
                image
                    .style()
                    .set_property("left", &format!("{}px", position.x))
                    .unwrap();
                image
                    .style()
                    .set_property("top", &format!("{}px", position.y))
                    .unwrap();
            }
            let (state, direction) =
                behavior.oswin_behavior(&mut self.world, entity_id, delta_time);
            character.set_state(state);
            if let Some(direction) = direction {
                image
                    .style()
                    .set_property(
                        "transform",
                        &format!(
                            "scaleX({})",
                            match direction {
                                WalkingDirection::Right => 1,
                                WalkingDirection::Left => -1,
                            }
                        ),
                    )
                    .unwrap();
            }
            self.oswins_switch.push((
                entity_id,
                AnimatedCharacter::new(image, character),
                behavior,
            ));
        }
        std::mem::swap(&mut self.oswins, &mut self.oswins_switch);
    }
}
