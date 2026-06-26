use crate::Entity;

pub struct GameState {
    entities: Vec<Box<dyn Entity>>,
}

impl GameState {
    fn render(&self) {
        self.entities.iter().for_each(|e| e.render());
    }

    fn update(&mut self, dt: f32) {
        self.entities.iter_mut().for_each(|e| e.update(dt));
    }
}
