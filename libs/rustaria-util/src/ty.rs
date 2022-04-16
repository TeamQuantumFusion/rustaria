//! A collection of types used in Rustaria.

use num::FromPrimitive;
use pos::Pos;
use std::ops::Add;

pub mod pos;

pub enum Error {
    OutOfBounds,
}

pub const CHUNK_SIZE: u8 = 16;
pub const CHUNK_SIZE_MASK: u8 = 0xf;
pub const CHUNK_SIZE_F: f32 = 16.0;

// lets later implement corner directions.
pub trait Offset<D>: Sized {
    fn wrapping_offset(self, displacement: D) -> Self;
    fn checked_offset(self, displacement: D) -> Option<Self>;
}

// ======================================== DIRECTION ========================================
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn clockwise(self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
    pub fn counterclockwise(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        }
    }
    pub fn rotate_180(self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
    pub fn horizontal_flip(self) -> Self {
        use Direction::*;
        match self {
            Left => Right,
            Right => Left,
            other => other,
        }
    }
    pub fn vertical_flip(self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            other => other,
        }
    }

    pub fn values() -> [Direction; 4] {
        use Direction::*;
        [Up, Left, Down, Right]
    }
}

// ======================================== POSITION ========================================
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct ChunkPos {
    pub x: u32,
    pub y: u32,
}
impl Offset<Direction> for ChunkPos {
    fn wrapping_offset(mut self, displacement: Direction) -> Self {
        use Direction::*;
        match displacement {
            Left => self.x = self.x.wrapping_sub(1),
            Right => self.x = self.x.wrapping_add(1),
            Down => self.y = self.y.wrapping_sub(1),
            Up => self.y = self.y.wrapping_add(1),
        }
        self
    }
    fn checked_offset(mut self, displacement: Direction) -> Option<Self> {
        use Direction::*;
        match displacement {
            Left => self.x = self.x.checked_sub(1)?,
            Right => self.x = self.x.checked_add(1)?,
            Down => self.y = self.y.checked_sub(1)?,
            Up => self.y = self.y.checked_add(1)?,
        };
        Some(self)
    }
}

impl Offset<(i32, i32)> for ChunkPos {
    fn wrapping_offset(self, (dx, dy): (i32, i32)) -> Self {
        // NOTE(leocth): no joke. this is how `wrapping_add_signed` is implemented.
        // https://doc.rust-lang.org/src/core/num/uint_macros.rs.html#1205-1207

        Self {
            x: self.x + dx as u32,
            y: self.y + dy as u32,
        }
    }
    fn checked_offset(self, (dx, dy): (i32, i32)) -> Option<Self> {
        let x = checked_add_signed_u32(self.x, dx)?;
        let y = checked_add_signed_u32(self.y, dy)?;
        Some(Self { x, y })
    }
}

#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct ChunkSubPos(u8);

impl ChunkSubPos {
    pub fn new(x: u8, y: u8) -> Self {
        assert!(x < CHUNK_SIZE, "x is out-of-bounds: {x} >= {CHUNK_SIZE}");
        assert!(y < CHUNK_SIZE, "y is out-of-bounds: {y} >= {CHUNK_SIZE}");
        Self::new_unchecked(x, y)
    }
    pub fn try_new(x: u8, y: u8) -> Option<Self> {
        if x < CHUNK_SIZE || y < CHUNK_SIZE {
            None
        } else {
            Some(Self::new_unchecked(x, y))
        }
    }
    fn new_unchecked(x: u8, y: u8) -> Self {
        Self((x << 4) | y)
    }

    pub fn x(self) -> u8 {
        self.0 >> 4
    }
    pub fn y(self) -> u8 {
        self.0 & CHUNK_SIZE_MASK
    }
    pub fn euclid_offset(self, (dx, dy): (i8, i8)) -> Self {
        // SAFETY: `rem_euclid` returns a number lesser than `CHUNK_SIZE`.
        Self::new_unchecked(
            (self.x() as i16 + dx as i16).rem_euclid(CHUNK_SIZE as i16) as u8,
            (self.y() as i16 + dy as i16).rem_euclid(CHUNK_SIZE as i16) as u8,
        )
    }
}

// TODO(leocth): add Offset<ChunkSubPos> impl for (i8, i8)
impl Offset<(i8, i8)> for ChunkSubPos {
    fn wrapping_offset(self, (dx, dy): (i8, i8)) -> Self {
        // NOTE(leocth): no joke. this is how `wrapping_add_signed` is implemented.
        // https://doc.rust-lang.org/src/core/num/uint_macros.rs.html#1205-1207

        let x = (self.x() + dx as u8) & CHUNK_SIZE_MASK;
        let y = (self.y() + dy as u8) & CHUNK_SIZE_MASK;
        // SAFETY: x and y are no greater than or equal to CHUNK_SIZE after ANDing with CHUNK_SIZE_MASK.
        Self::new_unchecked(x, y)
    }

    fn checked_offset(self, (dx, dy): (i8, i8)) -> Option<Self> {
        let x = checked_add_signed_u8(self.x(), dx)?;
        let y = checked_add_signed_u8(self.y(), dy)?;
        Self::try_new(x, y)
    }
}

#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct TilePos {
    pub chunk: ChunkPos,
    pub sub: ChunkSubPos,
}

impl Offset<(i8, i8)> for TilePos {
    fn wrapping_offset(self, displacement @ (dx, dy): (i8, i8)) -> Self {
        match Self::checked_offset(self, displacement) {
            Some(s) => s,
            None => Self {
                chunk: self.chunk.wrapping_offset((dx as i32, dy as i32)),
                sub: self.sub.euclid_offset(displacement),
            }
        }
    }

    fn checked_offset(self, displacement @ (dx, dy): (i8, i8)) -> Option<Self> {
        Some(match self.sub.checked_offset(displacement) {
            Some(sub) => Self {
                chunk: self.chunk,
                sub,
            },
            None => Self {
                chunk: self.chunk.checked_offset((dx as i32, dy as i32))?,
                sub: self.sub.euclid_offset(displacement),
            },
        })
    }
}

impl TryFrom<Pos> for ChunkPos {
    type Error = Error;

    fn try_from(value: Pos) -> Result<Self, Self::Error> {
        Ok(ChunkPos {
            x: u32::from_f32(value.x / CHUNK_SIZE_F).ok_or(Error::OutOfBounds)?,
            y: u32::from_f32(value.y / CHUNK_SIZE_F).ok_or(Error::OutOfBounds)?,
        })
    }
}

/// A boostable value has a base value that is basically static and a boost value which gets filled every tick.
pub struct BoostableValue<T: Add<Output = T> + Default + Copy> {
    base_value: T,
    boost_value: T,
}

impl<T: Add<Output = T> + Default + Copy> BoostableValue<T> {
    pub fn new(value: T) -> BoostableValue<T> {
        BoostableValue {
            base_value: value,
            boost_value: T::default(),
        }
    }

    pub fn val(&self) -> T {
        self.base_value + self.boost_value
    }
}

#[inline]
fn checked_add_signed_u32(a: u32, b: i32) -> Option<u32> {
    // XXX(leocth):
    // replace with std's `checked_add_signed` when `mixed_integer_ops` reaches stable.
    // see https://github.com/rust-lang/rust/issues/87840
    let (res, overflowed) = a.overflowing_add(b as u32);
    if overflowed ^ (b < 0) {
        None
    } else {
        Some(res)
    }
}

#[inline]
fn checked_add_signed_u8(a: u8, b: i8) -> Option<u8> {
    // XXX(leocth):
    // replace with std's `checked_add_signed` when `mixed_integer_ops` reaches stable.
    // see https://github.com/rust-lang/rust/issues/87840
    let (res, overflowed) = a.overflowing_add(b as u8);
    if overflowed ^ (b < 0) {
        None
    } else {
        Some(res)
    }
}

