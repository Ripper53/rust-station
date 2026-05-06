use rust_station_core::{
    DeltaTime,
    commands::create_command_channel,
    physics::{Bounds, Position, ProjectileSpeed, RadiansAngle, Speed},
    train::{ParallaxUpdateResponse, TrainBackground},
};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlDivElement, HtmlElement};

use crate::{
    anim::DamageFlashAnimator,
    commands::WorldCommand,
    parallax::ParallaxLayer,
    train::{TrainBounce, TrainCartVisual, TurretVisual},
    world::{create_world, hostile::HostileWorld},
};

mod anim;
mod characters;
mod commands;
mod parallax;
mod train;
mod world;

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as f32;

    let f: std::rc::Rc<std::cell::RefCell<Option<ScopedClosure<'_, dyn FnMut(f64)>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = std::rc::Rc::clone(&f);

    let train_carts = {
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
    let train_carts = std::rc::Rc::new(std::cell::RefCell::new(train_carts));
    let (command_sender, command_receiver) = create_command_channel();
    let hostile_world = {
        let train_carts_for_getter = std::rc::Rc::clone(&train_carts);
        let get_train_cart_positions = move || {
            let t = train_carts_for_getter.borrow();
            let t0 = t.get(0).unwrap();
            let t1 = t.get(1).unwrap();
            let t0_s = Bounds::new(t0.width() - 16.0, t0.height());
            let t1_s = Bounds::new(t1.width() - 16.0, t1.height());
            let t0 = Position::new(t0.pos_x() + 8.0, t0.pos_y());
            let t1 = Position::new(t1.pos_x() - 8.0, t1.pos_y());
            [(t0, t0_s), (t1, t1_s)]
        };

        let hostile_world = std::rc::Rc::new(std::cell::RefCell::new(Some(HostileWorld::new(
            command_sender.clone(),
            Bounds::new(width, height),
            get_train_cart_positions(),
        ))));
        let win = web_sys::window().unwrap();
        let hostile_world_pointer = std::rc::Rc::clone(&hostile_world);
        let train_carts = std::rc::Rc::clone(&train_carts);
        let card_world_0 = std::rc::Rc::clone(&card_world_0);
        let card_world_1 = std::rc::Rc::clone(&card_world_1);
        let closure = Closure::<dyn FnMut()>::new(move || {
            let bounds = Bounds::new(
                win.inner_width().unwrap().as_f64().unwrap() as f32,
                win.inner_height().unwrap().as_f64().unwrap() as f32,
            );
            hostile_world_pointer
                .borrow_mut()
                .as_mut()
                .unwrap()
                .set_bounds(bounds);
            hostile_world_pointer
                .borrow_mut()
                .as_mut()
                .unwrap()
                .set_train_cart_position(get_train_cart_positions());
            let train_carts = train_carts.borrow();
            let train_cart = train_carts.get(0).unwrap();
            card_world_0
                .borrow_mut()
                .set_offset(Position::new(128.0, train_cart.pos_y() + 64.0 - 4.0));
            let train_cart = train_carts.get(1).unwrap();
            card_world_1
                .borrow_mut()
                .set_offset(Position::new(512.0, train_cart.pos_y() + 64.0 - 4.0));
        });
        window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
        hostile_world
    };

    let mut train_carts_flash = {
        let train_carts_flash_elements = document.get_elements_by_class_name("train-cart-flash");
        let mut train_carts_flash =
            Vec::with_capacity(train_carts_flash_elements.length() as usize);
        for i in 0..train_carts_flash_elements.length() {
            let element = train_carts_flash_elements
                .item(i)
                .unwrap()
                .dyn_into::<web_sys::HtmlImageElement>()
                .unwrap();
            train_carts_flash.push(DamageFlashAnimator::new(element));
        }
        train_carts_flash
    };
    let mut turret_weapons = {
        let mut world = hostile_world.borrow_mut().take().unwrap();
        let (w, entity_id_a) = world
            .world
            .builder()
            .turret()
            .add_position(Position::new(196.0, 4.0))
            .add_angle(RadiansAngle::new(0.0))
            .add_rotation_speed(Speed::new(1.0))
            .add_projectile_speed(ProjectileSpeed::new(Speed::new(4.0)))
            .finish();
        let (w, entity_id_b) = w
            .builder()
            .turret()
            .add_position(Position::new(360.0, 4.0))
            .add_angle(RadiansAngle::new(0.0))
            .add_rotation_speed(Speed::new(1.0))
            .add_projectile_speed(ProjectileSpeed::new(Speed::new(4.0)))
            .finish();
        world.world = w;
        *hostile_world.borrow_mut() = Some(world);
        let train_carts = train_carts.borrow();
        let train_cart = train_carts.get(0).unwrap();
        let train_cart = train_cart.holder();
        let train_weapons = vec![
            (entity_id_a, TurretVisual::new(&document, train_cart)),
            (entity_id_b, TurretVisual::new(&document, train_cart)),
        ];
        train_weapons
    };
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        let time = time as f32 / 1000.0;
        let delta_time = DeltaTime::new(time - last_time);
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
            let mut hw = hostile_world.borrow_mut().take().unwrap();
            hw = hw.update(delta_time, &body);
            for (entity_id, turret) in turret_weapons.iter_mut() {
                turret.update(*entity_id, &hw.world);
            }
            *hostile_world.borrow_mut() = Some(hw);
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
        {
            let mut train_carts = train_carts.borrow_mut();
            for train_cart in train_carts.iter_mut() {
                train_cart.update(delta_time);
            }
        }

        for train_cart_flash in train_carts_flash.iter_mut() {
            train_cart_flash.update(delta_time);
        }
        while let Some(command) = command_receiver.receive() {
            match command {
                WorldCommand::DamageTrainCart(id) => {
                    train_carts_flash.get_mut(id.index()).unwrap().play();
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
