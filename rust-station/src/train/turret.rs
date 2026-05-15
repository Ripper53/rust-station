use rust_station_core::physics::{EntityID, World};
use wasm_bindgen::JsCast;

#[derive(Debug)]
pub struct TurretVisual {
    cart: web_sys::HtmlElement,
    weapon_element: web_sys::HtmlElement,
    base_image_element: web_sys::HtmlImageElement,
    barrel_image_element: web_sys::HtmlImageElement,
    offset_x: f32,
}

impl TurretVisual {
    pub fn new(document: &web_sys::Document, body: web_sys::HtmlElement, offset_x: f32) -> Self {
        let weapon_element = document
            .create_element("div")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        weapon_element.class_list().add_1("weapon").unwrap();
        let base_image_element = document
            .create_element("img")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        base_image_element.set_src("images/train/TrainGunBase.png");
        let barrel_image_element = document
            .create_element("img")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        barrel_image_element.set_src("images/train/TrainGunBarrel.png");
        barrel_image_element.class_list().add_1("barrel").unwrap();
        weapon_element.append_child(&base_image_element).unwrap();
        weapon_element.append_child(&barrel_image_element).unwrap();
        body.append_child(&weapon_element).unwrap();
        TurretVisual {
            cart: body,
            weapon_element,
            base_image_element,
            barrel_image_element,
            offset_x,
        }
    }
    pub fn destroy(self) {
        self.weapon_element.remove();
    }
    pub fn update(&mut self, entity_id: EntityID, world: &World, window_scroll_x: f32) {
        if let Some(position) = world.get_position(entity_id) {
            let rect = self.cart.get_bounding_client_rect();
            self.weapon_element
                .style()
                .set_property(
                    "left",
                    &format!(
                        "{}px",
                        position.x + rect.x() as f32 + self.offset_x + window_scroll_x
                    ),
                )
                .unwrap();
            self.weapon_element
                .style()
                .set_property("top", &format!("{}px", position.y - rect.y() as f32))
                .unwrap();
        }
        if let Some(angle) = world.get_angle(entity_id) {
            self.barrel_image_element
                .style()
                .set_property(
                    "transform",
                    &format!("rotate({}rad)", angle.value() + std::f32::consts::FRAC_PI_2),
                )
                .unwrap();
        }
    }
}
