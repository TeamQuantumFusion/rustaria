use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{DirEntry, File},
    io::Read,
    ops::Deref,
    path::{Path, PathBuf},
};

use eyre::{bail, eyre, Result};
use serde::Deserialize;
use tracing::{info, warn};
use zip::ZipArchive;

#[derive(Debug, Clone, Default)]
pub struct Plugins(HashMap<String, Plugin>);

impl Plugins {
    pub fn load(plugins_dir: &Path) -> Result<Self> {
        let plugins = match std::fs::read_dir(&plugins_dir) {
            Ok(read_dir) => {
                read_dir.filter_map(|entry| {
                    match entry {
                        Ok(entry) => Self::process_file(entry),
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
        Ok(Plugins(HashMap::from_iter(plugins.into_iter())))
    }

    fn process_file(entry: DirEntry) -> Option<(String, Plugin)> {
        let path = entry.path();

        // only look at zip files
        if let Some("zip") = path.extension().and_then(OsStr::to_str) {
            match Self::load_plugin(&path) {
                Ok(plugin) => return Some((plugin.manifest.plugin_id.clone(), plugin)),
                Err(e) => warn!(
                    "Error loading plugin [{}]: {e}",
                    file_name_or_unknown(&path)
                ),
            }
        }
        None
    }

    fn load_plugin(path: &Path) -> Result<Plugin> {
        let mut archive = PluginArchive::new(path)?;
        archive.enable_reading()?;

        let data = archive.get_asset(&ArchivePath::Manifest)?;
        let manifest: Manifest = serde_json::from_reader(data.as_slice())?;

        info!(
            "Loaded plugin {} v{} from [{}]",
            manifest.plugin_id,
            manifest.version,
            file_name_or_unknown(path)
        );
        Ok(Plugin { archive, manifest })
    }
}

impl Deref for Plugins {
    type Target = HashMap<String, Plugin>;

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
pub struct Plugin {
    pub archive: PluginArchive,
    pub manifest: Manifest,
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
    Asset(String),
    Src(String),
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
                    let path: PathBuf = components.collect();

                    let mut file_data = Vec::with_capacity(file.size() as usize);
                    file.read_to_end(&mut file_data)?;

                    data.insert(
                        match option.as_os_str().to_str().unwrap() {
                            "src" => ArchivePath::Src(path.as_os_str().to_str().unwrap().to_string()),
                            "asset" => ArchivePath::Asset(path.as_os_str().to_str().unwrap().to_string()),
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
        match &self.data {
            None => bail!("Reading not active"),
            Some(files) => files.get(path).ok_or_else(|| eyre!("Could not find file")),
        }
    }
}
