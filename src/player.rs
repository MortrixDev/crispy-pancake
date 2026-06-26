use crate::Entity;
use macroquad::math::Vec2;

pub struct Player {
    pos: Vec2,
}

impl Entity for Player {
    fn position(&self) -> &Vec2 {
        &self.pos
    }

    fn render(&self) {}

    fn update(&self, dt: f32) {
        _ = dt;
    }
}
