use std::ops::Index;

use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::{
	parse_macro_input, parse_quote, Attribute, FnArg, ImplItem, ImplItemMethod, Item, ItemFn,
	ItemImpl, ItemStruct, Meta, Pat, Token, Type,
};

use crate::util::get_method_register;

mod util;

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

#[proc_macro_attribute]
pub fn lua_field(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let func = parse_macro_input!(item as ItemFn);
	if !func.sig.ident.to_string().starts_with("__field_") {
		panic!("Field methods need to start with \"__field_\"")
	}

	quote!(#func).into()
}

#[proc_macro_attribute]
pub fn lua_method(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	item
}

#[proc_macro_attribute]
pub fn lua_impl(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let mut implementations = parse_macro_input!(item as ItemImpl);

	let mut out = TokenStream::new();
	'next: for item in &mut implementations.items {
		if let ImplItem::Method(func) = item {
			for attribute in &func.attrs {
				if let Some(ident) = attribute.path.get_ident() {
					if ident == "lua_method" {
						func.sig.ident = Ident::new(
							&(func.sig.ident.to_string() + "_lua"),
							func.sig.ident.span(),
						);
						let item = func.to_token_stream().into();
						let func = parse_macro_input!(item as ItemFn);
						let register: TokenStream = lua_method_registration(&func).into();
						out.append_all(quote!(
							#register
						));
						continue 'next;
					} else if ident == "lua_field" {
						continue 'next;
					}
				}
			}

			panic!("Every function needs to have a #[lua_method] or #[lua_field]")
		}
	}

	let ty = implementations.self_ty.clone();
	quote!(
		#implementations

		impl LuaUserData for #ty {
			fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
				#out
			}
		}
	)
	.into()
}

fn lua_method_registration(func: &ItemFn) -> proc_macro::TokenStream {
	let target_ident = func.sig.ident.clone();

	// Compile the target method invocation structure
	let mut args_types = Punctuated::<Box<Type>, Token![,]>::new();
	let mut args_names = Punctuated::<Box<Pat>, Token![,]>::new();
	let mut lua_param = false;
	for i in 1..func.sig.inputs.len() {
		let arg = func.sig.inputs.index(i);
		match arg {
			FnArg::Receiver(_) => panic!("self in this is cringe"),
			FnArg::Typed(ty) => {
				if let Pat::Ident(ident) = &*ty.pat {
					if ident.ident == "lua" {
						lua_param = true;
						continue;
					}
				}
				args_types.push_value(ty.ty.clone());
				args_names.push_value(ty.pat.clone());
			}
		}
	}
	let args = if lua_param {
		quote!(lua, #args_names)
	} else {
		quote!(#args_names)
	};

	let expand_args = if args_names.is_empty() {
		quote!(())
	} else {
		quote!(#args_names)
	};

	let register =
		get_method_register(func, expand_args, quote!(Self::#target_ident(value, #args)));

	quote!(
		#register;
	)
	.into()
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
				quote!(Err(#common::lua::LuaError::RuntimeError(
					format!("Invalid type {:?} at {}", lua_value, #error_string)
				)))
			};

			quote!(
				impl #common::lua::FromLua for #ty {
					fn from_lua(lua_value: #common::lua::LuaValue, _: &#common::lua::Lua) -> #common::lua::LuaResult<Self> {
						if let #common::lua::LuaValue::String(string) = lua_value {
							let string = string.to_str()?;
							match string {
								#variants
								_ => Err(#common::lua::LuaError::RuntimeError(format!("Unknown variant {}", string)))
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
				quote!(Err(#common::lua::LuaError::RuntimeError(
					format!("Invalid type {:?} at {}", lua_value, #error_string)
				)))
			};

			quote!(
				impl #common::lua::FromLua for #ty {
					fn from_lua(lua_value: #common::lua::LuaValue, _: &#common::lua::Lua) -> #common::lua::LuaResult<Self> {
						if let #common::lua::LuaValue::Table(table) = lua_value {
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
		impl rustaria_common::lua::ToLua for #ty {
			fn to_lua(self, lua: &rustaria_common::lua::Lua) -> rustaria_common::lua::LuaResult<rustaria_common::lua::LuaValue> {
				let table = lua.create_table()?;
				#getters
				Ok(rustaria_common::lua::LuaValue::Table(table))
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
