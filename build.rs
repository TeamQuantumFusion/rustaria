use std::{
    fs::{self, File},
    io::{self, BufWriter},
    path::PathBuf
};

use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

fn main() {
    println!("cargo:rerun-if-changed=rustaria-core/");
    fs::create_dir_all("run/plugins")
        .expect("Could not create plugins directory; check if your permissions are sufficient");
    let zip = File::create("run/plugins/rustaria-core.zip").expect("Could not create plugin file");
    let mut zip = ZipWriter::new(BufWriter::new(zip));

    let core_path = PathBuf::from("rustaria-core");

    for entry in WalkDir::new(&core_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .skip(1)
    {
        let path = entry.path();
        println!("Adding file [{}]", path.display());
        let mut file = File::open(path).expect("Could not open entry for reading");
        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

        let path = path.strip_prefix(&core_path).unwrap().to_string_lossy();
        let file_type = entry.file_type();

        if file_type.is_dir() {
            zip.add_directory(path, options).unwrap();
        } else if file_type.is_file() {
            zip.start_file(path, options)
                .expect("Could not start file for writing");
            io::copy(&mut file, &mut zip).expect("Failed to add and compress file");
        }
    }
}
