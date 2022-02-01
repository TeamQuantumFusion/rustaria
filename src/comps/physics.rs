use super::{health::Health, ToComponent};
use serde::Deserialize;

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

#[derive(Clone, Debug, Deserialize)]
pub struct PhysicsPrototype {}
impl ToComponent for PhysicsPrototype {
    type Comp = Physics;

    fn to_component(&self) -> Self::Comp {
        Self::Comp {
            pos: (0.0, 0.0),
            last_stable_ground: (0.0, 0.0),
            in_air: false,
        }
    }
}
