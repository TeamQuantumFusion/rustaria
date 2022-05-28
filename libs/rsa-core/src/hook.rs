use std::collections::HashMap;

use mlua::{Function, ToLuaMulti};

use crate::api::lua::error::LuaError;
use crate::error::Result;
use crate::ty::Tag;

#[derive(Default)]
pub struct HookInstance {
	pub(crate) hooks: HashMap<Tag, Vec<Function>>,
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
