use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use std::iter::Peekable;
use syn::punctuated::{Iter, Punctuated};
use syn::token::Comma;
use syn::{parse_quote, AngleBracketedGenericArguments, FnArg, Pat, PatType, PathArguments, ReturnType, Signature, Token, Type, GenericArgument};
use crate::import;

pub fn get_overwrite_name(sig: &Signature, attr: &proc_macro::TokenStream) -> String {
	if attr.is_empty() {
		sig.ident.to_string()
	} else {
		attr.to_string()
	}
}

// "var1: Ty1, var2: Ty2, var3: Ty3" -> ("var1, var2, var3", "Ty1, Ty2, Ty3")
pub fn compile_reg_args(
	args: Peekable<Iter<FnArg>>,
) -> (Punctuated<Pat, Token!(,)>, Punctuated<Type, Token!(,)>) {
	let mut types: Punctuated<Type, Token!(,)> = Punctuated::new();
	let mut patterns: Punctuated<Pat, Token!(,)> = Punctuated::new();
	for arg in args {
		if let FnArg::Typed(ty) = arg {
			types.push_value(*ty.ty.clone());
			patterns.push_value(*ty.pat.clone());
		}
	}

	(patterns, types)
}

// Returns if it has a lua or not
pub fn filter_self_lua(iter: Iter<FnArg>) -> (bool, Peekable<Iter<FnArg>>) {
	let mut peekable = iter.peekable();
	let mut lua = false;
	while let Some(arg) = peekable.peek() {
		match arg {
			FnArg::Typed(PatType { pat, .. }) => {
				if let Pat::Ident(ident) = pat.as_ref() {
					if ident.ident == "lua" {
						lua = true;
						peekable.next();
						continue;
					}
				}
			}
			FnArg::Receiver(_) => {
				peekable.next();
				continue;
			}
			_ => {}
		}

		break;
	}

	(lua, peekable)
}

pub fn compile_invoke_return(sig: &Signature, invoke: TokenStream) -> TokenStream {
	let ty = get_return_type(&sig.output);

	// glue
	let core = import!("rsa-core");
	if is_ref(ty) {
		quote!(
			#invoke.map(|res| #core::lua::glue::LuaGlue::new_raw(res))
		)
	} else {
		invoke
	}
}

fn get_return_type(return_type: &ReturnType) -> &Type {
	if let ReturnType::Type(_, ty) = return_type {
		if let Type::Path(path) = ty.as_ref() {
			for path in path.path.segments.iter() {
				if path.ident == "LuaResult" {
					if let PathArguments::AngleBracketed(brackets) = &path.arguments {
						if let GenericArgument::Type(ty) = brackets.args.first().expect("Result must have args") {
							return ty;
						}
						panic!("The result args must be a type")
					} else {
						panic!("The result args must be in <>")
					}
				}
			}
		};
	}

	panic!("Return must be LuaResult<>")
}

#[derive(Copy, Clone)]
pub enum SelfArg {
	None,
	Ref,
	RefMut
}

pub fn compile_args(
	self_arg: SelfArg,
	lua_arg: bool,
	value_arg: bool,
	mut names: Punctuated<Pat, Comma>,
	mut types: Punctuated<Type, Comma>,
) -> (
	Punctuated<TokenStream, Comma>,
	Punctuated<TokenStream, Comma>,
) {
	let mut closure_args: Punctuated<TokenStream, Comma> = Punctuated::new();
	let mut rust_args: Punctuated<TokenStream, Comma> = Punctuated::new();
	closure_args.push(parse_quote!(lua));


	match self_arg {
		SelfArg::None => {}
		SelfArg::Ref => {
			closure_args.push(parse_quote!(this));
			rust_args.push(parse_quote!(this));
		}
		SelfArg::RefMut => {
			closure_args.push(parse_quote!(this));
			rust_args.push(parse_quote!(&mut *(this as *const Self as *mut Self)));
		}
	}

	if lua_arg {
		rust_args.push(parse_quote!(lua));
	}

	let core = import!("rsa-core");

	if value_arg {
		if names.is_empty() {
			closure_args.push(parse_quote!(_: ()));
		} else {
			{
				for (pat, ty) in names.iter_mut().zip(types.iter()) {
					// Glue
					if let Type::Reference(_) = ty {
						rust_args.push(quote!(#pat.get_mut()));
						*pat = parse_quote!(mut #pat)
					} else {
						rust_args.push(pat.to_token_stream());
					}
				}
			}

			// Use glue
			for ty in &mut types {
				if let Type::Reference(reference) = ty {
					let elem = reference.elem.clone();
					*ty = parse_quote!(#core::lua::glue::LuaGlue<#elem>);
				}
			}

			closure_args.push(parse_quote!((#names): (#types)));
		}
	}

	(closure_args, rust_args)
}

fn is_ref(ty: &Type) -> bool {
	matches!(ty, Type::Reference(_))
}
