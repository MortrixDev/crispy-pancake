use macroquad::prelude::*;
use macroquad::miniquad;

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
    loop {
        clear_background(BLUE);
        draw_text("Hello, Macroquad!", 50.0, 50.0, 50.0, BLACK);

        next_frame().await
    }
}
