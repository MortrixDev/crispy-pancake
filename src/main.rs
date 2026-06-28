use macroquad::prelude::*;
use macroquad::miniquad;

mod game;
use game::GameState;

mod entity;
use entity::Entity;

mod shape;
use shape::Shape;

mod collider;
use collider::{Collider, Colliders};

mod assets;
use assets::Assets;

mod animation;
use animation::Animation;

mod tile;
use tile::{Tile, AnimatedTile};

mod level;
use level::Level;

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
    level::test_level().load(&mut state);

    loop {
        // Delta time
        let dt = get_frame_time();

        state.update(dt);
        state.render();

        next_frame().await
    }
}
