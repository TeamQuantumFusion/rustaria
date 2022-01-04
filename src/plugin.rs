use std::{ffi::OsStr, io::Read, path::Path};

use eyre::{Context, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;
use tracing::{info, warn};
use wasmer::{Exports, Instance, Module, Store};
use wasmer_wasi::{WasiEnv, WasiState};
use zip::ZipArchive;

use crate::api_impl;

pub struct PluginLoader {
    store: Store,
    wasi_env: WasiEnv,
    exports: Exports,
    pub plugins: Vec<Plugin>,
}

impl PluginLoader {
    pub fn new() -> Result<Self> {
        let store = Store::default();
        let wasi_env = WasiState::new("hello").finalize()?;
        let exports = api_impl::dump_exports(&store);

        Ok(Self {
            store,
            wasi_env,
            exports,
            plugins: vec![],
        })
    }

    pub async fn scan_and_load_plugins(&mut self, dir: &Path) -> Result<()> {
        info!("Scanning for plugins in directory {:?}", dir);
        self.plugins = ReadDirStream::new(fs::read_dir(&dir).await?)
            .filter_map(|entry| async {
                match entry {
                    Ok(entry) => {
                        // only look at zip files
                        let path = entry.path();
                        if let Some("zip") = path.extension().and_then(OsStr::to_str) {
                            match self.load_plugin(&path).await {
                                Ok(plugin) => return Some(plugin),
                                Err(e) => {
                                    warn!("Error loading plugin [{}]: {}", file_name_or_unknown(&path), e)
                                }
                            }
                        }
                    },
                    Err(e) => warn!("Unable to access file `{}` for reading! Permissions are perhaps insufficient!", e)
                }
                None
            })
            .collect()
            .await;

        Ok(())
    }

    // TODO: async-ify
    pub async fn load_plugin(&self, path: &Path) -> Result<Plugin> {
        let zip = std::fs::File::open(path)?;
        let mut zip = ZipArchive::new(zip)?;

        let manifest = zip
            .by_name("manifest.json")
            .wrap_err("manifest.json not found")?;

        let manifest: Manifest = serde_json::from_reader(manifest)?;
        let mut executable = zip.by_name(&manifest.executable_path).wrap_err_with(|| {
            format!(
                "Main executable not found at path {}!",
                manifest.executable_path
            )
        })?;
        let mut buf = vec![];
        executable.read_to_end(&mut buf)?;
        let module =
            Module::new(&self.store, &buf).wrap_err("Failed to compile main executable")?;

        info!(
            "Loaded plugin {} v{} from [{}]",
            manifest.name,
            manifest.version,
            file_name_or_unknown(&path)
        );
        Ok(Plugin { manifest, module })
    }

    pub fn run(&mut self) -> Result<()> {
        for Plugin { module, .. } in &self.plugins {
            let mut imports = self.wasi_env.import_object(module)?;
            imports.register("env", self.exports.clone());
            let instance = Instance::new(module, &imports)?;
            instance
                .exports
                .get_native_function::<(), ()>("initialize")?
                .call()?;
        }
        Ok(())
    }
}

pub struct Plugin {
    manifest: Manifest,
    module: Module,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    name: String,
    version: String,
    executable_path: String,
}

fn file_name_or_unknown(path: &Path) -> &str {
    path.file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("<unknown>")
}
