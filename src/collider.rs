use crate::{Shape, Vec2};

pub struct Manifold {
    normal: Vec2,
    depth: f32,
}

pub struct Collider {
    shape: Shape,
    offset: Vec2,
}

impl Collider {
    pub fn collides_with(&self, other: Self) -> Option<Manifold> {
        todo!();
    }
}
