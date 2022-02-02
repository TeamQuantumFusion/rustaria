//! A collection of types used in Rustaria.

pub const CHUNK_SIZE: usize = 24;

// lets later implement corner directions.
pub trait Offset {
    fn offset_x(self) -> i8;
    fn offset_y(self) -> i8;
}

// ======================================== DIRECTION ========================================
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Direction {
    Top,
    Left,
    Bottom,
    Right,
}

impl Direction {
    pub fn cw(self) -> Self {
        match self {
            Direction::Top => Direction::Left,
            Direction::Left => Direction::Bottom,
            Direction::Bottom => Direction::Right,
            Direction::Right => Direction::Top,
        }
    }

    pub fn ccw(self) -> Self {
        match self {
            Direction::Top => Direction::Right,
            Direction::Left => Direction::Top,
            Direction::Bottom => Direction::Left,
            Direction::Right => Direction::Bottom,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn all() -> [Direction; 4] {
        [
            Direction::Top,
            Direction::Left,
            Direction::Bottom,
            Direction::Right,
        ]
    }
}

impl Offset for Direction {
    fn offset_y(self) -> i8 {
        match self {
            Direction::Top => 1,
            Direction::Bottom => -1,
            _ => 0,
        }
    }

    fn offset_x(self) -> i8 {
        match self {
            Direction::Left => 1,
            Direction::Right => -1,
            _ => 0,
        }
    }
}

// ======================================== POSITION ========================================
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChunkPos {
    pub x: u32,
    pub y: u32,
}

impl ChunkPos {
    pub fn offset<O: Offset + Copy>(&self, offset: O) -> Option<Self> {
        // FIXME(leocth): this is cursed
        Some(Self {
            x: u32::try_from((self.x as i64).checked_add(offset.offset_x() as i64)?).ok()?,
            y: u32::try_from((self.y as i64).checked_add(offset.offset_y() as i64)?).ok()?,
        })
    }

    pub(crate) fn get_raw_pos(&self, (world_w, world_h): (u32, u32)) -> Option<usize> {
        if self.y >= world_w || self.x >= world_h {
            return None;
        }

        Some(self.x as usize + (self.y as usize * world_w as usize))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChunkSubPos {
    pub x: u8,
    pub y: u8,
}

impl ChunkSubPos {
    pub fn offset<O: Offset + Copy>(&self, offset: O) -> Option<Self> {
        let x_raw = u8::try_from((self.x as i16).checked_add(offset.offset_x() as i16)?).ok()?;
        let y_raw = u8::try_from((self.y as i16).checked_add(offset.offset_y() as i16)?).ok()?;
        if x_raw >= CHUNK_SIZE as u8 || y_raw >= CHUNK_SIZE as u8 {
            None
        } else {
            Some(Self { x: x_raw, y: y_raw })
        }
    }

    pub fn overflowing_offset<O: Offset + Copy>(&self, offset: O) -> Self {
        let mut x_raw = (self.x as i16).overflowing_add(offset.offset_x() as i16).0;
        let mut y_raw = (self.y as i16).overflowing_add(offset.offset_y() as i16).0;
        if x_raw >= CHUNK_SIZE as i16 {
            x_raw = 0;
        }

        if x_raw < 0 {
            x_raw = CHUNK_SIZE as i16 - 1;
        }

        if y_raw >= CHUNK_SIZE as i16 {
            y_raw = 0;
        }

        if y_raw < 0 {
            y_raw = CHUNK_SIZE as i16 - 1;
        }

        Self {
            x: x_raw as u8,
            y: y_raw as u8,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TilePos {
    chunk: ChunkPos,
    sub: ChunkSubPos,
}

impl TilePos {
    pub fn new(x: u64, y: u64) -> Option<Self> {
        Some(Self {
            chunk: ChunkPos {
                x: u32::try_from(x / CHUNK_SIZE as u64).ok()?,
                y: u32::try_from(y / CHUNK_SIZE as u64).ok()?,
            },
            sub: ChunkSubPos {
                x: (x % CHUNK_SIZE as u64) as u8,
                y: (y % CHUNK_SIZE as u64) as u8,
            },
        })
    }

    pub fn chunk_pos(&self) -> ChunkPos {
        self.chunk
    }

    pub fn sub_pos(&self) -> ChunkSubPos {
        self.sub
    }

    pub fn offset<O: Offset + Copy>(&self, offset: O) -> Option<Self> {
        Some(match self.sub.offset(offset) {
            Some(sub) => Self {
                chunk: self.chunk,
                sub,
            },
            None => Self {
                chunk: self.chunk.offset(offset)?,
                sub: self.sub.overflowing_offset(offset),
            },
        })
    }
}
