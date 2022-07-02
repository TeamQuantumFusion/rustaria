use proc_macro2::Ident;
use syn::{
	parse::{Parse, ParseStream},
	Error,
};

pub(crate) enum LuaBindingAttr {
	Method(MethodAttr),
	Field(FieldAttr),
	FromLua,
}

#[derive(Clone)]
pub(crate) struct MethodAttr {
	pub lua_name: Option<String>,
}

impl Parse for MethodAttr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(MethodAttr {
			lua_name: input.parse::<Option<Ident>>()?.map(|i| i.to_string()),
		})
	}
}

#[derive(Clone)]
pub(crate) enum FieldBindKind {
	Getter,
	Setter,
}

impl Parse for FieldBindKind {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let ident: Ident = input.parse()?;

		if ident == "get" {
			Ok(FieldBindKind::Getter)
		} else if ident == "set" {
			Ok(FieldBindKind::Setter)
		} else {
			Err(Error::new(
				ident.span(),
				"Field attribute needs to start with \"get\" or \"set\"",
			))
		}
	}
}

#[derive(Clone)]
pub(crate) struct FieldAttr {
	pub kind: FieldBindKind,
	pub lua_name: Option<String>,
}

impl Parse for FieldAttr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(FieldAttr {
			kind: input.parse()?,
			lua_name: input.parse::<Option<Ident>>()?.map(|i| i.to_string()),
		})
	}
}
