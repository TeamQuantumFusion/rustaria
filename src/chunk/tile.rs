use std::{collections::HashSet, hash::Hash};

use mlua::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::plugin::ArchivePath;
use crate::api::types::LockableValue;
use crate::registry::{RawId, Tag};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub id: RawId,
    pub collision: LockableValue<bool>,
    pub opaque: LockableValue<bool>,
}

#[cfg(test)]
pub mod tests {
    use crate::chunk::tile::LockableValue::Fixed;
    use crate::chunk::tile::Tile;
    use crate::registry::RawId;

    pub fn new(id: RawId) -> Tile {
        Tile {
            id,
            collision: Fixed(true),
            opaque: Fixed(true),
        }
    }
}
