use std::ops::{Add, AddAssign, SubAssign, Sub};

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}


impl From<[f32; 2]> for Pos {
    fn from([x, y]: [f32; 2]) -> Self {
        Pos { x, y }
    }
}
impl From<Pos> for [f32; 2] {
    fn from(pos: Pos) -> Self {
        [pos.x, pos.y]
    }
}
