use std::collections::HashMap;
use std::path::PathBuf;

use mlua::{Error, Function};
use mlua::prelude::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::api::plugin::Plugins;
use crate::chunk::tile::TilePrototype;
use crate::chunk::wall::WallPrototype;
use crate::registry::{Id, Registry, Tag};

mod log;
#[macro_use]
pub(crate) mod macros;
pub mod plugin;

pub struct RustariaApi<'lua> {
    lua: &'lua Lua,
    plugins: Plugins<'lua>,

    pub tiles: Registry<TilePrototype>,
    pub walls: Registry<WallPrototype>,
}

macro_rules! proto {
    ($($NAME:ident => $PROTO:ty | $REQUEST:ident),*) => {
        $(
            fn $NAME(lua: &Lua, send: UnboundedSender<PrototypeRequest>) -> LuaResult<Function> {
                lua.create_function(move |lua, _: ()| {
                    let send = send.clone();
                    lua.create_table_from([
                        ("register", lua.create_function(move |lua, prototypes: HashMap<String, _>| {
                            let send = send.clone();
                            for (key, prototype) in prototypes {
                                let tag = Tag::new(lua.globals().get("mod_id")?, key);
                                send.send(PrototypeRequest::$REQUEST(tag, prototype)).map_err(|err| Error::RuntimeError(err.to_string()))?;
                            }
                            Ok(())
                        })?),
                        ("default", lua.create_function(|lua, t| {
                            Ok(lua.from_value::<$PROTO>(LuaValue::Table(t)))
                        })?)
                    ])
                })
            }
        )*
    };
}


/// Registers Rustaria's Lua modding APIs.
pub fn register_rustaria_api(lua: &Lua) -> LuaResult<UnboundedReceiver<PrototypeRequest>> {
    let (send, rec) = unbounded_channel();
    let package: LuaTable = lua.globals().get("package")?;
    let preload: LuaTable = package.get("preload")?;

    preload.set("log", lua.create_function(log::package)?)?;
    preload.set("tile", tile(lua, send.clone())?)?;
    preload.set("wall", wall(lua, send.clone())?)?;
    Ok(rec)
}

proto! {
    tile => TilePrototype | Tile,
    wall => WallPrototype | Wall
}

pub async fn launch_rustaria_api<'lua>(plugins_dir: PathBuf, runtime: &'lua LuaRuntime) -> eyre::Result<RustariaApi<'lua>> {
    let lua = &runtime.lua;

    let mut receiver = register_rustaria_api(lua)?;
    let plugins = plugin::scan_and_load_plugins(&plugins_dir, lua).await?;
    plugins.init(lua)?;

    let mut tile = Registry::new();
    let mut wall = Registry::new();
    while let Ok(prototype) = receiver.try_recv() {
        match prototype {
            PrototypeRequest::Tile(id, pt) => tile.register(id, pt),
            PrototypeRequest::Wall(id, pt) => wall.register(id, pt),
        };
    }


    Ok(RustariaApi {
        lua,
        plugins,
        tiles: tile,
        walls: wall,
    })
}

pub struct LuaRuntime {
    lua: Lua,
}


impl LuaRuntime {
    pub fn new() -> Self {
        Self {
            lua: Lua::new()
        }
    }
}

pub enum PrototypeRequest {
    Tile(Tag, TilePrototype),
    Wall(Tag, WallPrototype),
}

pub trait Prototype<T> {
    fn create(&self, id: Id) -> T;
}
