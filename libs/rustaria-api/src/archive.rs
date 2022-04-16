use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
    sync::RwLock,
};

use zip::ZipArchive;

pub enum Archive {
    Directory(PathBuf),
    Zip(RwLock<ZipArchive<File>>),
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
                let mut file = zip.by_name(&path)?;
                let mut data = Vec::with_capacity(file.size() as usize);
                file.read_to_end(&mut data)?;
                Ok(data)
            }
        }
    }
}
