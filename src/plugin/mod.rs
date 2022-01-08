use std::{ffi::OsStr, io::Read, path::Path};

use eyre::{Context, Result};
use futures::StreamExt;
use memmap::Mmap;
use mlua::{Function, Lua};
use piz::{
    read::{as_tree, FileTree},
    ZipArchive,
};
use piz::read::DirectoryContents;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;
use tracing::{info, warn};

use crate::chunk::tile::Tile;

pub struct PluginLoader {
    pub lua: Lua,
}

macro_rules! lua_func {
    ($LUA:expr => $($METHOD:ident),*) => {
        $(
        $LUA.globals().set(stringify!($METHOD), $LUA.create_function(|_, v| $METHOD(v))?)?;
        )*
    };
}
impl PluginLoader {
    pub fn new() -> Self {
        PluginLoader {
            lua: Lua::new(),
        }
    }

    pub async fn scan_and_load_plugins_internal<'lua>(&'lua self, dir: &Path) -> Result<Plugins<'lua>> {
        info!("Scanning for plugins in directory {:?}", dir);

        let plugins = ReadDirStream::new(fs::read_dir(&dir).await?)
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
                    }
                    Err(e) => warn!("Unable to access file `{}` for reading! Permissions are perhaps insufficient!", e)
                }
                None
            })
            .collect()
            .await;
        Ok(Plugins(plugins))
    }

    pub async fn load_plugin<'lua>(&'lua self, path: &Path) -> Result<Plugin<'lua>> {
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

        let bootstrap_code = self.load_code(&zip, &tree, &manifest.bootstrap_path)?;
        let init_code = self.load_code(&zip, &tree, &manifest.init_path)?;
        info!(
            "Loaded plugin {} v{} from [{}]",
            manifest.name,
            manifest.version,
            file_name_or_unknown(&path)
        );
        Ok(Plugin { manifest, bootstrap_code, init_code })
    }

    fn load_code<'lua>(&'lua self, zip: &ZipArchive, tree: &DirectoryContents, path: &String) -> Result<Function<'lua>> {
        let metadata = tree.lookup(&path).wrap_err_with(|| {
            format!(
                "Main executable not found at path {}!",
                path
            )
        })?;

        let mut executable = zip.read(metadata)?;
        let mut code = Vec::with_capacity(metadata.size);
        executable.read_to_end(&mut code)?;


        Ok(self.lua.load(&code).into_function()?)
    }    
}

pub struct Plugins<'lua>(Vec<Plugin<'lua>>);

impl<'lua> Plugins<'lua> {
    pub fn bootstrap(&self, lua: &'lua Lua) -> Result<()> {
        for Plugin { bootstrap_code, .. } in &self.0 {
            lua_func!(lua => register_tile);

            bootstrap_code.call(())?;
        }
        Ok(())
    }

    pub fn init(&self, lua: &'lua Lua) -> Result<()> {
        for Plugin { init_code, .. } in &self.0 {
            lua_func!(lua => register_tile);
            init_code.call(())?;
        }
        Ok(())
    }
}


type LuaResult = Result<(), mlua::Error>;

pub fn register_tile((tag, value): (String, Tile)) -> LuaResult {
    println!("{}", value.flavour);
    Ok(())
}

pub struct Plugin<'lua> {
    manifest: Manifest,
    bootstrap_code: Function<'lua>,
    init_code: Function<'lua>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    name: String,
    version: String,
    bootstrap_path: String,
    init_path: String,
}

fn file_name_or_unknown(path: &Path) -> &str {
    path.file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("<unknown>")
}
