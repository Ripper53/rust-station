use rust_station_core::physics::{EntityID, World};
use wasm_bindgen::JsCast;

#[derive(Debug)]
pub struct TurretVisual {
    weapon_element: web_sys::HtmlElement,
    base_image_element: web_sys::HtmlImageElement,
    barrel_image_element: web_sys::HtmlImageElement,
}

impl TurretVisual {
    pub fn new(document: &web_sys::Document, body: &web_sys::Element) -> Self {
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
            weapon_element,
            base_image_element,
            barrel_image_element,
        }
    }
    pub fn destroy(self) {
        self.weapon_element.remove();
    }
    pub fn update(&mut self, entity_id: EntityID, world: &World) {
        if let Some(position) = world.get_position(entity_id) {
            self.weapon_element
                .style()
                .set_property("left", &format!("{}px", position.x))
                .unwrap();
            self.weapon_element
                .style()
                .set_property("top", &format!("{}px", position.y))
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
