use mlua::prelude::LuaFunction;
use mlua::ToLuaMulti;
use std::marker::PhantomData;

pub enum Hook<'lua, P>
where
    P: ToLuaMulti<'lua> + Copy + Clone,
{
    Inactive(PhantomData<P>),
    Active(Vec<LuaFunction<'lua>>),
}

impl<'lua, P> Hook<'lua, P>
where
    P: ToLuaMulti<'lua> + Copy + Clone,
{
    pub fn unused() -> Hook<'lua, P> {
        Hook::Inactive(PhantomData::default())
    }

    pub fn call(&self, parameters: P) {
        if let Hook::Active(functions) = &self {
            for func in functions {
                func.call::<P, ()>(parameters);
            }
        }
    }
}
