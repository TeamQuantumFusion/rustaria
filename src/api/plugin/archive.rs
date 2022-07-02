use std::{fs::File, io, io::Read, path::PathBuf};

use anyways::{audit::Audit, ext::AuditExt, Result};
use parking_lot::Mutex;
use zip::ZipArchive;

pub enum Archive {
	Zip(Mutex<ZipArchive<File>>),
	Directory(PathBuf),
}

impl Archive {
	pub fn new(path: &PathBuf) -> Result<Archive> {
		if path.is_file() {
			if let Some(extension) = path.extension() {
				let extension = extension
					.to_str()
					.ok_or("Could not convert extension to UTF-8")?;
				if extension == "zip" {
					let file = File::open(path).wrap_err("Could not open zip file.")?;
					let zip_archive =
						ZipArchive::new(file).wrap_err("Could not create ZipArchive")?;
					return Ok(Archive::Zip(Mutex::new(zip_archive)));
				}
			}
		} else if path.is_dir() {
			return Ok(Archive::Directory(path.clone()));
		}

		return Err(Audit::new(
			"Could not determine what kind of plugin archive to open.",
		));
	}

	pub fn get(&self, location: &str) -> io::Result<Vec<u8>> {
		match self {
			Archive::Zip(zip_archive) => {
				let mut guard = zip_archive.lock();
				let mut file = guard.by_name(location)?;
				let mut out = Vec::new();
				file.read_to_end(&mut out)?;
				Ok(out)
			}
			Archive::Directory(directory) => std::fs::read(directory.join(location)),
		}
	}
}
