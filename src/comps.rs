use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub struct Comps {
    pub health: HashMap<CompId, Health>,
    pub physics: HashMap<CompId, Physics>,
}

impl Comps {
    pub fn new() -> Self {
        Self {
            health: HashMap::new(),
            physics: HashMap::new(),
        }
    }
    pub fn update(&mut self) {
        for (id, comp) in self.physics.iter_mut() {
            let health = self.health.get_mut(id);
            comp.update(health);
        }
    }
}
pub struct Physics {
    pos: (f32, f32),
    last_stable_ground: (f32, f32),
    in_air: bool,
}
impl Physics {
    fn update(&mut self, health: Option<&mut Health>) {
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

type CompId = usize;

// TODO(leocth): could be generalized
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    current: f32,
    maximum: f32,
}

impl Health {
    pub fn new(current: f32, maximum: f32) -> Self {
        Self { current, maximum }
    }
    pub fn empty(maximum: f32) -> Self {
        Self::new(0.0, maximum)
    }
    pub fn full(maximum: f32) -> Self {
        Self::new(maximum, maximum)
    }
    pub fn get(self) -> f32 {
        self.current
    }
    pub fn set(mut self, v: f32) -> Self {
        self.current = v.clamp(0.0, self.maximum);
        self
    }
}
impl Add<f32> for Health {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        self.set(self.get() + rhs)
    }
}
impl Add<Health> for Health {
    type Output = Self;

    fn add(self, rhs: Health) -> Self::Output {
        self.set(self.get() + rhs.get())
    }
}
impl Sub<f32> for Health {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        self.set(self.get() - rhs)
    }
}
impl Sub<Health> for Health {
    type Output = Self;

    fn sub(self, rhs: Health) -> Self::Output {
        self.set(self.get() - rhs.get())
    }
}
impl AddAssign<f32> for Health {
    fn add_assign(&mut self, rhs: f32) {
        self.set(self.get() + rhs);
    }
}
impl AddAssign<Health> for Health {
    fn add_assign(&mut self, rhs: Health) {
        self.set(self.get() + rhs.get());
    }
}
impl SubAssign<f32> for Health {
    fn sub_assign(&mut self, rhs: f32) {
        self.set(self.get() - rhs);
    }
}
impl SubAssign<Health> for Health {
    fn sub_assign(&mut self, rhs: Health) {
        self.set(self.get() - rhs.get());
    }
}
