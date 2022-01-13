use mlua::prelude::LuaUserData;
use crate::chunk::tile::{BreakResistance, LockableValue};
use crate::registry::Id;

use serde::Deserialize;
use crate::api::Prototype;

#[derive(Copy, Clone, Debug)]
pub struct Wall {
    id: Id,
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
    fn create(&self, id: Id) -> Wall {
        Wall {
            id,
            opaque: self.opaque,
            break_resistance: self.break_resistance
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