use std::{collections::HashSet, hash::Hash};

use serde::{Deserialize, Serialize};
use rsa_common::types::{LockableValue, RawId};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub id: RawId,
    pub collision: LockableValue<bool>,
    pub opaque: LockableValue<bool>,
}

#[cfg(test)]
pub mod tests {
    use rsa_common::types::LockableValue::Fixed;
    use rsa_common::types::RawId;
    use crate::Tile;

    pub fn new(id: RawId) -> Tile {
        Tile {
            id,
            collision: Fixed(true),
            opaque: Fixed(true),
        }
    }
}
