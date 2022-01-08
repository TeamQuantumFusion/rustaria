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

use crate::chunk::tile;
use crate::chunk::tile::Tile;

pub struct PluginLoader<'lua> {
    plugins: Vec<Plugin<'lua>>,
    lua: Lua,
}

macro_rules! lua_func {
    ($LUA:expr => $($METHOD:ident),*) => {
        $(
        $LUA.globals().set(stringify!($METHOD), $LUA.create_function(|_, v| $METHOD(v))?)?;
        )*
    };
}
impl<'lua: 'load, 'load> PluginLoader<'lua> {
    pub fn new() -> Result<PluginLoader<'lua>> {
        let lua = Lua::new();
        Ok(PluginLoader {
            plugins: vec![],
            lua,
        })
    }

    pub async fn scan_and_load_plugins_internal(&'lua mut self, dir: &Path) -> Result<()> {
        info!("Scanning for plugins in directory {:?}", dir);

        let lua = &self.lua;

        let x: Vec<Plugin<'lua>> = ReadDirStream::new(fs::read_dir(&dir).await?)
            .filter_map(|entry| async {
                match entry {
                    Ok(entry) => {
                        // only look at zip files
                        let path = entry.path();
                        if let Some("zip") = path.extension().and_then(OsStr::to_str) {
                            match Self::load_plugin(&path, lua).await {
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
        self.plugins = x;


        Ok(())
    }

    pub async fn load_plugin(path: &Path, lua: &'load Lua) -> Result<Plugin<'load>> {
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

        let bootstrap_code = Self::load_code(&zip, &tree, &lua, &manifest.bootstrap_path)?;
        let init_code = Self::load_code(&zip, &tree, &lua, &manifest.init_path)?;
        info!(
            "Loaded plugin {} v{} from [{}]",
            manifest.name,
            manifest.version,
            file_name_or_unknown(&path)
        );
        Ok(Plugin { manifest, bootstrap_code, init_code })
    }

    fn load_code(zip: &ZipArchive, tree: &DirectoryContents, lua: &'load Lua, path: &String) -> Result<Function<'load>> {
        let metadata = tree.lookup(&path).wrap_err_with(|| {
            format!(
                "Main executable not found at path {}!",
                path
            )
        })?;

        let mut executable = zip.read(metadata)?;
        let mut code = Vec::with_capacity(metadata.size);
        executable.read_to_end(&mut code)?;


        Ok(lua.load(&code).into_function()?)
    }

    pub fn bootstrap(&self) -> Result<()> {
        for Plugin { bootstrap_code, .. } in &self.plugins {
            lua_func!(self.lua => register_tile);

            bootstrap_code.call(())?;
        }
        Ok(())
    }

    pub fn init(&self) -> Result<()> {
        for Plugin { init_code, .. } in &self.plugins {
            lua_func!(self.lua => register_tile);
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
