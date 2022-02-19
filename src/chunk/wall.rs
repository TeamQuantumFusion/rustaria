use crate::{
    api::types::{BreakResistance, LockableValue},
    registry::RawId,
};
use mlua::prelude::LuaUserData;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Wall {
    pub id: RawId,
    pub opaque: LockableValue<bool>,
    pub break_resistance: BreakResistance,
}

#[cfg(test)]
pub mod tests {
    use crate::api::types::{BreakResistance, LockableValue};
    use crate::chunk::wall::Wall;
    use crate::registry::RawId;

    pub fn new(id: RawId) -> Wall {
        Wall {
            id,
            break_resistance: BreakResistance::Any,
            opaque: LockableValue::Fixed(true),
        }
    }
}
