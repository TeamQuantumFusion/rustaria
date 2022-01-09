use std::{collections::HashSet, marker::PhantomData};

use mlua::prelude::*;
use serde::{Deserialize, Serialize};

use crate::registry::{AssetLocation, LanguageKey, Tag};

#[derive(Clone, Debug, Deserialize)]
pub struct Tile {
    // name: LanguageKey,
    // sprite_path: AssetLocation,
    // transitional: bool,
    // collision: DynamicValue<bool>,
    // opaque: DynamicValue<bool>,
    blast_resistance: BlastResistance,
    break_resistance: BreakResistance,
    // tile_type: TileType<Tag>,
}

impl LuaUserData for Tile {}

#[derive(Clone)]
pub enum DynamicValue<T> {
    // stored in the global tile
    Fixed,
    // stored per tile
    Dynamic(T),
}

#[derive(Clone, Debug)]
pub enum BlastResistance {
    Some(u32),
    Indestructible,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BreakResistance {
    Any,
    Indestructible,
    Axe(u32),
    Pickaxe(u32),
    Hammer(u32),
}

#[derive(Clone)]
pub enum TileType<T> {
    Default,
    Spreadable {
        spread_chance: f32,
        filter: Filter<T>,
    },
}

#[derive(Clone)]
pub enum Filter<T> {
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
