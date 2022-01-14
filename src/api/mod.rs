use std::path::PathBuf;
use crate::chunk::tile::TilePrototype;
use crate::chunk::wall::WallPrototype;
use crate::registry::{Id, Registry, RegistryStack, Tag};
use mlua::prelude::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use crate::plugin::PluginLoader;

mod log;
#[macro_use]
pub(crate) mod macros;
mod tile;

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

pub async fn launch_rustaria_api(plugins_dir: PathBuf, lua: &Lua) -> eyre::Result<RegistryStack> {
    register_rustaria_api(&lua)?;
    let loader = PluginLoader {
        plugins_dir,
    };
    let plugins = loader.scan_and_load_plugins(&lua).await?;
    let mut receiver = register_rustaria_api(&lua)?;
    // call initPath files
    plugins.init(&lua)?;

    // register all prototypes
    let mut tile_registry = Registry::new();
    let mut wall_registry = Registry::new();
    while let Some(prototype) = receiver.recv().await {
        match prototype {
            PrototypeRequest::Tile(id, pt) => tile_registry.register(id, pt),
            PrototypeRequest::Wall(id, pt) => wall_registry.register(id, pt),
        };
    }

    let stack = RegistryStack {
        tile: tile_registry,
        wall: wall_registry,
    };
    Ok(stack)
}

pub enum PrototypeRequest {
    Tile(Tag, TilePrototype),
    Wall(Tag, WallPrototype),
}

pub trait Prototype<T> {
    fn create(&self, id: Id) -> T;
}
