use std::cmp;
use std::os::raw::c_int;

use crate::error::{Error, Result};
use crate::ffi;
use crate::types::LuaRef;
use crate::util::{check_stack, error_traceback, pop_error, StackGuard};
use crate::value::{FromLuaMulti, ToLuaMulti};

#[cfg(any(
    feature = "lua54",
    all(feature = "luajit", feature = "vendored")
))]
use crate::function::Function;

#[cfg(feature = "async")]
use {
    crate::{
        lua::{Lua, ASYNC_POLL_PENDING},
        value::{MultiValue, Value},
    },
    futures_core::{future::Future, stream::Stream},
    std::{
        cell::RefCell,
        marker::PhantomData,
        pin::Pin,
        task::{Context, Poll, Waker},
    },
};
use crate::lua::LuaWeakRef;

/// Status of a Lua thread (or coroutine).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ThreadStatus {
    /// The thread was just created, or is suspended because it has called `coroutine.yield`.
    ///
    /// If a thread is in this state, it can be resumed by calling [`Thread::resume`].
    ///
    /// [`Thread::resume`]: crate::Thread::resume
    Resumable,
    /// Either the thread has finished executing, or the thread is currently running.
    Unresumable,
    /// The thread has raised a Lua error during execution.
    Error,
}

/// Handle to an internal Lua thread (or coroutine).
#[derive(Clone, Debug)]
pub struct Thread(pub(crate) LuaRef);

/// Thread (coroutine) representation as an async [`Future`] or [`Stream`].
///
/// Requires `feature = "async"`
///
/// [`Future`]: futures_core::future::Future
/// [`Stream`]: futures_core::stream::Stream
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
#[derive(Debug)]
pub struct AsyncThread<R> {
    thread: Thread,
    args0: RefCell<Option<Result<MultiValue>>>,
    ret: PhantomData<R>,
    recycle: bool,
}

impl Thread {
    /// Resumes execution of this thread.
    ///
    /// Equivalent to `coroutine.resume`.
    ///
    /// Passes `args` as arguments to the thread. If the coroutine has called `coroutine.yield`, it
    /// will return these arguments. Otherwise, the coroutine wasn't yet started, so the arguments
    /// are passed to its main function.
    ///
    /// If the thread is no longer in `Active` state (meaning it has finished execution or
    /// encountered an error), this will return `Err(CoroutineInactive)`, otherwise will return `Ok`
    /// as follows:
    ///
    /// If the thread calls `coroutine.yield`, returns the values passed to `yield`. If the thread
    /// `return`s values from its main function, returns those.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mlua::{Error, Lua, Result, Thread};
    /// # fn main() -> Result<()> {
    /// # let lua = Lua::new();
    /// let thread: Thread = lua.load(r#"
    ///     coroutine.create(function(arg)
    ///         assert(arg == 42)
    ///         local yieldarg = coroutine.yield(123)
    ///         assert(yieldarg == 43)
    ///         return 987
    ///     end)
    /// "#).eval()?;
    ///
    /// assert_eq!(thread.resume::<_, u32>(42)?, 123);
    /// assert_eq!(thread.resume::<_, u32>(43)?, 987);
    ///
    /// // The coroutine has now returned, so `resume` will fail
    /// match thread.resume::<_, u32>(()) {
    ///     Err(Error::CoroutineInactive) => {},
    ///     unexpected => panic!("unexpected result {:?}", unexpected),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn resume<A, R>(&self, args: A) -> Result<R>
    where
        A: ToLuaMulti,
        R: FromLuaMulti,
    {
        let lua = &self.0.lua.optional()?;
        let mut args = args.to_lua_multi(lua)?;
        let nargs = args.len() as c_int;
        let results = unsafe {
            let _sg = StackGuard::new(lua.state);
            check_stack(lua.state, cmp::max(nargs + 1, 3))?;

            let thread_state =
                lua.ref_thread_exec(|ref_thread| ffi::lua_tothread(ref_thread, self.0.index));

            let status = ffi::lua_status(thread_state);
            if status != ffi::LUA_YIELD && ffi::lua_gettop(thread_state) == 0 {
                return Err(Error::CoroutineInactive);
            }

            check_stack(thread_state, nargs)?;
            for arg in args.drain_all() {
                lua.push_value(arg)?;
            }
            ffi::lua_xmove(lua.state, thread_state, nargs);

            let mut nresults = 0;

            let ret = ffi::lua_resume(thread_state, lua.state, nargs, &mut nresults as *mut c_int);
            if ret != ffi::LUA_OK && ret != ffi::LUA_YIELD {
                protect_lua!(lua.state, 0, 0, |_| error_traceback(thread_state))?;
                return Err(pop_error(thread_state, ret));
            }

            let mut results = args; // Reuse MultiValue container
            check_stack(lua.state, nresults + 2)?; // 2 is extra for `lua.pop_value()` below
            ffi::lua_xmove(thread_state, lua.state, nresults);

            for _ in 0..nresults {
                results.push_front(lua.pop_value());
            }
            results
        };
        Ok(R::from_lua_multi(results, lua)?)
    }

    /// Gets the status of the thread.
    pub fn status(&self) -> ThreadStatus {
        let lua = &self.0.lua.required();
        unsafe {
            let thread_state =
                lua.ref_thread_exec(|ref_thread| ffi::lua_tothread(ref_thread, self.0.index));

            let status = ffi::lua_status(thread_state);
            if status != ffi::LUA_OK && status != ffi::LUA_YIELD {
                ThreadStatus::Error
            } else if status == ffi::LUA_YIELD || ffi::lua_gettop(thread_state) > 0 {
                ThreadStatus::Resumable
            } else {
                ThreadStatus::Unresumable
            }
        }
    }

    /// Resets a thread
    ///
    /// In [Lua 5.4]: cleans its call stack and closes all pending to-be-closed variables.
    /// Returns a error in case of either the original error that stopped the thread or errors
    /// in closing methods.
    ///
    /// In [LuaJIT]: resets to the initial state of a newly created Lua thread.
    /// Lua threads in arbitrary states (like yielded or errored) can be reset properly.
    ///
    /// Sets a Lua function for the thread afterwards.
    ///
    /// Requires `feature = "lua54"` OR `feature = "luajit,vendored"`
    ///
    /// [Lua 5.4]: https://www.lua.org/manual/5.4/manual.html#lua_resetthread
    /// [LuaJIT]: https://github.com/openresty/luajit2#lua_resetthread
    #[cfg(any(
        feature = "lua54",
        all(feature = "luajit", feature = "vendored")
    ))]
    pub fn reset(&self, func: Function) -> Result<()> {
        let lua = &self.0.lua.optional()?;
        unsafe {
            let _sg = StackGuard::new(lua.state);
            check_stack(lua.state, 2)?;

            lua.push_ref(&self.0);
            let thread_state = ffi::lua_tothread(lua.state, -1);

            #[cfg(feature = "lua54")]
            let status = ffi::lua_resetthread(thread_state);
            #[cfg(feature = "lua54")]
            if status != ffi::LUA_OK {
                return Err(pop_error(thread_state, status));
            }
            #[cfg(all(feature = "luajit", feature = "vendored"))]
            ffi::lua_resetthread(lua.state, thread_state);

            lua.push_ref(&func.0);
            ffi::lua_xmove(lua.state, thread_state, 1);

            Ok(())
        }
    }

    /// Converts Thread to an AsyncThread which implements [`Future`] and [`Stream`] traits.
    ///
    /// `args` are passed as arguments to the thread function for first call.
    /// The object calls [`resume()`] while polling and also allows to run rust futures
    /// to completion using an executor.
    ///
    /// Using AsyncThread as a Stream allows to iterate through `coroutine.yield()`
    /// values whereas Future version discards that values and poll until the final
    /// one (returned from the thread function).
    ///
    /// Requires `feature = "async"`
    ///
    /// [`Future`]: futures_core::future::Future
    /// [`Stream`]: futures_core::stream::Stream
    /// [`resume()`]: https://www.lua.org/manual/5.4/manual.html#lua_resume
    ///
    /// # Examples
    ///
    /// ```
    /// # use mlua::{Lua, Result, Thread};
    /// use futures::stream::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let lua = Lua::new();
    /// let thread: Thread = lua.load(r#"
    ///     coroutine.create(function (sum)
    ///         for i = 1,10 do
    ///             sum = sum + i
    ///             coroutine.yield(sum)
    ///         end
    ///         return sum
    ///     end)
    /// "#).eval()?;
    ///
    /// let mut stream = thread.into_async::<_, i64>(1);
    /// let mut sum = 0;
    /// while let Some(n) = stream.try_next().await? {
    ///     sum += n;
    /// }
    ///
    /// assert_eq!(sum, 286);
    ///
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn into_async<A, R>(self, args: A) -> AsyncThread<R>
    where
        A: ToLuaMulti,
        R: FromLuaMulti,
    {
        let lua = &self.0.lua.optional()?;
        let args = args.to_lua_multi(lua);
        AsyncThread {
            thread: self,
            args0: RefCell::new(Some(args)),
            ret: PhantomData,
            recycle: false,
        }
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(feature = "async")]
impl<R> AsyncThread<R> {
    #[inline]
    pub(crate) fn set_recyclable(&mut self, recyclable: bool) {
        self.recycle = recyclable;
    }
}

#[cfg(feature = "async")]
#[cfg(any(
    feature = "lua54",
    all(feature = "luajit", feature = "vendored")
))]
impl<R> Drop for AsyncThread<R> {
    fn drop(&mut self) {
        if self.recycle {
            unsafe {
                self.thread.0.lua.recycle_thread(&mut self.thread);
            }
        }
    }
}

#[cfg(feature = "async")]
impl<R> Stream for AsyncThread<R>
where
    R: FromLuaMulti,
{
    type Item = Result<R>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let lua = &self.thread.0.lua.optional()?;

        match self.thread.status() {
            ThreadStatus::Resumable => {}
            _ => return Poll::Ready(None),
        };

        let _wg = WakerGuard::new(lua, cx.waker().clone());
        let ret: MultiValue = if let Some(args) = self.args0.borrow_mut().take() {
            self.thread.resume(args?)?
        } else {
            self.thread.resume(())?
        };

        if is_poll_pending(&ret) {
            return Poll::Pending;
        }

        cx.waker().wake_by_ref();
        Poll::Ready(Some(R::from_lua_multi(ret, lua)))
    }
}

#[cfg(feature = "async")]
impl<R> Future for AsyncThread<R>
where
    R: FromLuaMulti,
{
    type Output = Result<R>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let lua = &self.thread.0.lua.optional()?;

        match self.thread.status() {
            ThreadStatus::Resumable => {}
            _ => return Poll::Ready(Err(Error::CoroutineInactive)),
        };

        let _wg = WakerGuard::new(lua, cx.waker().clone());
        let ret: MultiValue = if let Some(args) = self.args0.borrow_mut().take() {
            self.thread.resume(args?)?
        } else {
            self.thread.resume(())?
        };

        if is_poll_pending(&ret) {
            return Poll::Pending;
        }

        if let ThreadStatus::Resumable = self.thread.status() {
            // Ignore value returned via yield()
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        Poll::Ready(R::from_lua_multi(ret, lua))
    }
}

#[cfg(feature = "async")]
#[inline(always)]
fn is_poll_pending(val: &MultiValue) -> bool {
    match val.iter().enumerate().last() {
        Some((0, Value::LightUserData(ud))) => {
            std::ptr::eq(ud.0 as *const u8, &ASYNC_POLL_PENDING as *const u8)
        }
        _ => false,
    }
}

#[cfg(feature = "async")]
struct WakerGuard {
    lua: LuaWeakRef,
    prev: Option<Waker>,
}

#[cfg(feature = "async")]
impl WakerGuard {
    #[inline]
    pub fn new(lua: &Lua, waker: Waker) -> Result<WakerGuard> {
        unsafe {
            let prev = lua.set_waker(Some(waker));
            Ok(WakerGuard { lua: LuaWeakRef::new(lua), prev })
        }
    }
}

#[cfg(feature = "async")]
impl Drop for WakerGuard {
    fn drop(&mut self) {
        unsafe {
            self.lua.set_waker(self.prev.take());
        }
    }
}
