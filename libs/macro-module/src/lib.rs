use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, ItemFn, Stmt, Token};

#[proc_macro_attribute]
pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut item = parse_macro_input!(item as ItemFn);
	let attr = parse_macro_input!(attr as ItemAttr);

	// Remove first item.

	item.sig.inputs = item
		.sig
		.inputs
		.into_iter()
		.enumerate()
		.filter(|(id, v)| *id != 0)
		.map(|(_, v)| v)
		.collect();

	let parent = attr.parent;
	let self_field = attr.self_field;
	let stream: TokenStream = quote!(
		let this = &mut #parent.#self_field;
	)
	.into();

	item.block
		.stmts
		.insert(0, parse_macro_input!(stream as Stmt));

	item.into_token_stream().into()
}

struct ItemAttr {
	pub parent: Ident,
	pub thing: Token!(.),
	pub self_field: Ident,
}

impl Parse for ItemAttr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let parent = input.parse().expect("parent");
		let thing = input.parse().expect("thing");
		let field = input.parse().expect("field");
		Ok(ItemAttr {
			parent: parent,
			thing,
			self_field: field,
		})
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
