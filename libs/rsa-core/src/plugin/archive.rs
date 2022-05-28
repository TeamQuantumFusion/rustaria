use std::{
	fs::File,
	io::{self, Read},
	path::PathBuf,
	sync::RwLock,
};
use std::collections::HashMap;
use std::io::ErrorKind;

use zip::ZipArchive;

pub enum Archive {
	Directory(PathBuf),
	Zip(RwLock<ZipArchive<File>>),
	Direct(RwLock<HashMap<String, Vec<u8>>>),
}

impl Archive {
	pub fn new(path: &PathBuf) -> io::Result<Archive> {
		if path.is_dir() {
			Ok(Archive::Directory(path.clone()))
		} else {
			let file = File::open(&path)?;
			let zip = ZipArchive::new(file)?;
			Ok(Archive::Zip(RwLock::new(zip)))
		}
	}

	pub fn new_direct(data: HashMap<String, Vec<u8>>) -> io::Result<Archive> {
		Ok(Archive::Direct(RwLock::new(data)))
	}

	pub fn get_asset(&self, path: &str) -> io::Result<Vec<u8>> {
		match self {
			Archive::Directory(directory) => {
				let mut file_path = directory.clone();
				for ele in path.split('/') {
					file_path.push(ele);
				}

				let mut file = File::open(file_path)?;
				let mut data = file.metadata().map_or(Vec::new(), |metadata| {
					Vec::with_capacity(metadata.len() as usize)
				});

				file.read_to_end(&mut data)?;
				Ok(data)
			}
			Archive::Zip(zip) => {
				let mut zip = zip.write().unwrap();
				let mut file = zip.by_name(path)?;
				let mut data = Vec::with_capacity(file.size() as usize);
				file.read_to_end(&mut data)?;
				Ok(data)
			}
			Archive::Direct(values) => {
				let guard = values.write().unwrap();
				let option = guard.get(path).ok_or(ErrorKind::NotFound)?;
				Ok(option.clone())
			}
		}
	}
}

#[cfg(any(feature="test-utils", test))]
pub struct  TestAsset {
	path: String,
	data: Vec<u8>
}

#[cfg(any(feature="test-utils", test))]
impl TestAsset {
	pub fn lua(name: &str, code: &str) -> TestAsset {
		TestAsset {
			path: "./src/".to_string() + name + ".lua",
			data: code.as_bytes().to_vec()
		}
	}
}

#[cfg(any(feature="test-utils", test))]
impl Archive {
	pub fn new_test(assets: Vec<TestAsset>) -> Archive {
		let mut data = HashMap::new();
		for asset in assets {
			data.insert(asset.path, asset.data);
		}

		Archive::new_direct(data).unwrap()
	}
}