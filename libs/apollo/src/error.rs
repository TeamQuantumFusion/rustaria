#![allow(clippy::wrong_self_convention)]

use std::error::Error as StdError;
use std::fmt;
use std::fmt::Debug;
use std::io::Error as IoError;
use std::net::AddrParseError;
use std::result::Result as StdResult;
use std::str::Utf8Error;
use std::string::String as StdString;
use std::sync::Arc;
use anyways::audit::{Audit, AuditError, AuditSection, AuditSectionEntry};
use owo_colors::{AnsiColors, DynColors};
use crate::RefError;

/// Error type returned by `mlua` methods.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    /// Syntax error while parsing Lua source code.
    SyntaxError {
        /// The error message as returned by Lua.
        message: StdString,
        /// `true` if the error can likely be fixed by appending more input to the source code.
        ///
        /// This is useful for implementing REPLs as they can query the user for more input if this
        /// is set.
        incomplete_input: bool,
    },
    /// Lua runtime error, aka `LUA_ERRRUN`.
    ///
    /// The Lua VM returns this error when a builtin operation is performed on incompatible types.
    /// Among other things, this includes invoking operators on wrong types (such as calling or
    /// indexing a `nil` value).
    RuntimeError(StdString),
    /// Lua memory error, aka `LUA_ERRMEM`
    ///
    /// The Lua VM returns this error when the allocator does not return the requested memory, aka
    /// it is an out-of-memory error.
    MemoryError(StdString),
    /// Lua garbage collector error, aka `LUA_ERRGCMM`.
    ///
    /// The Lua VM returns this error when there is an error running a `__gc` metamethod.
    #[cfg(any(feature = "lua53", feature = "lua52", doc))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "lua53", feature = "lua52"))))]
    GarbageCollectorError(StdString),
    /// Potentially unsafe action in safe mode.
    SafetyError(StdString),
    /// Setting memory limit is not available.
    ///
    /// This error can only happen when Lua state was not created by us and does not have the
    /// custom allocator attached.
    MemoryLimitNotAvailable,
    /// Main thread is not available.
    ///
    /// This error can only happen in Lua5.1/LuaJIT module mode, when module loaded within a coroutine.
    /// These Lua versions does not have `LUA_RIDX_MAINTHREAD` registry key.
    MainThreadNotAvailable,
    /// A mutable callback has triggered Lua code that has called the same mutable callback again.
    ///
    /// This is an error because a mutable callback can only be borrowed mutably once.
    RecursiveMutCallback,
    /// Either a callback or a userdata method has been called, but the callback or userdata has
    /// been destructed.
    ///
    /// This can happen either due to to being destructed in a previous __gc, or due to being
    /// destructed from exiting a `Lua::scope` call.
    CallbackDestructed,
    /// Not enough stack space to place arguments to Lua functions or return values from callbacks.
    ///
    /// Due to the way `mlua` works, it should not be directly possible to run out of stack space
    /// during normal use. The only way that this error can be triggered is if a `Function` is
    /// called with a huge number of arguments, or a rust callback returns a huge number of return
    /// values.
    StackError,
    /// Too many arguments to `Function::bind`
    BindError,
    /// A Rust value could not be converted to a Lua value.
    ToLuaConversionError {
        /// Name of the Rust type that could not be converted.
        from: &'static str,
        /// Name of the Lua type that could not be created.
        to: &'static str,
        /// A message indicating why the conversion failed in more detail.
        message: Option<StdString>,
    },
    /// A Lua value could not be converted to the expected Rust type.
    FromLuaConversionError {
        /// Name of the Lua type that could not be converted.
        from: &'static str,
        /// Name of the Rust type that could not be created.
        to: &'static str,
        /// A string containing more detailed error information.
        message: Option<StdString>,
    },
    /// [`Thread::resume`] was called on an inactive coroutine.
    ///
    /// A coroutine is inactive if its main function has returned or if an error has occurred inside
    /// the coroutine.
    ///
    /// [`Thread::status`] can be used to check if the coroutine can be resumed without causing this
    /// error.
    ///
    /// [`Thread::resume`]: crate::Thread::resume
    /// [`Thread::status`]: crate::Thread::status
    CoroutineInactive,
    /// An [`AnyUserData`] is not the expected type in a borrow.
    ///
    /// This error can only happen when manually using [`AnyUserData`], or when implementing
    /// metamethods for binary operators. Refer to the documentation of [`UserDataMethods`] for
    /// details.
    ///
    /// [`AnyUserData`]: crate::AnyUserData
    /// [`UserDataMethods`]: crate::UserDataMethods
    UserDataTypeMismatch(&'static str),
    /// When a type is not what it was expected to be
    WrongType {
        expected: &'static str,
        received: &'static str,
    },
    /// An [`AnyUserData`] borrow failed because it has been destructed.
    ///
    /// This error can happen either due to to being destructed in a previous __gc, or due to being
    /// destructed from exiting a `Lua::scope` call.
    ///
    /// [`AnyUserData`]: crate::AnyUserData
    UserDataDestructed,
    /// An [`AnyUserData`] immutable borrow failed because it is already borrowed mutably.
    ///
    /// This error can occur when a method on a [`UserData`] type calls back into Lua, which then
    /// tries to call a method on the same [`UserData`] type. Consider restructuring your API to
    /// prevent these errors.
    ///
    /// [`AnyUserData`]: crate::AnyUserData
    /// [`UserData`]: crate::UserData
    UserDataBorrowError,
    /// An [`AnyUserData`] mutable borrow failed because it is already borrowed.
    ///
    /// This error can occur when a method on a [`UserData`] type calls back into Lua, which then
    /// tries to call a method on the same [`UserData`] type. Consider restructuring your API to
    /// prevent these errors.
    ///
    /// [`AnyUserData`]: crate::AnyUserData
    /// [`UserData`]: crate::UserData
    UserDataBorrowMutError,
    /// A [`MetaMethod`] operation is restricted (typically for `__gc` or `__metatable`).
    ///
    /// [`MetaMethod`]: crate::MetaMethod
    MetaMethodRestricted(StdString),
    /// A [`MetaMethod`] (eg. `__index` or `__newindex`) has invalid type.
    ///
    /// [`MetaMethod`]: crate::MetaMethod
    MetaMethodTypeError {
        method: StdString,
        type_name: &'static str,
        message: Option<StdString>,
    },
    /// A [`RegistryKey`] produced from a different Lua state was used.
    ///
    /// [`RegistryKey`]: crate::RegistryKey
    MismatchedRegistryKey,
    /// A Rust callback returned `Err`, raising the contained `Error` as a Lua error.
    CallbackError {
        /// Lua call stack backtrace.
        traceback: StdString,
        /// Original error returned by the Rust code.
        cause: Arc<Audit>,
    },
    /// A Rust panic that was previously resumed, returned again.
    ///
    /// This error can occur only when a Rust panic resumed previously was recovered
    /// and returned again.
    PreviouslyResumedPanic,
    /// Serialization error.
    #[cfg(feature = "serialize")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serialize")))]
    SerializeError(StdString),
    /// Deserialization error.
    #[cfg(feature = "serialize")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serialize")))]
    DeserializeError(StdString),
    /// A custom error.
    ///
    /// This can be used for returning user-defined errors from callbacks.
    ///
    /// Returning `Err(ExternalError(...))` from a Rust callback will raise the error as a Lua
    /// error. The Rust code that originally invoked the Lua code then receives a `CallbackError`,
    /// from which the original error (and a stack traceback) can be recovered.
    ExternalError(Arc<dyn StdError + Send + Sync>),
    LuaUnavailable,
    RefError(RefError),
    Audit(Arc<Audit>)
}

/// A specialized `Result` type used by `mlua`'s API.
pub type Result<T> = StdResult<T, Error>;

#[cfg(not(tarpaulin_include))]
impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SyntaxError { ref message, .. } => write!(fmt, "syntax error: {}", message),
            Error::RuntimeError(ref msg) => write!(fmt, "runtime error: {}", msg),
            Error::MemoryError(ref msg) => {
                write!(fmt, "memory error: {}", msg)
            }
            #[cfg(any(feature = "lua53", feature = "lua52"))]
            Error::GarbageCollectorError(ref msg) => {
                write!(fmt, "garbage collector error: {}", msg)
            }
            Error::SafetyError(ref msg) => {
                write!(fmt, "safety error: {}", msg)
            },
            Error::MemoryLimitNotAvailable => {
                write!(fmt, "setting memory limit is not available")
            }
            Error::MainThreadNotAvailable => {
                write!(fmt, "main thread is not available in Lua 5.1")
            }
            Error::RecursiveMutCallback => write!(fmt, "mutable callback called recursively"),
            Error::CallbackDestructed => write!(
                fmt,
                "a destructed callback or destructed userdata method was called"
            ),
            Error::StackError => write!(
                fmt,
                "out of Lua stack, too many arguments to a Lua function or too many return values from a callback"
            ),
            Error::BindError => write!(
                fmt,
                "too many arguments to Function::bind"
            ),
            Error::ToLuaConversionError { from, to, ref message } => {
                write!(fmt, "error converting {} to Lua {}", from, to)?;
                match *message {
                    None => Ok(()),
                    Some(ref message) => write!(fmt, " ({})", message),
                }
            }
            Error::FromLuaConversionError { from, to, ref message } => {
                write!(fmt, "error converting Lua {} to {}", from, to)?;
                match *message {
                    None => Ok(()),
                    Some(ref message) => write!(fmt, " ({})", message),
                }
            }
            Error::CoroutineInactive => write!(fmt, "cannot resume inactive coroutine"),
            Error::UserDataTypeMismatch(expected) => write!(fmt, "userdata is not {expected} type"),
            Error::UserDataDestructed => write!(fmt, "userdata has been destructed"),
            Error::UserDataBorrowError => write!(fmt, "userdata already mutably borrowed"),
            Error::UserDataBorrowMutError => write!(fmt, "userdata already borrowed"),
            Error::MetaMethodRestricted(ref method) => write!(fmt, "metamethod {} is restricted", method),
            Error::MetaMethodTypeError { ref method, type_name, ref message } => {
                write!(fmt, "metamethod {} has unsupported type {}", method, type_name)?;
                match *message {
                    None => Ok(()),
                    Some(ref message) => write!(fmt, " ({})", message),
                }
            }
            Error::MismatchedRegistryKey => {
                write!(fmt, "RegistryKey used from different Lua state")
            }
            Error::CallbackError { ref cause, ref traceback } => {
                writeln!(fmt, "callback error")?;
                // Trace errors down to the root
                let (mut cause, mut full_traceback) = (cause, None);
                while let Some(Error::CallbackError { cause: cause2, traceback: traceback2 }) = cause.downcast_ref::<Error>() {
                    cause = cause2;
                    full_traceback = Some(traceback2);
                }

                use std::fmt::Write;
                let mut stacktrace = String::new();
                if let Some(full_traceback) = full_traceback {
                    writeln!(&mut stacktrace, "{full_traceback}")?;
                    let traceback = traceback.trim_start_matches("stack traceback:");
                    let traceback = traceback.trim_start().trim_end();
                    // Try to find local traceback within the full traceback
                    if let Some(pos) = full_traceback.find(traceback) {
                        write!(&mut stacktrace, "{}", &full_traceback[..pos])?;
                        writeln!(&mut stacktrace, ">{}", &full_traceback[pos..].trim_end())?;
                    } else {
                        writeln!(&mut stacktrace, "{}", full_traceback.trim_end())?;
                    }
                } else {
                    writeln!(&mut stacktrace, "{}", traceback.trim_end())?;
                }

                for err in &cause.errors {
                    writeln!(&mut stacktrace, "  [rust]: {}", err.to_string().trim_end())?
                }
                write!(fmt, "{}", stacktrace)
            }
            Error::PreviouslyResumedPanic => {
                write!(fmt, "previously resumed panic returned again")
            }
            #[cfg(feature = "serialize")]
            Error::SerializeError(ref err) => {
                write!(fmt, "serialize error: {}", err)
            },
            #[cfg(feature = "serialize")]
            Error::DeserializeError(ref err) => {
                write!(fmt, "deserialize error: {}", err)
            },
            Error::ExternalError(ref err) => write!(fmt, "{}", err),
            Error::WrongType { expected, received } => {
                write!(fmt, "Wrong type, Expected {expected} but got {received}")
            }
            Error::LuaUnavailable => write!(fmt, "lua unavailable"),
            Error::Audit(ref report) => {
                writeln!(fmt, "{:?}", report)?;
                Ok(())
            }
            Error::RefError(ref err) => {
                write!(fmt, "{}", err)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            // An error type with a source error should either return that error via source or
            // include that source's error message in its own Display output, but never both.
            // https://blog.rust-lang.org/inside-rust/2021/07/01/What-the-error-handling-project-group-is-working-towards.html
            // Given that we include source to fmt::Display implementation for `CallbackError`, this call returns nothing.
            Error::CallbackError { .. } => None,
            Error::ExternalError(ref err) => err.source(),
            _ => None,
        }
    }
}

impl Error {
    pub fn external<T: Into<Box<dyn StdError + Send + Sync>>>(err: T) -> Error {
        Error::ExternalError(err.into().into())
    }

    pub fn report(self) -> Audit {
        match self {
            Error::CallbackError {
                traceback, cause
            } => {
                let (mut cause, mut full_traceback) = (cause, None);

                let mut cause = &cause;
                while let Some(Error::CallbackError { cause: cause2, traceback: traceback2 }) = cause.downcast_ref::<Error>() {
                    cause = cause2;
                    full_traceback = Some(traceback2);
                }

                let mut audit = Audit {
                    backtrace: cause.backtrace.clone(),
                    errors: cause.errors.iter().map(|v| {
                        AuditError {
                            error: format!("{}", v.error).into(),
                            location: v.location.clone()
                        }
                    }).collect(),
                    custom_sections: cause.custom_sections.clone(),
                };

                let mut section = AuditSection {
                    name: "Lua Stacktrace".to_string(),
                    color: DynColors::Ansi(AnsiColors::Blue),
                    entries: vec![]
                };

                for msg in traceback.trim_end().split("\n") {
                    let string = msg.replace("\t", "    ");
                    if string != "stack traceback:" {
                        section.entries.push(AuditSectionEntry::text(string));
                    }
                }
                audit.custom_sections.push(section);

                audit
            }
            Error::Audit(audit) => {
                Audit {
                    backtrace: audit.backtrace.clone(),
                    errors: audit.errors.iter().map(|v| {
                        AuditError {
                            error: format!("{}", v.error).into(),
                            location: v.location.clone()
                        }
                    }).collect(),
                    custom_sections: audit.custom_sections.clone(),
                }
            }
            error => {
                Audit::new(error)
            }
        }
    }
}

pub trait ExternalError {
    fn to_lua_err(self) -> Error;
}

impl<E: Into<Box<dyn StdError + Send + Sync>>> ExternalError for E {
    fn to_lua_err(self) -> Error {
        Error::external(self)
    }
}

pub trait ExternalResult<T> {
    fn to_lua_err(self) -> Result<T>;
}

impl<T, E> ExternalResult<T> for StdResult<T, E>
where
    E: ExternalError,
{
    fn to_lua_err(self) -> Result<T> {
        self.map_err(|e| e.to_lua_err())
    }
}

impl From<AddrParseError> for Error {
    fn from(err: AddrParseError) -> Self {
        Error::external(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::external(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::external(err)
    }
}

impl From<Audit> for Error {
    fn from(err: Audit) -> Self {
        Error::Audit(Arc::new(err))
    }
}

impl From<RefError> for Error {
    fn from(err: RefError) -> Self {
        Error::RefError(err)
    }
}


#[cfg(feature = "serialize")]
impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::SerializeError(msg.to_string())
    }
}

#[cfg(feature = "serialize")]
impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::DeserializeError(msg.to_string())
    }
}
