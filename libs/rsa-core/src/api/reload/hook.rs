use std::collections::hash_map::Entry;
use std::collections::HashMap;
use log::warn;
use mlua::{Function, Lua};
use mlua::prelude::LuaResult;
use crate::ty::Tag;
use apollo::*;
use crate::hook::HookInstance;
use crate::api::lua::get_meta;

pub struct LuaHookBuilder {
	builders: HashMap<Tag, LuaHookDataBuilder>
}

impl LuaHookBuilder {
	pub fn new() -> LuaHookBuilder {
		LuaHookBuilder {
			builders: Default::default()
		}
	}

	pub fn finish(self, instance: &mut HookInstance) {
		instance.hooks.clear();
		for (hook, builder) in self.builders {
			let mut subscribers = Vec::new();
			for (_subscriber, func) in builder.subscribers {
				subscribers.push(func);
			}
			instance.hooks.insert(hook, subscribers);
		}
	}
}

#[lua_impl]
impl LuaHookBuilder {
	#[lua_method]
	pub fn __index(&mut self, key: Tag) -> LuaResult<&mut LuaHookDataBuilder> {
		match self.builders.entry(key.clone()) {
			Entry::Occupied(mut _entry) => {
			}
			Entry::Vacant(entry) => {
				entry.insert(LuaHookDataBuilder {
					subscribers: Default::default()
				});
			}
		};
		Ok(self.builders.get_mut(&key).unwrap())
	}
}

pub struct LuaHookDataBuilder {
	subscribers: HashMap<Tag, Function>
}

#[lua_impl]
impl LuaHookDataBuilder {
	#[lua_method]
	pub fn subscribe(&mut self, lua: &Lua, name: Tag, function: Function) -> LuaResult<()> {
		if let Some(_) = self.subscribers.insert(name.clone(), function) {
			let meta = get_meta(lua);
			warn!("Plugin {} overwrote {name} hook.", meta.plugin_id)
		}

		Ok(())
	}
}
