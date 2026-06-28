use crate::{Animation, Shape, Vec2};

pub const TILE_SIZE: f32 = 16.0;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Tile {
    None,
    Block,
    Rope,
    Grass,
    Dirt,
    Special,
}

impl Tile {
    pub fn shape(&self) -> Option<Shape> {
        match self {
            Tile::None => None,
            _ => Some(Shape::Rect(Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            })),
        }
    }

    pub fn uv(&self) -> Vec2 {
        let (tx, ty): (u32, u32) = match self {
            Tile::None => unreachable!("Tile:None has no UV"),
            Tile::Block => (0, 0),
            Tile::Rope => (1, 0),
            Tile::Grass => (2, 0),
            Tile::Dirt => (2, 1),
            Tile::Special => (0, 1),
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
