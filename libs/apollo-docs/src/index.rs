use std::{collections::HashMap, fmt::Write, fs};

use lua_docs::ty::Type;
use proc_macro2::TokenStream;
use syn::{Attribute, ImplItem, ItemImpl};

use crate::{
	util::{attribute_contains, get_attribute, get_path_name, get_type_name},
	ClassInfo,
};

#[derive(Default)]
pub struct Index {
	userdata: HashMap<String, ClassInfo>,
	from_luas: HashMap<String, Type>,
}

impl Index {
	pub fn consume(&mut self, impl_item: &ItemImpl) -> eyre::Result<()> {
		if attribute_contains(&impl_item.attrs, "lua_impl") {
			let name = get_type_name(&*impl_item.self_ty);
			if !self.userdata.contains_key(&name) {
				self.userdata
					.insert(name.clone(), ClassInfo::new(impl_item)?);
			}

			self.userdata.get_mut(&name).unwrap().extend(impl_item)?;

			for item in &impl_item.items {
				if let ImplItem::Method(item) = item {
					if let Some(attr) = get_attribute(&item.attrs, "from_lua") {
						if let Some(lua) = self.scan_from_lua(attr) {
							self.from_luas.insert(name.clone(), lua);
						} else {
							println!("{name} does not have a #[from_lua] type setter.")
						}
					}
				}
			}
		} else if let Some((_, path, _)) = &impl_item.trait_ {
			if get_path_name(path) == "FromLua" {
				for item in &impl_item.items {
					if let ImplItem::Method(item) = item {
						let name = get_type_name(&*impl_item.self_ty);
						if let Some(attr) = get_attribute(&item.attrs, "from_lua") {
							if let Some(lua) = self.scan_from_lua(attr) {
								self.from_luas.insert(name, lua);
								return Ok(());
							}
						}
						println!("{name} does not have a #[from_lua] type setter.")
					}
				}
			}
		}

		Ok(())
	}

	fn scan_from_lua(&mut self, attr: &Attribute) -> Option<Type> {
		if !attr.tokens.is_empty() {
			let ty: TokenStream = attr.parse_args().unwrap();
			if !ty.is_empty() {
				return Some(Type::Custom {
					name: ty.to_string(),
					generics: vec![],
				});
			}
		};

		None
	}

	pub fn export(mut self) {
		for (_, info) in self.userdata {
			fs::write(
				format!("./docs/{}.lua", info.name),
				info.export(&mut self.from_luas),
			)
			.unwrap();
		}

		let mut out = String::new();
		for (name, ty) in self.from_luas {
			writeln!(&mut out, "--- @alias {name} {ty}").unwrap();
		}

		fs::write("./docs/Util.lua".to_string(), out).unwrap();
	}
}
