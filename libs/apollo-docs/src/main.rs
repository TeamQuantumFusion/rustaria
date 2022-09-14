use std::{collections::VecDeque, fs, fs::read_to_string, path::PathBuf, str::FromStr};

use attr::{FieldAttr, FieldBindKind};
use class::ClassInfo;
use syn::Item;

use crate::index::Index;

pub mod attr;
pub mod class;
mod index;
pub mod util;

pub struct Object {
	types: Vec<lua_docs::ty::Type>,
}

pub fn main() {
	let mut queue = VecDeque::new();
	let mut index = Index::default();

	queue.push_front(PathBuf::from_str("./src/").unwrap());
	queue.push_front(PathBuf::from_str("./libs/rsa-core").unwrap());
	queue.push_front(PathBuf::from_str("./libs/rsa-hash").unwrap());
	queue.push_front(PathBuf::from_str("./libs/rsa-item").unwrap());
	queue.push_front(PathBuf::from_str("./libs/rsa-network").unwrap());
	queue.push_front(PathBuf::from_str("./libs/rsa-player").unwrap());
	queue.push_front(PathBuf::from_str("./libs/rsa-registry").unwrap());
	queue.push_front(PathBuf::from_str("./libs/rsa-world").unwrap());
	queue.push_front(PathBuf::from_str("./client/src/").unwrap());
	while let Some(path) = queue.pop_back() {
		for entry in fs::read_dir(path).unwrap().flatten() {
			let path = entry.path();
			if path.is_dir() {
				queue.push_back(path);
			} else if path.is_file() {
				if let Some(extention) = path.extension() {
					if extention.to_str().unwrap() == "rs" {
						let file: syn::File =
							syn::parse_str(&read_to_string(path).unwrap()).unwrap();

						for item in &file.items {
							index.consume(item).unwrap();
						}
					}
				}
			}
		}
	}

	index.export();
}
