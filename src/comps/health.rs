use std::ops::{Add, AddAssign, Sub, SubAssign};

// TODO(leocth): could be generalized to other kinds of 'containers', ig
// like fluids and such
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
