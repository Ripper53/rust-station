use rand::{RngExt, seq::IndexedRandom};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlImageElement};

pub struct ParallaxLayer<'a> {
    holder: HtmlDivElement,
    generated_images: Vec<&'a str>,
    instantiated_images: Vec<HtmlImageElement>,
}

impl<'a> ParallaxLayer<'a> {
    pub fn new(holder: HtmlDivElement, generated_images: Vec<&'a str>) -> Self {
        ParallaxLayer {
            holder,
            generated_images,
            instantiated_images: Vec::new(),
        }
    }
    pub fn update_position(&mut self, position_x: f32) {
        self.holder
            .style()
            .set_property("left", &format!("{position_x}px"))
            .unwrap();
    }
    pub fn update_images(
        &mut self,
        document: &web_sys::Document,
        mut max_x: f32,
        density: usize,
        z_index: usize,
    ) {
        max_x = (max_x - 1024.0).min(0.0);
        while let Some(img) = self.instantiated_images.pop() {
            self.holder.remove_child(&img).unwrap();
        }
        let mut rng = rand::rng();
        for i in 0..density {
            if rng.random_bool(0.75) {
                let img = document
                    .create_element("img")
                    .unwrap()
                    .dyn_into::<HtmlImageElement>()
                    .unwrap();
                let src = self.generated_images.choose(&mut rng).unwrap();
                img.set_src(src);
                img.style()
                    .set_property("z-index", &format!("{z_index}"))
                    .unwrap();
                img.style()
                    .set_property(
                        "left",
                        &format!("{}px", max_x * (i as f32 / density as f32)),
                    )
                    .unwrap();
                img.style()
                    .set_property(
                        "transform",
                        &format!("scaleX({})", if rng.random_bool(0.5) { 1 } else { -1 }),
                    )
                    .unwrap();
                self.holder.append_child(&img).unwrap();
                self.instantiated_images.push(img);
            }
        }
    }
}
