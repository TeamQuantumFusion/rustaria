use std::collections::hash_map::Entry;
use std::collections::HashMap;
use log::warn;
use mlua::{Function, Lua};
use mlua::prelude::LuaResult;
use crate::ty::Tag;
use apollo::*;
use crate::lua::{get_api, PluginLua};
use crate::lua::def::hook::HookInstance;

pub struct LuaHooks {
	builders: HashMap<Tag, LuaHookBuilder>
}

impl LuaHooks {
	pub fn new() -> LuaHooks {
		LuaHooks {
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
impl LuaHooks {
	#[lua_method]
	pub fn __index(&mut self, key: Tag) -> LuaResult<&mut LuaHookBuilder> {
		match self.builders.entry(key.clone()) {
			Entry::Occupied(mut _entry) => {
			}
			Entry::Vacant(mut entry) => {
				entry.insert(LuaHookBuilder {
					subscribers: Default::default()
				});
			}
		};
		Ok(self.builders.get_mut(&key).unwrap())
	}
}

pub struct LuaHookBuilder {
	subscribers: HashMap<Tag, Function>
}

#[lua_impl]
impl LuaHookBuilder {
	#[lua_method]
	pub fn subscribe(&mut self, lua: &Lua, name: Tag, function: Function) -> LuaResult<()> {
		if let Some(_) = self.subscribers.insert(name.clone(), function) {
			warn!("Plugin {} overwrote {name} hook.", PluginLua::import(lua).id)
		}

		Ok(())
	}
}
