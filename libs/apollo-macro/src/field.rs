use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Signature, Token};
use crate::{FieldAttr, MethodAttr};
use crate::attr::FieldBindKind;
use crate::values::ValueManager;

pub(crate) fn bind_field(sig: &Signature, attr: FieldAttr) -> TokenStream {
	let mut manager = ValueManager::new(true);

	manager.add_local(Ident::new("v0", Span::call_site()), None);

	// Scan inputs
	let inputs: Punctuated<TokenStream, Token!(,)> = sig
		.inputs
		.iter()
		.map(|input| manager.unwrap_input(input))
		.collect();


	// Create the "run" this is what actually gets run in the closure
	let target_name = &sig.ident;

	let wrapped_return = manager.wrap_return(&sig.output, quote!(Self::#target_name(#inputs)));
	let run = quote!(unsafe { let v0 = this.borrow::<apollo::LuaWeak<Self>>()?; #wrapped_return });

	// Determine what kind of binding we are making
	let lua_name = attr.lua_name.unwrap_or_else(|| sig.ident.to_string());
	let names = &manager.closure_names;
	let types = &manager.closure_types;
	let args = quote!((#names): (#types));

	match attr.kind {
		FieldBindKind::Getter => {
			if !names.is_empty() {
				panic!("Cannot have arguments on a field getter")
			}
			quote!(fields.add_field_function_get(#lua_name, |lua, this| #run);)
		}
		FieldBindKind::Setter => {
			quote!(fields.add_field_function_set(#lua_name, |lua, this, #args| #run);)
		}
	}
}