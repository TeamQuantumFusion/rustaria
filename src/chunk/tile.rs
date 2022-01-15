use std::{collections::HashSet, hash::Hash};

use mlua::prelude::*;
use serde::Deserialize;
use crate::api::plugin::AssetPath;
use crate::api::Prototype;

use crate::registry::{AssetLocation, Id, Tag};

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    id: Id,
    collision: LockableValue<bool>,
    opaque: LockableValue<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TilePrototype {
    // name: LanguageKey,
    // sprite_path: AssetLocation,
    #[serde(default)]
    pub sprite: Option<AssetLocation>,
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

impl Prototype<Tile> for TilePrototype {
    fn create(&self, id: Id) -> Tile {
        Tile {
            id,
            collision: self.collision,
            opaque: self.opaque
        }
    }
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

impl LuaUserData for TilePrototype {}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockableValue<T> {
    Fixed(T),
    Dynamic(T),
}

#[derive(Copy, Clone, Debug)]
pub enum BlastResistance {
    Some(u32),
    Indestructible,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakResistance {
    Any,
    Indestructible,
    Axe(u32),
    Pickaxe(u32),
    Hammer(u32),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TileType<T: Hash + Eq> {
    Default,
    Spreadable {
        spread_chance: f32,
        filter: Filter<T>,
    },
}

impl<T: Hash + Eq> Default for TileType<T> {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Filter<T: Hash + Eq> {
    All,
    None,
    Whitelist(HashSet<T>),
    Blacklist(HashSet<T>),
}

mod blast_resistance_serde {
    use serde::{Deserialize, Deserializer};
    use serde::de::{Error, Visitor};

    use super::BlastResistance;

    impl<'de> Deserialize<'de> for BlastResistance {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
        {
            struct BRVisitor;
            impl<'de> Visitor<'de> for BRVisitor {
                type Value = BlastResistance;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, r#"either a string "indestructible" or a number"#)
                }
                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: Error,
                {
                    let v = u32::try_from(v).map_err(Error::custom)?;
                    Ok(BlastResistance::Some(v))
                }
                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: Error,
                {
                    if v.eq_ignore_ascii_case("indestructible") {
                        Ok(BlastResistance::Indestructible)
                    } else {
                        Err(Error::custom(format!(
                            r#"Expected string "indestructible"; found string "{}""#,
                            v
                        )))
                    }
                }
            }
            deserializer.deserialize_any(BRVisitor)
        }
    }
}
