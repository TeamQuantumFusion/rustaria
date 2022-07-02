#![allow(clippy::wrong_self_convention)]

use std::{
	any::type_name,
	borrow::Cow,
	collections::{BTreeMap, BTreeSet, HashMap, HashSet},
	convert::TryInto,
	ffi::{CStr, CString},
	hash::{BuildHasher, Hash},
	string::String as StdString,
};
use anyways::ext::AuditExt;

use bstr::{BStr, BString};
use num_traits::cast;
use anyways::Result;

use crate::{
	error::Error,
	function::Function,
	lua::Lua,
	string::String,
	table::Table,
	thread::Thread,
	types::{LightUserData, LuaPointer, MaybeSend},
	userdata::{AnyUserData, UserData},
	value::{FromLua, ToLua, Value},
};

impl ToLua for Value {
	#[inline]
	fn to_lua(self, _: &Lua) -> Result<Value> { Ok(self) }
}

impl FromLua for Value {
	#[inline]
	fn from_lua(lua_value: Value, _: &Lua) -> Result<Self> { Ok(lua_value) }
}

impl ToLua for String {
	#[inline]
	fn to_lua(self, _: &Lua) -> Result<Value> { Ok(Value::String(self)) }
}

impl FromLua for String {
	#[inline]
	fn from_lua(value: Value, lua: &Lua) -> Result<String> {
		let ty = value.type_name();
		lua.coerce_string(value)?
			.ok_or_else(|| Error::FromLuaConversionError {
				from: ty,
				to: "String",
				message: Some("expected string or number".to_string()),
			})
			.wrap_err(type_name::<Self>())
	}
}

impl ToLua for Table {
	#[inline]
	fn to_lua(self, _: &Lua) -> Result<Value> { Ok(Value::Table(self)) }
}

impl FromLua for Table {
	#[inline]
	fn from_lua(value: Value, _: &Lua) -> Result<Table> {
		match value {
			Value::Table(table) => Ok(table),
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "table",
				message: None,
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl ToLua for Function {
	#[inline]
	fn to_lua(self, _: &Lua) -> Result<Value> { Ok(Value::Function(self)) }
}

impl FromLua for Function {
	#[inline]
	fn from_lua(value: Value, _: &Lua) -> Result<Function> {
		match value {
			Value::Function(table) => Ok(table),
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "function",
				message: None,
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl ToLua for Thread {
	#[inline]
	fn to_lua(self, _: &Lua) -> Result<Value> { Ok(Value::Thread(self)) }
}

impl FromLua for Thread {
	#[inline]
	fn from_lua(value: Value, _: &Lua) -> Result<Thread> {
		match value {
			Value::Thread(t) => Ok(t),
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "thread",
				message: None,
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl ToLua for AnyUserData {
	#[inline]
	fn to_lua(self, _: &Lua) -> Result<Value> { Ok(Value::UserData(self)) }
}

impl FromLua for AnyUserData {
	#[inline]
	fn from_lua(value: Value, _: &Lua) -> Result<AnyUserData> {
		match value {
			Value::UserData(ud) => Ok(ud),
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "userdata",
				message: None,
			})
			.wrap_err(format!(
				"Failed to convert {} from {}",
				value.type_name(),
				type_name::<Self>()
			)),
		}
	}
}



impl<T: UserData + MaybeSend + 'static> ToLua for T {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		UserDataCell::new(self).to_lua(lua)
	}
}
impl<T: UserData + MaybeSend + 'static + Clone> FromLua for T {
	fn from_lua(lua_value: Value, lua: &Lua) -> Result<Self> {
		if let Some(value) = T::from_lua(lua_value.clone(), lua)  {
			return Ok(value?);
		} else {
			Ok(UserDataCell::<T>::from_lua(lua_value, lua)?.get("unknown")?.borrow().clone())
		}
	}
}

impl<T: UserData + MaybeSend + 'static> ToLua for UserDataCell<T> {
	#[inline]
	fn to_lua(self, lua: &Lua) -> Result<Value> { Ok(Value::UserData(lua.create_userdata(self)?)) }
}

impl<T: UserData + MaybeSend + 'static> FromLua for UserDataCell<T> {
	#[inline]
	fn from_lua(value: Value, lua: &Lua) -> Result<UserDataCell<T>> {
		match value {
			Value::UserData(ud) => {
				Ok(ud.get_cell::<T>().wrap_err("Failed to get UserDataCell")?)
			},
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "userdata",
				message: None,
			})
			.wrap_err(format!(
				"Failed to convert {} to {}",
				value.type_name(),
				type_name::<Self>()
			)),
		}
	}
}

impl ToLua for Error {
	#[inline]
	fn to_lua(self, _: &Lua) -> Result<Value> { Ok(Value::Error(self)) }
}

impl FromLua for Error {
	#[inline]
	fn from_lua(value: Value, lua: &Lua) -> Result<Error> {
		match value {
			Value::Error(err) => Ok(err),
			val => Ok(Error::RuntimeError(
				lua.coerce_string(val)?
					.and_then(|s| Some(s.to_str().ok()?.to_owned()))
					.unwrap_or_else(|| "<unprintable error>".to_owned()),
			)),
		}
	}
}

impl ToLua for bool {
	#[inline]
	fn to_lua(self, _: &Lua) -> Result<Value> { Ok(Value::Boolean(self)) }
}

impl FromLua for bool {
	#[inline]
	fn from_lua(v: Value, _: &Lua) -> Result<Self> {
		match v {
			Value::Nil => Ok(false),
			Value::Boolean(b) => Ok(b),
			_ => Ok(true),
		}
	}
}

impl ToLua for LightUserData {
	#[inline]
	fn to_lua(self, _: &Lua) -> Result<Value> { Ok(Value::LightUserData(self)) }
}

impl FromLua for LightUserData {
	#[inline]
	fn from_lua(value: Value, _: &Lua) -> Result<Self> {
		match value {
			Value::LightUserData(ud) => Ok(ud),
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "light userdata",
				message: None,
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl ToLua for StdString {
	#[inline]
	fn to_lua(self, lua: &Lua) -> Result<Value> { Ok(Value::String(lua.create_string(&self)?)) }
}

impl FromLua for StdString {
	#[inline]
	fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
		let ty = value.type_name();
		Ok(lua
			.coerce_string(value)?
			.ok_or_else(|| Error::FromLuaConversionError {
				from: ty,
				to: "String",
				message: Some("expected string or number".to_string()),
			})?
			.to_str()?
			.to_owned())
	}
}

impl ToLua for &str {
	#[inline]
	fn to_lua(self, lua: &Lua) -> Result<Value> { Ok(Value::String(lua.create_string(self)?)) }
}

impl ToLua for Cow<'_, str> {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::String(lua.create_string(self.as_bytes())?))
	}
}

impl ToLua for Box<str> {
	fn to_lua(self, lua: &Lua) -> Result<Value> { Ok(Value::String(lua.create_string(&*self)?)) }
}

impl FromLua for Box<str> {
	fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
		let ty = value.type_name();
		Ok(lua
			.coerce_string(value)?
			.ok_or_else(|| Error::FromLuaConversionError {
				from: ty,
				to: "Box<str>",
				message: Some("expected string or number".to_string()),
			})?
			.to_str()?
			.to_owned()
			.into_boxed_str())
	}
}

impl ToLua for CString {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::String(lua.create_string(self.as_bytes())?))
	}
}

impl FromLua for CString {
	fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
		let ty = value.type_name();
		let string = lua
			.coerce_string(value)?
			.ok_or_else(|| Error::FromLuaConversionError {
				from: ty,
				to: "CString",
				message: Some("expected string or number".to_string()),
			})?;

		match CStr::from_bytes_with_nul(string.as_bytes_with_nul()) {
			Ok(s) => Ok(s.into()),
			Err(_) => Err(Error::FromLuaConversionError {
				from: ty,
				to: "CString",
				message: Some("invalid C-style string".to_string()),
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl ToLua for &CStr {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::String(lua.create_string(self.to_bytes())?))
	}
}

impl ToLua for Cow<'_, CStr> {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::String(lua.create_string(self.to_bytes())?))
	}
}

impl ToLua for BString {
	fn to_lua(self, lua: &Lua) -> Result<Value> { Ok(Value::String(lua.create_string(&self)?)) }
}

impl FromLua for BString {
	fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
		let ty = value.type_name();
		Ok(BString::from(
			lua.coerce_string(value)?
				.ok_or_else(|| Error::FromLuaConversionError {
					from: ty,
					to: "String",
					message: Some("expected string or number".to_string()),
				})?
				.as_bytes()
				.to_vec(),
		))
	}
}

impl ToLua for &BStr {
	fn to_lua(self, lua: &Lua) -> Result<Value> { Ok(Value::String(lua.create_string(&self)?)) }
}
use crate::userdata::{Ref, RefMut, UserDataCell};

macro_rules! lua_convert_int {
	($x:ty) => {
		impl ToLua for $x {
			fn to_lua(self, _: &Lua) -> anyways::Result<Value> {
				cast(self)
					.map(Value::Integer)
					.or_else(|| cast(self).map(Value::Number))
					// This is impossible error because conversion to Number never fails
					.ok_or_else(|| {
						anyways::audit::Audit::new(Error::ToLuaConversionError {
							from: stringify!($x),
							to: "number",
							message: Some("out of range".to_owned()),
						})
					})
			}
		}

		impl FromLua for $x {
			fn from_lua(value: Value, lua: &Lua) -> anyways::Result<Self> {
				let ty = value.type_name();
				(if let Value::Integer(i) = value {
					cast(i)
				} else if let Some(i) = lua.coerce_integer(value.clone())? {
					cast(i)
				} else {
					cast(lua.coerce_number(value)?.ok_or_else(|| {
						anyways::audit::Audit::new(Error::FromLuaConversionError {
							from: ty,
							to: stringify!($x),
							message: Some(
								"expected number or string coercible to number".to_string(),
							),
						})
					})?)
				})
				.ok_or_else(|| {
					anyways::audit::Audit::new(Error::FromLuaConversionError {
						from: ty,
						to: stringify!($x),
						message: Some("out of range".to_owned()),
					})
				})
			}
		}
	};
}

lua_convert_int!(i8);
lua_convert_int!(u8);
lua_convert_int!(i16);
lua_convert_int!(u16);
lua_convert_int!(i32);
lua_convert_int!(u32);
lua_convert_int!(i64);
lua_convert_int!(u64);
lua_convert_int!(i128);
lua_convert_int!(u128);
lua_convert_int!(isize);
lua_convert_int!(usize);

macro_rules! lua_convert_float {
	($x:ty) => {
		impl ToLua for $x {
			fn to_lua(self, _: &Lua) -> Result<Value> {
				cast(self)
					.ok_or_else(|| Error::ToLuaConversionError {
						from: stringify!($x),
						to: "number",
						message: Some("out of range".to_string()),
					})
					.map(Value::Number)
					.wrap_err(type_name::<Self>())
			}
		}

		impl FromLua for $x {
			fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
				let ty = value.type_name();
				lua.coerce_number(value)?
					.ok_or_else(|| Error::FromLuaConversionError {
						from: ty,
						to: stringify!($x),
						message: Some("expected number or string coercible to number".to_string()),
					})
					.and_then(|n| {
						cast(n).ok_or_else(|| Error::FromLuaConversionError {
							from: ty,
							to: stringify!($x),
							message: Some("number out of range".to_string()),
						})
					})
					.wrap_err(type_name::<Self>())
			}
		}
	};
}

lua_convert_float!(f32);
lua_convert_float!(f64);

impl<'lua, T> ToLua for &[T]
where
	T: Clone + ToLua,
{
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::Table(
			lua.create_sequence_from(self.iter().cloned())?,
		))
	}
}

impl<'lua, T, const N: usize> ToLua for [T; N]
where
	T: ToLua,
{
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::Table(lua.create_sequence_from(self)?))
	}
}

impl<'lua, T, const N: usize> FromLua for [T; N]
where
	T: FromLua,
{
	fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
		match value {
			Value::Table(table) => {
				let vec = table.iter_values().collect::<Result<Vec<_>>>()?;
				vec.try_into()
					.map_err(|vec: Vec<T>| Error::FromLuaConversionError {
						from: "Table",
						to: "Array",
						message: Some(format!("expected table of length {}, got {}", N, vec.len())),
					})
					.wrap_err(type_name::<Self>())
			}
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "Array",
				message: Some("expected table".to_string()),
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl<'lua, T: ToLua> ToLua for Box<[T]> {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::Table(lua.create_sequence_from(self.into_vec())?))
	}
}

impl<'lua, T: FromLua> FromLua for Box<[T]> {
	fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
		Ok(Vec::<T>::from_lua(value, lua)?.into_boxed_slice())
	}
}

impl<'lua, T: ToLua> ToLua for Vec<T> {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::Table(lua.create_sequence_from(self)?))
	}
}

impl<'lua, T: FromLua> FromLua for Vec<T> {
	fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
		match value {
			Value::Table(table) => table.iter_values().collect(),
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "Vec",
				message: Some("expected table".to_string()),
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl<'lua, K: FromLua, V: FromLua> FromLua for Vec<(K, V)> {
	fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
		match value {
			Value::Table(table) => table.iter().collect(),
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "Vec",
				message: Some("expected table".to_string()),
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl<'lua, K: ToLua, V: ToLua, S: BuildHasher> ToLua for HashMap<K, V, S> {
	fn to_lua(self, lua: &Lua) -> Result<Value> { Ok(Value::Table(lua.create_table_from(self)?)) }
}

impl<'lua, K: Eq + Hash + FromLua, V: FromLua, S: BuildHasher + Default> FromLua
	for HashMap<K, V, S>
{
	fn from_lua(value: Value, _: &Lua) -> Result<Self> {
		if let Value::Table(table) = value {
			table.iter().collect()
		} else {
			Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "HashMap",
				message: Some("expected table".to_string()),
			})
			.wrap_err(type_name::<Self>())
		}
	}
}

impl<'lua, K: Ord + ToLua, V: ToLua> ToLua for BTreeMap<K, V> {
	fn to_lua(self, lua: &Lua) -> Result<Value> { Ok(Value::Table(lua.create_table_from(self)?)) }
}

impl<'lua, K: Ord + FromLua, V: FromLua> FromLua for BTreeMap<K, V> {
	fn from_lua(value: Value, _: &Lua) -> Result<Self> {
		if let Value::Table(table) = value {
			table.iter().collect()
		} else {
			Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "BTreeMap",
				message: Some("expected table".to_string()),
			})
			.wrap_err(type_name::<Self>())
		}
	}
}

impl<'lua, T: Eq + Hash + ToLua, S: BuildHasher> ToLua for HashSet<T, S> {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::Table(lua.create_table_from(
			self.into_iter().map(|val| (val, true)),
		)?))
	}
}

impl<'lua, T: Eq + Hash + FromLua, S: BuildHasher + Default> FromLua for HashSet<T, S> {
	fn from_lua(value: Value, _: &Lua) -> Result<Self> {
		match value {
			Value::Table(table) if table.len()? > 0 => table.iter_values().collect(),
			Value::Table(table) => table
				.iter::<T, Value>()
				.map(|res| res.map(|(k, _)| k))
				.collect(),
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "HashSet",
				message: Some("expected table".to_string()),
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl<'lua, T: Ord + ToLua> ToLua for BTreeSet<T> {
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		Ok(Value::Table(lua.create_table_from(
			self.into_iter().map(|val| (val, true)),
		)?))
	}
}

impl<'lua, T: Ord + FromLua> FromLua for BTreeSet<T> {
	fn from_lua(value: Value, _: &Lua) -> Result<Self> {
		match value {
			Value::Table(table) if table.len()? > 0 => table.iter_values().collect(),
			Value::Table(table) => table
				.iter::<T, Value>()
				.map(|res| res.map(|(k, _)| k))
				.collect(),
			_ => Err(Error::FromLuaConversionError {
				from: value.type_name(),
				to: "BTreeSet",
				message: Some("expected table".to_string()),
			})
			.wrap_err(type_name::<Self>()),
		}
	}
}

impl<T: ToLua> ToLua for Option<T> {
	#[inline]
	fn to_lua(self, lua: &Lua) -> Result<Value> {
		match self {
			Some(val) => val.to_lua(lua),
			None => Ok(Value::Nil),
		}
	}
}

impl<T: FromLua> FromLua for Option<T> {
	#[inline]
	fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
		match value {
			Value::Nil => Ok(None),
			value => Ok(Some(T::from_lua(value, lua)?)),
		}
	}
}