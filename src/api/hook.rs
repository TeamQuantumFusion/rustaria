use mlua::prelude::*;
use std::marker::PhantomData;

pub struct Hook<'lua, Args> {
    funcs: Vec<LuaFunction<'lua>>,
    _p: PhantomData<Args>,
}

impl<'lua, Args> Hook<'lua, Args> {
    pub fn new() -> Self {
        Self {
            funcs: vec![],
            _p: PhantomData,
        }
    }
}

impl<'lua, Args> Hook<'lua, Args>
where
    Args: ToLuaMulti<'lua> + Clone,
{
    pub fn call(&self, args: Args) -> LuaResult<()> {
        for func in &self.funcs {
            func.call::<Args, ()>(args.clone())?;
        }
        Ok(())
    }
}
