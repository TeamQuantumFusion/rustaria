use syn::{Attribute, FnArg, ImplItem, ImplItemMethod, ItemImpl};
use std::collections::HashMap;
use proc_macro2::Ident;
use eyre::WrapErr;
use lua_docs::{Class, ClassGenerics, Field, Function, Return};
use lua_docs::ty::Type;
use lua_docs::visibility::Visibility;
use crate::{FieldAttr, FieldBindKind, util};
use crate::util::{get_doc_comments, get_generics, get_type_name};
use std::fmt::Write;
#[derive(Debug)]
pub struct ClassInfo {
	pub doc_comments: Vec<String>,
	pub name: String,
	pub generics: ClassGenerics,
	pub fields: HashMap<String, Field>,
	pub methods: HashMap<String, Function>,
}

impl ClassInfo {
	pub fn new(item: &ItemImpl) -> eyre::Result<ClassInfo> {
		Ok(ClassInfo {
			doc_comments: get_doc_comments(&item.attrs),
			name: get_type_name(&*item.self_ty),
			generics: get_generics(&item.generics),
			fields: HashMap::new(),
			methods: HashMap::new(),
		})
	}

	pub fn export(&self, from_luas: &mut HashMap<String, Type>) -> String {
		let mut string = String::new();

		let name = if let Some(ty) = from_luas.remove(&self.name) {
			let userdata_name = format!("{}UserData", self.name);
			write!(&mut string, "--- @alias {} {} | {}\n\n", self.name, ty, userdata_name).unwrap();
			userdata_name
		} else {
			self.name.clone()
		};
		write!(&mut string, "{}", Class {
			doc_comments: self.doc_comments.clone(),
			name,
			parent: None,
			generics: self.generics.clone(),
			fields: self.fields.clone().into_values().collect(),
			functions: self.methods.clone().into_values().collect(),
			comment: Default::default(),
		}).unwrap();

		string
	}

	pub fn extend(&mut self, item: &ItemImpl) -> eyre::Result<()> {
		if util::attribute_contains(&item.attrs, "lua_impl") {
			for value in get_doc_comments(&item.attrs) {
				self.doc_comments.push(value);
			}
			for item in &item.items {
				if let ImplItem::Method(item) = item {
					for attr in &item.attrs {
						let attribute_name = util::get_path_name(&attr.path);
						if attribute_name == "lua_method" {
							let method = Self::fetch_method(attr, item).wrap_err("Failed to fetch method.")?;
							self.methods.insert(method.name.to_string(), method);
						} else if attribute_name == "lua_field" {
							let field = Self::fetch_field(attr, item).wrap_err("Failed to fetch field.")?;
							self.fields.insert(field.name.to_string(), field);
						}
					}
				}
			}
		}

		Ok(())
	}

	fn fetch_method(attr: &Attribute, item: &ImplItemMethod) -> eyre::Result<Function> {
		let name: Option<Ident> = if !attr.tokens.is_empty() { attr.parse_args()? } else { None };
		let name = name
			.map(|i| i.to_string())
			.unwrap_or_else(|| item.sig.ident.to_string());

		Ok(Function {
			doc_comments: get_doc_comments(&item.attrs),
			name,
			needs_self: item.sig.receiver().is_some(),
			generics: get_generics(&item.sig.generics).into(),
			parameters: util::get_parameters(&item.sig.inputs),
			ret: util::get_return_ty(&item.sig.output).map(|ty| Return {
				ty,
				comment: Default::default(),
			}),
		})
	}

	fn fetch_field(attr: &Attribute, item: &ImplItemMethod) -> eyre::Result<Field> {
		let attr: FieldAttr = attr.parse_args()?;
		let lua_name = attr.lua_name.unwrap_or_else(|| item.sig.ident.to_string());

		let ty = match attr.kind {
			FieldBindKind::Getter => {
				util::get_return_ty(&item.sig.output).expect("Getter needs to return")
			}
			FieldBindKind::Setter => {
				if let FnArg::Typed(ty) = item.sig.inputs.last().unwrap() {
					util::to_lua_type(&*ty.ty)
				} else {
					panic!("Setter needs a parameter")
				}
			}
		};

		Ok(Field {
			doc_comments: get_doc_comments(&item.attrs),
			name: lua_name,
			vis: Visibility::None,
			ty,
			comment: Default::default(),
		})
	}
}
