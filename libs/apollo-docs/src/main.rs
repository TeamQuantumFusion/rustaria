pub mod attr;
pub mod class;
pub mod util;
mod index;
mod from_lua;

pub struct Object {
	types: Vec<lua_docs::ty::Type>,
}

use std::{
	collections::{hash_map::Entry, HashMap, VecDeque},
	fs,
	fs::read_to_string,
	path::PathBuf,
	str::FromStr,
};

use attr::{FieldAttr, FieldBindKind};
use class::ClassInfo;
use eyre::WrapErr;
use lua_docs::ty::Type;
use proc_macro2::Ident;
use syn::{ImplItem, Item};
use crate::index::Index;

pub fn main() {
	let mut queue = VecDeque::new();
	let mut index = Index::default();

	queue.push_front(PathBuf::from_str("./src/").unwrap());
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
							if let Item::Impl(item) = item {
								index.consume(item).unwrap();
							}
						}
					}
				}
			}
		}
	}

	index.export();
}
