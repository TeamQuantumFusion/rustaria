use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use log::trace;

use crate::ty::Tag;
use crate::error::Result;
use mlua::{Function, ToLuaMulti, UserData, UserDataMethods};
use crate::lua::error::LuaError;

#[derive(Default)]
pub struct HookInstance {
	hooks: HashMap<Tag, Vec<Function>>,
}

impl HookInstance {
	pub fn trigger<F: FnOnce() -> A, A: ToLuaMulti + Clone>(
		&self,
		name: &Tag,
		func: F,
	) -> Result<()> {
		if let Some(subscribers) = self.hooks.get(name) {
			// Function used to lazily initialize the value.
			let args = func();
			for func in subscribers {
				func.call(args.clone()).lua_err()?;
			}
		}

		Ok(())
	}
}

#[derive(Default)]
pub struct HookInstanceBuilder {
	hooks: Arc<Mutex<HashMap<Tag, Vec<Function>>>>,
}

impl HookInstanceBuilder {
	pub fn lua(&self) -> HookInstanceLuaBuilder {
		HookInstanceLuaBuilder {
			hooks: Arc::downgrade(&self.hooks),
		}
	}

	pub fn export(self) -> HookInstance {
		// Yes that is a bit of unwrap. but it should NEVER fail.
		// if it does, then somehow this got cloned and the arc has multiple references.
		HookInstance {
			hooks: Arc::try_unwrap(self.hooks).unwrap().into_inner().unwrap(),
		}
	}
}

#[derive(Clone)]
pub struct HookInstanceLuaBuilder {
	hooks: Weak<Mutex<HashMap<Tag, Vec<Function>>>>,
}

impl UserData for HookInstanceLuaBuilder {
	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		methods.add_method("subscribe", |_, this, (tag, func): (Tag, Function)| {
			let arc = this.hooks.upgrade().unwrap();
			let mut write = arc.lock().unwrap();

			if let Some(functions) = write.get_mut(&tag) {
				functions.push(func);
			} else {
				write.insert(tag, vec![func]);
			}

			Ok(())
		})
	}
}
