use apollo::{Table, Value};

use crate::err::{audit::Audit, Result};

pub fn lua_table(value: Value) -> Result<Table> {
	match value {
		Value::Table(value) => Ok(value),
		_ => Err(Audit::new(format!("Expected table not {value:?}"))),
	}
}

pub fn lua_string(value: Value) -> Result<String> {
	match value {
		Value::String(value) => Ok(value.to_str()?.to_string()),
		_ => Err(Audit::new(format!("Expected string not {value:?}"))),
	}
}
