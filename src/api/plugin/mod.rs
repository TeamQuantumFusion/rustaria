use std::{collections::HashMap, path::PathBuf};

use anyways::{ext::AuditExt, Result};
use semver::{Version, VersionReq};
use apollo::Lua;

use crate::{
	api::{plugin::archive::Archive},
	ty::identifier::Identifier,
};

mod archive;

pub struct Plugin {
	pub id: String,
	pub manifest: Manifest,
	pub archive: Archive,
}

impl Plugin {
	#[cfg(feature = "testing")]
	pub fn test(entrypoint: String) -> Plugin {
		Self::new_test("test", HashMap::from(
			[
				("src/main.lua".to_string(), entrypoint.into_bytes())
			]
		))
	}

	#[cfg(feature = "testing")]
	pub fn new_test(id: impl ToString, files: HashMap<String, Vec<u8>>) -> Plugin {
		Plugin {
			id: id.to_string(),
			manifest: Manifest {
				plugin: ManifestPlugin {
					name: id.to_string(),
					id: id.to_string(),
					version: Version::new(0, 0, 0),
					license: None,
					author: None,
					contributors: None,
				},
				dependencies: None,
				breaks: None,
				supports: None,
			},
			archive: Archive::Direct(files),
		}
	}

	pub fn new(path: &PathBuf) -> Result<Plugin> {
		let archive = Archive::new(path)?;
		let data = archive
			.get("manifest.toml")
			.wrap_err("Could not find manifest.toml")?;
		let manifest: Manifest =
			toml::from_slice(&data).wrap_err("Could not parse manifest.toml")?;

		Ok(Plugin {
			id: manifest.plugin.id.clone(),
			manifest,
			archive,
		})
	}

	pub fn reload(&self, lua: &Lua) -> Result<()> {
		let identifier = Identifier::new("main.lua");
		let data = self.archive.get("src/main.lua").wrap_err(format!(
			"Could not find entrypoint \"main.lua\" for plugin {}",
			self.id
		))?;

		lua.load(&data).set_name(format!("{identifier}"))
			.wrap_err("Failed to load lua file")?
			.exec()
			.wrap_err("Failure while executing entrypoint")?;
		Ok(())
	}
}

#[derive(serde::Deserialize, PartialEq, Debug)]
pub struct Manifest {
	// core properties
	pub plugin: ManifestPlugin,

	// metadata
	pub dependencies: Option<HashMap<String, VersionReq>>,
	pub breaks: Option<HashMap<String, VersionReq>>,
	pub supports: Option<HashMap<String, VersionReq>>,
}

#[derive(serde::Deserialize, PartialEq, Debug)]
pub struct ManifestPlugin {
	pub name: String,
	pub id: String,
	pub version: Version,

	pub license: Option<String>,
	pub author: Option<String>,
	pub contributors: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_manifest() -> anyways::Result<()> {
		let manifest: Manifest = toml::from_str(
			r#"
	    [plugin]
		name = "Rustaria"
		id = "rustaria"
		version = "0.0.1"

		[dependencies]
		rustaria-graphics = "0.0.69"

	    "#,
		)?;

		assert_eq!(
			manifest,
			Manifest {
				plugin: ManifestPlugin {
					name: "Rustaria".to_string(),
					id: "rustaria".to_string(),
					version: Version::new(0, 0, 1),
					license: None,
					author: None,
					contributors: None,
				},
				dependencies: Some(HashMap::from([(
					"rustaria-graphics".to_string(),
					VersionReq::parse("0.0.69").unwrap()
				)])),
				breaks: None,
				supports: None,
			}
		);
		Ok(())
	}
}
