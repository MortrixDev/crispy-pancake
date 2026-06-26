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

pub fn handle_key_event_test(x_pos: &mut f32, y_pos: &mut f32) {
    if is_key_down(KeyCode::Left) {
        *x_pos -= 1.0;
    }
    if is_key_down(KeyCode::Right) {
        *x_pos += 1.0;
    }
    if is_key_down(KeyCode::Up) {
        *y_pos -= 1.0;
    }
    if is_key_down(KeyCode::Down) {
        *y_pos += 1.0;
    } 
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state: GameState;
    
    let (mut x_pos, mut y_pos) = (100.0, 100.0);
    
    let temp_player = load_texture("assets/Free/Main_Characters/Pink Man/Jump (32x32).png")
        .await
        .expect("Error loading player texture.");

    let background = load_texture("assets/Free/Background/Blue.png")
        .await
        .expect("Error loading background texture");

    loop {
        // Delta time
        let dt = get_frame_time();
        clear_background(BLUE);
        draw_text("Hello, Macroquad!", 50.0, 50.0, 50.0, BLACK);
        handle_key_event_test(&mut x_pos, &mut y_pos);

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
        // Draw player
        draw_texture(&temp_player, x_pos, y_pos, RED);

        next_frame().await
    }
}
