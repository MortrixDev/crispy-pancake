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
    let mut player = Player::init().await;

    let background = load_texture("assets/Free/Background/Blue.png")
        .await
        .expect("Error loading background texture");

    loop {
        // Delta time
        let dt = get_frame_time();
        clear_background(BLUE);
        draw_text("Hello, Macroquad!", 50.0, 50.0, 50.0, BLACK);

        // Draw background
        draw_texture_ex(
            &background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    screen_width(),
                    screen_height(),
                )),
                ..Default::default()
            },
        );

        player.update(dt);
        player.render();

        next_frame().await
    }
}
