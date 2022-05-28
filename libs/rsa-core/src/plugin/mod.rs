use std::path::PathBuf;
use semver::Version;
use std::collections::HashMap;
use mlua::Lua;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use crate::api::Api;
use crate::api::lua::error::LuaError;
use crate::api::lua::new_lua_state;
use crate::plugin::archive::Archive;
use crate::ty::PluginId;

pub mod archive;

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
	pub fn new(path: &PathBuf, api: &Api) -> eyre::Result<Plugin, PluginLoadError> {
		let archive = Archive::new(path)?;
		let manifest_binary = archive.get_asset("manifest.json")?;
		let manifest: Manifest = serde_json::from_slice(&manifest_binary)?;

		Ok(Plugin {
			lua_state: new_lua_state(&manifest, api)?,
			manifest,
			archive,
		})
	}

	pub fn reload(&self) -> eyre::Result<()> {
		if let Some(entry) = &self.manifest.common_entry {
			let lua_file = self.archive.get_asset(&("./src/".to_owned() + entry))?;
			self.lua_state.load(&lua_file).set_name(format!("{}:{}", self.manifest.id, entry))?.exec().lua_err()?;
		}
		Ok(())
	}
}

#[cfg(any(feature="test-utils", test))]
impl Plugin {
	pub fn new_test(name: &str, archive: Archive, api: &Api) -> Plugin {
		let manifest = Manifest {
			id: name.to_string(),
			name: name.to_string(),
			version: Version::new(0, 0, 0),
			common_entry: Some("entry.lua".to_string()),
			dependencies: Default::default(),
			incompatibilities: Default::default(),
			recommendations: Default::default()
		};
		Plugin {
			lua_state: new_lua_state(&manifest, api).unwrap(),
			manifest,
			archive,
		}
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
