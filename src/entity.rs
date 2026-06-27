use crate::{Colliders, GameState, Vec2};

pub trait Entity {
    fn position(&self) -> &Vec2;
    fn colliders(&self) -> Option<&Colliders> {
        None
    }
    fn render(&self);
    fn update(&mut self, state: &mut GameState, dt: f32);
}
