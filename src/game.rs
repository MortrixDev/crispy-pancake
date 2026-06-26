use crate::Entity;

pub struct GameState {
    entities: Vec<Box<dyn Entity>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn render(&self) {
        self.entities.iter().for_each(|e| e.render());
    }

    pub fn update(&mut self, dt: f32) {
        self.entities.iter_mut().for_each(|e| e.update(dt));
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }
}
