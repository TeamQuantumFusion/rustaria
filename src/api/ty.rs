use mlua::{FromLua, Lua, ToLua, Value};
use std::{collections::HashSet, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LockableValue<T> {
	Fixed(T),
	Dynamic(T),
}

impl<T> LockableValue<T> {
	pub fn default(&self) -> &T {
		match self {
			LockableValue::Dynamic(v) | LockableValue::Fixed(v) => v,
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Deserialize, Serialize, frogelua::FromLua)]
#[serde(rename_all = "snake_case")]
#[use_default]
pub enum ConnectionType {
	// air
	Isolated,
	// tiles
	Connected,
	// dirt
	Transitional,
}

impl Default for ConnectionType {
	fn default() -> Self {
		ConnectionType::Connected
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakResistance {
	Any,
	Indestructible,
	Axe(u32),
	Pickaxe(u32),
	Hammer(u32),
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
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

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Filter<T: Hash + Eq> {
	All,
	None,
	Whitelist(HashSet<T>),
	Blacklist(HashSet<T>),
}

#[derive(Clone, PartialEq, Debug, Serialize)]
pub enum BlastResistance {
	Some(u32),
	Indestructible,
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
