// util class for common stuff
pub const CHUNK_SIZE: usize = 24;

// lets later implement corner directions.
pub trait OffsetAble {
    fn y(self) -> i8;
    fn x(self) -> i8;
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
}

impl OffsetAble for Direction {
    fn y(self) -> i8 {
        match self {
            Direction::Top => 1,
            Direction::Bottom => -1,
            _ => 0,
        }
    }

    fn x(self) -> i8 {
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
    x: i32,
    y: u32,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChunkSubPos {
    x: u8,
    y: u8,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TilePos {
    chunk: ChunkPos,
    sub: ChunkSubPos,
}

impl ChunkPos {
    pub fn new(x: i32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn offset<O: OffsetAble + Copy>(&self, offset: O) -> Option<Self> {
        Some(Self {
            x: i32::try_from((self.x as i64).checked_add(offset.x() as i64)?).ok()?,
            y: u32::try_from((self.y as i64).checked_add(offset.y() as i64)?).ok()?,
        })
    }
}

impl ChunkSubPos {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn offset<O: OffsetAble + Copy>(&self, offset: O) -> Option<Self> {
        let x_raw = u8::try_from((self.x as i16).checked_add(offset.x() as i16)?).ok()?;
        let y_raw = u8::try_from((self.y as i16).checked_add(offset.y() as i16)?).ok()?;
        if x_raw >= CHUNK_SIZE as u8 || y_raw >= CHUNK_SIZE as u8 {
            None
        } else {
            Some(Self { x: x_raw, y: y_raw })
        }
    }

    pub fn overflowing_offset<O: OffsetAble + Copy>(&self, offset: O) -> Self {
        let mut x_raw = (self.x as i16).overflowing_add(offset.x() as i16).0;
        let mut y_raw = (self.y as i16).overflowing_add(offset.y() as i16).0;
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

        Self { x: x_raw as u8, y: y_raw as u8 }
    }
}

impl TilePos {
    pub fn new(x: i64, y: u64) -> Option<Self> {
        let (chunk_x, chunk_y) = (x / CHUNK_SIZE as i64, y / CHUNK_SIZE as u64);
        let (sub_x, sub_y) = (x % CHUNK_SIZE as i64, y % CHUNK_SIZE as u64);

        Some(Self {
            chunk: ChunkPos {
                x: i32::try_from(chunk_x).ok()?,
                y: u32::try_from(chunk_y).ok()?,
            },
            sub: ChunkSubPos {
                x: sub_x as u8,
                y: sub_y as u8,
            },
        })
    }

    pub fn chunk_pos(&self) -> ChunkPos {
        self.chunk
    }

    pub fn sub_pos(&self) -> ChunkSubPos {
        self.sub
    }

    pub fn offset<O: OffsetAble + Copy>(&self, offset: O) -> Option<Self> {
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
