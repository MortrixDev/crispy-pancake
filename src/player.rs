use crate::Entity;
use macroquad::prelude::*;

pub struct Player {
    position: Vec2,
    velocity: Vec2,
    texture: Texture2D,
}

impl Player {
    pub async fn init() -> Player {
        Player{
            position: Vec2{x: 100.0, y: 100.0}, 
            velocity: Vec2{x: 0.0, y: 0.0}, 
            texture: load_texture("assets/Free/Main_Characters/Pink Man/idle (32x32).png")
                .await
                .expect("Failed loading player."),
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

    fn update(&mut self, dt: f32) {
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
        self.position += self.velocity*dt;
    }
}
