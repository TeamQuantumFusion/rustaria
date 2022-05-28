use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{FnArg, ItemFn};

enum MethodType {
	AddMethod,
	AddMethodMut,
	AddFunctionMut,
	AddMetaMethod(TokenStream),
	AddMetaMethodMut(TokenStream),
	AddMetaFunctionMut(TokenStream),
}

impl MethodType {
	pub fn new(maybe_self_arg: Option<&FnArg>, name: &str) -> MethodType {
		match maybe_self_arg {
			Some(maybe_self_arg) => {
				match maybe_self_arg {
					// self
					FnArg::Receiver(this) => {
						// &mut self
						if this.mutability.is_some() {
							match get_metamethod(name) {
								Some(stream) => MethodType::AddMetaMethodMut(stream),
								None => MethodType::AddMethodMut,
							}
						} else {
							// &self
							match get_metamethod(name) {
								Some(stream) => MethodType::AddMetaMethod(stream),
								None => MethodType::AddMethod,
							}
						}
					}
					// anything else
					FnArg::Typed(_) => Self::function_type(name),
				}
			}
			// no args
			None => Self::function_type(name),
		}
	}

	fn function_type(name: &str) -> MethodType {
		match get_metamethod(name) {
			Some(stream) => MethodType::AddMetaFunctionMut(stream),
			None => MethodType::AddFunctionMut,
		}
	}
}

// (&self) add_method
// (&self) add_meta_method
// (&mut self) add_method_mut
// (&mut self) add_meta_method_mut
// () add_function_mut
// () add_meta_function_mut
pub fn get_method_register(item: &ItemFn, expand_args: TokenStream, invoke_target: TokenStream) -> TokenStream {
	// Check if its a method or a function
	let original_name = item.sig.ident.to_string();
	let target_name = original_name.trim_end_matches("_lua");
	let method = MethodType::new(item.sig.inputs.first(), target_name);

	let method_name = Literal::string(target_name);

	match method {
		MethodType::AddMethod => {
			quote!(methods.add_method(#method_name, |lua, value, #expand_args| {
				#invoke_target
			}))
		}
		MethodType::AddMethodMut => {
			quote!(methods.add_method_mut(#method_name, |lua, value, #expand_args| {
				#invoke_target
			}))
		}
		MethodType::AddFunctionMut => {
			quote!(methods.add_function_mut(#method_name, |lua, #expand_args| {
				#invoke_target
			}))
		}
		MethodType::AddMetaMethod(meta) => {
			quote!(methods.add_meta_method(#meta, |lua, value, #expand_args| {
				#invoke_target
			}))
		}
		MethodType::AddMetaMethodMut(meta) => {
			quote!(methods.add_meta_method_mut(#meta, |lua, value, #expand_args| {
				#invoke_target
			}))
		}
		MethodType::AddMetaFunctionMut(meta) => {
			quote!(methods.add_meta_function_mut(#meta, |lua, #expand_args| {
				#invoke_target
			}))
		}
	}
}

pub fn get_metamethod(name: &str) -> Option<TokenStream> {
	match name {
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
