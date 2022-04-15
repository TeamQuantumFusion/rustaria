use crate::{archive::Archive, lua::register_preload, ty::PluginId};
use mlua::{Lua, UserData};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub id: PluginId,
    pub name: String,
    pub version: Version,
    // Entry points
    pub common_pre_entry: Option<String>,
    pub common_entry: Option<String>,
    pub client_pre_entry: Option<String>,
    pub client_entry: Option<String>,
    // Other plugins
    #[serde(default)]
    pub dependencies: HashMap<PluginId, Version>,
    #[serde(default)]
    pub incompatibilities: HashMap<PluginId, Version>,
    #[serde(default)]
    pub recommendations: HashMap<PluginId, Version>,
}

pub struct Plugin {
    pub lua: Lua,
    pub manifest: Manifest,
    pub archive: Archive,
}

impl Plugin {
    pub fn new(path: &PathBuf) -> Result<Plugin, PluginLoadError> {
        let archive = Archive::new(path)?;
        let manifest_binary = archive.get_asset("manifest.json")?;
        let manifest: Manifest = serde_json::from_slice(&*manifest_binary)?;

        let lua = Lua::new();
        register_preload(&lua)?;
        lua.globals().set("ctx", PluginContext { id: manifest.id.clone() })?;

        Ok(Plugin {
            manifest,
            archive,
            lua,
        })
    }
}

#[derive(Error, Debug)]
pub enum PluginLoadError {
    #[error("Archive reading error `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Manifest parsing error `{0}`")]
    ManifestParsing(#[from] serde_json::Error),
    #[error("Initializing Lua error `{0}`")]
    Lua(#[from] mlua::Error),
}

/// Used in every lua context. global "ctx"
#[derive(Clone, Serialize, Deserialize)]
pub struct PluginContext {
    pub id: String,
}

impl UserData for PluginContext {}
