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
    let mut state = GameState::new();
    state.add_entity(Box::new(Player::new().await));

    loop {
        // Delta time
        let dt = get_frame_time();

        state.update(dt);
        state.render();

        next_frame().await
    }
}
