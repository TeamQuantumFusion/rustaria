use crate::chunk::tile::{BreakResistance, LockableValue};
use crate::registry::RawId;
use mlua::prelude::LuaUserData;

use crate::api::Prototype;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Wall {
    id: RawId,
    opaque: LockableValue<bool>,
    break_resistance: BreakResistance,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WallPrototype {
    #[serde(default = "WallPrototype::default_opaque")]
    opaque: LockableValue<bool>,
    #[serde(default = "WallPrototype::default_break_resistance")]
    break_resistance: BreakResistance,
}

impl Prototype<Wall> for WallPrototype {
    fn create(&self, id: RawId) -> Wall {
        Wall {
            id,
            opaque: self.opaque,
            break_resistance: self.break_resistance,
        }
    }
}

impl LuaUserData for WallPrototype {}

impl WallPrototype {
    pub fn default_opaque() -> LockableValue<bool> {
        LockableValue::Fixed(true)
    }

    pub fn default_break_resistance() -> BreakResistance {
        BreakResistance::Hammer(20)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::chunk::tile::LockableValue::Fixed;
    use crate::chunk::tile::{BreakResistance, Tile};
    use crate::chunk::wall::Wall;
    use crate::registry::RawId;

    pub fn new(id: RawId) -> Wall {
        Wall {
            id,
            break_resistance: BreakResistance::Any,
            opaque: Fixed(true),
        }
    }
}