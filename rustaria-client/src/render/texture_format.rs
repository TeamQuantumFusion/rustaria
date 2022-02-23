use rustaria::api::types::ConnectionType;

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
    pub fn new(up: ConnectionType, down: ConnectionType, left: ConnectionType, right: ConnectionType) -> TileImagePos {
        use ConnectionType::{Connected, Isolated};
        match (
            up,
            down,
            left,
            right,
        ) {
            (Connected, Connected, Connected, Connected) => TileImagePos::Solid,
            (Isolated, Isolated, Isolated, Isolated) => TileImagePos::Standalone,
            (Connected, Connected, Isolated, Isolated) => TileImagePos::Vertical,
            (Isolated, Isolated, Connected, Connected) => TileImagePos::Horizontal,

            (Isolated, Connected, Connected, Connected) => TileImagePos::UpFlat,
            (Isolated, Connected, Isolated, Isolated) => TileImagePos::UpEnd,
            (Isolated, Connected, Isolated, Connected) => TileImagePos::UpLeft,
            (Isolated, Connected, Connected, Isolated) => TileImagePos::UpRight,

            (Connected, Isolated, Connected, Connected) => TileImagePos::DownFlat,
            (Connected, Isolated, Isolated, Isolated) => TileImagePos::DownEnd,
            (Connected, Isolated, Isolated, Connected) => TileImagePos::DownLeft,
            (Connected, Isolated, Connected, Isolated) => TileImagePos::DownRight,

            (Connected, Connected, Isolated, Connected) => TileImagePos::LeftFlat,
            (Isolated, Isolated, Isolated, Connected) => TileImagePos::LeftEnd,
            (Connected, Connected, Connected, Isolated) => TileImagePos::RightFlat,
            (Isolated, Isolated, Connected, Isolated) => TileImagePos::RightEnd,
            _ => TileImagePos::Solid,
        }
    }

    pub fn get_tex_pos(self) -> (f32, f32) {
        match self {
            TileImagePos::Solid => (0.0, 3.0),
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
