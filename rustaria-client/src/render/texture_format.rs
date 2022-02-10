#[derive(Copy, Clone)]
pub enum TileImagePos {
    Solid,
    Vertical,
    Horizontal,
    Standalone,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    UpFlat,
    DownFlat,
    LeftFlat,
    RightFlat,
    UpEnd,
    DownEnd,
    LeftEnd,
    RightEnd,
}

impl TileImagePos {
    pub fn get_tex_pos(self) -> (f32, f32) {
        match self {
            TileImagePos::Solid => (1.0, 2.0),
            TileImagePos::Vertical => (0.0, 2.0),
            TileImagePos::Horizontal => (1.0, 3.0),
            TileImagePos::Standalone => (1.0, 2.0),
            TileImagePos::UpLeft => (0.0, 0.0),
            TileImagePos::UpRight => (1.0, 0.0),
            TileImagePos::DownLeft => (0.0, 1.0),
            TileImagePos::DownRight => (1.0, 1.0),
            TileImagePos::UpFlat => (3.0, 0.0),
            TileImagePos::DownFlat => (3.0, 1.0),
            TileImagePos::LeftFlat => (3.0, 2.0),
            TileImagePos::RightFlat => (3.0, 3.0),
            TileImagePos::UpEnd => (2.0, 0.0),
            TileImagePos::DownEnd => (2.0, 1.0),
            TileImagePos::LeftEnd => (2.0, 2.0),
            TileImagePos::RightEnd => (2.0, 3.0),
        }
    }
}