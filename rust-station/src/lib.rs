use rust_station_core::{
    anim::AnimationDuration,
    characters::Oswin,
    physics::{Bounds, BoxCollider, Gravity, PhysicsDuration, Position, Velocity, World},
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

use crate::characters::{AnimatedCharacter, OswinUpdate};

mod anim;
mod characters;

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let body = window.document().unwrap().body().unwrap();
    let mut world = World::new(
        Bounds::new(
            window.inner_width().unwrap().as_f64().unwrap() as f32,
            window.inner_height().unwrap().as_f64().unwrap() as f32,
        ),
        Gravity::new(Velocity::new(0.0, 50.0)),
    );
    web_sys::console::log_1(&format!("{:?}", world.bounds()).into());

    const OSWINS_COUNT: usize = 10;
    let mut oswins = Vec::with_capacity(OSWINS_COUNT);
    for _ in 0..OSWINS_COUNT {
        let i = HtmlImageElement::new().unwrap();
        i.class_list().add_2("character", "oswin").unwrap();
        body.append_child(&i).unwrap();
        let mut o = AnimatedCharacter::new(i, Oswin::new());
        o.character
            .set_state(rust_station_core::characters::OswinState::Walking);
        let (w, entity_id) = world
            .builder()
            .add_position_with_velocity(Position::new(0.0, 0.0), Velocity::new(0.0, 0.0))
            .add_collider(BoxCollider::new(16.0, 32.0))
            .finish();
        world = w;
        oswins.push((entity_id, o));
    }
    let mut oswins_0 = Vec::with_capacity(OSWINS_COUNT);
    let f: std::rc::Rc<std::cell::RefCell<Option<ScopedClosure<'_, dyn FnMut(f64)>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = std::rc::Rc::clone(&f);

    let mut last_time = 0.0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        let time = time as f32 / 1000.0;
        let delta_time = time - last_time;
        last_time = time;
        while let Some((
            entity_id,
            AnimatedCharacter {
                mut image,
                mut character,
            },
        )) = oswins.pop()
        {
            character = character.update(AnimationDuration::new(delta_time), &mut image);
            if let Some(position) = world.get_position(entity_id) {
                image
                    .style()
                    .set_property("left", &format!("{}px", position.x))
                    .unwrap();
                image
                    .style()
                    .set_property("top", &format!("{}px", position.y))
                    .unwrap();
            }
            oswins_0.push((entity_id, AnimatedCharacter { image, character }));
        }
        std::mem::swap(&mut oswins, &mut oswins_0);
        world.elapsed_duration(PhysicsDuration::new(delta_time));

        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut(f64)>));
    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}
