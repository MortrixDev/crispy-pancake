use macroquad::prelude::*;

pub enum Shape {
    Disk(f32),
    Rect(Vec2),
    Polygon(Vec<Vec2>),
}
