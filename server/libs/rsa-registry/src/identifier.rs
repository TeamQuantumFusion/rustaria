use std::fmt::{Display, Formatter, Write};

use apollo::{FromLua, Lua, ToLua, Value};


/// The identifier is a dual-string notifying which mod (namespace) the entry is from. and what it is.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Identifier {
	pub namespace: String,
	pub path: String,
}

impl Identifier {
	pub fn new(path: &'static str) -> Identifier {
		Identifier {
			namespace: "rustaria".to_string(),
			path: path.to_string(),
		}
	}

	pub fn new_lua(string: String) -> anyways::Result<Self> {
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
}

#[cfg(feature = "testing")]
impl Identifier {
	pub fn test(path: &'static str) -> Identifier {
		Identifier {
			namespace: "test".to_string(),
			path: path.to_string(),
		}
	}
}

impl FromLua for Identifier {
	fn from_lua(lua_value: Value, _: &Lua) -> anyways::Result<Self> {
		let string = lua_value.lua_string()?;
		Identifier::new_lua(string)
	}
}

impl ToLua for Identifier {
	fn to_lua(self, lua: &Lua) -> anyways::Result<Value> {
		Ok(Value::String(lua.create_string(&format!(
			"{}:{}",
			self.namespace, self.path
		))?))
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
