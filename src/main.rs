use macroquad::prelude::*;
use macroquad::miniquad;

mod game;
use game::GameState;

mod entity;
use entity::Entity;

mod entities;
use entities::player::Player;
use entities::moving_platform::MovingPlatform;

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
    let mut state = GameState::new();
    state.add_entity(Box::new(Player::new().await));
    state.add_entity(Box::new(MovingPlatform::new(
        Vec2{x: 200.0, y: 200.0}, 
        vec![Vec2{x: 100.0, y: 200.0}, Vec2{x: 300.0, y: 200.0}], 
        100.0, 
        load_texture("assets/Free/Other/Confetti.png").await.expect("Failed loading MP."))
    ));

    loop {
        // Delta time
        let dt = get_frame_time();

        state.update(dt);
        state.render();

        next_frame().await
    }
}
