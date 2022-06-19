use std::borrow::Cow;
use std::ffi::CString;
use std::io::Result as IoResult;
use std::path::{Path, PathBuf};
use std::string::String as StdString;

use crate::error::{Error, Result};
use crate::ffi;
use crate::function::Function;
use crate::lua::{Lua, LuaWeakRef};
use crate::value::{FromLuaMulti, ToLua, ToLuaMulti, Value};

#[cfg(feature = "async")]
use {futures_core::future::LocalBoxFuture, futures_util::future};

/// Trait for types [loadable by Lua] and convertible to a [`Chunk`]
///
/// [loadable by Lua]: https://www.lua.org/manual/5.4/manual.html#3.3.2
/// [`Chunk`]: crate::Chunk
pub trait AsChunk {
    /// Returns chunk data (can be text or binary)
    fn source(&self) -> IoResult<Cow<[u8]>>;

    /// Returns optional chunk name
    fn name(&self) -> Option<StdString> {
        None
    }

    /// Returns optional chunk [environment]
    ///
    /// [environment]: https://www.lua.org/manual/5.4/manual.html#2.2
    fn env(&self, _lua: &Lua) -> Result<Option<Value>> {
        Ok(None)
    }

    /// Returns optional chunk mode (text or binary)
    fn mode(&self) -> Option<ChunkMode> {
        None
    }
}

impl AsChunk for str {
    fn source(&self) -> IoResult<Cow<[u8]>> {
        Ok(Cow::Borrowed(self.as_ref()))
    }
}

impl AsChunk for StdString {
    fn source(&self) -> IoResult<Cow<[u8]>> {
        Ok(Cow::Borrowed(self.as_ref()))
    }
}

impl AsChunk for [u8] {
    fn source(&self) -> IoResult<Cow<[u8]>> {
        Ok(Cow::Borrowed(self))
    }
}

impl AsChunk for Vec<u8> {
    fn source(&self) -> IoResult<Cow<[u8]>> {
        Ok(Cow::Borrowed(self))
    }
}

impl AsChunk for Path {
    fn source(&self) -> IoResult<Cow<[u8]>> {
        std::fs::read(self).map(Cow::Owned)
    }

    fn name(&self) -> Option<StdString> {
        Some(format!("@{}", self.display()))
    }
}

impl AsChunk for PathBuf {
    fn source(&self) -> IoResult<Cow<[u8]>> {
        std::fs::read(self).map(Cow::Owned)
    }

    fn name(&self) -> Option<StdString> {
        Some(format!("@{}", self.display()))
    }
}

/// Returned from [`Lua::load`] and is used to finalize loading and executing Lua main chunks.
///
/// [`Lua::load`]: crate::Lua::load
#[must_use = "`Chunk`s do nothing unless one of `exec`, `eval`, `call`, or `into_function` are called on them"]
pub struct Chunk<'a> {
    pub(crate) lua: LuaWeakRef,
    pub(crate) source: IoResult<Cow<'a, [u8]>>,
    pub(crate) name: Option<StdString>,
    pub(crate) env: Result<Option<Value>>,
    pub(crate) mode: Option<ChunkMode>,
}

/// Represents chunk mode (text or binary).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkMode {
    Text,
    Binary,
}

impl<'a> Chunk<'a> {
    /// Sets the name of this chunk, which results in more informative error traces.
    pub fn set_name(mut self, name: impl AsRef<str>) -> Result<Self> {
        self.name = Some(name.as_ref().to_string());
        // Do extra validation
        let _ = self.convert_name()?;
        Ok(self)
    }

    /// Sets the first upvalue (`_ENV`) of the loaded chunk to the given value.
    ///
    /// Lua main chunks always have exactly one upvalue, and this upvalue is used as the `_ENV`
    /// variable inside the chunk. By default this value is set to the global environment.
    ///
    /// Calling this method changes the `_ENV` upvalue to the value provided, and variables inside
    /// the chunk will refer to the given environment rather than the global one.
    ///
    /// All global variables (including the standard library!) are looked up in `_ENV`, so it may be
    /// necessary to populate the environment in order for scripts using custom environments to be
    /// useful.
    pub fn set_environment<V: ToLua>(mut self, env: V) -> Result<Self> {
        // Prefer to propagate errors here and wrap to `Ok`
        let lua = self.lua.optional()?;
        self.env = Ok(Some(env.to_lua(&lua)?));
        Ok(self)
    }

    /// Sets whether the chunk is text or binary (autodetected by default).
    ///
    /// Be aware, Lua does not check the consistency of the code inside binary chunks.
    /// Running maliciously crafted bytecode can crash the interpreter.
    pub fn set_mode(mut self, mode: ChunkMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Execute this chunk of code.
    ///
    /// This is equivalent to calling the chunk function with no arguments and no return values.
    pub fn exec(self) -> Result<()> {
        self.call(())?;
        Ok(())
    }

    /// Asynchronously execute this chunk of code.
    ///
    /// See [`exec`] for more details.
    ///
    /// Requires `feature = "async"`
    ///
    /// [`exec`]: #method.exec
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn exec_async<'fut>(self) -> LocalBoxFuture<'fut, Result<()>>
    where
    {
        self.call_async(())
    }

    /// Evaluate the chunk as either an expression or block.
    ///
    /// If the chunk can be parsed as an expression, this loads and executes the chunk and returns
    /// the value that it evaluates to. Otherwise, the chunk is interpreted as a block as normal,
    /// and this is equivalent to calling `exec`.
    pub fn eval<R: FromLuaMulti>(self) -> Result<R> {
        // Bytecode is always interpreted as a statement.
        // For source code, first try interpreting the lua as an expression by adding
        // "return", then as a statement. This is the same thing the
        // actual lua repl does.
        if self.detect_mode() == ChunkMode::Binary {
            self.call(())
        } else if let Ok(function) = self.to_expression() {
            function.call(())
        } else {
            self.call(())
        }
    }

    /// Asynchronously evaluate the chunk as either an expression or block.
    ///
    /// See [`eval`] for more details.
    ///
    /// Requires `feature = "async"`
    ///
    /// [`eval`]: #method.eval
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn eval_async<'fut, R>(self) -> LocalBoxFuture<'fut, Result<R>>
    where
        R: FromLuaMulti + 'fut,
    {
        if self.detect_mode() == ChunkMode::Binary {
            self.call_async(())
        } else if let Ok(function) = self.to_expression() {
            function.call_async(())
        } else {
            self.call_async(())
        }
    }

    /// Load the chunk function and call it with the given arguments.
    ///
    /// This is equivalent to `into_function` and calling the resulting function.
    pub fn call<A: ToLuaMulti, R: FromLuaMulti>(self, args: A) -> Result<R> {
        self.into_function()?.call(args)
    }

    /// Load the chunk function and asynchronously call it with the given arguments.
    ///
    /// See [`call`] for more details.
    ///
    /// Requires `feature = "async"`
    ///
    /// [`call`]: #method.call
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn call_async<'fut, A, R>(self, args: A) -> LocalBoxFuture<'fut, Result<R>>
    where
        A: ToLuaMulti,
        R: FromLuaMulti + 'fut,
    {
        match self.into_function() {
            Ok(func) => func.call_async(args),
            Err(e) => Box::pin(future::err(e)),
        }
    }

    /// Load this chunk into a regular `Function`.
    ///
    /// This simply compiles the chunk without actually executing it.
    pub fn into_function(mut self) -> Result<Function> {
        let name = self.convert_name()?;
        self.lua.optional()?
            .load_chunk(self.source?.as_ref(), name.as_deref(), self.env?, self.mode)
    }

    fn to_expression(&self) -> Result<Function> {
        // We assume that mode is Text
        let source = self.source.as_ref();
        let source = source.map_err(|err| Error::RuntimeError(err.to_string()))?;
        let source = Self::expression_source(source);
        let name = self.convert_name()?;
        self.lua.optional()?
            .load_chunk(&source, name.as_deref(), self.env.clone()?, None)
    }

    fn detect_mode(&self) -> ChunkMode {
        match (self.mode, &self.source) {
            (Some(mode), _) => mode,
            (None, Ok(source)) if source.len() == 0 => ChunkMode::Text,
            (None, Ok(source)) => {
                if source.starts_with(ffi::LUA_SIGNATURE) {
                    return ChunkMode::Binary;
                }

                ChunkMode::Text
            }
            (None, Err(_)) => ChunkMode::Text, // any value is fine
        }
    }

    fn convert_name(&self) -> Result<Option<CString>> {
        self.name
            .clone()
            .map(CString::new)
            .transpose()
            .map_err(|err| Error::RuntimeError(format!("invalid name: {err}")))
    }

    fn expression_source(source: &[u8]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(b"return ".len() + source.len());
        buf.extend(b"return ");
        buf.extend(source);
        buf
    }
}
