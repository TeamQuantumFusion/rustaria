use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use log::trace;

use crate::ty::Tag;
use crate::error::Result;
use mlua::{Function, ToLuaMulti, UserData, UserDataMethods};
use crate::lua::error::LuaError;

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