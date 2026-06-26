use macroquad::math::Vec2;

pub trait Entity {
    fn position(&self) -> &Vec2;
    fn render(&self);
    fn update(&self, dt: f32);
}
