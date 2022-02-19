use std::{collections::HashMap, fmt::Debug};

use mlua::prelude::*;
use rsa_common::hash::{Hasher, RustariaHash};
use crate::registry::Registry;

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
pub mod registry;

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
    }
}

pub type ModList = HashMap<String, String>;

fn plugin_id(lua: &Lua) -> LuaResult<String> {
    let ctx = PluginInput::get(lua)?;
    Ok(ctx.id)
}
