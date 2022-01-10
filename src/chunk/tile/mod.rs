use std::{collections::HashSet, hash::Hash};

use mlua::prelude::*;
use serde::Deserialize;

use crate::registry::Tag;

#[derive(Clone, Debug, Deserialize)]
pub struct Tile {
    // name: LanguageKey,
    // sprite_path: AssetLocation,
    #[serde(default)]
    transitional: bool,
    #[serde(default = "Tile::default_collision")]
    collision: DynamicValue<bool>,
    #[serde(default = "Tile::default_opaque")]
    opaque: DynamicValue<bool>,
    #[serde(default = "Tile::default_blast_resistance")]
    blast_resistance: BlastResistance,
    #[serde(default = "Tile::default_break_resistance")]
    break_resistance: BreakResistance,
    #[serde(default)]
    tile_type: TileType<Tag>,
}
impl Tile {
    fn default_collision() -> DynamicValue<bool> {
        DynamicValue::Fixed(true)
    }
    fn default_opaque() -> DynamicValue<bool> {
        DynamicValue::Fixed(true)
    }
    fn default_blast_resistance() -> BlastResistance {
        BlastResistance::Some(3)
    }
    fn default_break_resistance() -> BreakResistance {
        BreakResistance::Any
    }
}

impl LuaUserData for Tile {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DynamicValue<T> {
    // stored in the global tile
    Fixed(T),
    // stored per tile
    Dynamic,
}

#[derive(Clone, Debug)]
pub enum BlastResistance {
    Some(u32),
    Indestructible,
}

#[derive(Clone, Debug, Deserialize)]
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
    use serde::de::{Error, Visitor};
    use serde::{Deserialize, Deserializer};

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
