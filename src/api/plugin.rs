use eyre::{bail, eyre, Result};
use mlua::prelude::*;
use serde::Deserialize;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{DirEntry, File},
    io::Read,
    ops::Deref,
    path::{Path, PathBuf},
};
use tracing::{info, warn};
use zip::ZipArchive;

#[derive(Debug, Clone, Default)]
pub struct Plugins<'lua>(Vec<Plugin<'lua>>);

impl<'lua> Plugins<'lua> {
    pub fn load(plugins_dir: &Path, lua: &'lua Lua) -> Result<Self> {
        let plugins = match std::fs::read_dir(&plugins_dir) {
            Ok(read_dir) => {
                read_dir.filter_map(|entry| {
                    match entry {
                        Ok(entry) => Self::process_file(entry, lua),
                        Err(e) => {
                            warn!("Unable to access file `{e}` for reading! Permissions are perhaps insufficient!");
                            None
                        }
                    }
                }).collect()
            }
            Err(e) => {
                warn!("Plugin directory not found! {e}");
                warn!("Creating an empty plugin directory...");
                std::fs::create_dir_all("plugins")?;
                vec![]
            }
        };
        info!("Found and loaded {} plugin(s)", plugins.len());
        Ok(Plugins(plugins))
    }

    fn process_file(entry: DirEntry, lua: &'lua Lua) -> Option<Plugin<'lua>> {
        let path = entry.path();

        // only look at zip files
        if let Some("zip") = path.extension().and_then(OsStr::to_str) {
            match Self::load_plugin(&path, lua) {
                Ok(plugin) => return Some(plugin),
                Err(e) => warn!(
                    "Error loading plugin [{}]: {e}",
                    file_name_or_unknown(&path)
                ),
            }
        }
        None
    }

    fn load_plugin(path: &Path, lua: &'lua Lua) -> Result<Plugin<'lua>> {
        let mut archive = PluginArchive::new(path)?;
        archive.enable_reading()?;

        let data = archive.get_asset(&ArchivePath::Manifest)?;
        let manifest: Manifest = serde_json::from_reader(data.as_slice())?;

        let source =
            archive.get_asset(&ArchivePath::Src(PathBuf::from(manifest.init_path.clone())))?;
        let init = lua.load(source).into_function()?;
        info!(
            "Loaded plugin {} v{} from [{}]",
            manifest.plugin_id,
            manifest.version,
            file_name_or_unknown(path)
        );
        Ok(Plugin {
            archive,
            manifest,
            init,
        })
    }
}

impl<'lua> Deref for Plugins<'lua> {
    type Target = Vec<Plugin<'lua>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn file_name_or_unknown(path: &Path) -> &str {
    path.file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("<unknown>")
}
#[derive(Debug, Clone)]
pub struct Plugin<'lua> {
    pub archive: PluginArchive,
    pub manifest: Manifest,
    pub init: LuaFunction<'lua>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub plugin_id: String,
    pub version: String,
    pub init_path: String,
}
#[derive(Debug, Clone)]
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
    pub fn new(path: &Path) -> Result<Self> {
        Ok(Self {
            path: PathBuf::from(path),
            data: None,
        })
    }

    pub fn enable_reading(&mut self) -> Result<()> {
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
                    file.read_to_end(&mut file_data)?;

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

    pub fn get_asset(&self, path: &ArchivePath) -> Result<&Vec<u8>> {
        let option = &self.data;
        match option {
            None => bail!("Reading not active"),
            Some(files) => files.get(path).ok_or_else(|| eyre!("Could not find file")),
        }
    }
}
