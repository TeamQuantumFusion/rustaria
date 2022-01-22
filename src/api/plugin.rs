#![allow(clippy::needless_lifetimes)]

use std::collections::HashMap;
use std::fs::File;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use std::io::Read;

use eyre::{bail, ContextCompat, eyre, Result};
use futures::StreamExt;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::fs::{self, DirEntry};
use tokio_stream::wrappers::ReadDirStream;
use tracing::{debug, info, warn};
use zip::ZipArchive;

use crate::api::meta::Meta;

pub async fn scan_and_load_plugins<'lua>(
    plugins_dir: &Path,
    lua: &'lua Lua,
) -> Result<Plugins<'lua>> {
    info!("Scanning for plugins in directory {:?}", plugins_dir);

    if let Ok(read_dir) = fs::read_dir(&plugins_dir).await {
        let plugins = ReadDirStream::new(read_dir).filter_map(|entry| async {
            match entry {
                Ok(entry) => {
                    process_file(entry, lua).await
                }
                Err(e) => {
                    warn!(
                            "Unable to access file `{}` for reading! Permissions are perhaps insufficient!",
                            e
                        );
                    None
                }
            }
        }).map(|plugin| (plugin.manifest.name.clone(), plugin));
        Ok(Plugins(plugins.collect().await))
    } else {
        warn!("Plugin directory not found! Creating one...");
        fs::create_dir_all("plugins").await?;
        Ok(Plugins(HashMap::new()))
    }
}

async fn process_file<'lua>(entry: DirEntry, lua: &'lua Lua) -> Option<Plugin<'lua>> {
    let path = entry.path();

    // only look at zip files
    if let Some("zip") = path.extension().and_then(OsStr::to_str) {
        match load_plugin(&path, lua).await {
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
    None
}

async fn load_plugin<'lua>(path: &Path, lua: &'lua Lua) -> eyre::Result<Plugin<'lua>> {
    let mut archive = PluginArchive::new(path)?;

    let data = archive.get_asset(&ArchivePath::Manifest)?;
    let manifest: Manifest = serde_json::from_reader(data.as_slice())?;

    let source = archive.get_asset(&ArchivePath::Src(PathBuf::from(&manifest.init_path)))?;
    let init = lua.load(source).into_function()?;
    info!(
        "Loaded plugin {} v{} from [{}]",
        manifest.name,
        manifest.version,
        file_name_or_unknown(path)
    );
    Ok(Plugin {
        archive,
        manifest,
        init,
    })
}

pub struct Plugins<'lua>(pub(crate) HashMap<String, Plugin<'lua>>);

impl<'lua> Plugins<'lua> {
    pub fn init(&self, lua: &'lua Lua) -> eyre::Result<()> {
        info!("Initializing plugins");
        let package: LuaTable = lua.globals().get("package")?;
        let preload: LuaTable = package.get("preload")?;

        for Plugin { manifest, init, .. } in self.0.values() {
            debug!("Initializing plugin {}", manifest.name);
            let meta = Meta {
                mod_id: manifest.name.clone(),
            };
            preload.set("meta", meta.into_module(lua)?)?;
            init.call(())?;
            debug!("Finished initializing plugin {}", manifest.name);
        }
        Ok(())
    }
}

pub struct Plugin<'lua> {
    pub archive: PluginArchive,
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

pub struct PluginArchive {
    path: PathBuf,
    data: Option<HashMap<ArchivePath, Vec<u8>>>,
}


#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize)]
pub enum ArchivePath {
    Asset(PathBuf),
    Src(PathBuf),
    Manifest,
}

impl PluginArchive {
    pub fn new(path: &Path) -> eyre::Result<Self> {
        Ok(Self {
            path: PathBuf::from(path),
            data: None,
        })
    }

    pub fn enable_reading(&mut self) -> eyre::Result<()> {
        let mut zip = ZipArchive::new(File::open(&self.path)?)?;
        let mut data = HashMap::new();
        for index in 0..zip.len() {
            let mut file = zip.by_index(index)?;
            if file.is_file() {
                if let Some(name) = file.enclosed_name() {
                    let buf = name.to_path_buf();
                    let mut components = buf.components();
                    let option = components.next().unwrap();
                    let path = components.collect();


                    let mut file_data = Vec::with_capacity(file.size() as usize);
                    file.read_to_end(&mut file_data);

                    data.insert(
                        match option.as_os_str().to_str().unwrap() {
                            "src" => ArchivePath::Src(path),
                            "asset" => ArchivePath::Asset(path),
                            "manifest.json" => ArchivePath::Manifest,
                            _ => bail!("Unknown File type."),
                        },
                        file_data,
                    );
                }
            }
        }

        self.data = Some(data);
        Ok(())
    }

    pub fn disable_reading(&mut self) {
        self.data = None;
    }

    pub fn get_asset(&mut self, path: &ArchivePath) -> Result<&Vec<u8>> {
        let option = &self.data;
        match option {
            None => Err(eyre::Error::msg("Reading not active")),
            Some(files) => {
                Ok(files.get(path).wrap_err("Could not find file")?)
            }
        }
    }
}
