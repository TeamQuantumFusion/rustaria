use shaderc::ShaderKind;

fn main() {
    println!("cargo:rerun-if-changed=src/shader");

    let mut compiler = shaderc::Compiler::new().unwrap();

    std::fs::create_dir_all("run/shaders").unwrap();
    for entry in std::fs::read_dir("src/shader").unwrap() {
        let entry = entry.unwrap();

        if entry.file_type().unwrap().is_file() {
            let in_path = entry.path();

            // Support only vertex and fragment shaders currently
            let shader_type = in_path.file_name().and_then(|ext| {
                let str = ext.to_str().unwrap();
                if str.ends_with("-fs.glsl") {
                    Some(ShaderKind::Fragment)
                } else if str.ends_with("-vs.glsl") {
                    Some(ShaderKind::Vertex)
                } else {
                    None
                }
            });
            if let Some(shader_kind) = shader_type {
                let source = std::fs::read_to_string(&in_path).unwrap();
                let binary_result = compiler
                    .compile_into_spirv(
                        &source,
                        shader_kind,
                        in_path.file_name().unwrap().to_str().unwrap(),
                        "main",
                        None,
                    )
                    .unwrap();

                let out_path = format!(
                    "run/shaders/{}.spv",
                    in_path.file_name().unwrap().to_string_lossy()
                );

                std::fs::write(&out_path, &binary_result.as_binary_u8()).unwrap();
            }
        }
    }
}
