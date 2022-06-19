use apollo::{Table, Value};

pub fn lua_table(value: Value) -> apollo::Result<Table> {
	match value {
		Value::Table(value) => Ok(value),
		_ => Err(apollo::Error::external(format!(
			"Expected table not {value:?}"
		))),
	}
}

pub fn lua_string(value: Value) -> apollo::Result<String> {
	match value {
		Value::String(value) => Ok(value.to_str()?.to_string()),
		_ => Err(apollo::Error::external(format!(
			"Expected string not {value:?}"
		))),
	}
}
