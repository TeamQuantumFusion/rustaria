use serde::{Deserialize, Serialize};
use rsa_common::types::{BreakResistance, LockableValue, RawId};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Wall {
    pub id: RawId,
    pub opaque: LockableValue<bool>,
    pub break_resistance: BreakResistance,
}

#[cfg(test)]
pub mod tests {
    use rsa_common::types::{BreakResistance, LockableValue, RawId};
    use crate::Wall;

    pub fn new(id: RawId) -> Wall {
        Wall {
            id,
            break_resistance: BreakResistance::Any,
            opaque: LockableValue::Fixed(true),
        }
    }
}
