use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::LitStr;

pub struct ItemsBuilder {
	stream: TokenStream,
	field: usize,
}
impl ItemsBuilder {
	pub fn new() -> ItemsBuilder {
		ItemsBuilder {
			stream: Default::default(),
			field: 1,
		}
	}

	pub fn push_field(&mut self, name: Option<&Ident>) {
		match name {
			None => {
				let number = self.field;
				self.stream.append_all(quote!(
					#number: table.get(#number)?,
				))
			}
			Some(name) => {
				let lua_name = LitStr::new(&name.to_string(), name.span());
				self.stream.append_all(quote!(
					#name: anyways::ext::AuditExt::wrap_err_with(table.get(#lua_name), || format!("Failed to get \"{}\"", #lua_name))?,
				))
			}
		}
		self.field += 1;
	}
}

impl ToTokens for ItemsBuilder {
	fn to_tokens(&self, tokens: &mut TokenStream) { self.stream.to_tokens(tokens) }
}
