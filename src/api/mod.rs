use std::collections::HashMap;
use std::path::PathBuf;

use eyre::Result;
use mlua::prelude::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::api::hook::Hook;
use crate::api::plugin::{PluginArchive, Plugins};
use crate::blake3::{Hasher, OUT_LEN};
use crate::chunk::tile::TilePrototype;
use crate::chunk::wall::WallPrototype;
use crate::entity::EntityPrototype;
use crate::registry::{Registry, RegistryBuilder, Tag};

use self::context::PluginContext;

mod log;
#[macro_use]
pub(crate) mod macros;
mod context;
mod hook;
mod meta;
pub mod plugin;

pub struct Rustaria<'lua> {
    pub plugins: Plugins<'lua>,

    pub hash: RustariaHash,
    pub tiles: Registry<TilePrototype>,
    pub walls: Registry<WallPrototype>,
    pub entities: Registry<EntityPrototype>,

    pub test_hook: Hook<'lua, (i32, i32)>,
}

impl<'lua> Rustaria<'lua> {
    pub async fn new(plugins_dir: PathBuf, lua: &'lua Lua) -> Result<Rustaria<'lua>> {
        let mut receiver = register_rustaria_api(lua)?;
        let plugins = plugin::scan_and_load_plugins(&plugins_dir, lua).await?;

        plugins.init(lua)?;

        let mut tiles = RegistryBuilder::new("tile");
        let mut walls = RegistryBuilder::new("wall");
        let mut entities = RegistryBuilder::new("entity");
        while let Ok(prototype) = receiver.try_recv() {
            match prototype {
                PrototypeRequest::Tile(id, pt) => tiles.register(id, pt),
                PrototypeRequest::Wall(id, pt) => walls.register(id, pt),
                PrototypeRequest::Entity(id, pt) => entities.register(id, pt),
            };
        }
        let mut hasher = Hasher::new();
        let tiles = tiles.build(&mut hasher);
        let walls = walls.build(&mut hasher);
        let entities = entities.build(&mut hasher);

        Ok(Self {
            plugins,
            hash: hasher.finalize(),
            tiles,
            walls,
            entities,
            test_hook: Hook::new(),
        })
    }

    pub fn get_plugin_assets(&self, plugin: &str) -> Option<&PluginArchive> {
        self.plugins.0.get(plugin).map(|plugin| &plugin.archive)
    }
}

fn get_plugin_id(lua: &Lua) -> LuaResult<String> {
    let ctx = PluginContext::get(lua)?;
    Ok(ctx.plugin_id)
}

macro_rules! proto {
    ($($name:ident => $proto:ty | $request:ident),* $(,)?) => {
        $(
            fn $name(lua: &Lua, send: UnboundedSender<PrototypeRequest>) -> LuaResult<LuaFunction> {
                lua.create_function(move |lua, _: ()| {
                    let send = send.clone();
                    lua.create_table_from([
                        ("register", lua.create_function(move |_, prototypes: HashMap<Tag, _>| {
                            let send = send.clone();
                            for (tag, prototype) in prototypes {
                                send.send(PrototypeRequest::$request(tag, prototype))
                                    .map_err(|err| LuaError::RuntimeError(err.to_string()))?;
                            }
                            Ok(())
                        })?),
                        ("default", lua.create_function(|lua, t| {
                            Ok(lua.from_value::<$proto>(LuaValue::Table(t)))
                        })?)
                    ])
                })
            }
        )*
    };
}

/// Registers Rustaria's Lua modding APIs.
pub fn register_rustaria_api(lua: &Lua) -> LuaResult<UnboundedReceiver<PrototypeRequest>> {
    let (tx, rx) = unbounded_channel();
    let package: LuaTable = lua.globals().get("package")?;
    let preload: LuaTable = package.get("preload")?;

    preload.set("log", log::package(lua)?)?;
    preload.set("meta", meta::package(lua)?)?;
    preload.set("wall", wall_methods(lua, tx.clone())?)?;
    preload.set("tile", tile_methods(lua, tx.clone())?)?;
    preload.set("entity", entity_methods(lua, tx.clone())?)?;
    Ok(rx)
}

proto! {
    wall_methods => WallPrototype | Wall,
    tile_methods => TilePrototype | Tile,
    entity_methods => EntityPrototype | Entity,
}

pub enum PrototypeRequest {
    Tile(Tag, TilePrototype),
    Wall(Tag, WallPrototype),
    Entity(Tag, EntityPrototype),
}

pub trait Prototype<T, Id = crate::registry::RawId> {
    fn create(&self, id: Id) -> T;
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize)]
pub struct RustariaHash {
    pub data: [u8; OUT_LEN],
}

impl RustariaHash {
    pub fn parse(data: Vec<u8>) -> RustariaHash {
        RustariaHash {
            data: <[u8; 32]>::try_from(data.as_slice()).unwrap(),
        }
    }
}
