//! A collection of types used in Rustaria.

use num::{FromPrimitive, PrimInt};

use crate::ty::Error::OOB;

pub enum Error {
    OOB,
}

pub const CHUNK_SIZE: usize = 16;

// lets later implement corner directions.
pub trait Offset {
    fn offset_x(self) -> i8;
    fn offset_y(self) -> i8;
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
    pub fn cw(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn ccw(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
    }
}

impl Offset for Direction {
    fn offset_x(self) -> i8 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }

    fn offset_y(self) -> i8 {
        match self {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        }
    }
}

impl Offset for (i8, i8) {
    fn offset_x(self) -> i8 {
        self.0
    }

    fn offset_y(self) -> i8 {
        self.1
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

impl ChunkPos {
    pub fn new(x: u64, y: u64) -> Option<ChunkPos> {
        Some(ChunkPos {
            x: u32::try_from(x).ok()?,
            y: u32::try_from(y).ok()?,
        })
    }

    pub fn offset<O: Into<i64> + Copy>(&self, offset: [O; 2]) -> Option<Self> {
        Some(Self {
            x: (self.x as i64).checked_add(offset[0].into())?.try_into().ok()?,
            y: (self.y as i64).checked_add(offset[1].into())?.try_into().ok()?,
        })
    }
}

#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct ChunkSubPos {
    pos: u8,
}

impl ChunkSubPos {
    pub fn new(x: u8, y: u8) -> ChunkSubPos {
        assert!(x < CHUNK_SIZE as u8);
        assert!(y < CHUNK_SIZE as u8);
        ChunkSubPos { pos: (x << 4) | y }
    }

    pub fn x(self) -> u8 {
        (self.pos >> 4) & 0xF
    }

    pub fn y(self) -> u8 {
        (self.pos) & 0xF
    }

    pub fn offset(&self, offset: [i8; 2]) -> Option<Self> {
        ChunkSubPos::try_from([
            u8::try_from((self.x() as i16).checked_add(offset[0] as i16)?).ok()?,
            u8::try_from((self.y() as i16).checked_add(offset[1] as i16)?).ok()?,
        ])
        .ok()
    }

    pub fn euclid_offset(&self, offset: [i8; 2]) -> Self {
        ChunkSubPos::new(
            (self.x() as i16)
                .checked_add(offset[0] as i16)
                .unwrap_or(0)
                .rem_euclid(CHUNK_SIZE as i16) as u8,
            (self.y() as i16)
                .checked_add(offset[1] as i16)
                .unwrap_or(0)
                .rem_euclid(CHUNK_SIZE as i16) as u8,
        )
    }
}

#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct TilePos {
    pub chunk: ChunkPos,
    pub sub: ChunkSubPos,
}

impl TilePos {
    pub fn offset(&self, offset: [i8; 2]) -> Option<Self> {
        Some(match self.sub.offset(offset) {
            Some(sub) => Self {
                chunk: self.chunk,
                sub,
            },
            None => Self {
                chunk: self.chunk.offset(offset)?,
                sub: self.sub.euclid_offset(offset),
            },
        })
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, serde::Serialize, serde::Deserialize)]

pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl From<Pos> for [f32; 2] {
    fn from(pos: Pos) -> Self {
        [pos.x, pos.y]
    }
}

impl From<Direction> for [i8; 2] {
    fn from(dir: Direction) -> Self {
        [dir.offset_x(), dir.offset_y()]
    }
}

impl From<[f32; 2]> for Pos {
    fn from(values: [f32; 2]) -> Self {
        Pos {
            x: values[0],
            y: values[1],
        }
    }
}

impl TryFrom<Pos> for ChunkPos {
    type Error = Error;

    fn try_from(value: Pos) -> Result<Self, Self::Error> {
        Ok(ChunkPos {
            x: u32::from_f32(value.x / CHUNK_SIZE as f32).ok_or(OOB)?,
            y: u32::from_f32(value.y / CHUNK_SIZE as f32).ok_or(OOB)?,
        })
    }
}

impl TryFrom<[u8; 2]> for ChunkSubPos {
    type Error = Error;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        if value[0] >= CHUNK_SIZE as u8 || value[1] >= CHUNK_SIZE as u8 {
            return Err(OOB);
        }

        Ok(Self::new(value[0], value[1]))
    }
}

impl TryFrom<Pos> for TilePos {
    type Error = Error;

    fn try_from(value: Pos) -> Result<Self, Self::Error> {
        Ok(TilePos {
            chunk: ChunkPos::try_from(value)?,
            sub: ChunkSubPos::new(
                u8::from_f32(value.x.rem_euclid(CHUNK_SIZE as f32)).ok_or(OOB)?,
                u8::from_f32(value.y.rem_euclid(CHUNK_SIZE as f32)).ok_or(OOB)?,
            ),
        })
    }
}
