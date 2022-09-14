use std::{collections::HashMap, fmt::Write, fs};

use lua_docs::ty::Type;
use proc_macro2::TokenStream;
use syn::{Attribute, ImplItem, Item, ItemImpl, ItemStruct};
use lua_docs::Field;
use lua_docs::visibility::Visibility;

use crate::{
	util::{attribute_contains, get_attribute, get_path_name, get_type_name},
	ClassInfo,
};
use crate::util::{derive_contains, get_doc_comments, to_lua_type};

#[derive(Default)]
pub struct Index {
	userdata: HashMap<String, ClassInfo>,
	from_luas: HashMap<Type, Type>,
}

impl Index {
	pub fn consume(&mut self, impl_item: &Item) -> anyways::Result<()> {
		match impl_item {
			Item::Enum(_) => Ok(()),
			Item::Impl(impl_item) => self.consume_impl(impl_item),
			Item::Struct(item) => self.consume_struct(item),
			_ => {
				Ok(())
			}
		}
	}

	fn consume_struct(&mut self, item: &ItemStruct)  -> anyways::Result<()> {
		if derive_contains(&item.attrs, "FromLua") {
			let name = item.ident.to_string();
			let class = self.add_class(&name, || {
				ClassInfo::new(&item.attrs, Type::Custom  {
					name,
					generics: vec![]
				}, &item.generics)
			})?;

			for field in &item.fields {
				let name = field.ident.as_ref().expect("unnamed fields are not supported").to_string();
				class.fields.insert(name.clone(), Field {
					doc_comments: get_doc_comments(&field.attrs),
					name,
					vis: Visibility::Public,
					ty: to_lua_type(&field.ty),
					comment: Default::default()
				});
			}
		}

		Ok(())
	}

	fn consume_impl(&mut self, impl_item: &ItemImpl) -> anyways::Result<()> {
		if attribute_contains(&impl_item.attrs, "lua_impl") {
			let name = get_type_name(&*impl_item.self_ty);
			let class = self.add_class(&name, || {
				ClassInfo::new(&impl_item.attrs, to_lua_type(&impl_item.self_ty), &impl_item.generics)
			})?;

			class.extend(impl_item)?;
			for item in &impl_item.items {
				if let ImplItem::Method(item) = item {
					if let Some(attr) = get_attribute(&item.attrs, "from_lua") {
						if let Some(lua) = self.scan_from_lua(attr) {
							self.from_luas.insert(to_lua_type(&impl_item.self_ty), lua);
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
						for ty in get_doc_comments(&item.attrs) {
							self.from_luas.insert(to_lua_type(&impl_item.self_ty), Type::Custom { name: ty, generics: vec![] });
							return Ok(());
						}
						println!("{name} does not have a #[from_lua] type setter.")
					}
				}
			}
		}

		Ok(())
	}

	fn add_class(&mut self, name: &str, func: impl FnOnce() -> anyways::Result<ClassInfo>) -> anyways::Result<&mut ClassInfo> {
		if !self.userdata.contains_key(name) {
			self.userdata
				.insert(name.to_string(), func()?);
		}

		Ok(self.userdata.get_mut(name).unwrap())
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
