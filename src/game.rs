use crate::tile::TILE_SIZE;
use crate::{AnimatedTile, Assets, Entity, Player, Tile};
use macroquad::prelude::*;

const NATIVE_W: f32 = TILE_SIZE * 18.;
const NATIVE_H: f32 = TILE_SIZE * 12.;

pub struct GameState {
    pub assets: Assets,
    pub player: Option<Player>,
    entities: Vec<Box<dyn Entity>>,
    pub tiles: Box<[Tile]>,
    pub level_width: usize,
    pub animated: Vec<AnimatedTile>,
    render_target: RenderTarget,
    camera_target: Vec2,
}

impl GameState {
    pub fn new() -> Self {
        let rt = render_target(NATIVE_W as u32, NATIVE_H as u32);
        rt.texture.set_filter(FilterMode::Nearest);
        Self {
            assets: Assets::new(),
            player: Some(Player::new()),
            entities: Vec::new(),
            tiles: Vec::new().into_boxed_slice(),
            level_width: 0,
            animated: Vec::new(),
            render_target: rt,
            camera_target: vec2(NATIVE_W * 0.5, NATIVE_H * 0.5),
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }

    pub fn render(&self) {
        set_camera(&Camera2D {
            zoom: vec2(2.0 / NATIVE_W, 2.0 / NATIVE_H),
            target: self.camera_target,
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        clear_background(SKYBLUE);

        for (i, tile) in self.tiles.iter().enumerate() {
            if matches!(tile, Tile::None) {
                continue;
            }
            let x = (i % self.level_width) as f32 * TILE_SIZE;
            let y = (i / self.level_width) as f32 * TILE_SIZE;
            let uv = tile.uv();
            draw_texture_ex(
                &self.assets.atlas,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(uv.x, uv.y, TILE_SIZE, TILE_SIZE)),
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
            );
        }

        if let Some(player) = &self.player {
            player.render(self);
        }
        self.entities.iter().for_each(|e| e.render(self));

        // Scale to fill screen width, letterbox top/bottom with black.
        set_default_camera();
        clear_background(BLACK);

        let scale = screen_width() / NATIVE_W;
        let y = (screen_height() - NATIVE_H * scale) * 0.5;
        draw_texture_ex(
            &self.render_target.texture,
            0.0,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), NATIVE_H * scale)),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, dt: f32) {
        let mut player = self.player.take().unwrap();
        player.update(self, dt);
        self.player = Some(player);

        let mut entities = std::mem::take(&mut self.entities);
        entities.iter_mut().for_each(|e| e.update(self, dt));
        self.entities = entities;

        self.camera_target = self
            .player
            .as_ref()
            .map_or(vec2(NATIVE_W * 0.5, NATIVE_H * 0.5), |p| p.position);
    }
}
