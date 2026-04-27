use rust_station_core::{
    physics::{Bounds, Position},
    train::{ParallaxUpdateResponse, TrainBackground},
};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlDivElement, HtmlElement};

use crate::{
    parallax::ParallaxLayer,
    train::{TrainBounce, TrainCartVisual},
    world::{create_world, hostile::HostileWorld},
};

mod anim;
mod characters;
mod parallax;
mod train;
mod world;

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    {
        let w = window.inner_width().unwrap().as_f64().unwrap();
        let h = window.inner_height().unwrap().as_f64().unwrap();
        web_sys::console::log_1(&format!("init bounds: {w} x {h}").into());
    }
    let width = window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as f32;

    let f: std::rc::Rc<std::cell::RefCell<Option<ScopedClosure<'_, dyn FnMut(f64)>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = std::rc::Rc::clone(&f);

    let mut train_carts = {
        let train_carts_elements = document.get_elements_by_class_name("train-cart");
        let mut train_carts = Vec::with_capacity(train_carts_elements.length() as usize);
        for i in 0..train_carts_elements.length() {
            let element = train_carts_elements
                .item(i)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap();
            element.style().set_property("top", "0px").unwrap();
            train_carts.push(TrainCartVisual::new(element, TrainBounce::new(0.5..4.0)));
        }
        train_carts
    };
    let mut train_background_a = TrainBackground::<3>::new(width, 2048.0, 0.5, width);
    let mut train_background_b = TrainBackground::<3>::new(width, 2048.0, 0.5, 0.0);
    let mut train_tracks_background = TrainBackground::<1>::new(width, 2048.0, 1.0, 0.0);
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
    let train_tracks_img = document
        .get_element_by_id("train-tracks")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    for (i, p) in parallax_layers_a.iter_mut().enumerate() {
        p.update_position(0.0);
        p.update_images(&document, width, 16, i);
    }
    let mut parallax_layers_b = generate_parallax_layers();
    let mut last_time = 0.0;
    let card_world_0 = create_world(
        &body,
        Bounds::new(
            train_carts[0].width() - 128.0 - 16.0,
            train_carts[0].height() - 64.0,
        ),
        Position::new(128.0, train_carts[0].pos_y() + 64.0 - 4.0),
    );
    let card_world_0 = std::rc::Rc::new(std::cell::RefCell::new(card_world_0));
    let card_world_1 = create_world(
        &body,
        Bounds::new(
            train_carts[1].width() - 16.0,
            train_carts[1].height() - 64.0,
        ),
        Position::new(512.0, train_carts[1].pos_y() + 64.0 - 4.0),
    );
    let card_world_1 = std::rc::Rc::new(std::cell::RefCell::new(card_world_1));
    let hostile_world = {
        let hostile_world = std::rc::Rc::new(std::cell::RefCell::new(HostileWorld::new(
            Bounds::new(width, height),
        )));
        let win = web_sys::window().unwrap();
        let hostile_world_pointer = std::rc::Rc::clone(&hostile_world);
        let closure = Closure::<dyn FnMut()>::new(move || {
            let bounds = Bounds::new(
                win.inner_width().unwrap().as_f64().unwrap() as f32,
                win.inner_height().unwrap().as_f64().unwrap() as f32,
            );
            hostile_world_pointer.borrow_mut().set_bounds(bounds);
        });
        window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
        hostile_world
    };
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        let time = time as f32 / 1000.0;
        let delta_time = time - last_time;
        last_time = time;
        {
            let mut world = card_world_0.borrow_mut();
            world.update(delta_time);
        }
        {
            let mut world = card_world_1.borrow_mut();
            world.update(delta_time);
        }
        {
            let mut hostile_world = hostile_world.borrow_mut();
            hostile_world.update(delta_time);
        }
        let width = window.inner_width().unwrap().as_f64().unwrap() as f32;
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
        train_tracks_background.set_background_max_position_x(width * 3.0);
        for response in train_tracks_background.elapsed_duration(delta_time) {
            match response {
                ParallaxUpdateResponse::UpdatePosition(x) => {
                    train_tracks_img
                        .style()
                        .set_property("left", &format!("{}px", x - width))
                        .unwrap();
                }
                ParallaxUpdateResponse::RestartAtPosition(x) => {
                    train_tracks_img
                        .style()
                        .set_property("left", &format!("{}px", x - width))
                        .unwrap();
                }
            }
        }
        for train_cart in train_carts.iter_mut() {
            train_cart.update(delta_time);
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
