use std::iter::{self, FromIterator};
use std::{slice, str, vec};

#[cfg(feature = "serialize")]
use {
    serde::ser::{self, Serialize, Serializer},
    std::convert::TryInto,
    std::result::Result as StdResult,
};

use crate::error::{Error, Result};
use crate::function::Function;
use crate::lua::Lua;
use crate::string::String;
use crate::table::Table;
use crate::thread::Thread;
use crate::types::{Integer, LightUserData, Number};
use crate::userdata::AnyUserData;

// #[derive(Debug)]
// pub struct WrongType;
//
macro_rules! try_into {
    ($ITEM:ident => $TY:ty) => {
        impl TryInto<$TY> for Value {
            type Error = crate::error::Error;
            fn try_into(self) -> StdResult<$TY, crate::error::Error> {
                if let Value::$ITEM(value) = self {
                    return Ok(value)
                } else {
                    Err(Error::WrongType  {
                        expected: stringify!($ITEM),
                        received: self.type_name()
                    })
                }
            }
        }
    };
}

/// A dynamically typed Lua value. The `String`, `Table`, `Function`, `Thread`, and `UserData`
/// variants contain handle types into the internal Lua state. It is a logic error to mix handle
/// types between separate `Lua` instances, and doing so will result in a panic.
#[derive(Debug, Clone)]
pub enum Value {
    /// The Lua value `nil`.
    Nil,
    /// The Lua value `true` or `false`.
    Boolean(bool),
    /// A "light userdata" object, equivalent to a raw pointer.
    LightUserData(LightUserData),
    /// An integer number.
    ///
    /// Any Lua number convertible to a `Integer` will be represented as this variant.
    Integer(Integer),
    /// A floating point number.
    Number(Number),
    /// An interned string, managed by Lua.
    ///
    /// Unlike Rust strings, Lua strings may not be valid UTF-8.
    String(String),
    /// Reference to a Lua table.
    Table(Table),
    /// Reference to a Lua function (or closure).
    Function(Function),
    /// Reference to a Lua thread (or coroutine).
    Thread(Thread),
    /// Reference to a userdata object that holds a custom type which implements `UserData`.
    /// Special builtin userdata types will be represented as other `Value` variants.
    UserData(AnyUserData),
    /// `Error` is a special builtin userdata type. When received from Lua it is implicitly cloned.
    Error(Error),
}

try_into!(Boolean => bool);
try_into!(LightUserData => LightUserData);
try_into!(Integer => Integer);
try_into!(Number => Number);
try_into!(String => String);
try_into!(Table => Table);
try_into!(Function => Function);
try_into!(Thread => Thread);
try_into!(UserData => AnyUserData);
try_into!(Error => Error);

pub use self::Value::Nil;
impl Value {
    pub const fn type_name(&self) -> &'static str {
        match *self {
            Value::Nil => "nil",
            Value::Boolean(_) => "boolean",
            Value::LightUserData(_) => "lightuserdata",
            Value::Integer(_) => "integer",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Table(_) => "table",
            Value::Function(_) => "function",
            Value::Thread(_) => "thread",
            Value::UserData(_) => "userdata",
            Value::Error(_) => "error",
        }
    }

    /// Compares two values for equality.
    ///
    /// Equality comparisons do not convert strings to numbers or vice versa.
    /// Tables, Functions, Threads, and Userdata are compared by reference:
    /// two objects are considered equal only if they are the same object.
    ///
    /// If Tables or Userdata have `__eq` metamethod then mlua will try to invoke it.
    /// The first value is checked first. If that value does not define a metamethod
    /// for `__eq`, then mlua will check the second value.
    /// Then mlua calls the metamethod with the two values as arguments, if found.
    pub fn equals<T: AsRef<Self>>(&self, other: T) -> anyways::Result<bool> {
        match (self, other.as_ref()) {
            (Value::Table(a), Value::Table(b)) => a.equals(b),
            (Value::UserData(a), Value::UserData(b)) => a.equals(b),
            _ => Ok(self == other.as_ref()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::LightUserData(a), Value::LightUserData(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => *a == *b,
            (Value::Integer(a), Value::Number(b)) => *a as Number == *b,
            (Value::Number(a), Value::Integer(b)) => *a == *b as Number,
            (Value::Number(a), Value::Number(b)) => *a == *b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Table(a), Value::Table(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => a == b,
            (Value::Thread(a), Value::Thread(b)) => a == b,
            (Value::UserData(a), Value::UserData(b)) => a == b,
            _ => false,
        }
    }
}

impl AsRef<Value> for Value {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

#[cfg(feature = "serialize")]
impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Nil => serializer.serialize_unit(),
            Value::Boolean(b) => serializer.serialize_bool(*b),
            #[allow(clippy::useless_conversion)]
            Value::Integer(i) => serializer
                .serialize_i64((*i).try_into().expect("cannot convert lua_Integer to i64")),
            #[allow(clippy::useless_conversion)]
            Value::Number(n) => serializer.serialize_f64(*n),
            Value::String(s) => s.serialize(serializer),
            Value::Table(t) => t.serialize(serializer),
            Value::UserData(ud) => ud.serialize(serializer),
            Value::LightUserData(ud) if ud.0.is_null() => serializer.serialize_none(),
            Value::Error(_) | Value::LightUserData(_) | Value::Function(_) | Value::Thread(_) => {
                let msg = format!("cannot serialize <{}>", self.type_name());
                Err(ser::Error::custom(msg))
            }
        }
    }
}

/// Trait for types convertible to `Value`.
pub trait ToLua {
    /// Performs the conversion.
    fn to_lua(self, lua: &Lua) -> anyways::Result<Value>;
}

#[cfg(feature = "macro")]
pub use apollo_macro::FromLua;

/// Trait for types convertible from `Value`.
pub trait FromLua: Sized {
    /// Performs the conversion.
    fn from_lua(lua_value: Value, lua: &Lua) -> anyways::Result<Self>;
}

/// Multiple Lua values used for both argument passing and also for multiple return values.
#[derive(Debug, Clone)]
pub struct MultiValue(Vec<Value>);

impl MultiValue {
    /// Creates an empty `MultiValue` containing no values.
    #[inline]
    pub fn new() -> MultiValue {
        MultiValue(Vec::new())
    }

    /// Similar to `new` but can return previously used container with allocated capacity.
    #[inline]
    pub(crate) fn new_or_cached(lua: &Lua) -> MultiValue {
        lua.new_or_cached_multivalue()
    }
}

impl Default for MultiValue {
    #[inline]
    fn default() -> MultiValue {
        MultiValue::new()
    }
}

impl FromIterator<Value> for MultiValue {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
        MultiValue::from_vec(Vec::from_iter(iter))
    }
}

impl IntoIterator for MultiValue {
    type Item = Value;
    type IntoIter = iter::Rev<vec::IntoIter<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().rev()
    }
}

impl<'a, 'lua> IntoIterator for &'a MultiValue {
    type Item = &'a Value;
    type IntoIter = iter::Rev<slice::Iter<'a, Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).iter().rev()
    }
}

impl MultiValue {
    #[inline]
    pub fn from_vec(mut v: Vec<Value>) -> MultiValue {
        v.reverse();
        MultiValue(v)
    }

    #[inline]
    pub fn into_vec(self) -> Vec<Value> {
        let mut v = self.0;
        v.reverse();
        v
    }

    #[inline]
    pub(crate) fn reserve(&mut self, size: usize) {
        self.0.reserve(size);
    }

    #[inline]
    pub(crate) fn push_front(&mut self, value: Value) {
        self.0.push(value);
    }

    #[inline]
    pub(crate) fn pop_front(&mut self) -> Option<Value> {
        self.0.pop()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> iter::Rev<slice::Iter<Value>> {
        self.0.iter().rev()
    }

    #[inline]
    pub(crate) fn drain_all(&mut self) -> iter::Rev<vec::Drain<Value>> {
        self.0.drain(..).rev()
    }

    #[inline]
    pub(crate) fn refill(
        &mut self,
        iter: impl IntoIterator<Item = anyways::Result<Value>>,
    ) -> anyways::Result<()> {
        self.0.clear();
        for value in iter {
            self.0.push(value?);
        }
        self.0.reverse();
        Ok(())
    }
}

/// Trait for types convertible to any number of Lua values.
///
/// This is a generalization of `ToLua`, allowing any number of resulting Lua values instead of just
/// one. Any type that implements `ToLua` will automatically implement this trait.
pub trait ToLuaMulti {
    /// Performs the conversion.
    fn to_lua_multi(self, lua: &Lua) -> anyways::Result<MultiValue>;
}

/// Trait for types that can be created from an arbitrary number of Lua values.
///
/// This is a generalization of `FromLua`, allowing an arbitrary number of Lua values to participate
/// in the conversion. Any type that implements `FromLua` will automatically implement this trait.
pub trait FromLuaMulti: Sized {
    /// Performs the conversion.
    ///
    /// In case `values` contains more values than needed to perform the conversion, the excess
    /// values should be ignored. This reflects the semantics of Lua when calling a function or
    /// assigning values. Similarly, if not enough values are given, conversions should assume that
    /// any missing values are nil.
    fn from_lua_multi(values: MultiValue, lua: &Lua) -> anyways::Result<Self>;
}
