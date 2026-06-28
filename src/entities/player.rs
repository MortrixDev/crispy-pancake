use crate::{Entity, GameState};
use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    velocity: Vec2,
    texture: Texture2D,
}

impl Player {
    pub fn new() -> Player {
        let texture = Texture2D::from_file_with_format(
            include_bytes!("../../assets/Free/Items/Boxes/Box1/Idle.png"),
            Some(ImageFormat::Png),
        );
        texture.set_filter(FilterMode::Nearest);

        Player {
            position: Vec2 { x: 100.0, y: 100.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            texture,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) {
            self.velocity.x = -100.0;
        }
        if is_key_released(KeyCode::Left) {
            self.velocity.x = 0.0;
        }
        if is_key_down(KeyCode::Right) {
            self.velocity.x = 100.0;
        }
        if is_key_released(KeyCode::Right) {
            self.velocity.x = 0.0;
        }

        self.position += self.velocity * dt;
    }

    pub fn render(&self) {
        draw_texture(&self.texture, self.position.x, self.position.y, WHITE);
    }
}

impl Entity for Player {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn render(&self) {
        draw_texture(&self.texture, self.position.x, self.position.y, WHITE);
    }

    fn update(&mut self, _state: &mut GameState, dt: f32) {
        self.update(dt);
    }
}
