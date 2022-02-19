use std::fmt::Debug;

use crate::{
    blake3::{Hasher, OUT_LEN},
    registry::Registry,
};
use mlua::prelude::*;

use self::{
    loader::{PluginInput, PluginOutput, PluginOutputs},
    prototypes::{EntityPrototype, TilePrototype, WallPrototype},
};

mod hook;
pub mod loader;
mod log;
mod meta;
pub mod plugin;
pub mod prototypes;
pub mod types;

#[derive(Default)]
pub struct Rustaria {
    pub mod_list: ModList,

    pub hash: RustariaHash,
    pub tiles: Registry<TilePrototype>,
    pub walls: Registry<WallPrototype>,
    pub entities: Registry<EntityPrototype>,
}

impl Rustaria {
    pub fn reload(&mut self, outputs: PluginOutputs) {
        if let Some(summarized) = outputs.into_iter().reduce(PluginOutput::combine) {
            let mut hasher = Hasher::new();
            self.tiles = summarized.tiles.finish(&mut hasher);
            self.walls = summarized.walls.finish(&mut hasher);
            self.entities = summarized.entities.finish(&mut hasher);
        }
    }
}

pub type ModList = Vec<(String, String)>;

// impl<'lua> Rustaria<'lua> {
//     pub fn new(plugins_dir: &Path, lua: &'lua LuaRuntime) -> Result<Rustaria<'lua>> {
//         let mut api = Self::default();
//         api.reload(plugins_dir, lua)?;
//         Ok(api)
//     }
//     pub fn reload(&mut self, plugins_dir: &Path, lua: &'lua LuaRuntime) -> Result<()> {
//         self.plugins = plugin::scan_and_load_plugins(plugins_dir, lua)?;
//         self.plugins.init(lua)?;

//         let mut hasher = Hasher::new();
//         self.tiles = RegistryBuilder::new("tiles")
//             .register_all(lua.registries.tile.read().clone())
//             .build(&mut hasher);
//         self.walls = RegistryBuilder::new("walls")
//             .register_all(lua.registries.wall.read().clone())
//             .build(&mut hasher);
//         self.entities = RegistryBuilder::new("entities")
//             .register_all(lua.registries.entity.read().clone())
//             .build(&mut hasher);
//         Ok(())
//     }
//     pub fn get_plugin_assets(&self, plugin: &str) -> Option<&PluginArchive> {
//         self.plugins.0.get(plugin).map(|plugin| &plugin.archive)
//     }
// }

// pub struct LuaRuntime {
//     lua: Lua,

//     registries: Registries,
// }
// impl LuaRuntime {
//     pub fn new() -> LuaResult<Self> {
//         let lua = Lua::new();
//         {
//             let package: LuaTable = lua.globals().get("package")?;
//             let preload: LuaTable = package.get("preload")?;

//             preload.set("log", log::package(&lua)?)?;
//             preload.set("meta", meta::package(&lua)?)?;
//         }

//         let registries = Registries::new(&lua)?;

//         Ok(Self { lua, registries })
//     }
//     pub fn reload(&mut self) {
//         self.registries.clear();
//     }
// }

#[derive(Default, Debug, PartialEq, Eq, Clone, serde::Serialize)]
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

fn plugin_id(lua: &Lua) -> LuaResult<String> {
    let ctx = PluginInput::get(lua)?;
    Ok(ctx.id)
}
