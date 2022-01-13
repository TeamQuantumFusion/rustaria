#![allow(clippy::needless_lifetimes)]

use std::{
    ffi::OsStr,
    io::Read,
    path::{Path, PathBuf},
};

use eyre::{Context, Result};
use futures::StreamExt;
use memmap::Mmap;
use mlua::prelude::*;
use piz::read::DirectoryContents;
use piz::{
    read::{as_tree, FileTree},
    ZipArchive,
};
use serde::{Deserialize, Serialize};
use tokio::fs::{self, File};
use tokio_stream::wrappers::ReadDirStream;
use tracing::{debug, info, warn};
use crate::registry::Registry;

pub struct PluginLoader {
    pub plugins_dir: PathBuf,
}

impl PluginLoader {
    pub async fn scan_and_load_plugins<'lua>(&self, lua: &'lua Lua) -> Result<Plugins<'lua>> {
        info!("Scanning for plugins in directory {:?}", self.plugins_dir);

        if let Ok(read_dir) = fs::read_dir(&self.plugins_dir).await {
            let plugins = ReadDirStream::new(read_dir).filter_map(|entry| async {
                match entry {
                    Ok(entry) => {
                        // only look at zip files
                        let path = entry.path();
                        if let Some("zip") = path.extension().and_then(OsStr::to_str) {
                            match self.load_plugin(&path, lua).await {
                                Ok(plugin) => return Some(plugin),
                                Err(e) => {
                                    warn!(
                                        "Error loading plugin [{}]: {}",
                                        file_name_or_unknown(&path),
                                        e
                                    )
                                }
                            }
                        }
                    }
                    Err(e) => warn!(
                        "Unable to access file `{}` for reading! Permissions are perhaps insufficient!",
                        e
                    ),
                }
                None
            });
            Ok(Plugins(plugins.collect().await))
        } else {
            warn!("Plugin directory not found! Creating one...");
            fs::create_dir_all("plugins").await?;
            Ok(Plugins(vec![]))
        }
    }

    pub async fn load_plugin<'lua>(&self, path: &Path, lua: &'lua Lua) -> Result<Plugin<'lua>> {
        let zip = File::open(path).await.wrap_err_with(|| {
            format!("Plugin archive [{}] not found", file_name_or_unknown(path))
        })?;
        let mapping = unsafe { Mmap::map(&zip.into_std().await)? };
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

        let init = self.load_code(&zip, &tree, manifest.init_path.as_ref(), lua)?;
        info!(
            "Loaded plugin {} v{} from [{}]",
            manifest.name,
            manifest.version,
            file_name_or_unknown(path)
        );
        Ok(Plugin { manifest, init })
    }

    fn load_code<'lua>(
        &self,
        zip: &ZipArchive,
        tree: &DirectoryContents,
        path: &Path,
        lua: &'lua Lua,
    ) -> Result<LuaFunction<'lua>> {
        let metadata = tree.lookup(&path).wrap_err_with(|| {
            format!(
                "Could not find file containing code (looking for {})!",
                file_name_or_unknown(path)
            )
        })?;

        let mut executable = zip.read(metadata)?;
        let mut code = Vec::with_capacity(metadata.size);
        executable.read_to_end(&mut code)?;

        Ok(lua.load(&code).into_function()?)
    }
}

pub struct Plugins<'lua>(Vec<Plugin<'lua>>);

impl<'lua> Plugins<'lua> {
    pub fn init(&self) -> Result<()> {
        info!("Initializing plugins");
        for Plugin { manifest, init } in &self.0 {
            debug!("Initializing plugin {}", manifest.name);
            init.call(())?;
            debug!("Finished initializing plugin {}", manifest.name);
        }
        Ok(())
    }
}

pub struct Plugin<'lua> {
    manifest: Manifest,
    init: LuaFunction<'lua>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    name: String,
    version: String,
    init_path: String,
}

fn file_name_or_unknown(path: &Path) -> &str {
    path.file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("<unknown>")
}
