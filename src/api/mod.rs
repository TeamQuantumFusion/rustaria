use std::{collections::HashMap, fmt::Debug, path::PathBuf};

use eyre::Result;
use mlua::prelude::*;
use tracing::info;

use crate::{
    blake3::{Hasher, OUT_LEN},
    registry::Registry,
};
use crate::api::loader::Loader;
use crate::api::plugin::Plugins;

use self::{
    loader::{PluginInput, PluginOutput},
    prototypes::{EntityPrototype, TilePrototype, WallPrototype},
};

mod hook;
pub mod loader;
mod log;
mod meta;
pub mod plugin;
pub mod prototypes;
pub mod types;

pub struct Rustaria {
    pub mod_list: ModList,

    pub lua: Lua,
    pub plugins_dir: PathBuf,
    pub plugins: Plugins,

    pub hash: RustariaHash,
    pub tiles: Registry<TilePrototype>,
    pub walls: Registry<WallPrototype>,
    pub entities: Registry<EntityPrototype>,
}

impl Rustaria {
    pub fn new(plugins_dir: impl Into<PathBuf>) -> Self {
        Self {
            lua: Lua::new(),
            plugins_dir: plugins_dir.into(),
            plugins: Default::default(),
            mod_list: Default::default(),
            hash: Default::default(),
            tiles: Default::default(),
            walls: Default::default(),
            entities: Default::default(),
        }
    }

    pub fn reload(&mut self) -> Result<()> {
        info!("Scanning for plugins in directory {:?}", self.plugins_dir);
        self.plugins = Plugins::load(&self.plugins_dir)?;
        info!("Executing plugins");
        let mut loader = Loader::default();
        let outputs = loader.init(&self.lua, &self.plugins)?;

        info!("Initializing API");
        self.mod_list.clear();
        self.tiles.clear();
        self.walls.clear();
        self.entities.clear();

        let mut hasher = Hasher::new();
        if let Some(summarized) = outputs
            .into_iter()
            .inspect(|o| {
                self.mod_list.insert(o.id.clone(), o.version.clone());
            })
            .reduce(PluginOutput::combine)
        {
            self.tiles = summarized.tiles.finish(&mut hasher);
            self.walls = summarized.walls.finish(&mut hasher);
            self.entities = summarized.entities.finish(&mut hasher);
        }

        Ok(())
    }
}

pub type ModList = HashMap<String, String>;

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
