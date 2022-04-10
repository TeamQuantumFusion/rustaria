use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;
use zip::ZipArchive;

use rustaria_util::{bail, eyre, Result};

#[derive(Debug, Clone)]
pub struct Archive {
    path: PathBuf,
    data: Option<HashMap<ArchivePath, Vec<u8>>>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize)]
pub enum ArchivePath {
    Asset(String),
    Code(String),
    Manifest,
}

impl Archive {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut archive = Self { path, data: None };

        archive.enable_reading()?;
        Ok(archive)
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
                            "src" => {
                                ArchivePath::Code(path.as_os_str().to_str().unwrap().to_string())
                            }
                            "asset" => {
                                ArchivePath::Asset(path.as_os_str().to_str().unwrap().to_string())
                            }
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
            Some(files) => {
                files.get(path).ok_or_else(|| eyre!("Could not find file"))
            },
        }
    }
}
