use crate::{Entity, GameState};
use macroquad::prelude::*;

pub struct MovingPlatform {
    position: Vec2,
    velocity: Vec2,
    end_points: Vec<Vec2>,
    speed: f32,
    texture: Texture2D,
}

impl MovingPlatform {
    pub fn new(p: Vec2, e_p: Vec<Vec2>, s: f32, t: Texture2D) -> MovingPlatform {
        Self {
            position: p,
            velocity: (e_p[0] - p).normalize(),
            end_points: e_p,
            speed: s,
            texture: t,
        }
    }
}

impl Entity for MovingPlatform {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn render(&self) {
        draw_texture(&self.texture, self.position.x, self.position.y, WHITE);
    }

    fn update(&mut self, state: &mut GameState, dt: f32) {
        if self.position == self.end_points[0] || self.position == self.end_points[1] {
            self.velocity *= -1.0;
        }

        self.position += self.velocity * self.speed * dt;
    }
}
