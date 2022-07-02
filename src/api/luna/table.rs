use std::{any::type_name, fmt::Display};
use anyways::ext::AuditExt;

use anyways::Result;
use apollo::{FromLua, Lua, LuaSerdeExt, Table, ToLua};
use serde::Deserialize;
use crate::ty::identifier::Identifier;

pub struct LunaTable<'a> {
	pub lua: &'a Lua,
	pub table: Table,
}

impl<'a> LunaTable<'a> {
	pub fn get<K: ToLua + Display + Clone, V: FromLua>(&self, key: K) -> Result<V> {
		self.table
			.get(key.clone())
			.wrap_err_with(|| format!("Getting field \"{}\"", key))
	}
	
	pub fn get_ser<'de, K: ToLua + Display + Clone, V: Deserialize<'de>>(
		&self,
		key: K,
	) -> Result<V> {
		self.lua
			.from_value(self.get(key.clone())?)
			.wrap_err_with(|| format!("Converting field \"{}\" to {}", key, type_name::<V>()))
	}
}
