use std::any::{type_name, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, RwLock};

use crate::error::{Error, Result};
use crate::ffi;
use crate::lua::Lua;
use crate::types::{Callback, MaybeSend};
use crate::userdata::{
    AnyUserData, MetaMethod, UserData, UserDataCell, UserDataFields, UserDataMethods,
};
use crate::util::{check_stack, get_userdata, StackGuard};
use crate::value::{FromLua, FromLuaMulti, ToLua, ToLuaMulti, Value};

#[cfg(not(feature = "send"))]
use std::rc::Rc;
use eyre::Report;

#[cfg(feature = "async")]
use {
    crate::types::AsyncCallback,
    futures_core::future::Future,
    futures_util::future::{self, TryFutureExt},
};

pub(crate) struct StaticUserDataMethods<T: 'static + UserData> {
    pub(crate) methods: Vec<(Vec<u8>, Callback<'static>)>,
    #[cfg(feature = "async")]
    pub(crate) async_methods: Vec<(Vec<u8>, AsyncCallback<'static>)>,
    pub(crate) meta_methods: Vec<(MetaMethod, Callback<'static>)>,
    #[cfg(feature = "async")]
    pub(crate) async_meta_methods: Vec<(MetaMethod, AsyncCallback<'static>)>,
    _type: PhantomData<T>,
}

impl<T: 'static + UserData> Default for StaticUserDataMethods<T> {
    fn default() -> StaticUserDataMethods<T> {
        StaticUserDataMethods {
            methods: Vec::new(),
            #[cfg(feature = "async")]
            async_methods: Vec::new(),
            meta_methods: Vec::new(),
            #[cfg(feature = "async")]
            async_meta_methods: Vec::new(),
            _type: PhantomData,
        }
    }
}

impl<T: 'static + UserData> UserDataMethods<T> for StaticUserDataMethods<T> {
    fn add_method<S, A, R, M>(&mut self, name: &S, method: M)
    where
        S: AsRef<[u8]> + ?Sized,
        A: FromLuaMulti,
        R: ToLuaMulti,
        M: 'static + MaybeSend + Fn(&Lua, &T, A) -> eyre::Result<R>,
    {
        self.methods
            .push((name.as_ref().to_vec(), Self::box_method(method)));
    }

    fn add_method_mut<S, A, R, M>(&mut self, name: &S, method: M)
    where
        S: AsRef<[u8]> + ?Sized,
        A: FromLuaMulti,
        R: ToLuaMulti,
        M: 'static + MaybeSend + FnMut(&Lua, &mut T, A) -> eyre::Result<R>,
    {
        self.methods
            .push((name.as_ref().to_vec(), Self::box_method_mut(method)));
    }

    #[cfg(feature = "async")]
    fn add_async_method<S, A, R, M, MR>(&mut self, name: &S, method: M)
    where
        T: Clone,
        S: AsRef<[u8]> + ?Sized,
        A: FromLuaMulti,
        R: ToLuaMulti,
        M: 'static + MaybeSend + Fn(&Lua, T, A) -> MR,
        MR: Future<Output = eyre::Result<R>>,
    {
        self.async_methods
            .push((name.as_ref().to_vec(), Self::box_async_method(method)));
    }

    fn add_function<S, A, R, F>(&mut self, name: &S, function: F)
    where
        S: AsRef<[u8]> + ?Sized,
        A: FromLuaMulti,
        R: ToLuaMulti,
        F: 'static + MaybeSend + Fn(&Lua, A) -> eyre::Result<R>,
    {
        self.methods
            .push((name.as_ref().to_vec(), Self::box_function(function)));
    }

    fn add_function_mut<S, A, R, F>(&mut self, name: &S, function: F)
    where
        S: AsRef<[u8]> + ?Sized,
        A: FromLuaMulti,
        R: ToLuaMulti,
        F: 'static + MaybeSend + FnMut(&Lua, A) -> eyre::Result<R>,
    {
        self.methods
            .push((name.as_ref().to_vec(), Self::box_function_mut(function)));
    }

    #[cfg(feature = "async")]
    fn add_async_function<S, A, R, F, FR>(&mut self, name: &S, function: F)
    where
        S: AsRef<[u8]> + ?Sized,
        A: FromLuaMulti,
        R: ToLuaMulti,
        F: 'static + MaybeSend + Fn(&Lua, A) -> FR,
        FR: Future<Output = eyre::Result<R>>,
    {
        self.async_methods
            .push((name.as_ref().to_vec(), Self::box_async_function(function)));
    }

    fn add_meta_method<S, A, R, M>(&mut self, meta: S, method: M)
    where
        S: Into<MetaMethod>,
        A: FromLuaMulti,
        R: ToLuaMulti,
        M: 'static + MaybeSend + Fn(&Lua, &T, A) -> eyre::Result<R>,
    {
        self.meta_methods
            .push((meta.into(), Self::box_method(method)));
    }

    fn add_meta_method_mut<S, A, R, M>(&mut self, meta: S, method: M)
    where
        S: Into<MetaMethod>,
        A: FromLuaMulti,
        R: ToLuaMulti,
        M: 'static + MaybeSend + FnMut(&Lua, &mut T, A) -> eyre::Result<R>,
    {
        self.meta_methods
            .push((meta.into(), Self::box_method_mut(method)));
    }

    #[cfg(all(feature = "async", not(any(feature = "lua51"))))]
    fn add_async_meta_method<S, A, R, M, MR>(&mut self, meta: S, method: M)
    where
        T: Clone,
        S: Into<MetaMethod>,
        A: FromLuaMulti,
        R: ToLuaMulti,
        M: 'static + MaybeSend + Fn(&Lua, T, A) -> MR,
        MR: 'lua + Future<Output = eyre::Result<R>>,
    {
        self.async_meta_methods
            .push((meta.into(), Self::box_async_method(method)));
    }

    fn add_meta_function<S, A, R, F>(&mut self, meta: S, function: F)
    where
        S: Into<MetaMethod>,
        A: FromLuaMulti,
        R: ToLuaMulti,
        F: 'static + MaybeSend + Fn(&Lua, A) -> eyre::Result<R>,
    {
        self.meta_methods
            .push((meta.into(), Self::box_function(function)));
    }

    fn add_meta_function_mut<S, A, R, F>(&mut self, meta: S, function: F)
    where
        S: Into<MetaMethod>,
        A: FromLuaMulti,
        R: ToLuaMulti,
        F: 'static + MaybeSend + FnMut(&Lua, A) -> eyre::Result<R>,
    {
        self.meta_methods
            .push((meta.into(), Self::box_function_mut(function)));
    }

    #[cfg(all(feature = "async", not(any(feature = "lua51"))))]
    fn add_async_meta_function<S, A, R, F, FR>(&mut self, meta: S, function: F)
    where
        S: Into<MetaMethod>,
        A: FromLuaMulti,
        R: ToLuaMulti,
        F: 'static + MaybeSend + Fn(&Lua, A) -> FR,
        FR: Future<Output = Result<R>>,
    {
        self.async_meta_methods
            .push((meta.into(), Self::box_async_function(function)));
    }

    // Below are internal methods used in generated code

    fn add_callback(&mut self, name: Vec<u8>, callback: Callback<'static>) {
        self.methods.push((name, callback));
    }

    #[cfg(feature = "async")]
    fn add_async_callback(&mut self, name: Vec<u8>, callback: AsyncCallback<'static>) {
        self.async_methods.push((name, callback));
    }

    fn add_meta_callback(&mut self, meta: MetaMethod, callback: Callback<'static>) {
        self.meta_methods.push((meta, callback));
    }

    #[cfg(feature = "async")]
    fn add_async_meta_callback(
        &mut self,
        meta: MetaMethod,
        callback: AsyncCallback<'static>,
    ) {
        self.async_meta_methods.push((meta, callback))
    }
}

impl<T: 'static + UserData> StaticUserDataMethods<T> {
    fn box_method<A, R, M>(method: M) -> Callback<'static>
    where
        A: FromLuaMulti,
        R: ToLuaMulti,
        M: 'static + MaybeSend + Fn(&Lua, &T, A) -> eyre::Result<R>,
    {
        Box::new(move |lua, mut args| {
            if let Some(front) = args.pop_front() {
                let userdata = AnyUserData::from_lua(front, lua)?;
                unsafe {
                    let _sg = StackGuard::new(lua.state);
                    check_stack(lua.state, 2)?;

                    let type_id = lua.push_userdata_ref(&userdata.0)?;
                    match type_id {
                        Some(id) if id == TypeId::of::<T>() => {
                            let ud = get_userdata_ref::<T>(lua.state)?;
                            Ok(method(lua, &ud, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua)?)
                        }
                        #[cfg(not(feature = "send"))]
                        Some(id) if id == TypeId::of::<Rc<RefCell<T>>>() => {
                            let ud = get_userdata_ref::<Rc<RefCell<T>>>(lua.state)?;
                            let ud = ud.try_borrow().map_err(|_| Error::UserDataBorrowError)?;
                            Ok(method(lua, &ud, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua))?
                        }
                        Some(id) if id == TypeId::of::<Arc<Mutex<T>>>() => {
                            let ud = get_userdata_ref::<Arc<Mutex<T>>>(lua.state)?;
                            let ud = ud.try_lock().map_err(|_| Error::UserDataBorrowError)?;
                            Ok(method(lua, &ud, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua)?)
                        }
                        Some(id) if id == TypeId::of::<Arc<RwLock<T>>>() => {
                            let ud = get_userdata_ref::<Arc<RwLock<T>>>(lua.state)?;
                            let ud = ud.try_read().map_err(|_| Error::UserDataBorrowError)?;
                            Ok(method(lua, &ud, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua)?)
                        }
                        _ => Err(Report::new(Error::UserDataTypeMismatch(type_name::<T>()))),
                    }
                }
            } else {
                Err(Report::new(Error::FromLuaConversionError {
                    from: "missing argument",
                    to: "userdata",
                    message: None,
                }))
            }
        })
    }

    fn box_method_mut<A, R, M>(method: M) -> Callback<'static>
    where
        A: FromLuaMulti,
        R: ToLuaMulti,
        M: 'static + MaybeSend + FnMut(&Lua, &mut T, A) -> eyre::Result<R>,
    {
        let method = RefCell::new(method);
        Box::new(move |lua, mut args| {
            if let Some(front) = args.pop_front() {
                let userdata = AnyUserData::from_lua(front, lua)?;
                let mut method = method
                    .try_borrow_mut()
                    .map_err(|_| Error::RecursiveMutCallback)?;
                unsafe {
                    let _sg = StackGuard::new(lua.state);
                    check_stack(lua.state, 2)?;

                    let type_id = lua.push_userdata_ref(&userdata.0)?;
                    match type_id {
                        Some(id) if id == TypeId::of::<T>() => {
                            let mut ud = get_userdata_mut::<T>(lua.state)?;
                            method(lua, &mut ud, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua)
                        }
                        #[cfg(not(feature = "send"))]
                        Some(id) if id == TypeId::of::<Rc<RefCell<T>>>() => {
                            let ud = get_userdata_mut::<Rc<RefCell<T>>>(lua.state)?;
                            let mut ud = ud
                                .try_borrow_mut()
                                .map_err(|_| Error::UserDataBorrowMutError)?;
                            method(lua, &mut ud, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua)
                        }
                        Some(id) if id == TypeId::of::<Arc<Mutex<T>>>() => {
                            let ud = get_userdata_mut::<Arc<Mutex<T>>>(lua.state)?;
                            let mut ud =
                                ud.try_lock().map_err(|_| Error::UserDataBorrowMutError)?;
                            method(lua, &mut ud, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua)
                        }
                        Some(id) if id == TypeId::of::<Arc<RwLock<T>>>() => {
                            let ud = get_userdata_mut::<Arc<RwLock<T>>>(lua.state)?;
                            let mut ud =
                                ud.try_write().map_err(|_| Error::UserDataBorrowMutError)?;
                            method(lua, &mut ud, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua)
                        }
                        _ => Err(Report::new(Error::UserDataTypeMismatch(type_name::<T>()))),
                    }
                }
            } else {
                Err(Report::new(Error::FromLuaConversionError {
                    from: "missing argument",
                    to: "userdata",
                    message: None,
                }))
            }
        })
    }

    #[cfg(feature = "async")]
    fn box_async_method<A, R, M, MR>(method: M) -> AsyncCallback<'static>
    where
        T: Clone,
        A: FromLuaMulti,
        R: ToLuaMulti,
        M: 'static + MaybeSend + Fn(&Lua, T, A) -> MR,
        MR: 'lua + Future<Output = eyre::Result<R>>,
    {
        Box::new(move |lua, mut args| {
            let fut_res = || {
                if let Some(front) = args.pop_front() {
                    let userdata = AnyUserData::from_lua(front, lua)?;
                    unsafe {
                        let _sg = StackGuard::new(lua.state);
                        check_stack(lua.state, 2)?;

                        let type_id = lua.push_userdata_ref(&userdata.0)?;
                        match type_id {
                            Some(id) if id == TypeId::of::<T>() => {
                                let ud = get_userdata_ref::<T>(lua.state)?;
                                Ok(method(lua, ud.clone(), A::from_lua_multi(args, lua)?))
                            }
                            #[cfg(not(feature = "send"))]
                            Some(id) if id == TypeId::of::<Rc<RefCell<T>>>() => {
                                let ud = get_userdata_ref::<Rc<RefCell<T>>>(lua.state)?;
                                let ud = ud.try_borrow().map_err(|_| Error::UserDataBorrowError)?;
                                Ok(method(lua, ud.clone(), A::from_lua_multi(args, lua)?))
                            }
                            Some(id) if id == TypeId::of::<Arc<Mutex<T>>>() => {
                                let ud = get_userdata_ref::<Arc<Mutex<T>>>(lua.state)?;
                                let ud = ud.try_lock().map_err(|_| Error::UserDataBorrowError)?;
                                Ok(method(lua, ud.clone(), A::from_lua_multi(args, lua)?))
                            }
                            Some(id) if id == TypeId::of::<Arc<RwLock<T>>>() => {
                                let ud = get_userdata_ref::<Arc<RwLock<T>>>(lua.state)?;
                                let ud = ud.try_read().map_err(|_| Error::UserDataBorrowError)?;
                                Ok(method(lua, ud.clone(), A::from_lua_multi(args, lua)?))
                            }
                            _ => Err(Error::UserDataTypeMismatch(type_name::<T>())),
                        }
                    }
                } else {
                    Err(Error::FromLuaConversionError {
                        from: "missing argument",
                        to: "userdata",
                        message: None,
                    })
                }
            };
            match fut_res() {
                Ok(fut) => Box::pin(fut.and_then(move |ret| future::ready(ret.to_lua_multi(lua)))),
                Err(e) => Box::pin(future::err(e)),
            }
        })
    }

    fn box_function<A, R, F>(function: F) -> Callback<'static>
    where
        A: FromLuaMulti,
        R: ToLuaMulti,
        F: 'static + MaybeSend + Fn(&Lua, A) -> eyre::Result<R>,
    {
        Box::new(move |lua, args| function(lua, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua))
    }

    fn box_function_mut<A, R, F>(function: F) -> Callback<'static>
    where
        A: FromLuaMulti,
        R: ToLuaMulti,
        F: 'static + MaybeSend + FnMut(&Lua, A) -> eyre::Result<R>,
    {
        let function = RefCell::new(function);
        Box::new(move |lua, args| {
            let function = &mut *function
                .try_borrow_mut()
                .map_err(|_| Error::RecursiveMutCallback)?;
            function(lua, A::from_lua_multi(args, lua)?)?.to_lua_multi(lua)
        })
    }

    #[cfg(feature = "async")]
    fn box_async_function<A, R, F, FR>(function: F) -> AsyncCallback<'static>
    where
        A: FromLuaMulti,
        R: ToLuaMulti,
        F: 'static + MaybeSend + Fn(&Lua, A) -> FR,
        FR: 'lua + Future<Output = eyre::Result<R>>,
    {
        Box::new(move |lua, args| {
            let args = match A::from_lua_multi(args, lua) {
                Ok(args) => args,
                Err(e) => return Box::pin(future::err(e)),
            };
            Box::pin(function(lua, args).and_then(move |ret| future::ready(ret.to_lua_multi(lua))))
        })
    }
}

pub(crate) struct StaticUserDataFields<T: 'static + UserData> {
    pub(crate) field_getters: Vec<(Vec<u8>, Callback<'static>)>,
    pub(crate) field_setters: Vec<(Vec<u8>, Callback<'static>)>,
    #[allow(clippy::type_complexity)]
    pub(crate) meta_fields: Vec<(
        MetaMethod,
        Box<dyn Fn(&Lua) -> Result<Value> + 'static>,
    )>,
    _type: PhantomData<T>,
}

impl<T: 'static + UserData> Default for StaticUserDataFields<T> {
    fn default() -> StaticUserDataFields<T> {
        StaticUserDataFields {
            field_getters: Vec::new(),
            field_setters: Vec::new(),
            meta_fields: Vec::new(),
            _type: PhantomData,
        }
    }
}

impl<T: 'static + UserData> UserDataFields<T> for StaticUserDataFields<T> {
    fn add_field_method_get<S, R, M>(&mut self, name: &S, method: M)
    where
        S: AsRef<[u8]> + ?Sized,
        R: ToLua,
        M: 'static + MaybeSend + Fn(&Lua, &T) -> eyre::Result<R>,
    {
        self.field_getters.push((
            name.as_ref().to_vec(),
            StaticUserDataMethods::box_method(move |lua, data, ()| method(lua, data)),
        ));
    }

    fn add_field_method_set<S, A, M>(&mut self, name: &S, method: M)
    where
        S: AsRef<[u8]> + ?Sized,
        A: FromLua,
        M: 'static + MaybeSend + FnMut(&Lua, &mut T, A) -> eyre::Result<()>,
    {
        self.field_setters.push((
            name.as_ref().to_vec(),
            StaticUserDataMethods::box_method_mut(method),
        ));
    }

    fn add_field_function_get<S, R, F>(&mut self, name: &S, function: F)
    where
        S: AsRef<[u8]> + ?Sized,
        R: ToLua,
        F: 'static + MaybeSend + Fn(&Lua, AnyUserData) -> eyre::Result<R>,
    {
        self.field_getters.push((
            name.as_ref().to_vec(),
            StaticUserDataMethods::<T>::box_function(function),
        ));
    }

    fn add_field_function_set<S, A, F>(&mut self, name: &S, mut function: F)
    where
        S: AsRef<[u8]> + ?Sized,
        A: FromLua,
        F: 'static + MaybeSend + FnMut(&Lua, AnyUserData, A) -> eyre::Result<()>,
    {
        self.field_setters.push((
            name.as_ref().to_vec(),
            StaticUserDataMethods::<T>::box_function_mut(move |lua, (data, val)| {
                function(lua, data, val)
            }),
        ));
    }

    fn add_meta_field_with<S, R, F>(&mut self, meta: S, f: F)
    where
        S: Into<MetaMethod>,
        R: ToLua,
        F: 'static + MaybeSend + Fn(&Lua) -> eyre::Result<R>,
    {
        let meta = meta.into();
        self.meta_fields.push((
            meta.clone(),
            Box::new(move |lua| {
                let value = f(lua)?.to_lua(lua)?;
                if meta == MetaMethod::Index || meta == MetaMethod::NewIndex {
                    match value {
                        Value::Nil | Value::Table(_) | Value::Function(_) => {}
                        _ => {
                            return Err(Error::MetaMethodTypeError {
                                method: meta.to_string(),
                                type_name: value.type_name(),
                                message: Some("expected nil, table or function".to_string()),
                            })
                        }
                    }
                }
                Ok(value)
            }),
        ));
    }

    // Below are internal methods

    fn add_field_getter(&mut self, name: Vec<u8>, callback: Callback<'static>) {
        self.field_getters.push((name, callback));
    }

    fn add_field_setter(&mut self, name: Vec<u8>, callback: Callback<'static>) {
        self.field_setters.push((name, callback));
    }
}

#[inline]
unsafe fn get_userdata_ref<'a, T>(state: *mut ffi::lua_State) -> Result<Ref<'a, T>> {
    (*get_userdata::<UserDataCell<T>>(state, -1)).try_borrow()
}

#[inline]
unsafe fn get_userdata_mut<'a, T>(state: *mut ffi::lua_State) -> Result<RefMut<'a, T>> {
    (*get_userdata::<UserDataCell<T>>(state, -1)).try_borrow_mut()
}

macro_rules! lua_userdata_impl {
    ($type:ty) => {
        impl<T: 'static + UserData> UserData for $type {
            fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
                let mut orig_fields = StaticUserDataFields::default();
                T::add_fields(&mut orig_fields);
                for (name, callback) in orig_fields.field_getters {
                    fields.add_field_getter(name, callback);
                }
                for (name, callback) in orig_fields.field_setters {
                    fields.add_field_setter(name, callback);
                }
            }

            fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
                let mut orig_methods = StaticUserDataMethods::default();
                T::add_methods(&mut orig_methods);
                for (name, callback) in orig_methods.methods {
                    methods.add_callback(name, callback);
                }
                #[cfg(feature = "async")]
                for (name, callback) in orig_methods.async_methods {
                    methods.add_async_callback(name, callback);
                }
                for (meta, callback) in orig_methods.meta_methods {
                    methods.add_meta_callback(meta, callback);
                }
                #[cfg(feature = "async")]
                for (meta, callback) in orig_methods.async_meta_methods {
                    methods.add_async_meta_callback(meta, callback);
                }
            }
        }
    };
}

#[cfg(not(feature = "send"))]
lua_userdata_impl!(Rc<RefCell<T>>);
lua_userdata_impl!(Arc<Mutex<T>>);
lua_userdata_impl!(Arc<RwLock<T>>);
