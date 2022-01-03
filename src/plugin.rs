use std::path::Path;

use eyre::Result;
use wasmer::{imports, ImportObject, Module, Store, Universal, UniversalEngine, Value, Global, Instance, wat2wasm, Function};

use crate::api_impl;

pub struct PluginLoader {
    store: Store,
    imports: ImportObject,
}

impl PluginLoader {
    pub fn new() -> Self {
        let store = Store::default();
        let imports = imports! {
            "env" => {
                "it_adds_two" => Function::new_native(&store, api_impl::it_adds_two)
            }
        };
        Self {
            store,
            imports,
        }
    }

    pub fn load_plugin_from_file(&self, path: impl AsRef<Path>) -> Result<()> {
        self.load_plugin_from_bytes(&std::fs::read(path)?)
    }

    pub fn load_plugin_from_bytes(&self, bytes: &[u8]) -> Result<()> {
        self.load_plugin(Module::new(&self.store, bytes)?)
    }

    pub fn load_plugin(&self, module: Module) -> Result<()> {
        let instance = Instance::new(&module, &self.imports)?;

        // The Wasm module exports a string under "name"
        let setup = instance.exports.get_native_function::<(), ()>("setup")?;
        setup.call()?;

        Ok(())
    }
}
