use rust_station_core::{
    anim::AnimationDuration,
    characters::Oswin,
    physics::{Bounds, BoxCollider, Gravity, PhysicsDuration, Position, Velocity, World},
    train::{ParallaxUpdateResponse, TrainBackground},
};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlDivElement, HtmlImageElement};

use crate::{
    characters::{AnimatedCharacter, OswinUpdate},
    parallax::ParallaxLayer,
};

mod anim;
mod characters;
mod parallax;

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
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

    let mut train_background_a =
        TrainBackground::<3>::new(world.bounds().width, 2048.0, 0.5, world.bounds().width);
    let mut train_background_b = TrainBackground::<3>::new(world.bounds().width, 2048.0, 0.5, 0.0);
    let generate_parallax_layers = || {
        let background = document.get_element_by_id("background").unwrap();
        let div_0 = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        let div_1 = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        let div_2 = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        background.append_child(&div_0).unwrap();
        background.append_child(&div_1).unwrap();
        background.append_child(&div_2).unwrap();
        [
            ParallaxLayer::new(
                div_0,
                vec![
                    "images/backgrounds/mountains/Mountain0.png",
                    "images/backgrounds/mountains/Mountain1.png",
                    "images/backgrounds/mountains/Mountain2.png",
                    "images/backgrounds/mountains/Mountain3.png",
                ],
            ),
            ParallaxLayer::new(
                div_1,
                vec![
                    "images/backgrounds/mountains/Mountain0.png",
                    "images/backgrounds/mountains/Mountain1.png",
                    "images/backgrounds/mountains/Mountain2.png",
                    "images/backgrounds/mountains/Mountain3.png",
                ],
            ),
            ParallaxLayer::new(
                div_2,
                vec![
                    "images/backgrounds/mountains/Mountain0.png",
                    "images/backgrounds/mountains/Mountain1.png",
                    "images/backgrounds/mountains/Mountain2.png",
                    "images/backgrounds/mountains/Mountain3.png",
                ],
            ),
        ]
    };
    let mut parallax_layers_a = generate_parallax_layers();
    for (i, p) in parallax_layers_a.iter_mut().enumerate() {
        p.update_position(0.0);
        p.update_images(&document, world.bounds().width, 16, i);
    }
    let mut parallax_layers_b = generate_parallax_layers();
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
        let width = world.bounds().width;
        train_background_a.set_background_max_position_x(width * 3.0);
        for (z_index, (layer, response)) in parallax_layers_a
            .iter_mut()
            .zip(train_background_a.elapsed_duration(delta_time))
            .enumerate()
        {
            match response {
                ParallaxUpdateResponse::UpdatePosition(x) => {
                    layer.update_position(x - width);
                }
                ParallaxUpdateResponse::RestartAtPosition(x) => {
                    layer.update_position(x - width);
                    layer.update_images(&document, width, 2, z_index);
                }
            }
        }
        train_background_b.set_background_max_position_x(width * 3.0);
        for (z_index, (layer, response)) in parallax_layers_b
            .iter_mut()
            .zip(train_background_b.elapsed_duration(delta_time))
            .enumerate()
        {
            match response {
                ParallaxUpdateResponse::UpdatePosition(x) => {
                    layer.update_position(x - width);
                }
                ParallaxUpdateResponse::RestartAtPosition(x) => {
                    layer.update_position(x - width);
                    layer.update_images(&document, width, 2, z_index);
                }
            }
        }

        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut(f64)>));
    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}
