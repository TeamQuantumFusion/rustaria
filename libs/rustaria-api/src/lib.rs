use std::any::{type_name, Any};
use std::collections::HashMap;
use std::path::PathBuf;

use glob::glob;
use mlua::Lua;
pub use mlua::UserData;
use typemap::TypeMap;

use plugin::id::PluginId;
use rustaria_util::blake3::Hasher;
use rustaria_util::{debug, info, Context, ContextCompat, Report, Result};

use crate::lua::PluginContext;
use crate::plugin::archive::ArchivePath;
use crate::plugin::Plugin;
use crate::prototype::Prototype;
use crate::registry::{Registry, RegistryBuilder};

pub mod lua;
pub mod plugin;
pub mod prototype;
pub mod registry;
pub mod tag;

// kernel identification
pub type RawId = u32;

pub struct ApiHandler {
    lua: Lua,
    plugins: HashMap<PluginId, Plugin>,
    registries: TypeMap,
    hash: [u8; 32],
}

impl ApiHandler {
    pub fn new() -> Result<ApiHandler> {
        let mut handler = ApiHandler {
            lua: Lua::new(),
            plugins: HashMap::new(),
            registries: TypeMap::new(),
            hash: [0u8; 32],
        };
        lua::register_api(&handler.lua)?;
        handler.load_plugins()?;
        Ok(handler)
    }

    pub fn reload(&mut self) -> ApiReloadInstance {
        ApiReloadInstance {
            api: self,
            hasher: Hasher::new(),
            registries: TypeMap::new(),
        }
    }

    pub fn apply(&mut self, builder: (Hasher, TypeMap)) {
        self.registries = builder.1;
        self.hash = builder.0.finalize();
    }

    pub fn get_plugin(&self, id: &PluginId) -> Option<&Plugin> {
        self.plugins.get(id)
    }

    pub fn get_registry<P: Prototype>(&self) -> &Registry<P> {
        self.registries
            .get::<Registry<P>>()
            .wrap_err("Invalid Registry")
            .unwrap()
    }
}

// Internal methods
impl ApiHandler {
    fn load_plugins(&mut self) -> Result<()> {
        debug!("Loading plugins");
        for path in glob("./plugins/*.zip")
            .wrap_err("Could not find plugin directory.")?
            .flatten()
        {
            self.load_plugin(path.clone()).wrap_err(format!("Failed to load plugin at {:?}", path))? ;
        }


        Ok(())
    }


    fn load_plugin(&mut self, path: PathBuf) -> Result<()> {
        let plugin = Plugin::new(path)?;

        info!("Loaded {} ({})", plugin.manifest.name, plugin.manifest.id);
        self.plugins.insert(plugin.manifest.id.clone(), plugin);
        Ok(())
    }
}

pub struct ApiReloadInstance<'a> {
    api: &'a mut ApiHandler,
    hasher: Hasher,
    registries: TypeMap,
}

impl ApiReloadInstance<'_> {
    pub fn register_builder<P: 'static + Prototype>(&mut self) -> Result<()> {
        let name = P::name();
        debug!("Registered {}", name);
        RegistryBuilder::<P>::new(name).register(&self.api.lua)?;
        Ok(())
    }

    pub fn reload(&mut self) -> Result<()> {
        macro_rules! entry_point {
            ($NAME:literal $FIELD:ident) => {
                for plugin in self.api.plugins.values() {
                    if let Some(path) = &plugin.manifest.$FIELD {
                        self.invoke_entrypoint(plugin, path, $NAME)
                            .wrap_err(format!(
                                "Error while reloading plugin {}",
                                plugin.manifest.id
                            ))?;
                    }
                }
            };
        }

        entry_point!("preEntry" common_pre_entry);
        entry_point!("entry" common_entry);

        #[cfg(feature = "client")]
        {
            entry_point!("preEntryClient" client_pre_entry);
            entry_point!("entryClient" client_entry);
        }

        Ok(())
    }

    pub fn compile_builder<P: 'static + Prototype>(&mut self) -> Result<()> {
        let name = P::name();
        debug!("Compiling {}", name);
        let builder: RegistryBuilder<P> = self.api.lua.globals().get(name)?;
        self.registries
            .insert::<Registry<P>>(builder.finish(&mut self.hasher));
        Ok(())
    }

    fn invoke_entrypoint(&self, plugin: &Plugin, path: &String, name: &str) -> Result<()> {
        debug!("Invoking {} {}", plugin.manifest.id, name);
        PluginContext::from(plugin).set(&self.api.lua)?;

        self.api
            .lua
            .load(
                plugin
                    .archive
                    .get_asset(&ArchivePath::Code(path.clone()))
                    .wrap_err(format!("Could not find entrypoint {}s file {}", name, path))?,
            )
            .call(())?;

        Ok(())
    }

    pub fn apply(self) {
        self.api.registries = self.registries;
        self.api.hash = self.hasher.finalize();
    }
}
