use std::path::{PathBuf};

use archive::Archive;
use manifest::Manifest;
use rustaria_util::{Context, Result};
use crate::plugin::archive::ArchivePath;


pub mod archive;
pub mod manifest;
pub mod id;

pub struct Plugin {
    pub manifest: Manifest,
    pub archive: Archive,
}

impl Plugin {
    pub fn new(path: PathBuf) -> Result<Plugin> {
        let archive = Archive::new(path)?;
        let manifest_binary = archive
            .get_asset(&ArchivePath::Manifest)
            .wrap_err("Could not find manifest.")?;
        let manifest =
            serde_json::from_slice(&*manifest_binary).wrap_err("Manifest format invalid.")?;
        Ok(Plugin { manifest, archive })
    }
}
