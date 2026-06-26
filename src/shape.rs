use macroquad::prelude::*;

pub enum Shape {
    disk(f32),
    rect(Vec2),
    polygon(Vec<Vec2>),
}
