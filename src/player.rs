pub struct Player {
    pub pos: (f32, f32),
    pub vel: (f32, f32),
}

impl Player {
    pub fn tick(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
    }
}
