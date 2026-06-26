use crate::{Entity, GameState};
use macroquad::prelude::*;

pub struct Player {
    position: Vec2,
    velocity: Vec2,
    texture: Texture2D,
}

impl Player {
    pub async fn new() -> Player {
        let texture = load_texture("assets/Free/Items/Boxes/Box1/Idle.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        
        Player {
            position: Vec2 { x: 100.0, y: 100.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            texture
        }
    }
}

impl Entity for Player {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn render(&self) {
        draw_texture(&self.texture, self.position.x, self.position.y, WHITE);
    }

    fn update(&mut self, state: &mut GameState, dt: f32) {
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
        state.camera_target = Some(self.position);
    }
}
