mod field;
mod method;
mod util;

use crate::field::FieldType;
use method::MethodType;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, ImplItem, ItemFn, ItemImpl, Token, ImplItemMethod};
use syn::token::Paren;

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

pub(crate) use import;

struct MethodAttr {
	lua_name: Option<Ident>,
}

impl Parse for MethodAttr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(MethodAttr {
			lua_name: input.parse()?,
		})
	}
}


/// Creates a lua method binding.
/// This will automatically create the correct registration dependant on your parameters and method name.
/// - If your method name is one of the lua metatable ones your method/function wil be a meta one.
/// - If your first parameter is &self or &mut self it will be a method, else a function
/// - If your self is a &mut self, then it will be a mut method, else it will be a normal method, functions are always the mut kind.
/// # Overwriting
/// If you want a different method name you can enter a custom name into the attribute.
#[proc_macro_attribute]
pub fn lua_method(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	item
}

fn lua_method_reg(item: &ImplItemMethod, attr: MethodAttr) -> TokenStream {
	let lua_name = attr
		.lua_name
		.unwrap_or_else(|| item.sig.ident.clone())
		.to_string();

	let (lua_arg, args) = util::filter_self_lua(item.sig.inputs.iter());
	let (names, types) = util::compile_reg_args(args);

	// Fetch what methodtype this is for lua
	let method_type = MethodType::new(&item.sig, &lua_name);

	// Basic setup
	let lua_reg_method = method_type.get_add_lua_ident();
	let lua_name = Literal::string(&lua_name);
	let rust_ident = item.sig.ident.clone();

	let (closure_args, rust_args) =
		util::compile_args(method_type.self_arg(), lua_arg, true, names, types);
	
	let ret = util::compile_invoke_return(&item.sig, quote!(Self::#rust_ident(#rust_args)));
	quote!(methods.#lua_reg_method(#lua_name, |#closure_args| unsafe {
		#ret
	});)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum FieldKind {
	Get,
	Set,
}

struct FieldAttr {
	lua_name: Option<Ident>,
}

impl Parse for FieldAttr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(FieldAttr {
			lua_name: input.parse()?,
		})
	}
}

/// Creates a lua field binding.
/// # Glue
/// If you want to pass a mutable reference to a field you need to return `LuaResult<*const Type>`
/// # Overwriting
/// If you want a different method name you can enter a custom name into the attribute.
#[proc_macro_attribute]
pub fn lua_field(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	item
}

fn lua_field_reg(item: &ImplItemMethod, attr: FieldAttr) -> TokenStream {
	let str = item.sig.ident.to_string();
	let (kind, lua_name) = if str.starts_with("get_") {
		(FieldKind::Get, str.trim_start_matches("get_"))
	} else  if str.starts_with("set_"){
		(FieldKind::Set, str.trim_start_matches("set_"))
	} else {
		panic!("Method needs to start with either \"set_\" or \"get_\"")
	};
	let lua_name = attr
		.lua_name
		.map(|v| v.to_string()).unwrap_or_else(|| lua_name.to_string());


	let (lua_arg, args) = util::filter_self_lua(item.sig.inputs.iter());
	let (names, types) = util::compile_reg_args(args);
	let field_type = FieldType::new(&item.sig, kind);


	// Basic setup
	let lua_reg_field = field_type.get_add_lua_ident();
	let lua_name = Literal::string(&lua_name);
	let rust_ident = item.sig.ident.clone();

	let (closure_args, rust_args) = util::compile_args(
		field_type.self_arg(),
		lua_arg,
		kind == FieldKind::Set,
		names,
		types,
	);

	let ret = util::compile_invoke_return(&item.sig, quote!(Self::#rust_ident(#rust_args)));
	quote!(fields.#lua_reg_field(#lua_name, |#closure_args| unsafe {
		#ret
	});)
}


#[proc_macro_attribute]
pub fn lua_impl(
	_: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let core = import!("rsa-core");

	let item = parse_macro_input!(item as ItemImpl);
	let mut methods = TokenStream::new();
	let mut fields = TokenStream::new();

	for item in &item.items {
		if let ImplItem::Method(item) = item {
			for attribute in &item.attrs {
				if let Some(ident) = attribute.path.get_ident() {
					if ident == "lua_field" {
						let attr = if attribute.tokens.is_empty() {
							FieldAttr { lua_name: None }
						} else {
							attribute.parse_args().expect("Failed to parse args")
						};
						fields.append_all(lua_field_reg(item, attr));
					} else if ident == "lua_method" {
						let attr = if attribute.tokens.is_empty() {
							MethodAttr { lua_name: None }
						} else {
							attribute.parse_args().expect("Failed to parse args")
						};
						methods.append_all(lua_method_reg(item, attr));
					}
				}
			}
		}
	}

	let ty = &item.self_ty;
	let generics = &item.generics;
	quote!(
		#item
		impl #generics #core::api::lua::LuaUserData for #ty {
			fn add_methods<M: #core::api::lua::LuaUserDataMethods<Self>>(methods: &mut M) {
				#methods
			}
			fn add_fields<M: #core::api::lua::LuaUserDataFields<Self>>(fields: &mut M) {
				#fields
			}
		}
	)
	.into()
}
