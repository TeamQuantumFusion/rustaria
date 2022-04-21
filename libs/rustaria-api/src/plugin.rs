use crate::lua::new_lua;
use crate::{archive::Archive, Api, Reloadable};
use crate::{lua, PluginId};
use mlua::Lua;
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
	pub common_entry: Option<String>,
	// Other plugins
	#[serde(default)]
	pub dependencies: HashMap<PluginId, Version>,
	#[serde(default)]
	pub incompatibilities: HashMap<PluginId, Version>,
	#[serde(default)]
	pub recommendations: HashMap<PluginId, Version>,
}

pub struct Plugin {
	pub manifest: Manifest,
	pub archive: Archive,
	pub lua_state: Lua,
}

impl Plugin {
	pub fn new(path: &PathBuf, api: &Api) -> Result<Plugin, PluginLoadError> {
		let archive = Archive::new(path)?;
		let manifest_binary = archive.get_asset("manifest.json")?;
		let manifest: Manifest = serde_json::from_slice(&manifest_binary)?;

		Ok(Plugin {
			lua_state: new_lua(&manifest, api)?,
			manifest,
			archive,
		})
	}

	pub fn reload(&self) -> eyre::Result<()> {
		if let Some(entry) = &self.manifest.common_entry {
			let lua_file = self.archive.get_asset(&("./src/".to_owned() + entry))?;
			self.lua_state.load(&lua_file).exec()?;
		}
		Ok(())
	}
}

#[derive(Error, Debug)]
pub enum PluginLoadError {
	#[error("Archive reading error `{0}`")]
	Io(#[from] std::io::Error),
	#[error("Manifest parsing error `{0}`")]
	ManifestParsing(#[from] serde_json::Error),
	#[error("Failed to initialize lua context `{0}`")]
	LuaInitialization(#[from] mlua::Error),
}

/// Used in every lua context. global "ctx"
#[derive(Clone, Serialize, Deserialize)]
pub struct PluginContext {
	pub id: String,
}
