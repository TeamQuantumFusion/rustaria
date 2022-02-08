extern crate gl_generator;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

fn main() {
    let path = Path::new(&"./src/gl.rs");
    if !path.exists() {
        let mut file = File::create(&path).unwrap();

        let mut bindings = Vec::new();
        Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, ["ARB_compute_shader", "ARB_shader_image_load_store"])
            .write_bindings(GlobalGenerator, &mut bindings)
            .unwrap();


        file.write_all(bindings.as_ref()).unwrap();
    }
}
