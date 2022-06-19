use std::cell::UnsafeCell;
use std::hash::{Hash, Hasher};
use std::os::raw::{c_int, c_void};
use std::sync::{Arc, Mutex};
use std::{fmt, mem, ptr};

#[cfg(feature = "lua54")]
use std::ffi::CStr;

#[cfg(feature = "async")]
use futures_core::future::LocalBoxFuture;

use crate::ffi;

use crate::hook::Debug;
use crate::lua::{ExtraData, Lua, LuaWeakRef};
use crate::util::{assert_stack, StackGuard};
use crate::value::MultiValue;

/// Type of Lua integer numbers.
pub type Integer = ffi::lua_Integer;
/// Type of Lua floating point numbers.
pub type Number = ffi::lua_Number;

/// A "light" userdata value. Equivalent to an unmanaged raw pointer.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LightUserData(pub *mut c_void);

pub(crate) type Callback<'a> =
    Box<dyn Fn(&'a Lua, MultiValue) -> eyre::Result<MultiValue> + 'a>;

pub(crate) struct Upvalue<T> {
    pub(crate) data: T,
    pub(crate) extra: Arc<UnsafeCell<ExtraData>>,
}

pub(crate) type CallbackUpvalue = Upvalue<Callback<'static>>;

#[cfg(feature = "async")]
pub(crate) type AsyncCallback<'a> =
    Box<dyn Fn(&Lua, MultiValue) -> LocalBoxFuture<eyre::Result<MultiValue>> + 'a>;

#[cfg(feature = "async")]
pub(crate) type AsyncCallbackUpvalue = Upvalue<AsyncCallback<'static>>;

#[cfg(feature = "async")]
pub(crate) type AsyncPollUpvalue = Upvalue<LocalBoxFuture<'static, eyre::Result<MultiValue>>>;

#[cfg(all(feature = "send"))]
pub(crate) type HookCallback = Arc<dyn Fn(&Lua, Debug) -> eyre::Result<()> + Send>;

#[cfg(all(not(feature = "send")))]
pub(crate) type HookCallback = Arc<dyn Fn(&Lua, Debug) -> Result<()>>;

#[cfg(all(feature = "send", feature = "lua54"))]
pub(crate) type WarnCallback = Box<dyn Fn(&Lua, &CStr, bool) -> eyre::Result<()> + Send>;

#[cfg(all(not(feature = "send"), feature = "lua54"))]
pub(crate) type WarnCallback = Box<dyn Fn(&Lua, &CStr, bool) -> Result<()>>;

#[cfg(feature = "send")]
pub trait MaybeSend: Send {}
#[cfg(feature = "send")]
impl<T: Send> MaybeSend for T {}

#[cfg(not(feature = "send"))]
pub trait MaybeSend {}
#[cfg(not(feature = "send"))]
impl<T> MaybeSend for T {}

pub(crate) struct DestructedUserdataMT;

/// An auto generated key into the Lua registry.
///
/// This is a handle to a value stored inside the Lua registry. It is not automatically
/// garbage collected on Drop, but it can be removed with [`Lua::remove_registry_value`],
/// and instances not manually removed can be garbage collected with [`Lua::expire_registry_values`].
///
/// Be warned, If you place this into Lua via a [`UserData`] type or a rust callback, it is *very
/// easy* to accidentally cause reference cycles that the Lua garbage collector cannot resolve.
/// Instead of placing a [`RegistryKey`] into a [`UserData`] type, prefer instead to use
/// [`AnyUserData::set_user_value`] / [`AnyUserData::get_user_value`].
///
/// [`UserData`]: crate::UserData
/// [`RegistryKey`]: crate::RegistryKey
/// [`Lua::remove_registry_value`]: crate::Lua::remove_registry_value
/// [`Lua::expire_registry_values`]: crate::Lua::expire_registry_values
/// [`AnyUserData::set_user_value`]: crate::AnyUserData::set_user_value
/// [`AnyUserData::get_user_value`]: crate::AnyUserData::get_user_value
pub struct RegistryKey {
    pub(crate) registry_id: c_int,
    pub(crate) unref_list: Arc<Mutex<Option<Vec<c_int>>>>,
}

impl fmt::Debug for RegistryKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegistryKey({})", self.registry_id)
    }
}

impl Hash for RegistryKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.registry_id.hash(state)
    }
}

impl PartialEq for RegistryKey {
    fn eq(&self, other: &RegistryKey) -> bool {
        self.registry_id == other.registry_id && Arc::ptr_eq(&self.unref_list, &other.unref_list)
    }
}

impl Eq for RegistryKey {}

impl Drop for RegistryKey {
    fn drop(&mut self) {
        let mut unref_list = mlua_expect!(self.unref_list.lock(), "unref list poisoned");
        if let Some(list) = unref_list.as_mut() {
            list.push(self.registry_id);
        }
    }
}

impl RegistryKey {
    // Destroys the RegistryKey without adding to the drop list
    pub(crate) fn take(self) -> c_int {
        let registry_id = self.registry_id;
        unsafe {
            ptr::read(&self.unref_list);
            mem::forget(self);
        }
        registry_id
    }
}

pub(crate) struct LuaRef {
    pub(crate) lua: LuaWeakRef,
    pub(crate) index: c_int,
}

impl fmt::Debug for LuaRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ref({})", self.index)
    }
}

impl Clone for LuaRef {
    fn clone(&self) -> Self {
        self.lua.required().clone_ref(self)
    }
}

impl Drop for LuaRef {
    fn drop(&mut self) {
        if let Ok(lua) = self.lua.optional() {
            if self.index > 0 {
                lua.drop_ref(self);
            }
        }
    }
}

impl PartialEq for LuaRef {
    fn eq(&self, other: &Self) -> bool {
        let lua = self.lua.required();
        unsafe {
            let _sg = StackGuard::new(lua.state);
            assert_stack(lua.state, 2);
            lua.push_ref(self);
            lua.push_ref(other);
            ffi::lua_rawequal(lua.state, -1, -2) == 1
        }
    }
}
