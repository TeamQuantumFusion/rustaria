use std::path::PathBuf;

fn main() {
	println!("cargo:rerun-if-changed=git_hooks/*");

	let hooks_path = PathBuf::from(".git/hooks/");
	std::fs::create_dir_all(&hooks_path).unwrap();

	for file in std::fs::read_dir("./git_hooks").unwrap() {
		let file = file.unwrap();
		let path = file.path();
		let new_path = hooks_path.join(&path.file_name().unwrap());

		std::fs::copy(path, new_path).unwrap();
	}
}
