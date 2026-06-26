use crate::GameState;
use macroquad::math::Vec2;

pub trait Entity {
    fn position(&self) -> &Vec2;
    fn render(&self);
    fn update(&mut self, state: &mut GameState, dt: f32);
}
