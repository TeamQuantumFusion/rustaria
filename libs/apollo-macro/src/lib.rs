mod attr;
mod method;
mod values;
mod field;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{Attribute, ImplItem, ItemImpl, parse_macro_input};
use syn::spanned::Spanned;
use crate::attr::{FieldAttr, LuaBindingAttr, MethodAttr};


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
	_: proc_macro::TokenStream,
	tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let mut item = parse_macro_input!(tokens as ItemImpl);

	let mut methods = TokenStream::new();
	let mut fields =  TokenStream::new();

	let mut new_items = Vec::new();
	let mut from_lua = None;
	let mut to_lua = None;
	'item: for (i, mut item) in item.items.into_iter().enumerate() {
		if let ImplItem::Method(method) = &mut item {
			for attribute in &method.attrs {
				if let Some(attr) = parse_binding_attr(attribute) {
					match attr {
						Ok(attr) => {
							match attr {
								LuaBindingAttr::Method(attr) => {
									if !attr.keep_original {
										method.sig.ident = Ident::new(&format!("__internal_stuff_{}_{i}", method.sig.ident), method.sig.ident.span());
									}
									methods.append_all(method::bind_method(&method.sig, attr));

								}
								LuaBindingAttr::Field(attr) => {
									if !attr.keep_original {
										method.sig.ident = Ident::new(&format!("__internal_stuff_{}_{i}", method.sig.ident), method.sig.ident.span());
									}
									fields.append_all(field::bind_field(&method.sig, attr));
								}
								LuaBindingAttr::FromLua => {
									if from_lua.is_some() {
										return syn::Error::new(item.span(), "Cannot have multiple from_lua implementations").into_compile_error().into();
									}

									from_lua = Some(item);
									// skip adding to new_items
									continue 'item;
								}
								LuaBindingAttr::ToLua => {
									if to_lua.is_some() {
										return syn::Error::new(item.span(), "Cannot have multiple to_lua implementations").into_compile_error().into();
									}
									to_lua = Some(item);
									// skip adding to new_items
									continue 'item;
								}
							}
						}
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
	let to_lua = to_lua.map(|v| quote!(
		impl #generics apollo::ToLua for #ty {
		    #v
		}
	)).unwrap_or_else(|| quote!(
		impl #generics apollo::UserDataToLua for #ty {}
	));

	let from_lua = from_lua.map(|v| quote!(
		impl #generics apollo::FromLua for #ty {
		    #v
		}
	)).unwrap_or_else(|| quote!(
		impl #generics apollo::UserDataFromLua for #ty {}
	));
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
        }

		#to_lua
		#from_lua
	).into()
}

fn parse_binding_attr(attribute: &Attribute) -> Option<syn::Result<LuaBindingAttr>> {
	let mut kind = attribute.path.segments[0].ident.to_string();
	if kind == "apollo" {
		kind = attribute.path.segments[1].ident.to_string();
	}

	if kind == "lua_field" {
		Some(attribute.parse_args().map(LuaBindingAttr::Field))
	} else if kind == "lua_method" {
		if attribute.tokens.is_empty()  {
			return Some(Ok(LuaBindingAttr::Method(MethodAttr {
				keep_original: true,
				lua_name: None
			})));
		}
		Some(attribute.parse_args().map(LuaBindingAttr::Method))
	}   else if kind == "from_lua" {
		Some(Ok(LuaBindingAttr::FromLua))
	} else if kind == "to_lua" {
		Some(Ok(LuaBindingAttr::ToLua))
	}  else {
		None
	}
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
