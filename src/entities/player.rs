use crate::tile::TILE_SIZE;
use crate::{Entity, GameState, Tile};
use macroquad::prelude::*;

const GRAVITY: f32 = 600.0;
const JUMP_SPEED: f32 = 280.0;
const MOVE_SPEED: f32 = 100.0;

pub struct Player {
    pub position: Vec2,
    velocity: Vec2,
    on_ground: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vec2 { x: 100.0, y: 100.0 },
            velocity: Vec2::ZERO,
            on_ground: false,
        }
    }
}

impl Entity for Player {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn render(&self, state: &GameState) {
        let uv = Tile::Special.uv();
        draw_texture_ex(
            &state.assets.atlas,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(uv.x, uv.y, TILE_SIZE, TILE_SIZE)),
                dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
        );
    }

    fn update(&mut self, state: &mut GameState, dt: f32) {
        let move_x = is_key_down(KeyCode::Right) as i32 - is_key_down(KeyCode::Left) as i32;
        self.velocity.x = move_x as f32 * MOVE_SPEED;

        self.velocity.y += GRAVITY * dt;

        if self.on_ground && is_key_pressed(KeyCode::Space) {
            self.velocity.y = -JUMP_SPEED;
            self.on_ground = false;
        }

        self.position.x += self.velocity.x * dt;
        resolve_x(self, &state.tiles, state.level_width);

        self.position.y += self.velocity.y * dt;
        self.on_ground = false;
        resolve_y(self, &state.tiles, state.level_width);
    }
}

fn touching_tiles(pos: Vec2, tiles: &[Tile], level_width: usize) -> Vec<(i32, i32)> {
    if tiles.is_empty() || level_width == 0 {
        return vec![];
    }
    let cols = level_width as i32;
    let rows = (tiles.len() / level_width) as i32;

    let x0 = (pos.x / TILE_SIZE).floor() as i32;
    let x1 = ((pos.x + TILE_SIZE) / TILE_SIZE).ceil() as i32 - 1;
    let y0 = (pos.y / TILE_SIZE).floor() as i32;
    let y1 = ((pos.y + TILE_SIZE) / TILE_SIZE).ceil() as i32 - 1;

    let mut result = Vec::new();
    for ty in y0.clamp(0, rows - 1)..=y1.clamp(0, rows - 1) {
        for tx in x0.clamp(0, cols - 1)..=x1.clamp(0, cols - 1) {
            if !matches!(tiles[(ty * cols + tx) as usize], Tile::None) {
                result.push((tx, ty));
            }
        }
    }
    result
}

fn resolve_x(player: &mut Player, tiles: &[Tile], level_width: usize) {
    let moving_right = player.velocity.x > 0.0;
    for (tx, _) in touching_tiles(player.position, tiles, level_width) {
        player.position.x =
            tx as f32 * TILE_SIZE + if moving_right { -TILE_SIZE } else { TILE_SIZE };
        player.velocity.x = 0.0;
    }
}

fn resolve_y(player: &mut Player, tiles: &[Tile], level_width: usize) {
    let moving_down = player.velocity.y >= 0.0;
    for (_, ty) in touching_tiles(player.position, tiles, level_width) {
        player.position.y =
            ty as f32 * TILE_SIZE + if moving_down { -TILE_SIZE } else { TILE_SIZE };
        player.on_ground |= moving_down;
        player.velocity.y = 0.0;
    }
}
