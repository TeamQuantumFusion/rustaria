use super::health::Health;

#[derive(Debug, Clone)]
pub struct Physics {
    pos: (f32, f32),
    last_stable_ground: (f32, f32),
    in_air: bool,
}
impl Physics {
    pub fn update(&mut self, health: Option<&mut Health>) {
        if self.in_air {
            if let Some(health) = health {
                let delta_y = self.last_stable_ground.1 - self.pos.1;
                if delta_y > 4.0 {
                    *health -= delta_y;
                }
            }
        }
    }
}
