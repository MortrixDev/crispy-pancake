use macroquad::prelude::*;
use macroquad::miniquad;

mod game;
use game::GameState;

mod entity;
use entity::Entity;

mod player;
use player::Player;

fn window_conf() -> Conf {
    Conf {
        window_title: "Crispy Pancake".to_owned(),
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state: GameState;
    
    loop {
        // Delta time
        let dt = get_frame_time();
        clear_background(BLUE);
        draw_text("Hello, Macroquad!", 50.0, 50.0, 50.0, BLACK);

        next_frame().await
    }
}
