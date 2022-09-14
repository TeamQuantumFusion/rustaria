use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
	punctuated::Punctuated, FnArg, GenericArgument, Lifetime, PathArguments, ReturnType, Token,
	Type,
};

#[derive(Copy, Clone)]
pub enum Receiver {
	Mutable,
	Immutable,
}

pub struct ValueManager {
	pub closure_names: Punctuated<Ident, Token!(,)>,
	pub closure_types: Punctuated<TokenStream, Token!(,)>,
	pub parameter_mappers: TokenStream,
	pub receiver: Option<Receiver>,
	lifetimes: HashMap<Lifetime, Ident>,
	fields: bool,
	var: usize,
}

impl ValueManager {
	pub fn new(fields: bool) -> ValueManager {
		ValueManager {
			closure_names: Default::default(),
			closure_types: Default::default(),
			parameter_mappers: Default::default(),
			receiver: None,
			fields,
			lifetimes: Default::default(),
			var: 0,
		}
	}

	pub fn unwrap_input(&mut self, input: &FnArg) -> TokenStream {
		match input {
			FnArg::Receiver(receiver) => {
				let mutable = receiver.mutability.is_some();
				let value = if self.fields {
					if mutable {
						quote!(&mut *(v0.value as *const Self as *mut Self))
					} else {
						quote!(&*v0.value)
					}
				} else {
					self.new_value(
						ParameterKind::UserData {
							mutable,
							name: "self".to_string(),
						},
						quote!(Self),
						receiver.lifetime().cloned(),
					)
					.to_token_stream()
				};

				if mutable {
					self.receiver = Some(Receiver::Mutable);
				} else {
					self.receiver = Some(Receiver::Immutable);
				}

				//if !self.fields {
				//	if mutable {
				//		quote!(#value.get_mut("self")?)
				//	} else {
				//		quote!(#value.get("self")?)
				//	}
				//} else {
				//quote!(#value)
				value
				//}
			}
			FnArg::Typed(ty) => {
				let name = ty.to_token_stream().to_string();
				match LuaValue::parse(*ty.ty.clone()) {
					LuaValue::Option { original: ty, .. }
					| LuaValue::LuaResult { original: ty, .. }
					| LuaValue::Normal(ty) => self
						.new_value(ParameterKind::ToLua, quote!(#ty), None)
						.to_token_stream(),
					LuaValue::Reference {
						inner,
						mutable,
						lifetime,
						..
					} => {
						let local = self
							.new_value(
								ParameterKind::UserData { mutable, name },
								quote!(#inner),
								lifetime,
							)
							.to_token_stream();

						local
					}
					LuaValue::Lua => {
						quote!(lua)
					}
				}
			}
		}
	}

	// to something we are returning in the closure
	pub fn wrap_return(&self, ret: &ReturnType, invoke: TokenStream) -> TokenStream {
		match ret {
			ReturnType::Default => {
				quote!(#invoke; Ok(()))
			}
			ReturnType::Type(_, ty) => {
				let value = LuaValue::parse(*ty.clone());
				let ret = self.wrap_return_internal(value, invoke);
				quote!(Ok(#ret))
			}
		}
	}

	fn wrap_return_internal(&self, value: LuaValue, invoke: TokenStream) -> TokenStream {
		match value {
			LuaValue::Lua | LuaValue::Normal(_) => {
				quote!(#invoke)
			}
			LuaValue::LuaResult { inner, .. } => {
				self.wrap_return_internal(*inner, quote!(#invoke?))
			}
			LuaValue::Reference {
				mutable, lifetime, ..
			} => {
				let var = self
					.lifetimes
					.get(&lifetime.unwrap_or_else(|| Lifetime::new("'v0", Span::call_site())))
					.expect("Lifetime does not exist");
				quote!(#var.map_inner(#invoke, #mutable)?)
			}
			LuaValue::Option { inner, .. } => {
				let stream = self.wrap_return_internal(*inner, quote!(value));

				quote!(match #invoke {
					Some(value) => Some(#stream),
					None => None,
				})
			}
		}
	}

	pub fn add_local(&mut self, ident: Ident, lifetime: Option<Lifetime>) {
		self.add_lifetime(ident, lifetime);
		self.var += 1;
	}

	fn new_value(
		&mut self,
		kind: ParameterKind,
		ty: TokenStream,
		lifetime: Option<Lifetime>,
	) -> TokenStream {
		let span = Span::call_site();
		let ident = Ident::new(&format!("v{}", self.var), span);
		self.add_lifetime(ident.clone(), lifetime);
		self.closure_names.push(ident.clone());
		self.closure_types.push(if kind.is_userdata() {
			quote!(apollo::UserDataCell<#ty>)
		} else {
			ty.clone()
		});

		self.var += 1;
		if let ParameterKind::UserData { mutable, name } = kind {
			self.parameter_mappers.append_all({
				if mutable {
					quote!(let mut #ident = #ident.get_mut(#name)?;)
				} else {
					quote!(let #ident = #ident.get(#name)?;)
				}
			});

			if mutable {
				quote!(&mut *(#ident.value as *const #ty as *mut #ty))
			} else {
				quote!(&*#ident.value)
			}
		} else {
			quote!(#ident)
		}
	}

	fn add_lifetime(&mut self, ident: Ident, lifetime: Option<Lifetime>) {
		let lifetime = match lifetime {
			None => Lifetime::new(&format!("'v{}", self.var), Span::call_site()),
			Some(lifetime) => lifetime,
		};

		self.lifetimes.insert(lifetime, ident);
	}
}

// (value) - use FromLua / IntoLua
// &value - LuaRef<value>
// &mut value - LuaMutRef<value>
// LuaResult<value> - LuaResult<value>
pub enum LuaValue {
	Reference {
		inner: Type,
		mutable: bool,
		lifetime: Option<Lifetime>,
	},
	LuaResult {
		inner: Box<LuaValue>,
		original: Type,
	},
	Option {
		inner: Box<LuaValue>,
		original: Type,
	},
	Normal(Type),
	Lua,
}

impl LuaValue {
	pub fn parse(original: Type) -> LuaValue {
		match original.clone() {
			Type::Paren(ty) => LuaValue::Normal(*ty.elem),
			Type::Path(path) => {
				{
					let path = path.path.segments.last().unwrap();
					if path.ident == "LuaResult" || path.ident == "Result" {
						if let PathArguments::AngleBracketed(brackets) = &path.arguments {
							if let GenericArgument::Type(ty) =
								brackets.args.first().expect("Result must have args")
							{
								return LuaValue::LuaResult {
									inner: Box::new(LuaValue::parse(ty.clone())),
									original,
								};
							}
							panic!("The result args must be a type")
						} else {
							panic!("The result args must be in <>")
						}
					} else if path.ident == "Option" {
						if let PathArguments::AngleBracketed(brackets) = &path.arguments {
							if let GenericArgument::Type(ty) =
								brackets.args.first().expect("Result must have args")
							{
								return LuaValue::Option {
									inner: Box::new(LuaValue::parse(ty.clone())),
									original,
								};
							}
							panic!("The result args must be a type")
						} else {
							panic!("The result args must be in <>")
						}
					}
				}

				LuaValue::Normal(Type::Path(path))
			}
			Type::Reference(reference) => {
				if let Type::Path(ty) = &*reference.elem {
					if ty.path.segments.last().unwrap().ident == "Lua" {
						return LuaValue::Lua;
					}
				}

				LuaValue::Reference {
					lifetime: reference.lifetime.clone(),
					inner: *reference.elem,
					mutable: reference.mutability.is_some(),
				}
			}
			ty => LuaValue::Normal(ty),
		}
	}
}

pub enum ParameterKind {
	ToLua,
	UserData { mutable: bool, name: String },
}

impl ParameterKind {
	pub fn is_userdata(&self) -> bool {
		match self {
			ParameterKind::ToLua => false,
			ParameterKind::UserData { .. } => true,
		}
	}
}
