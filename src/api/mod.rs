use std::path::PathBuf;

use mlua::prelude::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use crate::chunk::tile::TilePrototype;
use crate::chunk::wall::WallPrototype;
use crate::plugin::{PluginLoader, Plugins};
use crate::registry::{Id, Registry, Tag};

mod log;
#[macro_use]
pub(crate) mod macros;
mod tile;

pub struct RustariaApi<'lua> {
    lua: &'lua Lua,
    plugins: Plugins<'lua>,

    pub tiles: Registry<TilePrototype>,
    pub walls: Registry<WallPrototype>,
}

/// Registers Rustaria's Lua modding APIs.
pub fn register_rustaria_api(lua: &Lua) -> LuaResult<UnboundedReceiver<PrototypeRequest>> {
    let (send, rec) = unbounded_channel();
    let package: LuaTable = lua.globals().get("package")?;
    let preload: LuaTable = package.get("preload")?;

    preload.set("log", lua.create_function(log::package)?)?;
    preload.set(
        "tile",
        lua.create_function(move |lua, _: ()| tile::package(lua, send.clone()))?,
    )?;
    Ok(rec)
}

pub async fn launch_rustaria_api<'lua>(plugins_dir: PathBuf, runtime: &'lua LuaRuntime) -> eyre::Result<RustariaApi<'lua>> {
    let lua = &runtime.lua;

    let mut receiver = register_rustaria_api(lua)?;
    let plugins = PluginLoader { plugins_dir }.scan_and_load_plugins(lua).await?;
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
        walls: wall
    })
}

pub struct LuaRuntime {
    lua: Lua
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
