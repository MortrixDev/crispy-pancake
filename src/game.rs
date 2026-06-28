use crate::{AnimatedTile, Assets, Entity, Player, Tile};
use macroquad::prelude::*;

const TILE_SIZE: f32 = 16.0;
const TILES_WIDE: usize = 24;

pub struct GameState {
    pub assets: Assets,
    pub player: Player,
    entities: Vec<Box<dyn Entity>>,
    pub tiles: Box<[Tile]>,
    pub level_width: usize,
    pub animated: Vec<AnimatedTile>,
    camera: Camera2D,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            assets: Assets::new(),
            player: Player::new(),
            entities: Vec::new(),
            tiles: Vec::new().into_boxed_slice(),
            level_width: TILES_WIDE,
            animated: Vec::new(),
            camera: Camera2D::default(),
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }

    pub fn render(&self) {
        set_camera(&self.camera);

        for (i, tile) in self.tiles.iter().enumerate() {
            if matches!(tile, Tile::None) {
                continue;
            }
            let x = (i % self.level_width) as f32 * TILE_SIZE;
            let y = (i / self.level_width) as f32 * TILE_SIZE;
            let uv = tile.uv();
            draw_texture_ex(
                &self.assets.atlas,
                x, y, WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(uv.x + 0.5, uv.y + 0.5, TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
            );
        }

        self.player.render();
        self.entities.iter().for_each(|e| e.render());

        set_default_camera();
    }

    pub fn update(&mut self, dt: f32) {
        self.player.update(dt);

        let mut entities = std::mem::take(&mut self.entities);
        entities.iter_mut().for_each(|e| e.update(self, dt));
        self.entities = entities;

        let world_width = TILES_WIDE as f32 * TILE_SIZE;
        let world_height = world_width * screen_height() / screen_width();
        self.camera = Camera2D {
            zoom: vec2(2.0 / world_width, -2.0 / world_height),
            target: self.player.position,
            ..Default::default()
        };
    }
}
