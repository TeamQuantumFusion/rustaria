#![feature(proc_macro_span)]
#![allow(non_snake_case)]

use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, spanned::Spanned, Attribute, ImplItem, Item, ItemImpl};

// UserData for implementing methods and such
// FromLua for auto-implementing FromLua from fields
// ToLua for auto-implementing ToLua from fields
use crate::{
	attr::{FieldAttr, LuaBindingAttr, MethodAttr},
	from_lua::ItemsBuilder,
	lua_impl::{field, method},
};

mod attr;
mod from_lua;
mod lua_impl;

#[proc_macro_derive(FromLua)]
pub fn FromLua(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let item = parse_macro_input!(item as Item);
	let (ident, from_lua) = match item {
		Item::Enum(values) => {
			let mut variants = TokenStream::new();

			let ident = values.ident;
			for variant in values.variants {
				//let mut fields = ItemsBuilder::new();
				//for field in variant.fields {
				//	fields.push_field(field.ident.as_ref());
				//}
				let variant_ident = variant.ident;
				let variant_name = variant_ident.to_string();
				variants.append_all(quote!(
					#variant_name => {
						#ident::#variant_ident
					}
				))
			}

			(
				ident.clone(),
				quote!(
					let variant = table.get::<_, std::string::String>(1)?;
					Ok(
						match variant.as_str() {
							#variants
							_ => {
								return Err(rsa_core::err::audit::Audit::new(format!("{} is not a known {} variant.", variant, std::any::type_name::<Self>())))
							}
						}
					)
				),
			)
		}
		Item::Struct(item) => {
			let mut fields = ItemsBuilder::new();
			for field in item.fields {
				fields.push_field(field.ident.as_ref());
			}

			let ident = item.ident;
			(
				ident.clone(),
				quote!(
					Ok(#ident {
						#fields
					})
				),
			)
		}
		_ => panic!("Only Enum and Struct items are supported."),
	};

	quote!(
		impl FromLua for #ident {
			fn from_lua(lua_value: apollo::Value, _: &apollo::Lua) -> rsa_core::err::Result<Self> {
				if let apollo::Value::Table(table) = lua_value {
					#from_lua
				} else {
					Err(rsa_core::err::audit::Audit::new("Type is supposed to be a Table"))
				}
			}
		}
	)
	.into()
}

#[proc_macro_attribute]
pub fn from_lua(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	item
}

#[proc_macro_attribute]
pub fn to_lua(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	item
}

#[proc_macro_attribute]
pub fn lua_method(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	item
}

#[proc_macro_attribute]
pub fn lua_field(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	item
}

/// Scans this implementation for lua bindings.
///
/// # Examples
///
/// ```
/// #[lua_impl]
/// impl Thing {
///     #[lua_method]
///     pub fn add(&mut self, thing: &Thing) {
///         self.value += thing.value;
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn lua_impl(
	_attr: proc_macro::TokenStream,
	tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let mut item = parse_macro_input!(tokens as ItemImpl);

	let mut methods = TokenStream::new();
	let mut fields = TokenStream::new();

	let mut new_items = Vec::new();
	let mut from_lua = None;
	for mut item in item.items {
		if let ImplItem::Method(method) = &mut item {
			for attribute in &method.attrs {
				if let Some(attr) = parse_binding_attr(attribute) {
					match attr {
						Ok(attr) => match attr {
							LuaBindingAttr::Method(attr) => {
								methods.append_all(method::bind_method(method, attr));
							}
							LuaBindingAttr::Field(attr) => {
								fields.append_all(field::bind_field(method, attr));
							}
							LuaBindingAttr::FromLua => {
								if from_lua.is_some() {
									return syn::Error::new(
										item.span(),
										"Cannot have multiple from_lua implementations",
									)
									.into_compile_error()
									.into();
								}

								from_lua = Some(method.sig.ident.clone());
							}
						},
						Err(err) => {
							return err.into_compile_error().into();
						}
					}
				}
			}
		}
		new_items.push(item);
	}
	item.items = new_items;

	let generics = &item.generics;
	let ty = &item.self_ty;

	let from_lua = from_lua
		.map(|v| {
			quote!(
				fn from_lua(value: Value, lua: &Lua) -> Option<rsa_core::err::Result<Self>> {
					Some(Self::#v(value, lua))
				}
			)
		})
		.unwrap_or_else(|| quote!());

	quote!(
		#item

		#[allow(clippy::needless_question_mark)]
		impl #generics apollo::UserData for #ty {
			fn add_methods<M: apollo::UserDataMethods<Self>>(methods: &mut M) {
				#methods
			}
			fn add_fields<M: apollo::UserDataFields<Self>>(fields: &mut M) {
				#fields
			}

			#from_lua
		}
	)
	.into()
}
fn parse_binding_attr(attribute: &Attribute) -> Option<syn::Result<LuaBindingAttr>> {
	let mut kind = attribute.path.segments[0].ident.to_string();
	if kind == "apollo" {
		kind = attribute.path.segments[1].ident.to_string();
	}

	if kind == "lua_field" {
		Some(attribute.parse_args().map(LuaBindingAttr::Field))
	} else if kind == "lua_method" {
		if attribute.tokens.is_empty() {
			return Some(Ok(LuaBindingAttr::Method(MethodAttr { lua_name: None })));
		}
		Some(attribute.parse_args().map(LuaBindingAttr::Method))
	} else if kind == "from_lua" {
		Some(Ok(LuaBindingAttr::FromLua))
	} else {
		None
	}
}
