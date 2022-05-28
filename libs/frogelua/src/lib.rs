
use proc_macro2::{Ident, Literal, Span};
use quote::{quote, ToTokens};
use syn::{
	Attribute, Item, ItemStruct,
	Meta, parse_macro_input};


macro_rules! import {
    ($NAME:literal) => {
        match proc_macro_crate::crate_name($NAME).expect($NAME) {
           proc_macro_crate::FoundCrate::Itself => quote!( crate ),
           proc_macro_crate::FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                quote!( #ident )
            }
        }
    };
}

#[proc_macro_derive(FromLua, attributes(use_default))]
pub fn from_lua(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let struct_item = parse_macro_input!(item as Item);
	let common = import!("rsa-core");

	match struct_item {
		Item::Enum(enum_item) => {
			let ty = enum_item.ident;
			let mut variants = proc_macro2::TokenStream::new();

			for variant in &enum_item.variants {
				let ident = &variant.ident;
				let ident_string = Literal::string(&ident.to_string());
				quote!(
					#ident_string => Ok(#ty::#ident),
				)
				.to_tokens(&mut variants);
			}

			let else_statement = if has_use_default(&enum_item.attrs) {
				quote!(Ok(#ty::default()))
			} else {
				let error_string = Literal::string(&ty.to_string());
				quote!(Err(#common::api::lua::LuaError::RuntimeError(
					format!("Invalid type {:?} at {}", lua_value, #error_string)
				)))
			};

			quote!(
				impl #common::api::lua::FromLua for #ty {
					fn from_lua(lua_value: #common::api::lua::LuaValue, _: &#common::api::lua::Lua) -> #common::api::lua::LuaResult<Self> {
						if let #common::api::lua::LuaValue::String(string) = lua_value {
							let string = string.to_str()?;
							match string {
								#variants
								_ => Err(#common::api::lua::LuaError::RuntimeError(format!("Unknown variant {}", string)))
							}
						} else {
							#else_statement
						}
					}
				}
			)
			.into()
		}
		Item::Struct(struct_item) => {
			let ty = struct_item.ident;
			let mut getters = proc_macro2::TokenStream::new();

			for field in &struct_item.fields {
				let ident = field.ident.as_ref().expect("Fields require an identifier.");
				let ident_string = ident.to_string();
				quote!(
					#ident: table.get(#ident_string)?,
				)
				.to_tokens(&mut getters);
			}

			let else_statement = if has_use_default(&struct_item.attrs) {
				quote!(Ok(#ty::default()))
			} else {
				let error_string = Literal::string(&ty.to_string());
				quote!(Err(#common::api::lua::LuaError::RuntimeError(
					format!("Invalid type {:?} at {}", lua_value, #error_string)
				)))
			};

			quote!(
				impl #common::api::lua::FromLua for #ty {
					fn from_lua(lua_value: #common::api::lua::LuaValue, _: &#common::api::lua::Lua) -> #common::api::lua::LuaResult<Self> {
						if let #common::api::lua::LuaValue::Table(table) = lua_value {
							Ok(#ty {
								#getters
							})
						} else {
							#else_statement
						}
					}
				}
			)
			.into()
		}
		_ => panic!("Only enum and structs allowed."),
	}
}

#[proc_macro_derive(ToLua)]
pub fn to_lua(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let struct_item = parse_macro_input!(item as ItemStruct);

	let ty = struct_item.ident;
	let mut getters = proc_macro2::TokenStream::new();

	for field in &struct_item.fields {
		let ident = field.ident.as_ref().expect("Fields require an identifier.");
		let ident_string = ident.to_string();
		quote!(
			table.set(#ident_string, self.#ident)?;
		)
		.to_tokens(&mut getters);
	}

	quote!(
		impl rustaria_common::api::lua::ToLua for #ty {
			fn to_lua(self, lua: &rustaria_common::api::lua::Lua) -> rustaria_common::api::lua::LuaResult<rustaria_common::api::lua::LuaValue> {
				let table = lua.create_table()?;
				#getters
				Ok(rustaria_common::api::lua::LuaValue::Table(table))
			}
		}
	)
	.into()
}

fn has_use_default(attributes: &Vec<Attribute>) -> bool {
	for attr in attributes {
		if let Ok(Meta::Path(path)) = attr.parse_meta() {
			if let Some(ident) = path.get_ident() {
				if ident == &Ident::new("use_default", Span::call_site()) {
					return true;
				}
			}
		}
	}

	false
}
