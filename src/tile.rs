use crate::{Animation, Vec2};

const TILE_SIZE: f32 = 16.0;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Tile {
    None,
    Block,
    Rope,
    Grass,
    Dirt,
}

impl Tile {
    pub fn uv(&self) -> Vec2 {
        let (tx, ty): (u32, u32) = match self {
            Tile::None => unreachable!("Tile:None has no UV"),
            Tile::Block => (0, 0),
            Tile::Rope => (0, 1),
            Tile::Grass => (0, 2),
            Tile::Dirt => (1, 2),
        };
        Vec2 {
            x: tx as f32 * TILE_SIZE,
            y: ty as f32 * TILE_SIZE,
        }
    }
}

#[derive(Copy, Clone)]
pub struct AnimatedTile {
    pub position: (usize, usize),
    pub animation: Animation,
}
