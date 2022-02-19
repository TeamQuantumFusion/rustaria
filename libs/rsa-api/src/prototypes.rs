use mlua::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use rsa_chunk::tile::Tile;
use rsa_chunk::wall::Wall;
use rsa_common::types::{BlastResistance, BreakResistance, LockableValue, RawId, TileType};
use rsa_comps::health::HealthPrototype;
use rsa_comps::physics::PhysicsPrototype;
use rsa_comps::ToComponent;
use crate::registry::Tag;

pub trait Prototype {
    type Item;

    fn create(&self, id: RawId) -> Self::Item;
}

#[derive(Clone, Debug, Deserialize)]
pub struct TilePrototype {
    // name: LanguageKey,
    #[serde(default)]
    pub sprite: Option<Tag>,
    #[serde(default)]
    pub transitional: bool,
    #[serde(default = "TilePrototype::default_collision")]
    pub collision: LockableValue<bool>,
    #[serde(default = "TilePrototype::default_opaque")]
    pub opaque: LockableValue<bool>,
    #[serde(default = "TilePrototype::default_blast_resistance")]
    pub blast_resistance: BlastResistance,
    #[serde(default = "TilePrototype::default_break_resistance")]
    pub break_resistance: BreakResistance,
    #[serde(default)]
    pub tile_type: TileType<Tag>,
}
impl TilePrototype {
    fn default_collision() -> LockableValue<bool> {
        LockableValue::Dynamic(true)
    }
    fn default_opaque() -> LockableValue<bool> {
        LockableValue::Dynamic(true)
    }
    fn default_blast_resistance() -> BlastResistance {
        BlastResistance::Some(3)
    }
    fn default_break_resistance() -> BreakResistance {
        BreakResistance::Any
    }
}
impl Prototype for TilePrototype {
    type Item = Tile;

    fn create(&self, id: RawId) -> Tile {
        Tile {
            id,
            collision: self.collision,
            opaque: self.opaque,
        }
    }
}
impl LuaUserData for TilePrototype {}

#[derive(Clone, Debug, Deserialize)]
pub struct WallPrototype {
    #[serde(default = "WallPrototype::default_opaque")]
    opaque: LockableValue<bool>,
    #[serde(default = "WallPrototype::default_break_resistance")]
    break_resistance: BreakResistance,
}
impl WallPrototype {
    pub fn default_opaque() -> LockableValue<bool> {
        LockableValue::Fixed(true)
    }

    pub fn default_break_resistance() -> BreakResistance {
        BreakResistance::Hammer(20)
    }
}
impl Prototype for WallPrototype {
    type Item = Wall;

    fn create(&self, id: RawId) -> Self::Item {
        Self::Item {
            id,
            opaque: self.opaque,
            break_resistance: self.break_resistance,
        }
    }
}
impl LuaUserData for WallPrototype {}

#[derive(Debug, Clone, Deserialize)]
pub struct EntityPrototype {
    health: Option<HealthPrototype>,

    #[serde(default)]
    physics: PhysicsPrototype,
}
impl LuaUserData for EntityPrototype {}
