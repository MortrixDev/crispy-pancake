use crate::Entity;
use macroquad::prelude::*;

const TILE_SIZE: f32 = 16.0;
const TILES_WIDE: f32 = 24.0;

pub struct GameState {
    entities: Vec<Box<dyn Entity>>,
    camera: Camera2D,
    pub camera_target: Option<Vec2>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            camera: Camera2D::default(),
            camera_target: None,
        }
    }

    pub fn render(&self) {
        set_camera(&self.camera);
        self.entities.iter().for_each(|e| e.render());
        set_default_camera();
    }

    pub fn update(&mut self, dt: f32) {
        let mut entities = std::mem::take(&mut self.entities);
        entities.iter_mut().for_each(|e| e.update(self, dt));
        self.entities = entities;

        let world_width = TILES_WIDE * TILE_SIZE;
        let world_height = world_width * screen_height() / screen_width();
        let target = self
            .camera_target
            .unwrap_or(vec2(world_width / 2.0, world_height / 2.0));
        self.camera = Camera2D {
            zoom: vec2(2.0 / world_width, -2.0 / world_height),
            target,
            ..Default::default()
        };
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }
}
