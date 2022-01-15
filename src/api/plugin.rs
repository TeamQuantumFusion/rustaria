#![allow(clippy::needless_lifetimes)]


use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

use eyre::{ContextCompat, Report};
use futures::StreamExt;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::fs::{self, DirEntry};
use tokio_stream::wrappers::ReadDirStream;
use tracing::{debug, info, warn};
use zip::ZipArchive;

pub async fn scan_and_load_plugins<'lua>(plugins_dir: &Path, lua: &'lua Lua) -> eyre::Result<Plugins<'lua>> {
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

    let data = archive.get_asset(AssetPath::Manifest)?;
    let manifest: Manifest = serde_json::from_reader(data.as_slice())?;

    let source = archive.get_asset(AssetPath::Src(PathBuf::from(&manifest.init_path)))?;
    let init = lua.load(&source).into_function()?;
    info!(
            "Loaded plugin {} v{} from [{}]",
            manifest.name,
            manifest.version,
            file_name_or_unknown(path)
        );
    Ok(Plugin { archive , manifest, init })
}


pub struct Plugins<'lua>(pub(crate) HashMap<String, Plugin<'lua>>);

impl<'lua> Plugins<'lua> {
    pub fn init(&self, lua: &'lua Lua) -> eyre::Result<()> {
        info!("Initializing plugins");
        for Plugin { manifest, init, .. } in self.0.values() {
            debug!("Initializing plugin {}", manifest.name);
            lua.globals().set("mod_id", manifest.name.clone())?;
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
    index: HashMap<AssetPath, u64>,
    zip: Option<ZipArchive<File>>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize)]
pub enum AssetPath {
    Asset(PathBuf),
    Src(PathBuf),
    Manifest,
}

impl PluginArchive {
    pub fn new(path: &Path) -> eyre::Result<Self> {
        let mut zip = ZipArchive::new(File::open(path)?)?;
        let mut file_lookup = HashMap::new();
        for index in 0..zip.len() {
            let file = zip.by_index(index)?;
            if file.is_file() {
                if let Some(name) = file.enclosed_name() {
                    let buf = name.to_path_buf();
                    let mut components = buf.components();
                    let option = components.next().unwrap();

                    let path = components.collect();
                    file_lookup.insert(
                        match option.as_os_str().to_str().unwrap() {
                            "src" => AssetPath::Src(path),
                            "asset" => AssetPath::Asset(path),
                            "manifest.json" => AssetPath::Manifest,
                            _ => return Err(Report::msg("Unknown File type."))
                        }
                        , index as u64);
                }
            }
        }

        Ok(Self {
            path: PathBuf::from(path),
            index: file_lookup,
            zip: Some(zip),
        })
    }

    pub fn enable_reading(&mut self) -> eyre::Result<()> {
        if self.zip.is_none() {
            self.zip = Some(ZipArchive::new(File::open(&self.path)?)?)
        }
        Ok(())
    }

    pub fn disable_reading(&mut self) {
        self.zip = None;
    }

    pub fn get_asset(&mut self, path: AssetPath) -> eyre::Result<Vec<u8>> {
        match &mut self.zip {
            Some(zip) => {
                let index = self.index.get(&path).wrap_err(format!("Could not find file {:?}", path))?;
                let mut result = zip.by_index(*index as usize)?;
                let mut data = Vec::with_capacity(result.size() as usize);
                result.read_to_end(&mut data)?;
                Ok(data)
            }
            None => Err(Report::msg("Reading not active."))
        }
    }
}
