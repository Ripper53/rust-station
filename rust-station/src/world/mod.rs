use rust_station_core::{
    DeltaTime,
    anim::AnimationDeltaTime,
    characters::{Oswin, OswinBehavior, WalkingDirection},
    physics::{
        Bounds, BoxCollider, EntityID, Gravity, PhysicsDeltaTime, Position, Velocity, World,
    },
};
use wasm_bindgen::prelude::*;
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
    world: RcWorld,
    oswins: Vec<(EntityID, AnimatedCharacter<Oswin<'a>>, OswinBehavior)>,
    oswins_switch: Vec<(EntityID, AnimatedCharacter<Oswin<'a>>, OswinBehavior)>,
    offset: Position,
}
struct RcWorld(std::rc::Rc<std::cell::RefCell<Option<World>>>);
impl Clone for RcWorld {
    fn clone(&self) -> Self {
        RcWorld(std::rc::Rc::clone(&self.0))
    }
}
impl RcWorld {
    pub fn as_owned_world<R>(&mut self, f: impl FnOnce(World) -> (World, R)) -> R {
        let world = self.0.replace(None);
        let (world, r) = f(world.unwrap());
        let _ = self.0.replace(Some(world));
        r
    }
    pub fn as_world_mut<R>(&mut self, f: impl FnOnce(&mut World) -> R) -> R {
        let mut world = self.0.borrow_mut();
        f(world.as_mut().unwrap())
    }
    pub fn as_world<R>(&self, f: impl FnOnce(&World) -> R) -> R {
        let world = self.0.borrow();
        f(world.as_ref().unwrap())
    }
}

impl<'a> WorldRenderer<'a> {
    pub fn new(world: World, body: &HtmlElement, offset: Position) -> Self {
        const OSWINS_COUNT: usize = 10;
        let mut oswins = Vec::with_capacity(OSWINS_COUNT);
        let mut world = RcWorld(std::rc::Rc::new(std::cell::RefCell::new(Some(world))));
        let mut w = RcWorld::clone(&world);
        let pointer_move = Closure::<dyn FnMut(_)>::new(move |event: web_sys::PointerEvent| {
            let position = Position::new(event.client_x() as f32, event.client_y() as f32);
            w.as_world_mut(move |world| world.set_pointer_position(position));
        });
        body.add_event_listener_with_callback("pointermove", pointer_move.as_ref().unchecked_ref())
            .unwrap();
        pointer_move.forget();
        for _ in 0..OSWINS_COUNT {
            let i = HtmlImageElement::new().unwrap();
            i.class_list().add_2("character", "oswin").unwrap();
            i.set_draggable(true);
            body.append_child(&i).unwrap();
            let entity_id = world.as_owned_world(|world| {
                world
                    .builder()
                    .add_position_with_velocity(Position::new(0.0, 0.0), Velocity::new(0.0, 0.0))
                    .add_collider(BoxCollider::new(16.0, 32.0))
                    .finish()
            });
            let oswin = AnimatedCharacter::new(i, Oswin::new());
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
        self.world.as_world_mut(|world| {
            world.elapsed_duration(PhysicsDeltaTime::new(delta_time));
        });
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
            if let Some(mut position) = self.world.as_world(|world| world.get_position(entity_id)) {
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
            let (state, direction) = self
                .world
                .as_world_mut(|world| behavior.oswin_behavior(world, entity_id, delta_time));
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
