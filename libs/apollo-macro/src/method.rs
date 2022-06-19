use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{punctuated::Punctuated, Signature, Token};

use crate::{
	attr::MethodAttr,
	values::{Receiver, ValueManager},
};
use crate::values::LuaValue;

pub(crate) fn bind_method(sig: &Signature, attr: MethodAttr) -> TokenStream {
	let mut manager = ValueManager::new(false);

	// Scan inputs
	let inputs: Punctuated<TokenStream, Token!(,)> = sig
		.inputs
		.iter()
		.map(|input| manager.unwrap_input(input))
		.collect();


	// Create the "run" this is what actually gets run in the closure
	let target_name = &sig.ident;

	let wrapped_return = manager.wrap_return(&sig.output, quote!(Self::#target_name(#inputs)));
	let run = quote!(unsafe { #wrapped_return });

	// Determine what kind of binding we are making
	let lua_name = attr.lua_name.unwrap_or_else(|| sig.ident.to_string());
	let binding = MethodBinding::new( lua_name);


	binding.register(&mut manager, run)
}

// fn add_method<S, A, R, M>(&mut self, name: &S, method: M)
// 		M: 'static + MaybeSend + Fn(&Lua, &T, A) -> eyre::Result<R>;
//
// fn add_meta_method<S, A, R, M>(&mut self, meta: S, method: M)
// 		M: 'static + MaybeSend + Fn(&Lua, &T, A) -> eyre::Result<R>;
//

// fn add_method_mut<S, A, R, M>(&mut self, name: &S, method: M)
// 		M: 'static + MaybeSend + FnMut(&Lua, &mut T, A) -> eyre::Result<R>;
//
// fn add_meta_method_mut<S, A, R, M>(&mut self, meta: S, method: M)
// 		M: 'static + MaybeSend + FnMut(&Lua, &mut T, A) -> eyre::Result<R>;
//

// fn add_function<S, A, R, F>(&mut self, name: &S, function: F)
// 		F: 'static + MaybeSend + Fn(&Lua, A) -> eyre::Result<R>;
//
// fn add_meta_function<S, A, R, F>(&mut self, meta: S, function: F)
// 		F: 'static + MaybeSend + Fn(&Lua, A) -> eyre::Result<R>;
//

// fn add_function_mut<S, A, R, F>(&mut self, name: &S, function: F)
// 		F: 'static + MaybeSend + FnMut(&Lua, A) -> eyre::Result<R>;
//
// fn add_meta_function_mut<S, A, R, F>(&mut self, meta: S, function: F)
// 		F: 'static + MaybeSend + FnMut(&Lua, A) -> eyre::Result<R>;
enum MethodBinding {
	//AddMethod(String),
	//AddMethodMeta(MetaMethod),
	//AddMethodMut(String),
	//AddMethodMutMeta(MetaMethod),
	AddFunctionMut(String),
	AddFunctionMutMeta(MetaMethod),
}

impl MethodBinding {
	pub fn new( lua_name: String) -> MethodBinding {
		match MetaMethod::new(&lua_name) {
			None => {
				MethodBinding::AddFunctionMut(lua_name)
				//match receiver {
				//	None => MethodBinding::AddFunctionMut(lua_name),
				//	Some(Receiver::Immutable) => MethodBinding::AddMethod(lua_name),
				//	Some(Receiver::Mutable) => MethodBinding::AddMethodMut(lua_name),
				//}
			}
			Some(meta) => {
				MethodBinding::AddFunctionMutMeta(meta)
				//match receiver {
				//	None => MethodBinding::AddFunctionMutMeta(meta),
				//	Some(Receiver::Immutable) => MethodBinding::AddMethodMeta(meta),
				//	Some(Receiver::Mutable) => MethodBinding::AddMethodMutMeta(meta),
				//}
			}
		}
	}

	pub fn register(&self, value: &mut ValueManager, run: TokenStream) -> TokenStream {
		let names = &value.closure_names;
		let types = &value.closure_types;
		let args = quote!((#names): (#types));
		match self {
			//MethodBinding::AddMethod(value) => {
			//	quote!(methods.add_method(#value, |lua, this, #args| #run);)
			//}
			//MethodBinding::AddMethodMeta(value) => {
			//	quote!(methods.add_meta_method(#value, |lua, this, #args| #run);)
			//}
			//MethodBinding::AddMethodMut(value) => {
			//	quote!(methods.add_method_mut(#value, |lua, this, #args| #run);)
			//}
			//MethodBinding::AddMethodMutMeta(value) => {
			//	quote!(methods.add_meta_method_mut(#value, |lua, this, #args| #run);)
			//}
			MethodBinding::AddFunctionMut(value) => {
				quote!(methods.add_function_mut(#value, |lua, #args| #run);)
			}
			MethodBinding::AddFunctionMutMeta(value) => {
				quote!(methods.add_meta_function_mut(#value, |lua, #args| #run);)
			}
		}
	}
}

#[derive(Copy, Clone)]
pub enum MetaMethod {
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Pow,
	Unm,
	IDiv,
	BAnd,
	BOr,
	BXor,
	BNot,
	Shl,
	Shr,
	Concat,
	Len,
	Eq,
	Lt,
	Le,
	Index,
	NewIndex,
	Call,
	ToString,
	Pairs,
	IPairs,
	Close,
}

impl MetaMethod {
	pub fn new(lua_method: &str) -> Option<MetaMethod> {
		use MetaMethod::*;
		match lua_method {
			"__add" => Some(Add),
			"__sub" => Some(Sub),
			"__mul" => Some(Mul),
			"__div" => Some(Div),
			"__mod" => Some(Mod),
			"__pow" => Some(Pow),
			"__unm" => Some(Unm),
			"__idiv" => Some(IDiv),
			"__band" => Some(BAnd),
			"__bor" => Some(BOr),
			"__bxor" => Some(BXor),
			"__bnot" => Some(BNot),
			"__shl" => Some(Shl),
			"__shr" => Some(Shr),
			"__concat" => Some(Concat),
			"__len" => Some(Len),
			"__eq" => Some(Eq),
			"__lt" => Some(Lt),
			"__le" => Some(Le),
			"__index" => Some(Index),
			"__newindex" => Some(NewIndex),
			"__call" => Some(Call),
			"__tostring" => Some(ToString),
			"__pairs" => Some(Pairs),
			"__ipairs" => Some(IPairs),
			"__close" => Some(Close),
			_ => None,
		}
	}
}

impl ToTokens for MetaMethod {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(match self {
			MetaMethod::Add => quote!(apollo::MetaMethod::Add),
			MetaMethod::Sub => quote!(apollo::MetaMethod::Sub),
			MetaMethod::Mul => quote!(apollo::MetaMethod::Mul),
			MetaMethod::Div => quote!(apollo::MetaMethod::Div),
			MetaMethod::Mod => quote!(apollo::MetaMethod::Mod),
			MetaMethod::Pow => quote!(apollo::MetaMethod::Pow),
			MetaMethod::Unm => quote!(apollo::MetaMethod::Unm),
			MetaMethod::IDiv => quote!(apollo::MetaMethod::IDiv),
			MetaMethod::BAnd => quote!(apollo::MetaMethod::BAnd),
			MetaMethod::BOr => quote!(apollo::MetaMethod::BOr),
			MetaMethod::BXor => quote!(apollo::MetaMethod::BXor),
			MetaMethod::BNot => quote!(apollo::MetaMethod::BNot),
			MetaMethod::Shl => quote!(apollo::MetaMethod::Shl),
			MetaMethod::Shr => quote!(apollo::MetaMethod::Shr),
			MetaMethod::Concat => quote!(apollo::MetaMethod::Concat),
			MetaMethod::Len => quote!(apollo::MetaMethod::Len),
			MetaMethod::Eq => quote!(apollo::MetaMethod::Eq),
			MetaMethod::Lt => quote!(apollo::MetaMethod::Lt),
			MetaMethod::Le => quote!(apollo::MetaMethod::Le),
			MetaMethod::Index => quote!(apollo::MetaMethod::Index),
			MetaMethod::NewIndex => quote!(apollo::MetaMethod::NewIndex),
			MetaMethod::Call => quote!(apollo::MetaMethod::Call),
			MetaMethod::ToString => quote!(apollo::MetaMethod::ToString),
			MetaMethod::Pairs => quote!(apollo::MetaMethod::Pairs),
			MetaMethod::IPairs => quote!(apollo::MetaMethod::IPairs),
			MetaMethod::Close => quote!(apollo::MetaMethod::Close),
		})
	}
}
