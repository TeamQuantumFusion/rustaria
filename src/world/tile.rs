use serde::{Deserialize, Serialize};

use rustaria_api::RawId;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Tile {
    pub id: RawId,
    pub collision: bool,
    pub opaque: bool,
}
