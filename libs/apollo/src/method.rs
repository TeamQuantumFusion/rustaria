use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{FnArg, Receiver, Signature};

use crate::util::SelfArg;

pub enum MethodType {
	AddMethod,
	AddMethodMut,
	AddFunctionMut,
	AddMetaMethod(TokenStream),
	AddMetaMethodMut(TokenStream),
	AddMetaFunctionMut(TokenStream),
}

impl MethodType {
	pub fn new(sig: &Signature, lua_name: &str) -> MethodType {
		match sig.inputs.first() {
			// &mut self
			Some(FnArg::Receiver(Receiver {
				mutability: Some(_),
				..
			})) => match metamethod_from_name(lua_name) {
				Some(stream) => MethodType::AddMetaMethodMut(stream),
				None => MethodType::AddMethodMut,
			},
			// &self
			Some(FnArg::Receiver(Receiver {
				mutability: None, ..
			})) => match metamethod_from_name(lua_name) {
				Some(stream) => MethodType::AddMetaMethod(stream),
				None => MethodType::AddMethod,
			},
			// no self
			_ => match metamethod_from_name(lua_name) {
				Some(stream) => MethodType::AddMetaFunctionMut(stream),
				None => MethodType::AddFunctionMut,
			},
		}
	}

	pub fn self_arg(&self) -> SelfArg {
		match self {
			MethodType::AddMethod | MethodType::AddMetaMethod(_) => SelfArg::Ref,
			MethodType::AddMethodMut | MethodType::AddMetaMethodMut(_) => SelfArg::RefMut,
			MethodType::AddFunctionMut | MethodType::AddMetaFunctionMut(_) => SelfArg::None,
		}
	}

	pub fn get_add_lua_ident(&self) -> Ident {
		Ident::new(
			match self {
				MethodType::AddMethod => "add_method",
				MethodType::AddMethodMut => "add_method_mut",
				MethodType::AddFunctionMut => "add_function_mut",
				MethodType::AddMetaMethod(_) => "add_meta_method",
				MethodType::AddMetaMethodMut(_) => "add_meta_method_mut",
				MethodType::AddMetaFunctionMut(_) => "add_meta_function_mut",
			},
			Span::call_site(),
		)
	}
}

pub fn metamethod_from_name(lua_name: &str) -> Option<TokenStream> {
	match lua_name {
		"__add" => Some(quote!(LuaMetaMethod::Add)),
		"__sub" => Some(quote!(LuaMetaMethod::Sub)),
		"__mul" => Some(quote!(LuaMetaMethod::Mul)),
		"__div" => Some(quote!(LuaMetaMethod::Div)),
		"__mod" => Some(quote!(LuaMetaMethod::Mod)),
		"__pow" => Some(quote!(LuaMetaMethod::Pow)),
		"__unm" => Some(quote!(LuaMetaMethod::Unm)),
		"__idiv" => Some(quote!(LuaMetaMethod::IDiv)),
		"__band" => Some(quote!(LuaMetaMethod::BAnd)),
		"__bor" => Some(quote!(LuaMetaMethod::BOr)),
		"__bxor" => Some(quote!(LuaMetaMethod::BXor)),
		"__bnot" => Some(quote!(LuaMetaMethod::BNot)),
		"__shl" => Some(quote!(LuaMetaMethod::Shl)),
		"__shr" => Some(quote!(LuaMetaMethod::Shr)),
		"__concat" => Some(quote!(LuaMetaMethod::Concat)),
		"__len" => Some(quote!(LuaMetaMethod::Len)),
		"__eq" => Some(quote!(LuaMetaMethod::Eq)),
		"__lt" => Some(quote!(LuaMetaMethod::Lt)),
		"__le" => Some(quote!(LuaMetaMethod::Le)),
		"__index" => Some(quote!(LuaMetaMethod::Index)),
		"__newindex" => Some(quote!(LuaMetaMethod::NewIndex)),
		"__call" => Some(quote!(LuaMetaMethod::Call)),
		"__tostring" => Some(quote!(LuaMetaMethod::ToString)),
		"__pairs" => Some(quote!(LuaMetaMethod::Pairs)),
		"__ipairs" => Some(quote!(LuaMetaMethod::IPairs)),
		"__close" => Some(quote!(LuaMetaMethod::Close)),
		_ => None,
	}
}
