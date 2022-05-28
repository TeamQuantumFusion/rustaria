// fn add_field_method_get      M: Fn   (&Lua, &T) -> Result<R>;
// fn add_field_method_set      M: FnMut(&Lua, &mut T, A) -> Result<()>;
// fn add_field_function_get    F: Fn   (&Lua, AnyUserData) -> Result<R>;
// fn add_field_function_set    F: FnMut(&Lua, AnyUserData, A) -> Result<()>;
// fn add_meta_field_with       F: Fn   (&Lua) -> Result<R>,

use proc_macro2::{Ident, Span, TokenStream};
use syn::{FnArg, parse_quote, Pat, Receiver, Signature, Type};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::FieldKind;
use crate::util::SelfArg;

pub enum FieldType {
	GetFieldMethod(SelfArg),
	SetFieldMethod(SelfArg),
	GetFieldFunction,
	SetFieldFunction,
}

impl FieldType {
	pub(crate) fn new(sig: &Signature, kind: FieldKind) -> FieldType {
		let x = match sig.inputs.first() {
			// self
			Some(FnArg::Receiver(receiver)) => {
				match kind {
					FieldKind::Get => FieldType::GetFieldMethod(receiver.mutability.map(|_| SelfArg::RefMut).unwrap_or(SelfArg::Ref)),
					FieldKind::Set => FieldType::SetFieldMethod(receiver.mutability.map(|_| SelfArg::RefMut).unwrap_or(SelfArg::Ref)),
				}
			}
			// no self
			_ => match kind {
				FieldKind::Get => FieldType::GetFieldFunction,
				FieldKind::Set => FieldType::SetFieldFunction,
			}
		};
		x
	}

	pub fn self_arg(&self) -> SelfArg {
		match self {
			FieldType::GetFieldMethod(v) | FieldType::SetFieldMethod(v) => *v,
			FieldType::GetFieldFunction | FieldType::SetFieldFunction => SelfArg::None,
		}
	}

	pub fn get_add_lua_ident(&self) -> Ident {
		Ident::new(
			match self {
				FieldType::GetFieldMethod(_) => "add_field_method_get",
				FieldType::SetFieldMethod(_) => "add_field_method_set",
				FieldType::GetFieldFunction => "add_field_function_get",
				FieldType::SetFieldFunction => "add_field_function_set",
			},
			Span::call_site(),
		)
	}

}
