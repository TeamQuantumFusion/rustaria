use anyways::audit::Audit;
use apollo::{FromLua, Lua, Value};
use crate::Identifier;

#[derive(Debug)]
pub struct RegistryKey {
	pub(crate) identifier: Identifier,
	pub(crate) priority: Option<f32>,
}

impl FromLua for RegistryKey {
	fn from_lua(lua_value: Value, lua: &Lua) -> anyways::Result<Self> {
		Ok(match lua_value {
			Value::String(_) => RegistryKey {
				identifier: Identifier::from_lua(lua_value, lua)?,
				priority: None,
			},
			Value::Table(table) => RegistryKey {
				identifier: Identifier::new_lua(table.get("name")?)?,
				priority: table.get::<_, Option<f32>>("priority")?,
			},
			_ => {
				return Err(Audit::new(
					"Registry type must be Table { name = , priority = } or an identifier",
				));
			}
		})
	}
}
