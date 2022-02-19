use std::{collections::HashMap, sync::Arc};

use mlua::prelude::*;
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tracing::debug;

use crate::{registry::Tag, chunk::{wall::WallPrototype, tile::TilePrototype}, entity::EntityPrototype};

pub type TsMap<T> = Arc<RwLock<HashMap<Tag, T>>>;

struct PrototypeRegistry<T> {
    data: TsMap<T>,
}
impl<T: 'static + DeserializeOwned + LuaUserData + Clone + Debug> PrototypeRegistry<T> {
    fn new<'lua>(data: TsMap<T>, lua: &'lua Lua) -> LuaResult<LuaFunction<'lua>> {
        lua.create_function(move |lua, _: ()| {
            let data = data.clone();
            let registry = PrototypeRegistry::<T> { data };

            lua.create_table_from([
                (
                    "register",
                    lua.create_function(move |lua, prototypes| {
                        Self::register(lua, prototypes, registry.data.clone())
                    })?,
                ),
                ("default", lua.create_function(Self::default)?),
            ])
        })
    }
    fn register(_lua: &Lua, prototypes: HashMap<Tag, T>, data: TsMap<T>) -> LuaResult<()> {
        debug!(?prototypes);
        data.write().extend(prototypes);
        Ok(())
    }
    fn default(lua: &Lua, settings: LuaValue) -> LuaResult<T> {
        debug!(?settings);
        let a = lua.from_value::<T>(settings)?;
        debug!(?a);
        Ok(a)
    }
}

macro_rules! registries {
    ($($name:ident => $ty:ty);+ $(;)?) => {
        pub struct Registries {
            $(
                pub $name: TsMap<$ty>
            ),+
        }
        impl Registries {
            pub fn new(lua: &Lua) -> LuaResult<Self> {
                let package: LuaTable = lua.globals().get("package")?;
                let preload: LuaTable = package.get("preload")?;

                Ok(Self {$(
                    $name: {
                        let map = TsMap::default();
                        preload.set(stringify!($name), PrototypeRegistry::<$ty>::new(map.clone(), lua)?)?;
                        map
                    },
                )+})
            }
            pub fn clear(&mut self) {
                $(
                    self.$name.write().clear();
                )+
            }
        }
    };
}
registries! {
    wall => WallPrototype;
    tile => TilePrototype;
    entity => EntityPrototype;
}
