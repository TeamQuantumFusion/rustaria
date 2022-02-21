pub struct Player {
    pub pos: (f32, f32),
    pub vel: (f32, f32),
}

impl Player {
    pub fn tick(&mut self, delta: f32) {
        self.pos.0 += self.vel.0 * delta;
        self.pos.1 += self.vel.1 * delta;
    }
}
