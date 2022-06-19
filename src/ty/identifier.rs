use std::fmt::{Display, Formatter, Write};

use apollo::{FromLua, Lua, Value};

use crate::api::util;
use apollo::impl_macro::*;

/// The identifier is a dual-string notifying which mod (namespace) the entry is from. and what it is.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Identifier {
	pub namespace: String,
	pub path: String,
}

#[lua_impl]
impl Identifier {
	pub fn new(path: &'static str) -> Identifier {
		Identifier {
			namespace: "rustaria".to_string(),
			path: path.to_string(),
		}
	}

	#[lua_method(new)]
	pub fn new_lua(value: Value) -> eyre::Result<Self> {
		let string = util::lua_string(value)?;
		if let Some((namespace, path)) = string.split_once(':') {
			Ok(Identifier {
				namespace: namespace.to_string(),
				path: path.to_string(),
			})
		} else {
			Ok(Identifier {
				namespace: "rustaria".to_string(),
				path: string,
			})
		}
	}

	#[lua_method]
	pub fn namespace(&mut self) -> (String) {
		self.namespace.clone()
	}

	#[lua_method]
	pub fn path(&mut self) -> (String) {
		self.path.clone()
	}

	#[from_lua]
	fn from_lua(lua_value: Value, _: &Lua) -> eyre::Result<Self> {
		Identifier::new_lua(lua_value)
	}
}

impl Display for Identifier {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.namespace)?;
		f.write_char(':')?;
		f.write_str(&self.path)?;
		Ok(())
	}
}
