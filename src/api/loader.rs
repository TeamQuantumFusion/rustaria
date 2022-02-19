use std::collections::HashMap;

use mlua::prelude::*;
use serde::de::DeserializeOwned;
use tracing::debug;

use crate::{
    blake3::Hasher,
    registry::{Registry, Tag},
};

use super::{
    log, meta,
    plugin::Plugin,
    prototypes::{EntityPrototype, TilePrototype, WallPrototype},
};

#[derive(Default)]
pub struct Loader;

impl Loader {
    pub fn init<'lua>(&self, lua: &'lua Lua, plugins: &[Plugin<'lua>]) -> LuaResult<PluginOutputs> {
        plugins.iter().map(|p| Self::plugin_exec(lua, p)).collect()
    }
    fn plugin_exec<'lua>(lua: &'lua Lua, plugin: &'lua Plugin) -> LuaResult<PluginOutput> {
        // setup
        let package: LuaTable = lua.globals().get("package")?;
        let preload: LuaTable = package.get("preload")?;

        RegistryBuilder::<TilePrototype>::register(lua, "tile")?;
        RegistryBuilder::<WallPrototype>::register(lua, "wall")?;
        RegistryBuilder::<EntityPrototype>::register(lua, "entity")?;

        let input = PluginInput {
            id: plugin.manifest.plugin_id.clone(),
        };
        input.set(lua)?;

        preload.set("log", lua.create_function(log::package)?)?;
        preload.set("meta", lua.create_function(meta::package)?)?;

        plugin.init.call(())?;

        let globals = lua.globals();
        Ok(PluginOutput {
            id: plugin.manifest.plugin_id.clone(),
            version: plugin.manifest.version.clone(),

            tiles: globals.get("tile")?,
            walls: globals.get("wall")?,
            entities: globals.get("entity")?,
        })
    }
}

pub type PluginOutputs = Vec<PluginOutput>;

#[derive(Debug, Clone)]
pub struct PluginInput {
    pub id: String,
}
impl PluginInput {
    pub fn get(lua: &Lua) -> LuaResult<Self> {
        lua.globals().get("_ctx")
    }
    pub fn set(self, lua: &Lua) -> LuaResult<()> {
        lua.globals().set("_ctx", self)
    }
}
impl LuaUserData for PluginInput {}

#[derive(Clone)]
pub struct RegistryBuilder<T> {
    name: &'static str,
    data: Vec<(Tag, T)>,
}
impl<T: 'static + LuaUserData + DeserializeOwned + Clone> RegistryBuilder<T> {
    pub fn register(lua: &Lua, name: &'static str) -> LuaResult<()> {
        lua.globals().set(
            name,
            Self {
                name,
                data: Default::default(),
            },
        )
    }
    pub fn combine(&mut self, other: Self) {
        self.data.extend(other.data);
    }
    pub fn finish(mut self, hasher: &mut Hasher) -> Registry<T> {
        self.data
            .sort_by(|(i1, _), (i2, _)| i1.to_string().cmp(&i2.to_string()));

        for (id, (tag, _)) in self.data.iter().enumerate() {
            hasher.update(&id.to_be_bytes());
            hasher.update(tag.to_string().as_bytes());
        }

        let mut registry = Registry::new();

        for (id, (tag, item)) in self.data.into_iter().enumerate() {
            registry.entries.push(item);
            registry.id_to_tag.push(tag.clone());
            registry.tag_to_id.insert(tag, id as u32);
        }

        registry
    }
}
impl<T: 'static + DeserializeOwned + LuaUserData + Clone> LuaUserData for RegistryBuilder<T> {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(m: &mut M)
    where
        T: FromLua<'lua>,
    {
        m.add_method_mut("register", |_lua, this, t: HashMap<Tag, T>| {
            debug!(name = ?this.name, "Registered entries to registry");
            for k in t.keys() {
                debug!("{k}")
            }
            this.data.extend(t);
            Ok(())
        });
        m.add_method("default", |lua, _this, t| lua.from_value::<T>(t));
    }
}

pub struct PluginOutput {
    pub id: String,
    pub version: String,

    pub tiles: RegistryBuilder<TilePrototype>,
    pub walls: RegistryBuilder<WallPrototype>,
    pub entities: RegistryBuilder<EntityPrototype>,
}
impl PluginOutput {
    pub fn combine(mut self, other: Self) -> Self {
        self.tiles.combine(other.tiles);
        self.walls.combine(other.walls);
        self.entities.combine(other.entities);
        self
    }
}
