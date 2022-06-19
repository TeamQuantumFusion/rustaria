use proc_macro2::Ident;
use syn::{Error, Token};
use syn::parse::{Parse, ParseStream};

pub(crate) enum LuaBindingAttr {
	Method(MethodAttr),
	Field(FieldAttr),
	FromLua,
	ToLua
}

#[derive(Clone)]
pub(crate) struct MethodAttr {
	pub keep_original: bool,
	pub lua_name: Option<String>
}

impl Parse for MethodAttr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(MethodAttr {
			keep_original: input.parse::<Option<Token!(!)>>()?.is_none(),
			lua_name: input.parse::<Option<Ident>>()?.map(|i| i.to_string()),
		})
	}
}

#[derive(Clone)]
pub(crate) enum FieldBindKind {
	Getter,
	Setter
}

impl Parse for FieldBindKind {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let ident: Ident = input.parse()?;

		if ident == "get" {
			Ok(FieldBindKind::Getter)
		} else if ident == "set" {
			Ok(FieldBindKind::Setter)
		} else {
			Err(Error::new(ident.span(), "Field attribute needs to start with \"get\" or \"set\""))
		}
	}
}

#[derive(Clone)]
pub(crate) struct FieldAttr {
	pub keep_original: bool,
	pub kind: FieldBindKind,
	pub lua_name: Option<String>,
}

impl Parse for FieldAttr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(FieldAttr {
			keep_original: input.parse::<Option<Token!(!)>>()?.is_none(),
			kind: input.parse()?,
			lua_name: input.parse::<Option<Ident>>()?.map(|i| i.to_string()),
		})
	}
}

