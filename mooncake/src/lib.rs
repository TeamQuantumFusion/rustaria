use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, AttributeArgs, FnArg, ItemFn, Meta, NestedMeta, PatType,
};

#[proc_macro_attribute]
pub fn mooncake(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let enable_lua = args.into_iter().any(|m| {
        if let NestedMeta::Meta(Meta::Path(p)) = m {
            p.is_ident("lua")
        } else {
            false
        }
    });

    let mut input = parse_macro_input!(item as ItemFn);
    let mut inputs = input.sig.inputs;
    // ignore any parameter containing Lua; we'll consider them in a sec

    if inputs.is_empty() {
        inputs.push(parse_quote!(_: ()));
    } else {
        let (pats, tys): (Vec<_>, Vec<_>) = inputs
            .iter_mut()
            .map(|i| match i {
                FnArg::Receiver(_) => panic!("methods are currently unsupported"),
                FnArg::Typed(PatType { pat, ty, .. }) => (pat, ty),
            })
            .unzip();
        inputs = parse_quote!((#(#pats),*): (#(#tys),*));
    }

    let lua_param = if enable_lua {
        parse_quote!(lua: &mlua::Lua)
    } else {
        parse_quote!(_lua: &mlua::Lua)
    };
    inputs.insert(0, lua_param);
    input.sig.inputs = inputs;

    let items = quote! {
        #input
    };
    TokenStream::from(items)
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/*.rs");
}
