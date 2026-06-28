#[derive(Copy, Clone)]
pub struct Animation {
    pub frames: &'static [(u32, u32)],
    pub current_frame: usize,
    pub timer: f32,
    pub frame_duration: f32,
}

impl Animation {
    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        if self.timer >= self.frame_duration {
            self.timer = 0.0;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }

    pub fn uv(&self) -> (u32, u32) {
        self.frames[self.current_frame]
    }
}
