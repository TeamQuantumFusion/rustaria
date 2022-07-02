use lua_docs::{ty::Type as LuaType, ClassGenerics, Parameter};
use quote::ToTokens;
use syn::{
	punctuated::Punctuated, Attribute, FnArg, GenericArgument, GenericParam, Lit, Meta,
	MetaNameValue, Pat, PathArguments, ReturnType, Token, Type,
};

pub fn attribute_contains(attrs: &Vec<Attribute>, string: &str) -> bool {
	for attr in attrs {
		if attr.path.segments.last().unwrap().ident == string {
			return true;
		}
	}

	false
}

pub fn get_attribute<'a>(attrs: &'a Vec<Attribute>, name: &str) -> Option<&'a Attribute> {
	for attr in attrs {
		if attr.path.segments.last().unwrap().ident == name {
			return Some(attr);
		}
	}

	None
}

pub fn to_lua_type(ty: &Type) -> LuaType {
	match ty {
		Type::Array(arr) => LuaType::Array {
			element: Box::new(to_lua_type(&*arr.elem)),
		},
		Type::Never(_) => LuaType::Nil,
		Type::Path(path) => {
			let segment = path.path.segments.last().unwrap();
			let string = segment.ident.to_string();
			match string.as_str() {
				"bool" => {
					return LuaType::Boolean;
				}
				"Value" => {
					return LuaType::Any;
				}
				"Self" => {
					return LuaType::This;
				}
				"i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "f32" | "i64" | "u64" | "f64"
				| "i128" | "u128" => {
					return LuaType::Number;
				}
				"Table" => {
					return LuaType::Table {
						key: Box::new(LuaType::Any),
						value: Box::new(LuaType::Any),
					};
				}
				"String" => {
					return LuaType::String;
				}
				"LuaResult" | "Result" => {
					if let PathArguments::AngleBracketed(angle) = &segment.arguments {
						if let GenericArgument::Type(ty) = angle.args.first().unwrap() {
							return to_lua_type(ty);
						}
					}
				}
				"Option" => {
					if let PathArguments::AngleBracketed(angle) = &segment.arguments {
						if let GenericArgument::Type(ty) = angle.args.first().unwrap() {
							return LuaType::Union(vec![LuaType::Nil, to_lua_type(ty)]);
						}
					}
				}
				"HashMap" => {
					if let PathArguments::AngleBracketed(angle) = &segment.arguments {
						let key = if let GenericArgument::Type(ty) = angle.args.first().unwrap() {
							ty
						} else {
							panic!("key where")
						};
						let value = if let GenericArgument::Type(ty) = angle.args.last().unwrap() {
							ty
						} else {
							panic!("value where")
						};

						return LuaType::Union(vec![LuaType::Table {
							key: Box::new(to_lua_type(key)),
							value: Box::new(to_lua_type(value)),
						}]);
					}
				}
				_ => {}
			}

			let mut generics = vec![];
			if let PathArguments::AngleBracketed(angle) = &segment.arguments {
				for arg in &angle.args {
					if let GenericArgument::Type(ty) = arg {
						generics.push(to_lua_type(ty));
					}
				}
			}
			LuaType::Custom {
				name: string,
				generics,
			}
		}
		Type::Tuple(tuple) => {
			if tuple.elems.is_empty() {
				LuaType::Nil
			} else if tuple.elems.len() == 1 {
				to_lua_type(&tuple.elems[0])
			} else {
				panic!("Cannot convert tuple")
			}
		}
		Type::Paren(paren) => to_lua_type(&*paren.elem),
		Type::Ptr(pointer) => to_lua_type(&*pointer.elem),
		Type::Reference(reference) => to_lua_type(&*reference.elem),
		Type::Slice(ty) => LuaType::Array {
			element: Box::new(to_lua_type(&*ty.elem)),
		},
		v => {
			panic!("cannot convert type {}", v.to_token_stream())
		}
	}
}

pub fn get_parameters(inputs: &Punctuated<FnArg, Token![,]>) -> Vec<Parameter> {
	let mut parameters = Vec::new();
	for arg in inputs {
		if let FnArg::Typed(pat) = arg {
			if let Pat::Ident(ident) = &*pat.pat {
				let string = ident.ident.to_string();
				if string.as_str() != "lua" {
					parameters.push(Parameter {
						name: string,
						ty: to_lua_type(&*pat.ty),
						comment: Default::default(),
					});
				}
			} else {
				panic!("only ident patterns are supported");
			}
		}
	}

	parameters
}

pub fn get_doc_comments(attributes: &Vec<Attribute>) -> Vec<String> {
	let mut comments = Vec::new();
	for attr in attributes {
		if let Ok(value) = attr.parse_meta() {
			if let Meta::NameValue(
				MetaNameValue {
					lit: Lit::Str(string),
					..
				},
				..,
			) = value
			{
				comments.push(string.value());
			}
		}
	}

	comments
}

pub fn get_return_ty(ret: &ReturnType) -> Option<LuaType> {
	match ret {
		ReturnType::Default => None,
		ReturnType::Type(_, ty) => Some(to_lua_type(ty)),
	}
}

pub fn get_path_name(path: &syn::Path) -> String { path.segments.last().unwrap().ident.to_string() }

pub fn get_type_name(ty: &Type) -> String {
	if let Type::Path(path) = ty {
		get_path_name(&path.path)
	} else {
		panic!("not a path type")
	}
}

pub fn get_generics(generics: &syn::Generics) -> ClassGenerics {
	let mut out = ClassGenerics { entries: vec![] };
	for generic in &generics.params {
		if let GenericParam::Type(ty) = &generic {
			out.entries.push(ty.ident.to_string());
		}
	}

	out
}
