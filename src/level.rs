use crate::{AnimatedTile, GameState, Tile};
use macroquad::prelude::*;

pub struct Level {
    pub name: &'static str,
    pub width: usize,
    pub height: usize,
    pub tiles: Box<[Tile]>,
    pub animated_tiles: Vec<AnimatedTile>,
    pub player_spawn: Vec2,
}

impl Level {
    pub fn load(&self, state: &mut GameState) {
        state.tiles = self.tiles.clone();
        state.level_width = self.width;
        state.animated = self.animated_tiles.clone();
        state.player.as_mut().unwrap().position = self.player_spawn;
    }
}

pub struct LevelBuilder {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    animated_tiles: Vec<AnimatedTile>,
    player_spawn: Vec2,
}

impl LevelBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::None; width * height],
            animated_tiles: Vec::new(),
            player_spawn: Vec2::ZERO,
        }
    }

    pub fn fill(mut self, x: usize, y: usize, w: usize, h: usize, tile: Tile) -> Self {
        for row in y..y + h {
            for col in x..x + w {
                self.tiles[row * self.width + col] = tile;
            }
        }
        self
    }

    pub fn spawn(mut self, pos: Vec2) -> Self {
        self.player_spawn = pos;
        self
    }

    pub fn build(self, name: &'static str) -> Level {
        Level {
            name,
            width: self.width,
            height: self.height,
            tiles: self.tiles.into_boxed_slice(),
            animated_tiles: self.animated_tiles,
            player_spawn: self.player_spawn,
        }
    }
}

pub fn test_level() -> Level {
    LevelBuilder::new(24, 14)
        .fill(0, 12, 24, 1, Tile::Grass)
        .fill(0, 13, 24, 1, Tile::Dirt)
        .fill(7, 6, 5, 1, Tile::Block)   // high platform
        .fill(4, 8, 5, 1, Tile::Block)   // left mid platform
        .fill(16, 8, 5, 1, Tile::Block)  // right mid platform
        .spawn(vec2(32.0, 160.0))
        .build("Test")
}
