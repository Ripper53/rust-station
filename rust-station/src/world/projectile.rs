use rust_station_core::physics::{EntityID, Position, World};
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

#[derive(Debug)]
pub struct ProjectileVisual {
    img_element: HtmlImageElement,
}

impl ProjectileVisual {
    pub fn new(
        document: &web_sys::Document,
        node: &web_sys::HtmlElement,
        position: Position,
    ) -> Self {
        let img_element = document
            .create_element("img")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();
        img_element.class_list().add_1("projectile").unwrap();
        img_element
            .style()
            .set_property("left", &format!("{}px", position.x))
            .unwrap();
        img_element
            .style()
            .set_property("top", &format!("{}px", position.y))
            .unwrap();
        img_element.set_src("images/projectiles/Bullet.png");
        node.append_child(&img_element).unwrap();
        ProjectileVisual { img_element }
    }
    pub fn update(&mut self, entity_id: EntityID, world: &World) {
        if let Some(position) = world.get_position(entity_id) {
            self.img_element
                .style()
                .set_property("left", &format!("{}px", position.x))
                .unwrap();
            self.img_element
                .style()
                .set_property("top", &format!("{}px", position.y))
                .unwrap();
        }
    }
    pub fn destroy(self) {
        self.img_element.remove();
    }
}
