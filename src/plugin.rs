use std::{ffi::OsStr, io::Read, path::Path};

use eyre::{Context, Result};
use futures::StreamExt;
use memmap::Mmap;
use mlua::Lua;
use piz::{
    read::{as_tree, FileTree},
    ZipArchive,
};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;
use tracing::{info, warn};

pub struct PluginLoader {
    pub plugins: Vec<Plugin>,
    lua: Lua,
}

impl PluginLoader {
    pub fn new() -> Result<Self> {
        Ok(Self {
            plugins: vec![],
            lua: Lua::new(),
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
        let zip = std::fs::File::open(path).wrap_err_with(|| {
            format!("Plugin archive [{}] not found", file_name_or_unknown(path))
        })?;
        let mapping = unsafe { Mmap::map(&zip)? };
        let zip = ZipArchive::new(&mapping).wrap_err_with(|| {
            format!(
                "Archive file [{}] could not be read",
                file_name_or_unknown(path)
            )
        })?;
        let tree = as_tree(zip.entries())?;

        let manifest = tree
            .lookup("manifest.json")
            .wrap_err("manifest.json not found")?;

        let manifest: Manifest = serde_json::from_reader(zip.read(manifest)?)?;
        let executable = tree.lookup(&manifest.executable_path).wrap_err_with(|| {
            format!(
                "Main executable not found at path {}!",
                manifest.executable_path
            )
        })?;
        let mut executable = zip.read(executable)?;
        let mut code = vec![];
        executable.read_to_end(&mut code)?;

        info!(
            "Loaded plugin {} v{} from [{}]",
            manifest.name,
            manifest.version,
            file_name_or_unknown(&path)
        );
        Ok(Plugin { manifest, code })
    }

    pub fn run(&mut self) -> Result<()> {
        for Plugin { code, .. } in &self.plugins {
            self.lua.load(code).exec()?
        }
        Ok(())
    }
}

pub struct Plugin {
    manifest: Manifest,
    code: Vec<u8>,
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
