use std::{
    collections::HashMap,
    io::{self, ErrorKind},
    path::PathBuf,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use lua::reload::{RegistryBuilder};
use mlua::UserData;
use plugin::Plugin;
use registry::Registry;
use rustaria_util::{blake3::Hasher, info, trace, warn};
use ty::{PluginId, Prototype, Tag, LuaConvertableCar};
use type_map::concurrent::TypeMap;

mod archive;
pub mod lua;

pub mod plugin;
pub mod registry;
pub mod ty;

pub struct Api {
    plugins: HashMap<PluginId, Plugin>,
}

impl Api {
    pub fn new(plugins_dir: PathBuf) -> io::Result<Api> {
        let mut plugins = HashMap::new();
        for entry in std::fs::read_dir(plugins_dir)?.flatten() {
            let path = entry.path();

            if match path.extension() {
                Some(extention) if extention == "zip" => true,
                _ => path.is_dir(),
            } {
                match Plugin::new(&path) {
                    Ok(plugin) => {
                        trace!("Loaded plugin {}.", plugin.manifest.id);
                        plugins.insert(plugin.manifest.id.clone(), plugin);
                    }
                    Err(error) => {
                        warn!("Could not load plugin at {:?}. Reason: {:?}", path, error);
                    }
                }
            }
        }

        Ok(Api { plugins })
    }

    pub fn get_asset(&self, location: &Tag) -> io::Result<Vec<u8>> {
        self.plugins
            .get(location.plugin_id())
            .ok_or(ErrorKind::NotFound)?
            .archive
            .get_asset(&("asset/".to_owned() + location.identifier()))
    }

    pub fn reload<'a>(&'a mut self, stack: &'a mut Carrier) -> ApiReload<'a> {
        info!("Reloading {} plugins", self.plugins.len());
        ApiReload {
            api: self,
            stack: stack
                .data
                .write()
                .expect("Could not aquire write lock of RegistryStack."),
            registry_builders: TypeMap::new(),
            hasher: Hasher::new(),
        }
    }
}

/// This is the most cringe part of the codebase for the better.
/// ## add_reload_registry
/// This should be called for every prototype that the system has. It adds a builder for that registry.
/// ## reload
/// This is the first big step. It invokes every plugin entrypoint that fills the builders.
/// This is also where we reset the `RegistryStack` because the next step fills the registries.
/// ## add_apply_registry
/// This step compiles all of the builders and fills the `RegistryStack` with the registries.
/// This also appends the `Hasher` with all of the entries for syncing.
/// ## apply
/// This is the last step. It just compiles the hash and sets it on the `RegistryStack`
pub struct ApiReload<'a> {
    api: &'a mut Api,
    stack: RwLockWriteGuard<'a, (TypeMap, [u8; 32])>,
    registry_builders: TypeMap,
    hasher: Hasher,
}

impl<'a> ApiReload<'a> {
    pub fn add_reload_registry<P: Prototype + LuaConvertableCar>(&mut self) -> mlua::Result<()> {
        let mut builder = RegistryBuilder::<P>::new();
        for (_, plugin) in &mut self.api.plugins {
            builder.register(&plugin.lua)?;
        }

        self.registry_builders.insert::<RegistryBuilder<P>>(builder);
        Ok(())
    }

    pub fn reload(&mut self) {
        // Reset registry stack
        self.stack.0.clear();
        self.stack.1 = [0u8; 32];

        // Reload plugins
        for (id, plugin) in &mut self.api.plugins {
            if let Some(entry) = &plugin.manifest.common_entry {
                match plugin.archive.get_asset(&("src/".to_owned() + entry)) {
                    Ok(code) => {
                        if let Err(err) = plugin.lua.load(&code).call::<_, ()>(()) {
                            panic!("Entrypoint error: \n{err}")
                        }
                    }
                    Err(err) => {
                        warn!(target: id, "Could not find entrypoint because {err}")
                    }
                }
            }
        }
    }

    pub fn add_apply_registry<P: Prototype + LuaConvertableCar>(&mut self) -> mlua::Result<()> {
        // Clear references
        for (_, plugin) in &mut self.api.plugins {
            plugin.lua.globals().raw_remove(P::lua_registry_name())?;
        }

        // Aquire builder
        let builder = self
            .registry_builders
            .remove::<RegistryBuilder<P>>()
            .expect("Cannot find registry");

        // Insert registry
        self.stack
            .0
            .insert::<Registry<P>>(builder.finish(&mut self.hasher)?);
        Ok(())
    }

    pub fn apply(self) {
        // Set the hash
        let mut stack = self.stack;
        stack.1 = self.hasher.finalize();
    }
}

/// A carrier of all of the registries and the core hash.
/// Carrier has arrived!
#[derive(Clone)]
pub struct Carrier {
    data: Arc<RwLock<(TypeMap, [u8; 32])>>,
}

impl Carrier {
    pub fn new() -> Carrier {
        Carrier {
            data: Arc::new(RwLock::new((TypeMap::new(), [0u8; 32]))),
        }
    }

    pub fn lock(&self) -> RegistryStackAccess {
        RegistryStackAccess { 
            lock: 
            self.data.read().unwrap()
         }
    }
}

pub struct RegistryStackAccess<'a> {
    lock: RwLockReadGuard<'a, (TypeMap, [u8; 32])>
}

impl<'a> RegistryStackAccess<'a> {
    pub fn get_hash(&self) -> [u8; 32] {
        self.lock.1
    }

    // needs to be func because of lock issues
    pub fn get_registry<P: Prototype>(&self) -> &Registry<P> {
        self.lock.0.get::<Registry<P>>().expect("Could not find registry")
    }
}