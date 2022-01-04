use std::path::Path;

use eyre::Result;
use wasmer::{Exports, Function, Instance, Module, Store};
use wasmer_wasi::{WasiEnv, WasiState};

use crate::api_impl;

pub struct PluginLoader {
    store: Store,
    wasi_env: WasiEnv,
    exports: Exports,
}

impl PluginLoader {
    pub fn new() -> Result<Self> {
        let store = Store::default();
        let wasi_env = WasiState::new("hello").finalize()?;
        let mut exports = Exports::new();

        exports.insert(
            "it_adds_two",
            Function::new_native(&store, api_impl::it_adds_two),
        );
        Ok(Self {
            store,
            wasi_env,
            exports,
        })
    }

    pub fn load_plugin_from_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.load_plugin_from_bytes(&std::fs::read(path)?)
    }

    pub fn load_plugin_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.load_plugin(Module::new(&self.store, bytes)?)
    }

    pub fn load_plugin(&mut self, module: Module) -> Result<()> {
        let mut imports = self.wasi_env.import_object(&module)?;
        imports.register("env", self.exports.clone());

        let instance = Instance::new(&module, &imports)?;

        let setup = instance.exports.get_native_function::<(), ()>("_setup")?;
        setup.call()?;

        Ok(())
    }
}
