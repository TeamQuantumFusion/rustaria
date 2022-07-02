use anyways::audit::Audit;
use apollo::{Table, Value};

pub fn lua_table(value: Value) -> anyways::Result<Table> {
	match value {
		Value::Table(value) => Ok(value),
		_ => Err(Audit::new(format!("Expected table not {value:?}"))),
	}
}

pub fn lua_string(value: Value) -> anyways::Result<String> {
	match value {
		Value::String(value) => Ok(value.to_str()?.to_string()),
		_ => Err(Audit::new(format!("Expected string not {value:?}"))),
	}
}
