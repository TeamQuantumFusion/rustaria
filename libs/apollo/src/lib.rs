#![feature(box_into_inner)]
//! # High-level bindings to Lua
//!
//! The `mlua` crate provides safe high-level bindings to the [Lua programming language].
//!
//! # The `Lua` object
//!
//! The main type exported by this library is the [`Lua`] struct. In addition to methods for
//! [executing] Lua chunks or [evaluating] Lua expressions, it provides methods for creating Lua
//! values and accessing the table of [globals].
//!
//! # Converting data
//!
//! The [`ToLua`] and [`FromLua`] traits allow conversion from Rust types to Lua values and vice
//! versa. They are implemented for many data structures found in Rust's standard library.
//!
//! For more general conversions, the [`ToLuaMulti`] and [`FromLuaMulti`] traits allow converting
//! between Rust types and *any number* of Lua values.
//!
//! Most code in `mlua` is generic over implementors of those traits, so in most places the normal
//! Rust data structures are accepted without having to write any boilerplate.
//!
//! # Custom Userdata
//!
//! The [`UserData`] trait can be implemented by user-defined types to make them available to Lua.
//! Methods and operators to be used from Lua can be added using the [`UserDataMethods`] API.
//! Fields are supported using the [`UserDataFields`] API.
//!
//! # Serde support
//!
//! The [`LuaSerdeExt`] trait implemented for [`Lua`] allows conversion from Rust types to Lua values
//! and vice versa using serde. Any user defined data type that implements [`serde::Serialize`] or
//! [`serde::Deserialize`] can be converted.
//! For convenience, additional functionality to handle `NULL` values and arrays is provided.
//!
//! The [`Value`] enum implements [`serde::Serialize`] trait to support serializing Lua values
//! (including [`UserData`]) into Rust values.
//!
//! Requires `feature = "serialize"`.
//!
//! # Async/await support
//!
//! The [`create_async_function`] allows creating non-blocking functions that returns [`Future`].
//! Lua code with async capabilities can be executed by [`call_async`] family of functions or polling
//! [`AsyncThread`] using any runtime (eg. Tokio).
//!
//! Requires `feature = "async"`.
//!
//! # `Send` requirement
//! By default `mlua` is `!Send`. This can be changed by enabling `feature = "send"` that adds `Send` requirement
//! to [`Function`]s and [`UserData`].
//!
//! [Lua programming language]: https://www.lua.org/
//! [`Lua`]: crate::Lua
//! [executing]: crate::Chunk::exec
//! [evaluating]: crate::Chunk::eval
//! [globals]: crate::Lua::globals
//! [`ToLua`]: crate::ToLua
//! [`FromLua`]: crate::FromLua
//! [`ToLuaMulti`]: crate::ToLuaMulti
//! [`FromLuaMulti`]: crate::FromLuaMulti
//! [`Function`]: crate::Function
//! [`UserData`]: crate::UserData
//! [`UserDataFields`]: crate::UserDataFields
//! [`UserDataMethods`]: crate::UserDataMethods
//! [`LuaSerdeExt`]: crate::LuaSerdeExt
//! [`Value`]: crate::Value
//! [`create_async_function`]: crate::Lua::create_async_function
//! [`call_async`]: crate::Function::call_async
//! [`AsyncThread`]: crate::AsyncThread
//! [`Future`]: std::future::Future
//! [`serde::Serialize`]: https://docs.serde.rs/serde/ser/trait.Serialize.html
//! [`serde::Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html

// mlua types in rustdoc of other crates get linked to here.
#![doc(html_root_url = "https://docs.rs/mlua/0.8.0-beta.3")]
// Deny warnings inside doc tests / examples. When this isn't present, rustdoc doesn't show *any*
// warnings at all.
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "serialize")]
#[doc(inline)]
pub use crate::serde::{
	de::Options as DeserializeOptions, ser::Options as SerializeOptions, LuaSerdeExt,
};
#[cfg(feature = "async")]
pub use crate::thread::AsyncThread;
pub use crate::{
	chunk::{AsChunk, Chunk, ChunkMode},
	error::{Error, ExternalError, ExternalResult, Result},
	ffi::{lua_CFunction, lua_State},
	function::Function,
	hook::{Debug, DebugEvent, DebugNames, DebugSource, DebugStack, HookTriggers},
	lua::{GCMode, Lua, LuaOptions},
	multi::Variadic,
	scope::{LuaScope, RefError},
	stdlib::StdLib,
	string::String,
	table::{Table, TableExt, TablePairs, TableSequence},
	thread::{Thread, ThreadStatus},
	types::{Integer, LightUserData, Number, RegistryKey},
	userdata::{
		AnyUserData, MetaMethod, Ref, RefMut, UserData, UserDataCell, UserDataFields,
		UserDataMetatable, UserDataMethods,
	},
	value::{FromLua, FromLuaMulti, MultiValue, Nil, ToLua, ToLuaMulti, Value},
};

#[macro_use]
mod util_macros;

mod chunk;
mod conversion;
mod error;
mod ffi;
mod function;
mod hook;
mod lua;
mod multi;
mod scope;
mod stdlib;
mod string;
mod table;
mod thread;
mod types;
mod userdata;
mod userdata_impl;
mod util;
mod value;

pub mod prelude;

#[cfg(feature = "macro")]
pub mod macros {
	pub use apollo_macro::{from_lua, lua_field, lua_impl, lua_method, to_lua};
}

#[cfg(feature = "serialize")]
#[cfg_attr(docsrs, doc(cfg(feature = "serialize")))]
pub mod serde;
