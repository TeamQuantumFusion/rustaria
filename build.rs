//! Automated buildscript for building and bundling `rustaria-core`.
//!
//! This, in a nutshell, simply bundles the `rustaria-core` folder into a .zip file,
//! and puts said .zip file into the `run/plugins/` folder where developers
//! could test the engine with the core plugin installed.
//!
//! Still, this is very, very cursed. Proceed with caution.
use std::{
    fs::{self, File},
    io::{self, BufWriter},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

fn main() {
    println!("cargo:rerun-if-changed=plugin/");

   // pack_core_plugins(&[Path::new("runtime/client/run/plugins")])
}

fn pack_core_plugins(paths: &[&Path]) {
    for path in paths {
        fs::create_dir_all(path)
            .expect("Could not create plugins directory; check if your permissions are sufficient");
    }

    let mut zips: Vec<_> = paths
        .iter()
        .map(|path| {
            let path = path.join("rustaria-core.zip");
            let zip = File::create(path).expect("Could not create plugin file");
            ZipWriter::new(BufWriter::new(zip))
        })
        .collect();

    let core_path = PathBuf::from("plugin");

    for entry in WalkDir::new(&core_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .skip(1)
    // skip root directory
    {
        let file_path = entry.path();
        println!("Adding file [{}]", file_path.display());
        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

        let path = file_path
            .strip_prefix(&core_path)
            .unwrap()
            .to_string_lossy();
        let file_type = entry.file_type();

        if file_type.is_dir() {
            for zip in &mut zips {
                zip.add_directory(path.clone(), options)
                    .expect("Failed to create subdirectory");
            }
        } else if file_type.is_file() {
            for zip in &mut zips {
                zip.start_file(path.clone(), options)
                    .expect("Could not start file for writing");
                let mut file = File::open(file_path).expect("Could not open entry for reading");
                io::copy(&mut file, zip).expect("Failed to add and compress file");
            }
        }
    }
}
