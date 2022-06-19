use apollo::{Table, Value};

pub fn lua_table(value: Value) -> eyre::Result<Table> {
	match value {
		Value::Table(value) => Ok(value),
		_ => Err(eyre::Report::msg(format!(
			"Expected table not {value:?}"
		))),
	}
}

pub fn lua_string(value: Value) -> eyre::Result<String> {
	match value {
		Value::String(value) => Ok(value.to_str()?.to_string()),
		_ => Err(eyre::Report::msg(format!(
			"Expected string not {value:?}"
		))),
	}
}
