use std::collections::HashMap;

use rand::seq::IndexedRandom;
use rust_station_core::{
    DeltaTime,
    anim::AnimationDeltaTime,
    characters::{Character, FirstMinion, FirstMinionBehavior, Health, HealthType},
    commands::CommandSender,
    enemies::wave::{EnemyWaves, WaveAmount},
    physics::{
        Bounds, BoxCollider, ColliderOverlap, EntityID, Gravity, PhysicsDeltaTime, Position,
        Velocity, World, WorldHistory,
    },
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Request, RequestInit};

use crate::{
    characters::{AnimatedCharacter, FirstMinionUpdate},
    commands::{TrainCartID, WorldCommand},
    world::ProjectileVisual,
};

#[derive(Debug)]
pub struct HostileWorld<'a> {
    document: web_sys::Document,
    body: web_sys::HtmlElement,
    pub world: World,
    first_minions: Vec<(
        EntityID,
        AnimatedCharacter<FirstMinion<'a>>,
        FirstMinionBehavior,
        Character,
    )>,
    first_minions_switch: Vec<(
        EntityID,
        AnimatedCharacter<FirstMinion<'a>>,
        FirstMinionBehavior,
        Character,
    )>,
    train_cart_positions: [(Position, Bounds); 2],
    projectiles: HashMap<rust_station_core::physics::EntityID, ProjectileVisual>,
    command_sender: CommandSender<WorldCommand>,
    wave: WaveAmount,
    enemy_waves: std::rc::Rc<std::cell::RefCell<Option<EnemyWaves>>>,
    to_destroy_enemies: Vec<EntityID>,
}

impl<'a> HostileWorld<'a> {
    pub fn new(
        command_sender: CommandSender<WorldCommand>,
        bounds: Bounds,
        train_cart_positions: [(Position, Bounds); 2],
    ) -> Self {
        let enemy_waves = std::rc::Rc::new(std::cell::RefCell::new(None));
        let w = std::rc::Rc::clone(&enemy_waves);
        load_wave(move |enemy_waves| {
            web_sys::console::log_1(&format!("LOADED WAVES: {:#?}", enemy_waves).into());
            *w.borrow_mut() = Some(enemy_waves);
        });
        let world = World::new(bounds, Gravity::new(Velocity::new(0.0, 0.0)));
        let document = web_sys::window().unwrap().document().unwrap();
        HostileWorld {
            command_sender,
            body: document.body().unwrap(),
            document,
            world,
            first_minions: Vec::new(),
            first_minions_switch: Vec::new(),
            train_cart_positions,
            projectiles: HashMap::new(),
            wave: WaveAmount::new(0),
            enemy_waves,
            to_destroy_enemies: Vec::new(),
        }
    }
    pub fn set_bounds(&mut self, bounds: Bounds) {
        self.world.set_bounds(bounds)
    }
    pub fn set_train_cart_position(&mut self, train_cart_positions: [(Position, Bounds); 2]) {
        self.train_cart_positions = train_cart_positions;
    }
    pub fn update(mut self, delta_time: DeltaTime, body: &web_sys::HtmlElement) -> Self {
        self.world
            .elapsed_duration(PhysicsDeltaTime::new(delta_time));
        {
            let r = self.enemy_waves.borrow();
            if let Some(ref enemy_waves) = *r
                && let Some(enemy_wave) = enemy_waves.get_wave(self.wave)
            {
                self.wave = WaveAmount::new(self.wave.amount() + 1);
                let max = enemy_wave.first_minion.amount();
                drop(r);
                for _ in 0..max {
                    self = self.create_first_minion(body);
                }
            }
        }
        for (entity_id, projectile) in self.projectiles.iter_mut() {
            projectile.update(*entity_id, &self.world);
        }
        while let Some(history) = self.world.pop_history() {
            match history {
                WorldHistory::SpawnProjectile {
                    entity_id,
                    position,
                } => {
                    let _ = self.projectiles.insert(
                        entity_id,
                        ProjectileVisual::new(&self.document, &self.body, position),
                    );
                }
                WorldHistory::DestroyProjectile(entity_id) => {
                    let projectile = self.projectiles.remove(&entity_id).unwrap();
                    projectile.destroy();
                }
                WorldHistory::DestroyEnemy(entity_id) => {
                    self.to_destroy_enemies.push(entity_id);
                }
            }
        }
        while let Some((
            entity_id,
            AnimatedCharacter {
                mut image,
                mut character,
            },
            mut behavior,
            c,
        )) = self.first_minions.pop()
        {
            if let Some(index) = self
                .to_destroy_enemies
                .iter()
                .enumerate()
                .find(|(_, id)| entity_id == **id)
                .map(|(i, _)| i)
            {
                let _ = self.to_destroy_enemies.swap_remove(index);
                image.remove();
            } else {
                character = character.update(AnimationDeltaTime::new(delta_time), &mut image);
                if let Some((position, velocity)) = self.world.get_dynamic_position(entity_id) {
                    if let Some(collider) = self.world.get_collider(entity_id) {
                        let collider = collider.with_position(position);
                        let r = self.train_cart_positions.iter_mut().enumerate().find_map(
                            |(i, (position, bounds, ..))| {
                                let position = Position::new(position.x, position.y + 64.0);
                                if collider.overlap(
                                    BoxCollider::new(bounds.width, bounds.height)
                                        .with_position(position),
                                ) {
                                    Some(i)
                                } else {
                                    None
                                }
                            },
                        );
                        if let Some(i) = r {
                            self.command_sender
                                .send(WorldCommand::DamageTrainCart(TrainCartID::new(i)));
                            image.remove();
                            self.world.remove_entity(entity_id);
                            continue;
                        }
                    }
                    image
                        .style()
                        .set_property("left", &format!("{}px", position.x - 4.0))
                        .unwrap();
                    image
                        .style()
                        .set_property("top", &format!("{}px", position.y - 40.0))
                        .unwrap();
                    image
                        .style()
                        .set_property(
                            "transform",
                            &format!("scaleX({})", if velocity.x > 0.0 { 1 } else { -1 }),
                        )
                        .unwrap();
                }
                behavior.update_first_minion(&mut self.world, entity_id, delta_time);
                self.first_minions_switch.push((
                    entity_id,
                    AnimatedCharacter::new(image, character),
                    behavior,
                    c,
                ));
            }
        }
        std::mem::swap(&mut self.first_minions, &mut self.first_minions_switch);
        self
    }
    fn create_first_minion(mut self, body: &web_sys::HtmlElement) -> Self {
        let (mut target_position, target_bounds, ..) =
            *self.train_cart_positions.choose(&mut rand::rng()).unwrap();
        target_position.x = rand::random_range(
            (target_position.x + 16.0)..(target_position.x + target_bounds.width - 16.0),
        );
        target_position.y += 64.0;
        let position = Position::new(
            rand::random_range(16.0..(self.world.bounds().width - 16.0)),
            12.0,
        );
        const MIN_SPEED: f32 = 192.0;
        const MAX_SPEED: f32 = MIN_SPEED + 64.0;
        let speed = rand::random_range(MIN_SPEED..MAX_SPEED);
        let (world, entity_id) = self
            .world
            .builder()
            .add_enemy()
            .add_collider(BoxCollider::new(56.0, 8.0))
            .add_position_with_velocity(
                position,
                Velocity::target(target_position, position).normalize() * speed,
            )
            .finish();
        self.world = world;
        let img = self
            .document
            .create_element("img")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        img.class_list().add_2("character", "first-minion").unwrap();
        body.append_child(&img).unwrap();
        self.first_minions.push((
            entity_id,
            AnimatedCharacter::new(img, FirstMinion::new()),
            FirstMinionBehavior::new(target_position, speed),
            Character::new(HealthType::Normal(Health::new(1))),
        ));
        self
    }
}

fn load_wave(callback: impl FnOnce(EnemyWaves) + 'static) {
    spawn_local(async move {
        let waves = fetch_url_as_string("waves/wave_0.toml").await.unwrap();
        let waves: EnemyWaves = toml::de::from_str(&waves).unwrap();
        callback(waves);
    })
}

async fn fetch_url_as_string(url: &str) -> Result<String, wasm_bindgen::JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(web_sys::RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value =
        wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: web_sys::Response = resp_value.dyn_into()?;

    let text = wasm_bindgen_futures::JsFuture::from(resp.text()?).await?;

    Ok(text.as_string().unwrap_or_default())
}
