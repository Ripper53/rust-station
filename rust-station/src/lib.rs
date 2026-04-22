use rust_station_core::{anim::AnimationDuration, characters::Oswin};
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

use crate::characters::{AnimatedCharacter, OswinUpdate};

mod anim;
mod characters;

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let body = window.document().unwrap().body().unwrap();

    const OSWINS_COUNT: usize = 10;
    let mut oswins = Vec::with_capacity(OSWINS_COUNT);
    for _ in 0..OSWINS_COUNT {
        let i = HtmlImageElement::new().unwrap();
        i.class_list().add_2("character", "oswin").unwrap();
        body.append_child(&i).unwrap();
        let o = AnimatedCharacter::new(i, Oswin::new());
        oswins.push(o);
    }
    let mut oswins_0 = Vec::with_capacity(OSWINS_COUNT);
    let f: std::rc::Rc<std::cell::RefCell<Option<ScopedClosure<'_, dyn FnMut(f64)>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = std::rc::Rc::clone(&f);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_time: f64| {
        while let Some(AnimatedCharacter {
            mut image,
            mut character,
        }) = oswins.pop()
        {
            character = character.update(AnimationDuration::new(0.02), &mut image);
            oswins_0.push(AnimatedCharacter { image, character });
        }
        std::mem::swap(&mut oswins, &mut oswins_0);

        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut(f64)>));
    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}
